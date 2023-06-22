use embedded_languages;
use rusty_jsc::JSValue;

fn main() -> Result<(), JSValue> {
    embedded_languages::init_logger();

    let script = r#"
    console.log(1 + 5)
    console.debug(1 + 5)
    console.info(1 + 5)
    console.warn(1 + 5)
    console.error(1 + 5)

    "#;

    embedded_languages::javascriptcore::run(script)?;

    Ok(())
}
