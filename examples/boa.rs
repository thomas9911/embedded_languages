use boa_engine::JsResult;
use embedded_languages;

fn main() -> JsResult<()> {
    embedded_languages::init_logger();

    let script = r#"
    console.debug(1 + 5)
    console.log(1 + 5)
    console.info(1 + 5)
    console.warn(1 + 5)
    console.error(1 + 5)
    "#;

    embedded_languages::boa::run(script)?;

    Ok(())
}
