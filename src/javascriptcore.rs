use rusty_jsc::JSContext;
use rusty_jsc::JSObject;
use rusty_jsc::JSValue;

fn inner_log(
    loglevel: log::Level,
    context: JSContext,
    _function: JSObject,
    _this: JSObject,
    args: &[JSValue],
) -> Result<JSValue, JSValue> {
    let out: Result<Vec<String>, JSValue> = args
        .iter()
        .map(|x| {
            let y = x.to_string(&context)?;
            Ok(y.to_string())
        })
        .collect();
    let out = out?.join(" ");
    log::log!(loglevel, "{:?}", out);

    Ok(JSValue::undefined(&context))
}

#[rusty_jsc::callback]
fn debug(context: JSContext, function: JSObject, this: JSObject, args: &[JSValue]) {
    inner_log(log::Level::Debug, context, function, this, args)
}

#[rusty_jsc::callback]
fn info(context: JSContext, function: JSObject, this: JSObject, args: &[JSValue]) {
    inner_log(log::Level::Info, context, function, this, args)
}

#[rusty_jsc::callback]
fn warn(context: JSContext, function: JSObject, this: JSObject, args: &[JSValue]) {
    inner_log(log::Level::Warn, context, function, this, args)
}

#[rusty_jsc::callback]
fn error(context: JSContext, function: JSObject, this: JSObject, args: &[JSValue]) {
    inner_log(log::Level::Error, context, function, this, args)
}

pub fn run(script: &str) -> Result<(), JSValue> {
    let mut context = JSContext::default();

    let console = JSObject::new(&context);
    let callback = JSValue::callback(&context, Some(info));
    console.set_property(&context, "log", callback)?;
    let callback = JSValue::callback(&context, Some(info));
    console.set_property(&context, "info", callback)?;
    let callback = JSValue::callback(&context, Some(debug));
    console.set_property(&context, "debug", callback)?;
    let callback = JSValue::callback(&context, Some(warn));
    console.set_property(&context, "warn", callback)?;
    let callback = JSValue::callback(&context, Some(error));
    console.set_property(&context, "error", callback)?;

    let global = context.get_global_object();
    global.set_property(&context, "console", console.to_jsvalue())?;

    if let Ok(_value) = context.evaluate_script(script, 0) {
        ()
    }
    Ok(())
}
