use std::{fs::File, process::ExitCode};

use anyhow::{Context, Result};
use raw_window_handle::{HasDisplayHandle, HasWindowHandle};
use sdl3::{event::Event, keyboard::Keycode};
use tracing_appender::non_blocking;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::engine::Engine;

mod engine;

fn main() -> ExitCode {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let file = File::create("engine.log").expect("failed to create engine.log");
    let (file_writer, guard) = non_blocking(file);

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

fn run() -> Result<()> {
    let sdl = sdl3::init().context("Error initializing SDL3")?;
    let sdl_video = sdl
        .video()
        .context("Error initializing SDL3 video subsystem")?;

    let window = sdl_video
        .window("Wired", 1280, 720)
        .vulkan()
        .position_centered()
        .build()
        .context("Error creating SDL3 window: ")?;
    let display_handle = window.display_handle().unwrap().as_raw();
    let window_handle = window.window_handle().unwrap().as_raw();

    let engine = Engine::new(display_handle, window_handle)?;

    let mut event_pump = sdl.event_pump().context("Error creating SDL3 event pump")?;

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
