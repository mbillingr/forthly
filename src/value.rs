use crate::errors::Result;
use crate::interpreter::Op;
use crate::symbol::Symbol;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Value {
    True,
    False,
    Int(i64),
    Flt(f64),
    Str(Arc<String>),
    Symbol(Symbol),
    Tuple(Arc<Vec<Value>>),
    Block(Arc<[Op]>),
}

impl Value {
    pub fn get_type(&self) -> Symbol {
        match self {
            Value::True => Symbol::from_static("Bln"),
            Value::False => Symbol::from_static("Bln"),
            Value::Int(_) => Symbol::from_static("Int"),
            Value::Flt(_) => Symbol::from_static("Flt"),
            Value::Str(_) => Symbol::from_static("Str"),
            Value::Symbol(_) => Symbol::from_static("Sym"),
            Value::Block(_) => Symbol::from_static("Ops"),
            Value::Tuple(fields) => match fields.as_slice() {
                [Value::Symbol(tag), ..] => tag.clone(),
                _ => panic!("invalid tuple"),
            },
        }
    }

    pub fn expect_bool(self) -> Result<bool> {
        match self {
            Value::True => Ok(true),
            Value::False => Ok(false),
            _ => Err(format!(
                "Found a {} where Bln was expected",
                self.get_type()
            )),
        }
    }

    pub fn expect_int(self) -> Result<i64> {
        match self {
            Value::Int(x) => Ok(x),
            _ => Err(format!(
                "Found a {} where Int was expected",
                self.get_type()
            )),
        }
    }

    pub fn expect_float(self) -> Result<f64> {
        match self {
            Value::Flt(x) => Ok(x),
            _ => Err(format!(
                "Found a {} where Flt was expected",
                self.get_type()
            )),
        }
    }

    pub fn expect_string(self) -> Result<Arc<String>> {
        match self {
            Value::Str(x) => Ok(x),
            _ => Err(format!(
                "Found a {} where Str was expected",
                self.get_type()
            )),
        }
    }
}
