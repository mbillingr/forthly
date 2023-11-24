use crate::errors::Result;
use crate::symbol::Symbol;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Flt(f64),
    Str(Arc<String>),
}

impl Value {
    pub fn get_type(&self) -> Symbol {
        match self {
            Value::Int(_) => Symbol::from_static("Int"),
            Value::Flt(_) => Symbol::from_static("Flt"),
            Value::Str(_) => Symbol::from_static("Str"),
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
