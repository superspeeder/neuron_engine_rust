use neuron_common::plugin::AppPluginsSpecification;
use neuron_runtime::runtime::Runtime;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // env_logger::init();
    pretty_env_logger::init();
    
    let mut runtime = Runtime::new();
    let ptl_file = std::fs::read_to_string(
        std::env::var("NEURON_TARGET_APP_MANIFEST").unwrap_or("manifest.toml".to_string()),
    )?;
    let plugins_to_load = toml::from_str::<AppPluginsSpecification>(ptl_file.as_str()).unwrap();
    runtime.construct_all(plugins_to_load)?;
    runtime.load_all();
    runtime.unload_all();

    Ok(())
}
