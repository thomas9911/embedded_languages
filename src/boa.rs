use boa_engine::class::{Class, ClassBuilder};
use boa_engine::JsResult;
use boa_engine::{Context, JsValue};
use boa_gc::{Finalize, Trace};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Trace, Finalize)]
struct Console;

impl Console {
    fn args_to_string(args: &[JsValue], context: &mut Context) -> JsResult<String> {
        let stringified_args: Result<Vec<_>, _> = args
            .iter()
            .map(|x| JsValue::to_string(x, context))
            .collect();
        let out = stringified_args?.join(" ");
        Ok(out)
    }

    pub(crate) fn log(_: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let out = Self::args_to_string(args, context)?;
        log::info!("{out}");
        Ok(JsValue::Undefined)
    }

    pub(crate) fn info(_: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let out = Self::args_to_string(args, context)?;
        log::info!("{out}");
        Ok(JsValue::Undefined)
    }

    pub(crate) fn debug(_: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let out = Self::args_to_string(args, context)?;
        log::debug!("{out}");
        Ok(JsValue::Undefined)
    }

    pub(crate) fn warn(_: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let out = Self::args_to_string(args, context)?;
        log::warn!("{out}");
        Ok(JsValue::Undefined)
    }

    pub(crate) fn error(_: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
        let out = Self::args_to_string(args, context)?;
        log::error!("{out}");
        Ok(JsValue::Undefined)
    }
}

impl Class for Console {
    const NAME: &'static str = "console";

    const LENGTH: usize = 0;

    fn constructor(_this: &JsValue, _args: &[JsValue], context: &mut Context) -> JsResult<Self> {
        Err(context.construct_type_error("console is not a constructor"))
    }

    /// This is where the object is initialized.
    fn init(class: &mut ClassBuilder) -> JsResult<()> {
        class.static_method("info", 12, Self::info);
        class.static_method("debug", 12, Self::debug);
        class.static_method("log", 12, Self::log);
        class.static_method("warn", 12, Self::warn);
        class.static_method("error", 12, Self::error);

        Ok(())
    }
}

pub fn run(script: &str) -> JsResult<()> {
    // Instantiate the execution context
    let mut context = Context::default();
    context.register_global_class::<Console>()?;

    match inner_run(&mut context, script) {
        Ok(x) => x,
        Err(e) => {
            let string_error = JsValue::to_string(&e, &mut context)?;
            return Err(JsValue::String(string_error));
        }
    };

    Ok(())
}

fn inner_run(context: &mut Context, script: &str) -> JsResult<()> {
    context.eval(script)?;

    Ok(())
}
