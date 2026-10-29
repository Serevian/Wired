use sdl3::{event::Event, keyboard::Keycode};

use crate::engine::Engine;

mod engine;

fn main() {
    let sdl = sdl3::init().expect("Error initializing SDL3");
    let sdl_video = sdl
        .video()
        .expect("Error initializing SDL3 video subsystem");

    let window = sdl_video
        .window("Wired", 1280, 720)
        .vulkan()
        .position_centered()
        .build()
        .expect("Error creating SDL3 window");

    let engine = Engine::new(&window);
    let engine = match engine {
        Ok(o) => o,
        Err(err) => {
            std::println!("{err}");
            std::process::exit(1);
        }
    };

    let mut event_pump = sdl.event_pump().expect("Error creating SDL3 event pump");
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Engine work
    }
}
