use lazy_static::lazy_static;
use std::collections::HashSet;
use std::fmt::Formatter;
use std::sync::RwLock;

// predefined symbols
/*pub const INT: Symbol = Symbol("Int");
pub const FLT: Symbol = Symbol("Flt");
pub const STR: Symbol = Symbol("Str");*/

lazy_static! {
    static ref SYMBOLS: RwLock<HashSet<&'static str>> = {
        let symbols = HashSet::new();
        /*symbols.insert(INT.0);
        symbols.insert(FLT.0);
        symbols.insert(STR.0);*/
        RwLock::new(symbols)
    };
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialOrd)]
pub struct Symbol(pub &'static str);

impl Symbol {
    pub fn new(s: &str) -> Self {
        if let Some(sym) = Self::get_interned(s) {
            return Symbol(sym);
        }

        let sym = Box::leak(s.to_string().into_boxed_str());

        Self::set_interned(sym);
        Symbol(Self::get_interned(s).unwrap())
    }
    pub fn from_static(sym: &'static str) -> Self {
        if let Some(sym) = Self::get_interned(sym) {
            return Symbol(sym);
        }

        Self::set_interned(sym);
        Symbol(sym)
    }

    fn get_interned(s: &str) -> Option<&'static str> {
        SYMBOLS
            .read()
            .expect("could not read from symbol registry")
            .get(s)
            .copied()
    }

    fn set_interned(sym: &'static str) {
        SYMBOLS
            .write()
            .expect("could not write to symbol registry")
            .insert(sym);
    }

    pub fn is_type(&self) -> bool {
        self.0
            .chars()
            .next()
            .map(char::is_uppercase)
            .unwrap_or(false)
    }
}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

impl std::fmt::Display for Symbol {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
