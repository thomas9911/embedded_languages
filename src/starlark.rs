use starlark::environment::{Globals, Module};
use starlark::eval::Evaluator;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::Value;

pub fn run(script: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ast: AstModule =
        AstModule::parse("hello_world.star", script.to_owned(), &Dialect::Standard)?;

    let globals: Globals = Globals::extended();

    let module: Module = Module::new();

    let mut eval: Evaluator = Evaluator::new(&module);

    let _res: Value = eval.eval_module(ast, &globals)?;

    Ok(())
}
