// globalThis.console = { log: Deno[Deno.internal].core.ops.op_hello };

// const console = { log: Deno[Deno.internal].core.ops.op_hello };

// export {
//     console
// }

const ops = Deno.core.ops;
const console = {
    debug:  ops.op_print_debug,
    log: ops.op_print_log,
    info: ops.op_print_info,
    warn: ops.op_print_warn,
    error: ops.op_print_error
}

globalThis.console = console;
