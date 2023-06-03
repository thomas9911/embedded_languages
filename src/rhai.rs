// use std::fmt::{Debug, Display};

use rhai::{Engine, EvalAltResult};

// fn show_it<T: Display + Debug>(x: &mut T) -> String {
//     println!("here => {:?}", x);
//     log::info!("{}", x);
//     String::new()
// }

pub fn run(script: &str) -> Result<(), Box<EvalAltResult>> {
    let mut engine = Engine::new();

    // engine.register_fn("print", show_it::<String>)
    //       .register_fn("print", show_it::<Dynamic>);

    engine
        .on_print(|s| log::info!("{}", s))
        .on_debug(|s, _, _| log::debug!("{}", s));

    // let _ = engine.eval::<Dynamic>(script)?;
    let _ = engine.run(script)?;

    Ok(())
}
