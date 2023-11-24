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

    primitive(e, "%f.", |intp| {
        println!("{}", intp.pop_flt()?);
        Ok(())
    });

    primitive(e, "%s.", |intp| {
        println!("{}", intp.pop_str()?);
        Ok(())
    });

    primitive(e, "%ii+", |intp| {
        let b = intp.pop_int()?;
        let a = intp.pop_int()?;
        intp.push_int(a + b);
        Ok(())
    });

    primitive(e, "%ff+", |intp| {
        let b = intp.pop_flt()?;
        let a = intp.pop_flt()?;
        intp.push_flt(a + b);
        Ok(())
    });

    primitive(e, "%ii-", |intp| {
        let b = intp.pop_int()?;
        let a = intp.pop_int()?;
        intp.push_int(a - b);
        Ok(())
    });

    primitive(e, "%ff-", |intp| {
        let b = intp.pop_flt()?;
        let a = intp.pop_flt()?;
        intp.push_flt(a - b);
        Ok(())
    });

    primitive(e, "%ii*", |intp| {
        let b = intp.pop_int()?;
        let a = intp.pop_int()?;
        intp.push_int(a * b);
        Ok(())
    });

    primitive(e, "%ff*", |intp| {
        let b = intp.pop_flt()?;
        let a = intp.pop_flt()?;
        intp.push_flt(a * b);
        Ok(())
    });

    primitive(e, "%ii/", |intp| {
        let b = intp.pop_int()?;
        let a = intp.pop_int()?;
        intp.push_int(a / b);
        Ok(())
    });

    primitive(e, "%ff/", |intp| {
        let b = intp.pop_flt()?;
        let a = intp.pop_flt()?;
        intp.push_flt(a / b);
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
