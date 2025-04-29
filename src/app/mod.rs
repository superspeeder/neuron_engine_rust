use crate::os::window::WindowManager;
use crate::render::RenderSystem;
use hashbrown::HashMap;
use pollster::block_on;
use std::error::Error;
use std::sync::Arc;
use winit::window::WindowId;

pub mod configuration;

pub struct App {
    render_system: Arc<RenderSystem>,
    window_manager: WindowManager,
}

impl App {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let render_system = Arc::new(block_on(RenderSystem::new())?);
        let window_manager = WindowManager::new(render_system.clone());
        Ok(Self {
            render_system,
            window_manager,
        })
    }

    pub fn render_system(&self) -> &Arc<RenderSystem> {
        &self.render_system
    }

    pub fn window_manager(&self) -> &WindowManager {
        &self.window_manager
    }

    pub fn window_manager_mut(&mut self) -> &mut WindowManager {
        &mut self.window_manager
    }
}
