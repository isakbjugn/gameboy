use log::{error, info};
use std::rc::Rc;
use pixels::{PixelsBuilder, SurfaceTexture};
use wasm_bindgen::prelude::*;
use winit::dpi::LogicalSize;
use winit::event_loop::EventLoop;
use winit::keyboard::{Key, NamedKey};
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;
use winit::window::Window;

use gameboy_core::{SCREEN_HEIGHT, SCREEN_WIDTH};
use gameboy_core::frame_buffer::FrameBuffer;
use gameboy_core::game_boy::GameBoy;
use gameboy_core::joypad::JoypadKey;

#[wasm_bindgen]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Info).expect("error initializing logger");
    wasm_bindgen_futures::spawn_local(run())
}

async fn run() {
    let cartridge = include_bytes!("../../roms/links_awakening.gb");
    let mut game_boy = match GameBoy::new(Vec::from(cartridge), None) {
        Ok(game_boy) => game_boy,
        Err(error_str) => panic!("{}", error_str),
    };

    let event_loop = EventLoop::new().unwrap();
    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);
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
        let surface_texture = SurfaceTexture::new(SCREEN_WIDTH, SCREEN_HEIGHT, window.clone());
        let builder = PixelsBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)
            .texture_format(pixels::wgpu::TextureFormat::Rgba8Unorm)
            .surface_texture_format(pixels::wgpu::TextureFormat::Bgra8Unorm);

        builder.build_async().await.expect("Pixels error")
    };

    info!("Pixels opprettet");

    let cpu_cycles_per_frame = (4194204f64 / 1000.0 * 16.0).round() as u32;
    let mut cpu_cycles: u32 = 0;

    let res = event_loop.run(|event, elwt| {
        use winit::event::ElementState::{Pressed, Released};
        use winit::event::{Event, WindowEvent};


        match event {
            Event::AboutToWait => {
                while cpu_cycles < cpu_cycles_per_frame {
                    cpu_cycles += game_boy.emulate();
                }
                cpu_cycles -= cpu_cycles_per_frame;

                if let Some(data) = game_boy.updated_frame_buffer() {
                    data.write_to_rbga_buffer(pixels.frame_mut());
                    if let Err(err) = pixels.render() {
                        error!("Feil under tegning til skjerm!");
                        elwt.exit();
                    }
                }

                window.request_redraw();
            }
            Event::WindowEvent { event: WindowEvent::KeyboardInput { event: key_event, .. }, .. } => {
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
