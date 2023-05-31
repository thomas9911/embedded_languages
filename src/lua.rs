use mlua::prelude::{Lua, LuaResult};

pub fn run(script: &str) -> LuaResult<()> {
    let lua = Lua::new();

    lua.load(script).exec()?;

    Ok(())
}
