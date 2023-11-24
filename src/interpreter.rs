use crate::default_env::default_env;
use crate::errors::Result;
use crate::symbol::Symbol;
use crate::value::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub trait ExecutionContext {
    fn define_word<'a>(&mut self, ops: &mut impl Iterator<Item = &'a Op>) -> Result<()>;
}

pub struct Interpreter {
    main_stack: Vec<Value>,
    secondary_stack: Vec<Value>,
    env: HashMap<Symbol, Binding>,
}

#[derive(Debug)]
pub enum Binding {
    Primitive(fn(&mut Interpreter) -> Result<()>),
    Composite(Arc<RwLock<Vec<Method>>>),
}

#[derive(Debug)]
pub struct Method {
    effect: Arc<StackEffect>,
    body: Arc<[Op]>,
}

#[derive(Clone, Debug)]
pub enum Op {
    Literal(Value),
    Symbol(Symbol),

    BeginDef,
    End,

    Effect(Arc<StackEffect>),
}

#[derive(Debug)]
pub struct StackEffect {
    pub pre: Vec<Symbol>,
    pub post: Vec<Symbol>,
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter {
            main_stack: vec![],
            secondary_stack: vec![],
            env: default_env(),
        }
    }
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            main_stack: vec![],
            secondary_stack: vec![],
            env: Default::default(),
        }
    }

    pub fn exec(&mut self, ops: &[Op]) -> Result<()> {
        println!("exec {ops:?}");
        let mut ops = ops.iter();
        while let Some(op) = ops.next() {
            match op {
                Op::Literal(value) => self.main_stack.push(value.clone()),
                Op::Symbol(name) => match self.lookup(*name)? {
                    Binding::Primitive(prim) => prim(self)?,
                    Binding::Composite(methods) => {
                        let body = self
                            .find_matching_method(&*methods.read().unwrap())
                            .map_err(|e| format!("{e} for {name}"))?;
                        self.exec(&body)?;
                    }
                },
                Op::End => return Err(format!("Unexpected {}", op)),
                Op::BeginDef => self.define_word(&mut ops)?,
                _ => todo!("{op:?}"),
            }
        }
        Ok(())
    }

    fn find_matching_method(&self, methods: &[Method]) -> Result<Arc<[Op]>> {
        'method: for Method { effect, body } in methods.iter().rev() {
            if effect.pre.len() > self.main_stack.len() {
                continue;
            }

            for (e, s) in effect.pre.iter().rev().zip(self.main_stack.iter().rev()) {
                println!(
                    "{e:?} {s:?}, {}, {:p}, {:p}",
                    s.get_type() == *e,
                    e.0,
                    s.get_type().0
                );
                if e.is_type() && s.get_type() != *e {
                    continue 'method;
                }
            }

            return Ok(body.clone());
        }
        Err(format!("found no matching method"))
    }

    pub fn lookup(&self, name: Symbol) -> Result<&Binding> {
        self.env
            .get(&name)
            .ok_or_else(|| format!("Unknown {}", name))
    }

    pub fn pop(&mut self) -> Result<Value> {
        self.main_stack
            .pop()
            .ok_or_else(|| format!("Pop from empty stack"))
    }

    pub fn pop_int(&mut self) -> Result<i64> {
        self.pop()?.expect_int()
    }

    pub fn pop_flt(&mut self) -> Result<f64> {
        self.pop()?.expect_float()
    }

    pub fn pop_str(&mut self) -> Result<Arc<String>> {
        self.pop()?.expect_string()
    }

    pub fn push(&mut self, value: Value) {
        self.main_stack.push(value)
    }

    pub fn push_int(&mut self, value: i64) {
        self.main_stack.push(Value::Int(value))
    }

    pub fn push_flt(&mut self, value: f64) {
        self.main_stack.push(Value::Flt(value))
    }

    pub fn push_str(&mut self, value: String) {
        self.main_stack.push(Value::Str(Arc::new(value)))
    }

    fn parse_func<'a>(&self, ops: &mut impl Iterator<Item = &'a Op>) -> Result<(Symbol, Method)> {
        let name = match ops.next() {
            Some(Op::Symbol(Symbol(name))) if name.starts_with(':') || name.starts_with('%') => {
                return Err(format!("User definitions may not start with : or %"))
            }
            Some(Op::Symbol(name)) => *name,
            _ => return Err(format!("Expected name")),
        };

        let effect = match ops.next() {
            Some(Op::Effect(effect)) => effect.clone(),
            _ => return Err(format!("Expected stack effect declaration")),
        };

        let mut body = vec![];
        loop {
            match ops.next() {
                None => return Err(format!("Undelimited function definition")),
                Some(Op::End) => break,
                Some(op) => body.push(op.clone()),
            }
        }

        Ok((
            name,
            Method {
                effect,
                body: body.into(),
            },
        ))
    }
}

impl ExecutionContext for Interpreter {
    fn define_word<'a>(&mut self, ops: &mut impl Iterator<Item = &'a Op>) -> Result<()> {
        let (name, method) = self.parse_func(ops)?;

        match self
            .env
            .entry(name)
            .or_insert(Binding::Composite(Arc::new(RwLock::new(vec![]))))
        {
            Binding::Primitive(_) => return Err(format!("cannot redefine primitive {name}")),
            Binding::Composite(methods) => methods.write().unwrap().push(method),
        }

        Ok(())
    }
}
