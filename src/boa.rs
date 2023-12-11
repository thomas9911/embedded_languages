use boa_engine::class::{Class, ClassBuilder};
use boa_engine::{JsResult, JsNativeError};
use boa_engine::{Context, JsValue};
use boa_gc::{Finalize, Trace};
use boa_engine::NativeFunction;
use boa_engine::Source;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Trace, Finalize)]
struct Console;

impl Console {
    fn args_to_string(args: &[JsValue], context: &mut Context) -> JsResult<String> {
        let stringified_args: Result<Vec<_>, _> = args
            .iter()
            .map(|x| JsValue::to_string(x, context).map(|x| x.to_std_string_escaped()))
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

    fn constructor(_this: &JsValue, _args: &[JsValue], _context: &mut Context) -> JsResult<Self> {
        Err(JsNativeError::typ().with_message("console is not a constructor").into())
    }

    /// This is where the object is initialized.
    fn init(class: &mut ClassBuilder) -> JsResult<()> {
        class.static_method("info", 12, NativeFunction::from_fn_ptr(Self::info));
        class.static_method("debug", 12, NativeFunction::from_fn_ptr(Self::debug));
        class.static_method("log", 12, NativeFunction::from_fn_ptr(Self::log));
        class.static_method("warn", 12, NativeFunction::from_fn_ptr(Self::warn));
        class.static_method("error", 12, NativeFunction::from_fn_ptr(Self::error));

        Ok(())
    }
}

pub fn run(script: &str) -> JsResult<()> {
    // Instantiate the execution context
    let mut context = Context::default();
    context.register_global_class::<Console>()?;

    match inner_run(&mut context, script) {
        Ok(x) => x,
        Err(e) => return Err(e)
    };

    Ok(())
}

fn inner_run(context: &mut Context, script: &str) -> JsResult<()> {
    context.eval(Source::from_bytes(script.as_bytes()))?;

    Ok(())
}
