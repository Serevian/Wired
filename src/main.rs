use std::process::ExitCode;

use sdl3::{event::Event, keyboard::Keycode};
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::engine::Engine;

mod engine;

fn main() -> ExitCode {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let file_appender = rolling::daily("logs", "engine.log");
    let (file_writer, guard) = non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(true))
        .with(fmt::layer().with_writer(file_writer).with_ansi(false))
        .init();

    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            tracing::error!("fatal: {err}");
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let sdl = sdl3::init().map_err(|e| format!("Error initializing SDL3: {e}"))?;
    let sdl_video = sdl
        .video()
        .map_err(|e| format!("Error initializing SDL3 video subsystem: {e}"))?;

    let window = sdl_video
        .window("Wired", 1280, 720)
        .vulkan()
        .position_centered()
        .build()
        .map_err(|e| format!("Error creating SDL3 window: {e}"))?;

    let engine = Engine::new(&window).map_err(|e| format!("Error creating engine: {e}"))?;

    let mut event_pump = sdl
        .event_pump()
        .map_err(|e| format!("Error creating SDL3 event pump: {e}"))?;

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

    tracing::info!("shutting down");
    Ok(())
}
