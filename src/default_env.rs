use crate::errors::Result;
use crate::interpreter::{Binding, Interpreter};
use crate::symbol::Symbol;
use std::collections::HashMap;

pub fn default_env() -> HashMap<Symbol, Binding> {
    let mut env = HashMap::new();
    let e = &mut env;

    housekeeping_primitives(e);
    stackop_primitives(e);
    integer_primitives(e);
    boolean_primitives(e);
    float_primitives(e);
    string_primitives(e);

    env
}

fn housekeeping_primitives(e: &mut HashMap<Symbol, Binding>) {
    primitive(e, "%error", |intp| {
        let msg = intp.pop_str()?;
        Err(msg.to_string())
    });

    primitive(e, ":stacks", |intp| {
        println!("  Main Stack: {:?}", intp.main_stack);
        println!("Second Stack: {:?}", intp.secondary_stack);
        Ok(())
    });

    primitive(e, ":words", |intp| {
        let mut names: Vec<_> = intp.env.keys().collect();
        names.sort();
        for name in names {
            print!("{:>40} | ", name.to_string());
            match &intp.env[name] {
                Binding::Primitive(_) => {}
                Binding::Composite(method) => {
                    print!("{}", method.read().unwrap().last().unwrap().doc)
                }
            };
            println!();
        }
        Ok(())
    });
}

fn stackop_primitives(e: &mut HashMap<Symbol, Binding>) {
    primitive(e, "%drop", |intp| {
        let _ = intp.pop();
        Ok(())
    });

    primitive(e, "%dup", |intp| {
        let x = intp.pop()?;
        intp.push(x.clone());
        intp.push(x);
        Ok(())
    });

    primitive(e, "%swap", |intp| {
        let b = intp.pop()?;
        let a = intp.pop()?;
        intp.push(b);
        intp.push(a);
        Ok(())
    });

    primitive(e, "%rot", |intp| {
        let c = intp.pop()?;
        let b = intp.pop()?;
        let a = intp.pop()?;
        intp.push(b);
        intp.push(c);
        intp.push(a);
        Ok(())
    });
}

fn boolean_primitives(e: &mut HashMap<Symbol, Binding>) {
    primitive(e, "%b.", |intp| {
        println!("{}", intp.pop_bool()?);
        Ok(())
    });

    primitive(e, "%bb=", |intp| {
        let b = intp.pop_bool()?;
        let a = intp.pop_bool()?;
        intp.push_bool(a == b);
        Ok(())
    });

    primitive(e, "%ii&", |intp| {
        let b = intp.pop_bool()?;
        let a = intp.pop_bool()?;
        intp.push_bool(a && b);
        Ok(())
    });

    primitive(e, "%ii|", |intp| {
        let b = intp.pop_bool()?;
        let a = intp.pop_bool()?;
        intp.push_bool(a || b);
        Ok(())
    });
}

fn integer_primitives(e: &mut HashMap<Symbol, Binding>) {
    primitive(e, "%i.", |intp| {
        println!("{}", intp.pop_int()?);
        Ok(())
    });

    primitive(e, "%ii=", |intp| {
        let b = intp.pop_int()?;
        let a = intp.pop_int()?;
        intp.push_bool(a == b);
        Ok(())
    });

    primitive(e, "%ii<", |intp| {
        let b = intp.pop_int()?;
        let a = intp.pop_int()?;
        intp.push_bool(a < b);
        Ok(())
    });

    primitive(e, "%ii+", |intp| {
        let b = intp.pop_int()?;
        let a = intp.pop_int()?;
        intp.push_int(a + b);
        Ok(())
    });

    primitive(e, "%ii-", |intp| {
        let b = intp.pop_int()?;
        let a = intp.pop_int()?;
        intp.push_int(a - b);
        Ok(())
    });

    primitive(e, "%ii*", |intp| {
        let b = intp.pop_int()?;
        let a = intp.pop_int()?;
        intp.push_int(a * b);
        Ok(())
    });

    primitive(e, "%ii/", |intp| {
        let b = intp.pop_int()?;
        let a = intp.pop_int()?;
        intp.push_int(a / b);
        Ok(())
    });
}

fn float_primitives(e: &mut HashMap<Symbol, Binding>) {
    primitive(e, "%f.", |intp| {
        println!("{}", intp.pop_flt()?);
        Ok(())
    });

    primitive(e, "%ff=", |intp| {
        let b = intp.pop_flt()?;
        let a = intp.pop_flt()?;
        intp.push_bool(a == b);
        Ok(())
    });

    primitive(e, "%ff<", |intp| {
        let b = intp.pop_flt()?;
        let a = intp.pop_flt()?;
        intp.push_bool(a < b);
        Ok(())
    });

    primitive(e, "%ff+", |intp| {
        let b = intp.pop_flt()?;
        let a = intp.pop_flt()?;
        intp.push_flt(a + b);
        Ok(())
    });

    primitive(e, "%ff-", |intp| {
        let b = intp.pop_flt()?;
        let a = intp.pop_flt()?;
        intp.push_flt(a - b);
        Ok(())
    });

    primitive(e, "%ff*", |intp| {
        let b = intp.pop_flt()?;
        let a = intp.pop_flt()?;
        intp.push_flt(a * b);
        Ok(())
    });

    primitive(e, "%ff/", |intp| {
        let b = intp.pop_flt()?;
        let a = intp.pop_flt()?;
        intp.push_flt(a / b);
        Ok(())
    });
}

fn string_primitives(e: &mut HashMap<Symbol, Binding>) {
    primitive(e, "%s.", |intp| {
        println!("{:?}", intp.pop_str()?);
        Ok(())
    });

    primitive(e, "%ss=", |intp| {
        let b = intp.pop_str()?;
        let a = intp.pop_str()?;
        intp.push_bool(a == b);
        Ok(())
    });
}

fn primitive(
    env: &mut HashMap<Symbol, Binding>,
    name: &'static str,
    fun: fn(&mut Interpreter) -> Result<()>,
) {
    env.insert(Symbol::from_static(name), Binding::Primitive(fun));
}
