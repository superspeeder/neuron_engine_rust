use std::collections::HashMap;

#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct PluginSpecification {
    pub binary_path: Vec<String>,
    #[serde(default)]
    pub assets_path: String,
}

#[derive(Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct AppPluginsSpecification {
    #[serde(default)]
    pub plugins: HashMap<String, PluginSpecification>,
}