mod file_battery_save;

use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use log::{error, LevelFilter};
use simplelog::{TermLogger, TerminalMode};

use gameboy_core::frame_buffer::FrameBuffer;
use gameboy_core::game_boy::GameBoy;
use gameboy_core::joypad::JoypadKey;
use gameboy_core::{SCREEN_WIDTH, SCREEN_HEIGHT, NANOSECONDS_PER_FRAME, CPU_CYCLES_PER_FRAME, AUDIO_SAMPLE_RATE};
use crate::file_battery_save::FileBatterySave;

fn main() -> Result<(), Box<dyn Error>> {
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

    let scale = matches.get_one::<u8>("scale").copied().unwrap();
    let cartridge_path = PathBuf::from(matches.get_one::<String>("cartridge_path").unwrap());
    let file_battery_save = FileBatterySave::new(cartridge_path.clone());
    let mut cartridge_data = vec![];
    File::open(&cartridge_path).and_then(|mut f| f.read_to_end(&mut cartridge_data)).expect("Could not read ROM");

    let game_boy = match GameBoy::new(cartridge_data, Some(Box::new(file_battery_save))) {
        Ok(game_boy) => game_boy,
        Err(error_str) => panic!("{}", error_str),
    };

    run_game_loop(game_boy, scale)
}

fn run_game_loop(mut game_boy: Box<GameBoy>, scale: u8) -> Result<(), Box<dyn Error>> {
    use std::thread;
    use std::time::{Duration, Instant};
    use pixels::{PixelsBuilder, SurfaceTexture};
    use pixels::wgpu::PresentMode::Mailbox;
    #[cfg(feature = "sound")]
    use sdl2::audio::AudioSpecDesired;
    use winit::dpi::LogicalSize;
    use winit::event_loop::{ControlFlow, EventLoop};
    use winit::window::Window;

    let event_loop = EventLoop::new()?;
    event_loop.set_control_flow(ControlFlow::Poll);
    let size = LogicalSize::new(SCREEN_WIDTH as f64 * scale as f64, SCREEN_HEIGHT as f64 * scale as f64);

    let window = event_loop.create_window(
        Window::default_attributes()
            .with_title(if cfg!(feature = "test") { "Test mode".to_string() } else { game_boy.title() })
            .with_inner_size(size)
            .with_min_inner_size(size)
    )?;

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        PixelsBuilder::new(SCREEN_WIDTH, SCREEN_HEIGHT, surface_texture)
            .enable_vsync(false)
            .present_mode(Mailbox)
            .build()?
    };

    let frame_duration = Duration::from_nanos(NANOSECONDS_PER_FRAME);
    let mut cpu_cycles = 0;
    let mut next_frame = Instant::now() + frame_duration;

    #[cfg(feature = "sound")]
    let audio = sdl2::init()?.audio()?;

    #[cfg(feature = "sound")]
    let audio_queue = audio.open_queue(
        None,
        &AudioSpecDesired {
            freq: Some(AUDIO_SAMPLE_RATE as i32),
            channels: Some(2),
            samples: None,
        }
    )?;

    #[cfg(feature = "sound")]
    audio_queue.resume();

    let res = event_loop.run(|event, elwt| {
        use winit::event::{Event, WindowEvent};
        use winit::event::ElementState::{Pressed, Released};
        use winit::keyboard::{Key, NamedKey};

        while cpu_cycles < CPU_CYCLES_PER_FRAME {
            cpu_cycles += game_boy.emulate();
        }

        cpu_cycles -= CPU_CYCLES_PER_FRAME;

        if let Some(data) = game_boy.updated_frame_buffer() {
            data.write_to_rbga_buffer(pixels.frame_mut());
            if let Err(_err) = pixels.render() {
                error!("Feil under tegning til skjerm!");
                elwt.exit();
            }
        }

        #[cfg(feature = "sound")] {
            let sound_data: Vec<f32> = game_boy.sound_buffer()
                .into_iter()
                .flat_map(|(l, r)| [l, r])
                .collect();
            let _ = audio_queue.queue_audio(&sound_data);
        }

        if let Event::WindowEvent { event: WindowEvent::KeyboardInput { event: key_event, .. }, .. } = &event {
            match (key_event.state, key_event.logical_key.as_ref()) {
                (Pressed, Key::Named(NamedKey::Escape)) => {
                    elwt.exit();
                    window.request_redraw();
                }
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
        }

        if let Event::WindowEvent { event: WindowEvent::CloseRequested, .. } = &event {
            elwt.exit();
            window.request_redraw();
        }

        let now = Instant::now();
        if next_frame > now {
            thread::sleep(next_frame - now);
        }
        next_frame += frame_duration;
    });

    Ok(res?)
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
