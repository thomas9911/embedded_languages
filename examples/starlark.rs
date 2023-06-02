use embedded_languages;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    embedded_languages::init_logger();

    let script = r#"
print({"test": 1 + 5}, 123)
    
"#;

    embedded_languages::starlark::run(script)?;

    Ok(())
}
