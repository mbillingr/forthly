use crate::errors::Result;
use crate::interpreter::{Op, StackEffect};
use crate::symbol::Symbol;
use crate::value::Value;
use std::sync::Arc;

pub fn parse(src: &str) -> Result<Vec<Op>> {
    let mut tokens = Tokenizer { input: src };

    let mut ops = vec![];
    while let Some(token) = tokens.next() {
        if token == "" {
            continue;
        } else if token == "(" {
            ops.push(parse_stack_effect(&mut tokens)?);
        } else {
            let op = parse_op(token)?;
            ops.push(op);
        }
    }

    Ok(ops)
}

fn parse_op(token: &str) -> Result<Op> {
    Ok(match token {
        ";" => Op::End,
        ":" => Op::BeginDef,
        _ if token.starts_with('"') => {
            Op::Literal(Value::Str(token.trim_matches('"').to_string().into()))
        }
        _ => {
            if let Ok(x) = token.parse() {
                Op::Literal(Value::Int(x))
            } else if let Ok(x) = token.parse() {
                Op::Literal(Value::Flt(x))
            } else {
                Op::Symbol(Symbol::new(token))
            }
        }
    })
}

fn parse_stack_effect<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<Op> {
    let mut pre = vec![];
    loop {
        match next_token(tokens)? {
            "--" => break,
            token => pre.push(Symbol::new(token)),
        }
    }

    let mut post = vec![];
    loop {
        match next_token(tokens)? {
            ")" => break,
            token => post.push(Symbol::new(token)),
        }
    }

    Ok(Op::Effect(Arc::new(StackEffect { pre, post })))
}

fn next_token<'a>(tokens: &mut impl Iterator<Item = &'a str>) -> Result<&'a str> {
    tokens
        .next()
        .ok_or_else(|| format!("Unexpected End of Input"))
}

struct Tokenizer<'i> {
    input: &'i str,
}

impl<'i> Iterator for Tokenizer<'i> {
    type Item = &'i str;

    fn next(&mut self) -> Option<Self::Item> {
        let input = self.input.trim_start();
        if input.is_empty() {
            return None;
        }

        let (token, rest) = if input.starts_with('"') {
            let idx = 2 + input.strip_prefix('"').unwrap().find('"')?;
            (&input[..idx], &input[idx..])
        } else {
            match input.split_once(char::is_whitespace) {
                None => (input, &input[0..0]),
                Some((t, r)) => (t, r),
            }
        };

        self.input = rest;
        Some(token)
    }
}
