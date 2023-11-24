mod conversions;
mod default_env;
mod errors;
mod interpreter;
mod parser;
mod serialize;
mod symbol;
mod value;

use crate::errors::Result;
use crate::interpreter::Interpreter;
use crate::parser::parse;
use reedline::{DefaultPrompt, Reedline, Signal};

fn main() {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    let mut interpreter = Interpreter::default();

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => match eval(&buffer, &mut interpreter) {
                Ok(()) => {}
                Err(e) => println!("Error: {}", e),
            },
            Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
                println!("\nBye!");
                break;
            }
            x => {
                println!("Signal: {:?}", x);
            }
        }
    }
}

fn eval(src: &str, interpreter: &mut Interpreter) -> Result<()> {
    let ops = parse(&src)?;
    interpreter.exec(&ops)
}
