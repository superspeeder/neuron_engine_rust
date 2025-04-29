use winit::window::WindowAttributes;

pub struct WindowProperties {
    pub attributes: WindowAttributes,
    pub tag: Option<&'static str>,
}

impl From<WindowAttributes> for WindowProperties {
    fn from(attributes: WindowAttributes) -> Self {
        Self {
            attributes,
            tag: None,
        }
    }
}

impl From<(WindowAttributes, &'static str)> for WindowProperties {
    fn from(value: (WindowAttributes, &'static str)) -> Self {
        match value {
            (attributes, tag) => Self { attributes, tag: Some(tag) },
        }
    }
}

pub struct AppConfiguration {
    pub primary_window: WindowProperties,
    pub secondary_windows: Vec<WindowProperties>,
}


