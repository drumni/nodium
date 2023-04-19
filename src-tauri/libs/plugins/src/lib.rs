// libs/plugins/src/plugins.rs
mod plugins;
mod registry;
mod extract_crate_file;

// export Plugins and Registry

pub use plugins::Plugins;
pub use registry::Registry;
pub use extract_crate_file::extract_crate_file;