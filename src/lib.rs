use log::LevelFilter;

pub mod deno;
pub mod lua;
pub mod starlark;

pub fn init_logger() {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Info)
        .init();
}
