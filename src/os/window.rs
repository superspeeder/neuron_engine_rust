pub use winit::window::WindowAttributes;

use crate::render::RenderSystem;
use hashbrown::HashMap;
use hashbrown::hash_map::Values;
use std::ops::Deref;
use std::sync::Arc;
use thiserror::Error;
use wgpu::SurfaceConfiguration;
use winit::error::OsError;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

pub struct WindowData {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    tag: Option<&'static str>,
    current_surface_configuration: Option<SurfaceConfiguration>,
}

#[derive(Debug, Error)]
pub enum CreateWindowError {
    #[error(transparent)]
    CreateSurfaceError(#[from] wgpu::CreateSurfaceError),

    #[error(transparent)]
    OsError(#[from] OsError),
}

impl WindowData {
    pub fn new(
        window: Arc<Window>,
        render_system: &RenderSystem,
        tag: Option<&'static str>,
    ) -> Result<Self, wgpu::CreateSurfaceError> {
        let surface = render_system.instance().create_surface(window.clone())?;

        Ok(Self {
            window,
            surface,
            tag,
            current_surface_configuration: None,
        })
    }

    pub fn reconfigure_surface(&mut self, render_system: &RenderSystem) {
        let size = self.window.inner_size();
        match self.current_surface_configuration.clone() {
            Some(mut surface_configuration) => {
                surface_configuration.width = size.width;
                surface_configuration.height = size.height;
                self.current_surface_configuration = Some(surface_configuration);
            }
            None => {
                let surface_caps = self.surface.get_capabilities(render_system.adapter());
                let format = surface_caps
                    .formats
                    .iter()
                    .find(|f| f.is_srgb())
                    .copied()
                    .unwrap_or(surface_caps.formats[0]);
                let present_mode = surface_caps
                    .present_modes
                    .iter()
                    .find(|p| p == &&wgpu::PresentMode::Mailbox)
                    .copied()
                    .unwrap_or(wgpu::PresentMode::Fifo);

                self.current_surface_configuration = Some(SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format,
                    width: size.width,
                    height: size.height,
                    present_mode,
                    desired_maximum_frame_latency: 2,
                    alpha_mode: wgpu::CompositeAlphaMode::Opaque,
                    view_formats: vec![],
                })
            }
        }

        // I can safely unwrap_unchecked here because in the code right before this I always set the value to something
        self.surface.configure(render_system.device(), unsafe {
            self.current_surface_configuration
                .as_ref()
                .unwrap_unchecked()
        });
    }
}

pub struct WindowManager {
    render_system: Arc<RenderSystem>,
    primary_window: Option<WindowId>,
    windows_by_id: HashMap<WindowId, WindowData>,
    windows_by_tag: HashMap<&'static str, WindowId>,
}

impl WindowManager {
    pub fn new(render_system: Arc<RenderSystem>) -> Self {
        Self {
            render_system,
            primary_window: None,
            windows_by_id: Default::default(),
            windows_by_tag: Default::default(),
        }
    }

    pub fn register_window(
        &mut self,
        window: Arc<Window>,
        tag: Option<&'static str>,
    ) -> Result<(), wgpu::CreateSurfaceError> {
        let window_id = window.id();
        if let Some(tag) = tag {
            self.windows_by_tag.insert(tag, window_id);
        }

        let window_data = WindowData::new(window, &self.render_system, tag.clone())?;
        self.windows_by_id.insert(window_id, window_data);

        Ok(())
    }

    pub fn register_primary_window(
        &mut self,
        window: Arc<Window>,
        tag: Option<&'static str>,
    ) -> Result<(), wgpu::CreateSurfaceError> {
        self.primary_window = Some(window.id());
        self.register_window(window, tag)
    }

    pub fn create_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        attributes: WindowAttributes,
        tag: Option<&'static str>,
    ) -> Result<WindowId, CreateWindowError> {
        let window = Arc::new(event_loop.create_window(attributes)?);
        let id = window.id();
        self.register_window(window.clone(), tag)?;

        Ok(id)
    }

    pub fn create_primary_window(
        &mut self,
        event_loop: &ActiveEventLoop,
        attributes: WindowAttributes,
        tag: Option<&'static str>,
    ) -> Result<WindowId, CreateWindowError> {
        let window = Arc::new(event_loop.create_window(attributes)?);
        let id = window.id();
        self.register_primary_window(window.clone(), tag)?;

        Ok(id)
    }

    pub fn close_window(&mut self, window_id: WindowId) {
        if Some(window_id) == self.primary_window {
            self.primary_window = None;
            self.windows_by_id.clear(); // closing primary means close all
        } else {
            self.windows_by_id.remove(&window_id);
        }
    }

    pub fn get_window(&self, window_id: WindowId) -> Option<&WindowData> {
        self.windows_by_id.get(&window_id)
    }

    pub fn get_tagged_window(&self, tag: &'static str) -> Option<&WindowData> {
        self.windows_by_tag
            .get(&tag)
            .and_then(|id| self.windows_by_id.get(id))
    }

    pub fn get_windows(&self) -> Values<'_, WindowId, WindowData> {
        self.windows_by_id.values()
    }

    pub fn reconfigure_surface(&mut self, window_id: WindowId) {
        if let Some(window_data) = self.windows_by_id.get_mut(&window_id) {
            window_data.reconfigure_surface(&self.render_system);
        }
    }
}

impl Deref for WindowData {
    type Target = Window;
    fn deref(&self) -> &Self::Target {
        &self.window
    }
}
