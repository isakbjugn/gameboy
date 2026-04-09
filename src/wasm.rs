use log::info;
use pixels::{PixelsBuilder, SurfaceTexture};
use wasm_bindgen::prelude::*;
use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::platform::web::{EventLoopExtWebSys, WindowBuilderExtWebSys};
use winit::window::WindowBuilder;

use crate::frame_buffer::FrameBuffer;
use crate::game_boy::GameBoy;
use crate::joypad::JoypadKey;

const SCREEN_WIDTH: u32 = 160;
const SCREEN_HEIGHT: u32 = 144;

fn js_key_to_joypad(key: Key<&str>) -> Option<JoypadKey> {
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

#[wasm_bindgen]
pub fn start_emulator(rom_data: &[u8]) {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Info).ok();

    let rom = rom_data.to_vec();

    wasm_bindgen_futures::spawn_local(async move {
        let mut game_boy = *GameBoy::from_bytes(rom)
            .expect("Kunne ikke laste ROM");

        let event_loop = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);

        let canvas = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.get_element_by_id("canvas"))
            .and_then(|e| e.dyn_into::<web_sys::HtmlCanvasElement>().ok())
            .expect("Fant ikke canvas-element med id 'canvas'");

        let canvas_width = canvas.width();
        let canvas_height = canvas.height();
        info!("Canvas-størrelse: {}x{}", canvas_width, canvas_height);

        let window: &'static winit::window::Window = Box::leak(Box::new(
            WindowBuilder::new()
                .with_canvas(Some(canvas))
                .build(&event_loop)
                .unwrap()
        ));

        // Sett vindusstørrelse eksplisitt etter bygging
        let _ = window.request_inner_size(PhysicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT));

        let window_size = window.inner_size();
        info!("Vindusstørrelse: {}x{}", window_size.width, window_size.height);

        // Bruk kjente dimensjoner for surface, ikke window.inner_size() som kan være 0
        let surface_width = if window_size.width > 0 { window_size.width } else { SCREEN_WIDTH };
        let surface_height = if window_size.height > 0 { window_size.height } else { SCREEN_HEIGHT };
        info!("Surface-størrelse: {}x{}", surface_width, surface_height);

        let surface_texture = SurfaceTexture::new(surface_width, surface_height, window);
        let mut pixels = PixelsBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)
            .wgpu_backend(pixels::wgpu::Backends::GL)
            .build_async()
            .await
            .expect("Kunne ikke opprette Pixels");

        info!("Pixels opprettet, starter emulering");

        let cpu_cycles_per_frame = (4194304f64 / 60.0).round() as u32;
        let mut cpu_cycles: u32 = 0;

        event_loop.spawn(move |event, _elwt| {
            match &event {
                Event::AboutToWait => {
                    while cpu_cycles < cpu_cycles_per_frame {
                        cpu_cycles += game_boy.emulate();
                    }
                    cpu_cycles -= cpu_cycles_per_frame;

                    if let Some(data) = game_boy.updated_frame_buffer() {
                        data.write_to_rbga_buffer(pixels.frame_mut());
                        let _ = pixels.render();
                    }

                    window.request_redraw();
                }
                Event::WindowEvent { event: WindowEvent::KeyboardInput { event: key_event, .. }, .. } => {
                    match (key_event.state, key_event.logical_key.as_ref()) {
                        (ElementState::Pressed, key) => {
                            if let Some(joypad_key) = js_key_to_joypad(key) {
                                game_boy.key_down(joypad_key);
                            }
                        }
                        (ElementState::Released, key) => {
                            if let Some(joypad_key) = js_key_to_joypad(key) {
                                game_boy.key_up(joypad_key);
                            }
                        }
                    }
                }
                _ => {}
            }
        });
    });
}
