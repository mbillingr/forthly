use std::collections::HashMap;
use std::sync::Arc;
use crate::symbol::Symbol;

pub struct Interpreter {
    main_stack: Vec<Value>,
    secondary_stack: Vec<Value>,
    env: HashMap<Symbol, Binding>,
}

#[derive(Clone, Debug)]
pub enum Value {

}

pub enum Binding {
    Primitive(fn(&mut Interpreter)),
    Composite(Arc<[Op]>)
}

#[derive(Debug)]
pub enum Op {
    Literal(Value)
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            main_stack: vec![],
            secondary_stack: vec![],
            env: Default::default(),
        }
    }

    pub fn exec(&mut self, ops: &[Op]) {
        for op in ops {
            match op {
                _ => todo!("{op:?}")
            }
        }
    }
}