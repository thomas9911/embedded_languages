use mlua::prelude::{Lua, LuaResult};

pub fn run(script: &str) -> LuaResult<()> {
    let lua = Lua::new();
    let new_print = lua.create_function(|_, data: String| {
        log::info!("{}", data);
        Ok(())
    })?;

    let globals = lua.globals();
    globals.set("print", new_print)?;

    lua.load(script).exec()?;

    Ok(())
}
