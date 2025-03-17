use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;
use pixels::{Error, Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::game_boy::GameBoy;
use crate::joypad::JoypadKey;

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

    let (key_sender, key_receiver) = mpsc::channel();
    let (screen_sender, screen_receiver) = mpsc::sync_channel(1);

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

    let game_boy_thread = thread::spawn(move || run_game_boy(game_boy, screen_sender, key_receiver));

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)?
    };

    let res = event_loop.run(|event, elwt| {
        use winit::event::{Event, WindowEvent};
        use winit::event::ElementState::{Pressed, Released};
        use winit::keyboard::{Key, KeyCode, NamedKey};
        
        if let Event::WindowEvent { event: WindowEvent::KeyboardInput { event: key_event, .. }, .. } = &event {
            match (key_event.state, key_event.logical_key.as_ref()) {
                (Pressed, winit_key) => {
                    if let Some(key) = winit_to_joypad(winit_key) {
                        let _ = key_sender.send(GameBoyEvent::KeyDown(key));
                    }
                }
                (Released, winit_key) => {
                    if let Some(key) = winit_to_joypad(winit_key) {
                        let _ = key_sender.send(GameBoyEvent::KeyUp(key));
                    }
                }
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

    drop(screen_receiver);
    let _ = game_boy_thread.join();
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

enum GameBoyEvent {
    KeyUp(JoypadKey),
    KeyDown(JoypadKey),
}

fn run_game_boy(mut game_boy: Box<GameBoy>, sender: SyncSender<Vec<u8>>, receiver: Receiver<GameBoyEvent>) {

}

fn winit_to_joypad(key: winit::keyboard::Key<&str>) -> Option<JoypadKey> {
    use winit::keyboard::{Key, NamedKey};
    match key {
        Key::Character("Z" | "z") => Some(JoypadKey::A),
        Key::Character("X" | "x") => Some(JoypadKey::B),
        Key::Named(NamedKey::ArrowUp) => Some(JoypadKey::Up),
        Key::Named(NamedKey::ArrowDown) => Some(JoypadKey::Down),
        Key::Named(NamedKey::ArrowLeft) => Some(JoypadKey::Left),
        Key::Named(NamedKey::ArrowRight) => Some(JoypadKey::Right),
        Key::Named(NamedKey::Space) => Some(JoypadKey::Select),
        Key::Named(NamedKey::Enter) => Some(JoypadKey::Start),
        _ => None,
    }
}
