use crate::api::ScriptApiRuntime;
use log::{info, LevelFilter, Log, Metadata, Record};

pub trait Plugin {
    fn load(&mut self, loading_context: PluginLoadingContext<'_>);
    fn unload(&mut self);

    fn name(&self) -> &'static str;
}

pub struct PluginLoadingContext<'a> {
    pub runtime: &'a dyn ScriptApiRuntime,
}

pub struct PluginCreationContext {
    pub logger: &'static dyn Log,
    pub log_level: LevelFilter,
}

#[repr(transparent)]
pub struct PluginContainer(pub Box<dyn Plugin>);

impl PluginCreationContext {
    pub fn generic_setup(&self) {
        log::set_logger(self.logger).expect("Failed to set logger");
        log::set_max_level(self.log_level);
    }
}

pub struct ForwardingLogger {
    base_logger: &'static dyn Log,
}

impl Log for ForwardingLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.base_logger.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        println!("log");
        self.base_logger.log(record);
        info!("test");
    }

    fn flush(&self) {
        self.base_logger.flush();
    }
}
