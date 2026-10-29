use anyhow::{Ok, Result};

use crate::engine::context::Context;

mod context;

pub struct Engine {
    context: Context,
}

impl Engine {
    pub fn new(sdl_window: &sdl3::video::Window) -> Result<Self> {
        let context = Context::new(sdl_window)?;

        Ok(Self { context })
    }
}
