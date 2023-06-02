use starlark::environment::{Globals, GlobalsBuilder, Module};
use starlark::eval::Evaluator;
use starlark::starlark_module;
use starlark::syntax::{AstModule, Dialect};
use starlark::values::none::NoneType;
use starlark::values::Value;

#[starlark_module]
fn starlark_print(builder: &mut GlobalsBuilder) {
    fn print(#[starlark(args)] args: Vec<Value>) -> anyhow::Result<NoneType> {
        let data: Vec<_> = args.iter().map(|x| x.to_str()).collect();
        log::info!("{}", data.join(" "));

        Ok(NoneType)
    }
}

pub fn run(script: &str) -> Result<(), Box<dyn std::error::Error>> {
    let ast: AstModule = AstModule::parse("demo.star", script.to_owned(), &Dialect::Standard)?;

    let globals: Globals = GlobalsBuilder::new().with(starlark_print).build();
    // let globals: Globals = Globals::extended();

    let module: Module = Module::new();

    let mut eval: Evaluator = Evaluator::new(&module);

    let _res: Value = eval.eval_module(ast, &globals)?;

    Ok(())
}
