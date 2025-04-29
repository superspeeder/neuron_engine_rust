use std::error::Error;
use std::ptr::NonNull;
use log::info;
use neuron_script_api::plugin::{Plugin, PluginCreationContext, PluginLoadingContext};
use neuron_script_api::plugin_entry;

pub struct MyPlugin {

}

impl Plugin for MyPlugin {
    fn load(&mut self, loading_context: PluginLoadingContext) {
        info!("Plugin loaded with {} version {}", loading_context.runtime.name(), loading_context.runtime.version_string());
    }

    fn unload(&mut self) {
        info!("Unloading plugin");
    }

    fn name(&self) -> &'static str {
        env!("CARGO_PKG_NAME")
    }
}

#[plugin_entry]
pub fn main(_creation_context: NonNull<PluginCreationContext>) -> Result<MyPlugin, Box<dyn Error>> {
    Ok(MyPlugin {})
}