use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::KeyCode;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::game_boy::GameBoy;

mod cpu;
mod flags_register;
mod memory_bus;
mod registers;
mod mbc;
mod ppu;
mod joypad;
mod bootrom;
mod timer;
mod game_boy;
mod cartridge;

const SCREEN_WIDTH: u32 = 160;
const SCREEN_HEIGHT: u32 = 144;

fn main() -> Result<(), Error> {
    let matches = clap::Command::new("gameboy")
        .version("0.1")
        .author("Isak Kyrre Lichtwarck Bjugn")
        .about("A Gameboy emulator written in Rust")
        .arg(clap::Arg::new("cartridge_name")
            .help("Sets the ROM file to load")
            .required(true))
        .get_matches();
    let cartridge_name = matches.get_one::<String>("cartridge_name").unwrap();

    let game_boy = match GameBoy::new(cartridge_name) {
        Ok(game_boy) => game_boy,
        Err(error_str) => panic!("{}", error_str),
    };

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);
        WindowBuilder::new()
            .with_title(game_boy.title())
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)?
    };

    let res = event_loop.run(|event, elwt| {
        if let Event::WindowEvent {
            event: WindowEvent::RedrawRequested,
            ..
        } = event
        {
            if let Err(err) = pixels.render() {
                elwt.exit();
                return;
            }
        }

        if input.update(&event) {
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                elwt.exit();
                return;
            }

            window.request_redraw();
        }
    });
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}
