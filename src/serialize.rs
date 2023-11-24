use crate::interpreter::{Op, StackEffect};
use crate::value::Value;
use std::fmt::{Display, Formatter};

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(x) => write!(f, "{x}"),
            Value::Flt(x) => write!(f, "{x}"),
            Value::Str(x) => write!(f, "{x:?}"),
        }
    }
}

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Op::Literal(value) => value.fmt(f),
            Op::Symbol(name) => name.fmt(f),
            Op::BeginDef => write!(f, ":"),
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
