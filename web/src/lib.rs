mod local_storage_battery_save;

use log::{error, info};
use std::rc::Rc;
use pixels::{PixelsBuilder, SurfaceTexture};
use wasm_bindgen::prelude::*;
use web_sys::{AudioBuffer, AudioBufferOptions, AudioContext, AudioContextOptions, AudioContextState};
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::keyboard::{Key, NamedKey};
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;
use winit::window::Window;

use gameboy_core::{AUDIO_SAMPLE_RATE, CPU_CYCLES_PER_FRAME, NANOSECONDS_PER_FRAME, SCREEN_HEIGHT, SCREEN_WIDTH};
use gameboy_core::battery_save::BatterySave;
use gameboy_core::frame_buffer::FrameBuffer;
use gameboy_core::game_boy::GameBoy;
use gameboy_core::joypad::JoypadKey;
use crate::local_storage_battery_save::LocalStorageBatterySave;

#[wasm_bindgen]
pub fn main(game_title: String, rom_data: Vec<u8>) {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Info).expect("error initializing logger");
    wasm_bindgen_futures::spawn_local(run(game_title, rom_data))
}

async fn run(game_title: String, rom_data: Vec<u8>) {
    let local_storage_battery_save = LocalStorageBatterySave::new(&game_title)
        .map(|battery_save| Box::new(battery_save) as Box<dyn BatterySave>);

    let mut game_boy = match GameBoy::new(rom_data, local_storage_battery_save) {
        Ok(game_boy) => game_boy,
        Err(error_str) => panic!("{}", error_str),
    };

    let event_loop = EventLoop::new().unwrap();
    let scale = 3;
    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH as f64 * scale as f64, SCREEN_HEIGHT as f64 * scale as f64);
        event_loop.create_window(
            Window::default_attributes()
                .with_title("Game Boy Web")
                .with_inner_size(size)
                .with_min_inner_size(size)
        )
            .unwrap()
    };

    let window = Rc::new(window);

    web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.get_element_by_id("screen"))
        .and_then(|container| {
            container
                .append_child(&web_sys::Element::from(window.canvas().unwrap()))
                .ok()
        })
        .expect("Kunne ikke legge canvas til DOM");

    let mut pixels = {
        let surface_width = SCREEN_WIDTH * scale;
        let surface_height = SCREEN_HEIGHT * scale;
        let surface_texture = SurfaceTexture::new(surface_width, surface_height, window.clone());
        let builder = PixelsBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)
            .texture_format(pixels::wgpu::TextureFormat::Rgba8Unorm)
            .surface_texture_format(pixels::wgpu::TextureFormat::Rgba8Unorm);

        builder.build_async().await.expect("Pixels error")
    };

    info!("Pixels opprettet");

    let mut cpu_cycles: u32 = 0;

    let frames_between_saves = 120;
    let mut frames_since_save = 0;

    let audio_context_options = AudioContextOptions::new();
    audio_context_options.set_sample_rate(AUDIO_SAMPLE_RATE as f32);
    let audio_context = AudioContext::new_with_context_options(&audio_context_options).unwrap();
    let audio_sample_rate = audio_context.sample_rate();
    let mut next_start_time = audio_context.current_time() + 0.05;

    let performance = web_sys::window()
        .and_then(|w| w.performance())
        .expect("performance.now() ikke tilgjengelig");
    let frame_duration_ms = NANOSECONDS_PER_FRAME as f64 / 1_000_000.0;
    let mut next_frame_ms = performance.now() + frame_duration_ms;

    let res = event_loop.run(|event, elwt| {
        use winit::event::ElementState::{Pressed, Released};
        use winit::event::{Event, WindowEvent};

        match event {
            Event::AboutToWait => {
                let now = performance.now();
                if now < next_frame_ms {
                    window.request_redraw();
                    return;
                }
                next_frame_ms += frame_duration_ms;
                if next_frame_ms < now {
                    next_frame_ms = now + frame_duration_ms;
                }

                while cpu_cycles < CPU_CYCLES_PER_FRAME {
                    cpu_cycles += game_boy.emulate();
                }
                cpu_cycles -= CPU_CYCLES_PER_FRAME;

                if let Some(data) = game_boy.updated_frame_buffer() {
                    data.write_to_rbga_buffer(pixels.frame_mut());
                    if let Err(err) = pixels.render() {
                        error!("Feil under tegning til skjerm!");
                        elwt.exit();
                    }
                }

                let sound_data = game_boy.sound_buffer();
                if audio_context.state() == AudioContextState::Running {
                    let now = audio_context.current_time();
                    if next_start_time < now {
                        next_start_time = now + 0.05;
                    }
                    let number_of_audio_samples = sound_data.len() as u32;
                    let buffer = AudioBuffer::new(
                        &AudioBufferOptions::new(number_of_audio_samples, audio_sample_rate)
                    ).unwrap();
                    buffer.copy_to_channel(&sound_data, 0).unwrap();

                    let source = audio_context.create_buffer_source().unwrap();
                    source.set_buffer(Some(&buffer));
                    source.connect_with_audio_node(&audio_context.destination()).unwrap();
                    source.start_with_when(next_start_time).unwrap();

                    next_start_time += number_of_audio_samples as f64 / audio_sample_rate as f64;
                }

                frames_since_save += 1;
                if frames_since_save >= frames_between_saves {
                    game_boy.manual_save();
                    frames_since_save = 0;
                }

                window.request_redraw();
            }
            Event::WindowEvent { event: WindowEvent::KeyboardInput { event: key_event, .. }, .. } => {
                if audio_context.state() == AudioContextState::Suspended {
                    let _ = audio_context.resume();
                    next_start_time = audio_context.current_time() + 0.05;
                }
                match (key_event.state, key_event.logical_key.as_ref()) {
                    (Pressed, winit_key) => {
                        if let Some(key) = winit_to_joypad(winit_key) {
                            game_boy.key_down(key);
                        }
                    }
                    (Released, winit_key) => {
                        if let Some(key) = winit_to_joypad(winit_key) {
                            game_boy.key_up(key);
                        }
                    }
                }
                window.request_redraw();
            }
            Event::WindowEvent { event: WindowEvent::RedrawRequested, .. } => {
                if let Err(err) = pixels.render() {
                    error!("pixels.render() failed: {}", err);
                    elwt.exit();
                    return;
                }
                window.request_redraw();
            }
            _ => {}
        }
    });
    res.unwrap();
}

fn winit_to_joypad(key: Key<&str>) -> Option<JoypadKey> {
    match key {
        Key::Character("Z" | "z") => Some(JoypadKey::A),
        Key::Character("X" | "x") => Some(JoypadKey::B),
        Key::Named(NamedKey::ArrowUp) => Some(JoypadKey::Up),
        Key::Named(NamedKey::ArrowDown) => Some(JoypadKey::Down),
        Key::Named(NamedKey::ArrowLeft) => Some(JoypadKey::Left),
        Key::Named(NamedKey::ArrowRight) => Some(JoypadKey::Right),
        Key::Named(NamedKey::Backspace) => Some(JoypadKey::Select),
        Key::Named(NamedKey::Enter) => Some(JoypadKey::Start),
        _ => None,
    }
}
