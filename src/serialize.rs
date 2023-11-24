use crate::interpreter::{Op, StackEffect};
use crate::value::Value;
use std::fmt::{Display, Formatter};

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::True => write!(f, "true"),
            Value::False => write!(f, "false"),
            Value::Int(x) => write!(f, "{x}"),
            Value::Flt(x) => write!(f, "{x}"),
            Value::Str(x) => write!(f, "{x:?}"),
            Value::Symbol(s) => write!(f, "{s}"),
            Value::Tuple(_) => unimplemented!(),
            Value::Block(ops) => {
                write!(f, "[")?;
                for op in ops.iter() {
                    write!(f, " {op}")?;
                }
                write!(f, " ]")
            }
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Literal(value) => value.fmt(f),
            Op::Symbol(name) => name.fmt(f),
            Op::Tuple(_) => unimplemented!(),
            Op::If => write!(f, "if"),
            Op::BeginDef => write!(f, ":"),
            Op::BeginTypeDef => write!(f, ":t"),
            Op::End => write!(f, ";"),
            Op::Effect(effect) => effect.fmt(f),
        }
    }
}

impl Display for StackEffect {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;

        for p in &self.pre {
            write!(f, " {p}")?;
        }

        write!(f, " --")?;

        for p in &self.post {
            write!(f, " {p}")?;
        }

        write!(f, " )")
    }
}

pub struct DisplayBlock<'a>(pub &'a [Op]);
impl Display for DisplayBlock<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut ops = self.0.iter();
        if let Some(op) = ops.next() {
            write!(f, "{op}")?;
        }
        for op in ops {
            write!(f, " {op}")?;
        }
        Ok(())
    }
}
