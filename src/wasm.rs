use log::info;
use pixels::{PixelsBuilder, SurfaceTexture};
use std::rc::Rc;
use std::sync::Arc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use winit::dpi::LogicalSize;
use winit::event::{ElementState, Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::keyboard::{Key, NamedKey};
use winit::platform::web::WindowExtWebSys;
use winit::window::Window;

use crate::frame_buffer::FrameBuffer;
use crate::game_boy::GameBoy;
use crate::joypad::JoypadKey;

const SCREEN_WIDTH: u32 = 160;
const SCREEN_HEIGHT: u32 = 144;

fn get_window_size() -> LogicalSize<f64> {
    let client_window = web_sys::window().unwrap();
    LogicalSize::new(
        client_window.inner_width().unwrap().as_f64().unwrap(),
        client_window.inner_height().unwrap().as_f64().unwrap(),
    )
}

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

        #[allow(deprecated)]
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("Game Boy Emulator")
                        .with_inner_size(LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64))
                        .with_min_inner_size(LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64)),
                )
                .unwrap(),
        );
        let window = Rc::new(window);

        // Legg winit-canvas til DOM
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.get_element_by_id("screen"))
            .and_then(|container| {
                container
                    .append_child(&web_sys::Element::from(window.canvas().unwrap()))
                    .ok()
            })
            .expect("Kunne ikke legge canvas til DOM");

        // Lytt på resize-events fra nettleseren
        let resize_closure = wasm_bindgen::closure::Closure::wrap(Box::new({
            let window = Rc::clone(&window);
            move |_e: web_sys::Event| {
                let _ = window.request_inner_size(get_window_size());
            }
        }) as Box<dyn FnMut(_)>);
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref())
            .unwrap();
        resize_closure.forget();

        // Sett initial størrelse til nettleservinduet
        let _ = window.request_inner_size(get_window_size());

        let window_size = get_window_size().to_physical::<u32>(window.scale_factor());
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, window.clone());
        let texture_format = pixels::wgpu::TextureFormat::Rgba8Unorm;
        let mut pixels = PixelsBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)
            .texture_format(texture_format)
            .surface_texture_format(texture_format)
            .build_async()
            .await
            .expect("Kunne ikke opprette Pixels");

        info!("Pixels opprettet, starter emulering");

        const CYCLES_PER_MS: f64 = 4194304.0 / 1000.0;
        let performance = web_sys::window().unwrap().performance().unwrap();
        let mut last_timestamp = performance.now();
        let mut cycle_debt: f64 = 0.0;

        #[allow(deprecated)]
        event_loop.run(move |event, elwt| {
            match event {
                Event::AboutToWait => {
                    let now = performance.now();
                    let elapsed_ms = now - last_timestamp;
                    last_timestamp = now;

                    // Begrens til maks 33ms for å unngå spiral ved fryser/tab-bytte
                    let capped_ms = elapsed_ms.min(33.0);
                    cycle_debt += capped_ms * CYCLES_PER_MS;

                    let cycles_to_run = cycle_debt as u32;
                    cycle_debt -= cycles_to_run as f64;

                    let mut cycles_run: u32 = 0;
                    while cycles_run < cycles_to_run {
                        cycles_run += game_boy.emulate();
                    }

                    if let Some(data) = game_boy.updated_frame_buffer() {
                        data.write_to_rbga_buffer(pixels.frame_mut());
                        let _ = pixels.render();
                    }

                    window.request_redraw();
                }
                Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                    if size.width > 0 && size.height > 0 {
                        if let Err(err) = pixels.resize_surface(size.width, size.height) {
                            log::error!("pixels.resize_surface() feilet: {err}");
                            elwt.exit();
                        }
                    }
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
        }).unwrap();
    });
}
