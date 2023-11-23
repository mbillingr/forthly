use std::collections::HashSet;
use lazy_static::lazy_static;
use std::sync::RwLock;

lazy_static! {
    static ref symbols: RwLock<HashSet<&'static str>> = Default::default();
}

#[derive(Clone, Copy, Debug, Eq, Hash)]
pub struct Symbol(&'static str);


impl Symbol {
    pub fn new(s: &str) -> Self {
        if let Some(sym) = Self::get_interned(s) {
            return Symbol(sym)
        }

        let sym = Box::leak(s.to_string().into_boxed_str());

        symbols.write().expect("could not write to symbol registry").insert(sym);
        Symbol(Self::get_interned(s).unwrap())
    }
    pub fn from_static(sym: &'static str) -> Self {
        if let Some(sym) = Self::get_interned(sym) {
            return Symbol(sym)
        }

        Self::set_interned(sym);
        Symbol(sym)
    }

    fn get_interned(s: &str) -> Option<&'static str> {
        symbols.read().expect("could not read from symbol registry").get(s).copied()
    }

    fn set_interned(sym: &'static str) {
        symbols.write().expect("could not write to symbol registry").insert(sym);
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

