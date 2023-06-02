use embedded_languages;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    embedded_languages::init_logger();

    let script = r#"
    print(1 + 5)
    
    "#;

    embedded_languages::lua::run(script)?;

    Ok(())
}
