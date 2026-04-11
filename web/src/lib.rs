use log::{error, info};
use std::rc::Rc;
use pixels::{PixelsBuilder, SurfaceTexture};
use wasm_bindgen::prelude::*;
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
#[cfg(target_arch = "wasm32")]
use winit::platform::web::WindowExtWebSys;
use winit::window::Window;
use gameboy_core::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[wasm_bindgen]
pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Info).expect("error initializing logger");
    wasm_bindgen_futures::spawn_local(run())
}

enum Color {
    Red,
    Green,
    Blue,
}

struct World {
    color: Color,
}

impl World {
    fn draw(&self, frame: &mut [u8]) {
        for pixel in frame.chunks_exact_mut(4) {
            let color = match self.color {
                Color::Red => [0xff, 0x00, 0x00, 0xff],
                Color::Green => [0x00, 0xff, 0x00, 0xff],
                Color::Blue => [0x00, 0x00, 0xff, 0xff],
            };
            pixel.copy_from_slice(&color);
        }
    }
}

async fn run() {
    let mut world = World { color: Color::Red };
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

    world.draw(pixels.frame_mut());
    pixels.render().expect("Klarte ikke å rendre pixels");

    let res = event_loop.run(|event, elwt| {
        match event {
            Event::AboutToWait => {
                window.request_redraw();
            }
            Event::WindowEvent { event: WindowEvent::KeyboardInput { .. }, .. } => {
                let new_color = match world.color {
                    Color::Red => Color::Green,
                    Color::Green => Color::Blue,
                    Color::Blue => Color::Red,
                };
                world.color = new_color;
                world.draw(pixels.frame_mut());
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
