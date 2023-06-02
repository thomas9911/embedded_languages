use deno_runtime::deno_core;
use deno_runtime::deno_core::error::AnyError;
use deno_runtime::deno_core::op;
use deno_runtime::deno_core::ModuleSpecifier;
use deno_runtime::permissions::PermissionsContainer;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use std::path::Path;

deno_runtime::deno_core::extension!(
    my_printing,
    ops = [
        op_print_debug,
        op_print_log,
        op_print_info,
        op_print_warn,
        op_print_error
    ],
    esm_entry_point = "ext:my_printing/logger.js",
    esm = ["logger.js"],
    customizer = |ext: &mut deno_runtime::deno_core::ExtensionBuilder| {
        ext.force_op_registration();
    },
);

#[op]
fn op_print_debug(text: serde_json::Value) {
    log::debug!("{}", text);
}

#[op]
fn op_print_log(text: serde_json::Value) {
    log::info!("{}", text);
}

#[op]
fn op_print_info(text: serde_json::Value) {
    log::info!("{}", text);
}

#[op]
fn op_print_warn(text: serde_json::Value) {
    log::warn!("{}", text);
}

#[op]
fn op_print_error(text: serde_json::Value) {
    log::error!("{}", text);
}

pub async fn run(script: &str) -> Result<(), AnyError> {
    let js_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/deno.js");
    let main_module = ModuleSpecifier::from_file_path(js_path).unwrap();
    let mut worker = MainWorker::bootstrap_from_options(
        main_module.clone(),
        PermissionsContainer::allow_all(),
        WorkerOptions {
            extensions: vec![my_printing::init_ops_and_esm()],
            ..Default::default()
        },
    );
    // worker.execute_main_module(&main_module).await?;
    worker.execute_script("<deno>", script.to_string().into())?;
    worker.run_event_loop(false).await?;
    Ok(())
}
