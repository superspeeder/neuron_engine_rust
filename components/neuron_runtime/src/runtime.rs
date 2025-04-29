use log::debug;
use neuron_common::plugin::{AppPluginsSpecification, PluginSpecification};
use neuron_script_api::api::ScriptApiRuntime;
use neuron_script_api::plugin::{
    Plugin, PluginContainer, PluginCreationContext, PluginLoadingContext,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use thiserror::Error;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PluginState {
    Unloaded,
    Loaded,
}

pub struct LoadedPlugin {
    plugin: RefCell<Box<dyn Plugin>>,

    #[allow(dead_code)]
    assets_path: String,

    #[allow(dead_code)]
    binary: libloading::Library,
    state: PluginState,
}

pub struct Runtime {
    plugin_registry: HashMap<&'static str, LoadedPlugin>,
}

#[derive(Error, Debug)]
pub enum PluginConstructionError {
    #[error(transparent)]
    LibraryLoadingError(#[from] libloading::Error),

    #[error("No valid library path")]
    NoValidLibraryPath,
}

impl Runtime {
    pub fn construct_plugin(
        &mut self,
        spec: PluginSpecification,
    ) -> Result<&'static str, PluginConstructionError> {
        let mut bpath: Option<String> = None;
        for bp in spec.binary_path.iter() {
            if std::fs::exists(bp).unwrap_or(false) {
                bpath = Some(bp.clone());
                break;
            }
        }
        if bpath == None {
            return Err(PluginConstructionError::NoValidLibraryPath);
        }

        let library = unsafe { libloading::Library::new(bpath.unwrap_unchecked())? };

        let plugin = unsafe {
            let mut creation_context = PluginCreationContext {
                logger: log::logger(),
                log_level: log::max_level(),
            };

            let entry = library.get::<unsafe extern "C" fn(
                *mut PluginCreationContext,
            ) -> *mut PluginContainer>(b"_plugin_entry")?;
            let plugin_container = Box::from_raw(entry(&mut creation_context));
            plugin_container.0
        };

        let plugin_name = plugin.name();

        self.plugin_registry.insert(
            plugin_name,
            LoadedPlugin {
                plugin: RefCell::new(plugin),
                assets_path: spec.assets_path,
                binary: library,
                state: PluginState::Unloaded,
            },
        );

        Ok(plugin_name)
    }

    pub fn load_plugin(&mut self, plugin_name: &str) -> bool {
        let Some(plugin) = self.plugin_registry.get(plugin_name) else {
            return false;
        };
        if plugin.state == PluginState::Loaded {
            return true;
        }

        plugin
            .plugin
            .borrow_mut()
            .load(PluginLoadingContext { runtime: self });
        self.plugin_registry.get_mut(plugin_name).unwrap().state = PluginState::Loaded;
        true
    }

    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            plugin_registry: HashMap::new(),
        })
    }

    pub fn construct_all(
        &mut self,
        all_spec: AppPluginsSpecification,
    ) -> Result<(), PluginConstructionError> {
        for (name, spec) in all_spec.plugins {
            let _ = self.construct_plugin(spec)?;
            debug!("Constructed plugin {}", name);
        }

        Ok(())
    }

    pub fn load_all(&mut self) {
        for name in self
            .plugin_registry
            .keys()
            .cloned()
            .collect::<Vec<&'static str>>()
        {
            unsafe {
                if self.plugin_registry.get(name).unwrap_unchecked().state == PluginState::Unloaded
                {
                    self.plugin_registry
                        .get(name)
                        .unwrap_unchecked()
                        .plugin
                        .borrow_mut()
                        .load(PluginLoadingContext { runtime: self });
                    self.plugin_registry.get_mut(name).unwrap_unchecked().state =
                        PluginState::Loaded;
                    debug!("Loaded plugin {}", name);
                }
            }
        }
    }

    pub fn unload_all(&mut self) {
        for name in self
            .plugin_registry
            .keys()
            .cloned()
            .collect::<Vec<&'static str>>()
        {
            unsafe {
                let plugin = self.plugin_registry.get_mut(name).unwrap_unchecked();
                if plugin.state == PluginState::Loaded {
                    plugin.plugin.borrow_mut().unload();
                    plugin.state = PluginState::Unloaded;
                    debug!("Unloaded plugin {}", name);
                }
            }
        }
    }
}

impl ScriptApiRuntime for Runtime {
    fn name(&self) -> &'static str {
        "neuron-rt"
    }

    fn version_string(&self) -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
}
