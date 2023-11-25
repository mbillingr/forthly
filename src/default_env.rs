use crate::errors::Result;
use crate::interpreter::{Binding, Interpreter};
use crate::symbol::Symbol;
use std::collections::HashMap;

pub fn default_env() -> HashMap<Symbol, Binding> {
    let mut env = HashMap::new();
    let e = &mut env;

    housekeeping_primitives(e);
    stackop_primitives(e);
    generic_primitives(e);
    integer_primitives(e);
    boolean_primitives(e);
    float_primitives(e);
    string_primitives(e);
    symbol_primitives(e);

    env
}

fn housekeeping_primitives(e: &mut HashMap<Symbol, Binding>) {
    primitive(e, "%error", |intp| {
        let msg = intp.pop_str()?;
        Err(msg.to_string())
    });

    primitive(e, "%apply", |intp| {
        let ops = intp.pop_ops()?;
        intp.exec(&ops)
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
            print!("{:>40} ", name.to_string());
            match &intp.env[name] {
                Binding::Primitive(_) => println!("|"),
                Binding::Composite(methods) => {
                    let methods = methods.read().unwrap();
                    let n = methods.len();
                    for (i, method) in methods.iter().enumerate() {
                        println!("| {:>30} | {}", method.effect.to_string(), method.doc);
                        if i < n - 1 {
                            print!("{:<40} ", "")
                        }
                    }
                }
            };
        }
        Ok(())
    });
}

fn stackop_primitives(e: &mut HashMap<Symbol, Binding>) {
    primitive(e, "%>>", |intp| {
        let x = intp.pop()?;
        intp.secondary_stack.push(x);
        Ok(())
    });

    primitive(e, "%<<", |intp| {
        let x = intp
            .secondary_stack
            .pop()
            .ok_or_else(|| format!("pop from empty second stack"))?;
        intp.push(x);
        Ok(())
    });

    primitive(e, "%@", |intp| {
        let idx = intp.pop_int()? as usize;
        let x = intp
            .secondary_stack
            .get(intp.secondary_stack.len() - 1 - idx)
            .ok_or_else(|| format!("index out of bounds"))?
            .clone();
        intp.push(x);
        Ok(())
    });

    primitive(e, "%drop", |intp| {
        let _ = intp.pop()?;
        Ok(())
    });

    primitive(e, "%dup", |intp| {
        let x = intp.pop()?;
        intp.push(x.clone());
        intp.push(x);
        Ok(())
    });

    primitive(e, "%over", |intp| {
        let b = intp.pop()?;
        let a = intp.pop()?;
        intp.push(a.clone());
        intp.push(b);
        intp.push(a);
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

fn generic_primitives(e: &mut HashMap<Symbol, Binding>) {
    primitive(e, "%.", |intp| {
        let x = intp.pop()?;
        println!("{x:?}");
        Ok(())
    });

    primitive(e, "%.=", |intp| {
        let b = intp.pop()?;
        let a = intp.pop()?;
        intp.push_bool(a == b);
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

    primitive(e, "%bb&", |intp| {
        let b = intp.pop_bool()?;
        let a = intp.pop_bool()?;
        intp.push_bool(a && b);
        Ok(())
    });

    primitive(e, "%bb|", |intp| {
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

    primitive(e, "%i->f", |intp| {
        let x = intp.pop_int()?;
        intp.push_flt(x as f64);
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

    primitive(e, "%fsqrt", |intp| {
        let x = intp.pop_flt()?;
        intp.push_flt(x.sqrt());
        Ok(())
    });

    primitive(e, "%flog", |intp| {
        let x = intp.pop_flt()?;
        intp.push_flt(x.ln());
        Ok(())
    });

    primitive(e, "%fsin", |intp| {
        let x = intp.pop_flt()?;
        intp.push_flt(x.sin());
        Ok(())
    });

    primitive(e, "%fcos", |intp| {
        let x = intp.pop_flt()?;
        intp.push_flt(x.cos());
        Ok(())
    });
}

fn string_primitives(e: &mut HashMap<Symbol, Binding>) {
    primitive(e, "%s.", |intp| {
        println!("{:?}", intp.pop_str()?);
        Ok(())
    });
    primitive(e, "%println", |intp| {
        println!("{}", intp.pop_str()?);
        Ok(())
    });

    primitive(e, "%ss=", |intp| {
        let b = intp.pop_str()?;
        let a = intp.pop_str()?;
        intp.push_bool(a == b);
        Ok(())
    });

    primitive(e, "%fmt", |intp| {
        let fmt_str = intp.pop_str()?;
        let mut fmt_str = fmt_str.chars();
        let mut str_out = String::new();
        while let Some(ch) = fmt_str.next() {
            if ch != '%' {
                str_out.push(ch);
                continue;
            }
            match fmt_str.next() {
                Some('%') => str_out.push('%'),
                Some('i') => str_out += &intp.pop_int()?.to_string(),
                Some('f') => str_out += &intp.pop_flt()?.to_string(),
                Some('s') => str_out += &intp.pop_str()?,
                _ => {}
            }
        }
        intp.push_str(str_out);
        Ok(())
    })
}

fn symbol_primitives(e: &mut HashMap<Symbol, Binding>) {
    primitive(e, "%'.", |intp| {
        println!("{:?}", intp.pop_sym()?);
        Ok(())
    });

    primitive(e, "%''=", |intp| {
        let b = intp.pop_sym()?;
        let a = intp.pop_sym()?;
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
