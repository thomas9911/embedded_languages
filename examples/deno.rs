use deno_runtime::deno_core::error::AnyError;
use embedded_languages;

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let script = r#"
    console.log(1 + 5)
    
    "#;

    embedded_languages::deno::run(script).await?;

    Ok(())
}
