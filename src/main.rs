mod interpreter;
mod symbol;

use reedline::{DefaultPrompt, Reedline, Signal};

fn main() {
    println!("Hello, world!");

    let mut line_editor = Reedline::create();
    let prompt = DefaultPrompt::default();

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
