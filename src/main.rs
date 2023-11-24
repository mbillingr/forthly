mod interpreter;
mod symbol;

use reedline::{DefaultPrompt, Reedline, Signal};
use crate::interpreter::Interpreter;

fn main() {
    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

    let mut interpreter = Interpreter::new();

    loop {
        let sig = line_editor.read_line(&prompt);
        match sig {
        Ok(Signal::Success(buffer)) => {
            println!("We processed: {}", buffer);
        }
        Ok(Signal::CtrlD) | Ok(Signal::CtrlC) => {
            println!("\nAborted!");
            break;
        }
        x => {
            println!("Event: {:?}", x);
        }
    }
    }

}
