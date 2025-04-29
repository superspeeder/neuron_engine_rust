pub trait ScriptApiRuntime {
    fn name(&self) -> &'static str;
    fn version_string(&self) -> &'static str;
}