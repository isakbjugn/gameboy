use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;
use log::{error, info, LevelFilter};
use pixels::{Error, Pixels, SurfaceTexture};
use simplelog::{TermLogger, TerminalMode};
use winit::dpi::LogicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, NamedKey};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
use crate::frame_buffer::FrameBuffer;
use crate::game_boy::GameBoy;
use crate::joypad::JoypadKey;

mod cpu;
mod address_bus;
mod mbc;
mod ppu;
mod joypad;
mod bootrom;
mod timer;
mod game_boy;
mod cartridge;
mod frame_buffer;
mod apu;

const SCREEN_WIDTH: u32 = 160;
const SCREEN_HEIGHT: u32 = 144;

fn main() -> Result<(), Error> {
    TermLogger::init(
        LevelFilter::Info,
        simplelog::Config::default(),
        TerminalMode::Mixed,
        simplelog::ColorChoice::Auto,
    )
        .expect("Kunne ikke sette opp logger");

    let matches = clap::Command::new("gameboy")
        .version("0.1")
        .author("Isak Kyrre Lichtwarck Bjugn")
        .about("A Gameboy emulator written in Rust")
        .arg(clap::Arg::new("cartridge_path")
            .help("Sets the path to the ROM file to load")
            .required(true))
        .arg(clap::Arg::new("scale")
            .help("Scales the display. Default is 2")
            .short('x')
            .long("scale")
             .default_value("2")
             .value_parser(|s: &str| {
                 s.parse::<u8>()
                     .map_err(|e| format!("Invalid scale value: {}", e))
             }))
        .get_matches();

    let cartridge_path = matches.get_one::<String>("cartridge_path").unwrap();
    let scale = matches.get_one::<u8>("scale").copied().unwrap();

    let game_boy = match GameBoy::new(cartridge_path) {
        Ok(game_boy) => game_boy,
        Err(error_str) => panic!("{}", error_str),
    };

    let (key_sender, key_receiver) = mpsc::channel();
    let (screen_sender, screen_receiver) = mpsc::sync_channel(1);

    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH as f64 * scale as f64, SCREEN_HEIGHT as f64 * scale as f64);
        WindowBuilder::new()
            .with_title(if cfg!(feature = "test") { "Test mode".to_string() } else { game_boy.title() })
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
        use winit::keyboard::KeyCode;

        if let Event::WindowEvent { event: WindowEvent::KeyboardInput { event: key_event, .. }, .. } = &event {
            match (key_event.state, key_event.logical_key.as_ref()) {
                (Pressed, winit_key) => {
                    if winit_key == Key::Named(NamedKey::Space) {
                        let _ = key_sender.send(GameBoyEvent::FastForward);
                    }
                    else if let Some(key) = winit_to_joypad(winit_key) {
                        let _ = key_sender.send(GameBoyEvent::KeyDown(key));
                    }
                }
                (Released, winit_key) => {
                    if winit_key == Key::Named(NamedKey::Space) {
                        let _ = key_sender.send(GameBoyEvent::NormalSpeed);
                    }
                    else if let Some(key) = winit_to_joypad(winit_key) {
                        let _ = key_sender.send(GameBoyEvent::KeyUp(key));
                    }
                }
            }
        }

        if input.update(&event) {
            if input.key_pressed(KeyCode::Escape) || input.close_requested() {
                info!("Skrur av Game Boy!");
                elwt.exit();
            }

            window.request_redraw();
        }
        
        match screen_receiver.recv() {
            Ok(data) => {
                data.write_to_rbga_buffer(pixels.frame_mut());
                if let Err(err) = pixels.render() {
                    error!("Feil under tegning til skjerm!");
                    elwt.exit();
                }
            }
            Err(..) => elwt.exit(),
        }
    });

    drop(screen_receiver);
    let _ = game_boy_thread.join();
    res.map_err(|e| Error::UserDefined(Box::new(e)))
}

enum GameBoyEvent {
    KeyUp(JoypadKey),
    KeyDown(JoypadKey),
    FastForward,
    NormalSpeed,
}

fn run_game_boy(mut game_boy: Box<GameBoy>, sender: SyncSender<Vec<u8>>, receiver: Receiver<GameBoyEvent>) {
    use std::sync::mpsc::{TryRecvError, TrySendError};
    use std::time::{Duration, Instant};

    let frame_duration = Duration::from_millis(16);
    let cpu_cycles_per_frame = (4194204f64 / 1000.0 * 16.0).round() as u32;
    let mut cpu_cycles = 0;
    let mut fast_forward_active = false;
    
    'emulate: loop {
        let start = Instant::now();
        
        while cpu_cycles < cpu_cycles_per_frame {
            cpu_cycles += game_boy.emulate();
        }
        
        cpu_cycles -= cpu_cycles_per_frame;

        if let Some(data) = game_boy.updated_frame_buffer() {
            if let Err(TrySendError::Disconnected(..)) = sender.try_send(data) {
                info!("Game Boy mistet forbindelse med skjermen!");
                break
            }
        }
        
        'joypad_input: loop {
            match receiver.try_recv() {
                Ok(GameBoyEvent::KeyDown(key)) => game_boy.key_down(key),
                Ok(GameBoyEvent::KeyUp(key)) => game_boy.key_up(key),
                Ok(GameBoyEvent::FastForward) => fast_forward_active = true,
                Ok(GameBoyEvent::NormalSpeed) => fast_forward_active = false,
                Err(TryRecvError::Empty) => break 'joypad_input,
                Err(TryRecvError::Disconnected) => break 'emulate,
            }
        }
        
        if !fast_forward_active {
            let time_elapsed = start.elapsed();
            if frame_duration > time_elapsed {
                thread::sleep(frame_duration - time_elapsed);
            }
        }
    }
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
        Key::Named(NamedKey::Backspace) => Some(JoypadKey::Select),
        Key::Named(NamedKey::Enter) => Some(JoypadKey::Start),
        _ => None,
    }
}
