use embedded_languages;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let script = r#"
print(1 + 5)
    
"#;

    embedded_languages::starlark::run(script)?;

    Ok(())
}
