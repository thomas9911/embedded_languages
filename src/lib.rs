use log::LevelFilter;

pub mod boa;
pub mod deno;
pub mod javascriptcore;
pub mod lua;
pub mod quickjs;
pub mod rhai;
pub mod starlark;

pub fn init_logger() {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();
}
