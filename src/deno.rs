use deno_runtime::deno_core::error::AnyError;
use deno_runtime::deno_core::ModuleSpecifier;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use std::path::Path;

pub async fn run(script: &str) -> Result<(), AnyError> {
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/deno.js");
    let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();
    let mut worker = MainWorker::bootstrap_from_options(
        main_module.clone(),
        PermissionsContainer::allow_all(),
        WorkerOptions {
            ..Default::default()
        },
    );
    // worker.execute_main_module(&main_module).await?;
    worker.execute_script("<deno>", script.to_string().into())?;
    worker.run_event_loop(false).await?;
    Ok(())
}
