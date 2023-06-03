use rquickjs::{Context, Func, Object, Runtime};

fn value_printer(value: &rquickjs::Value, buffer: &mut String) -> Result<(), rquickjs::Error> {
    let type_ = value.type_of();
    use rquickjs::Type::*;
    match type_ {
        Bool | Int | Float => match type_ {
            Bool => buffer.push_str(&value.as_bool().unwrap().to_string()),
            Int => buffer.push_str(&value.as_int().unwrap().to_string()),
            Float => buffer.push_str(&value.as_float().unwrap().to_string()),
            _ => unreachable!(),
        },
        String => {
            let txt = unsafe { value.ref_string() }.to_string();
            buffer.push('"');
            buffer.push_str(&txt?);
            buffer.push('"');
        }
        Symbol | Object | Array | Function => {
            buffer.push_str("{ ..other }");
        }
        _ => (),
    }
    Ok(())
}

fn debug(value: rquickjs::Value) {
    let mut buffer = String::new();
    value_printer(&value, &mut buffer).ok();
    log::debug!("{}", buffer);
}

fn log(value: rquickjs::Value) {
    let mut buffer = String::new();
    value_printer(&value, &mut buffer).ok();
    log::info!("{}", buffer);
}

fn info(value: rquickjs::Value) {
    let mut buffer = String::new();
    value_printer(&value, &mut buffer).ok();
    log::info!("{}", buffer);
}

fn warn(value: rquickjs::Value) {
    let mut buffer = String::new();
    value_printer(&value, &mut buffer).ok();
    log::warn!("{}", buffer);
}

fn error(value: rquickjs::Value) {
    let mut buffer = String::new();
    value_printer(&value, &mut buffer).ok();
    log::error!("{}", buffer);
}

pub fn run(script: &str) -> Result<(), rquickjs::Error> {
    let rt = Runtime::new()?;
    let ctx = Context::full(&rt)?;
    ctx.with(|ctx| {
        let global = ctx.globals();
        let console = Object::new(ctx)?;
        console.set("debug", Func::new("debug", debug))?;
        console.set("info", Func::new("info", info))?;
        console.set("log", Func::new("log", log))?;
        console.set("warn", Func::new("warn", warn))?;
        console.set("error", Func::new("error", error))?;
        global.set("console", console)?;

        let _ = ctx.compile("test", script)?;
        Ok::<(), rquickjs::Error>(())
    })?;

    Ok(())
}
