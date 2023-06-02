use deno_runtime::deno_core::error::AnyError;
use embedded_languages;

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    embedded_languages::init_logger();

    let script = r#"
    // Deno[Deno.internal].core.ops.op_print_log(1)
    console.debug(1 + 5)
    console.log(1 + 5)
    console.info(1 + 5)
    console.warn(1 + 5)
    console.error(1 + 5)
    
    "#;

    embedded_languages::deno::run(script).await?;

    Ok(())
}
