# Embedded languages

A list of scripting languages that can be embedded in a rust application

## Scripting languages

### Lua

The OG embeddable language. Implemented here with `mlua` with the `luau` or
roblox lua syntax.

### Javascript

Two implementations:

- `Deno`, which uses v8 under the hood
- `quickjs`, which uses its own engine.

### Python

Not exactly python but close, `Starlark`

### [Rhai](https://github.com/rhaiscript/rhai)

Embeddable en extensible scripting language made in Rust
