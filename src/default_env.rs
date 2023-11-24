use crate::errors::Result;
use crate::interpreter::{Binding, Interpreter};
use crate::symbol::Symbol;
use std::collections::HashMap;

pub fn default_env() -> HashMap<Symbol, Binding> {
    let mut env = HashMap::new();
    let e = &mut env;

    primitive(e, "%error", |intp| {
        let msg = intp.pop_str()?;
        Err(msg.to_string())
    });

    primitive(e, "%i.", |intp| {
        println!("{}", intp.pop_int()?);
        Ok(())
    });

    env
}

fn primitive(
    env: &mut HashMap<Symbol, Binding>,
    name: &'static str,
    fun: fn(&mut Interpreter) -> Result<()>,
) {
    env.insert(Symbol::from_static(name), Binding::Primitive(fun));
}
