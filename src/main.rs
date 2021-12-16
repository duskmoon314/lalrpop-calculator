#[macro_use]
extern crate lalrpop_util;

use rustyline::error::ReadlineError;
use rustyline::Editor;

mod ast;
lalrpop_mod!(calculator);

fn main() {
    let mut rl = Editor::<()>::new();
    let cal = calculator::ExprParser::new();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                println!("{}", cal.parse(line.as_str()).unwrap().eval());
            }
            Err(ReadlineError::Interrupted) => {
                println!("Interrupted CTRL-C: Bye!");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
}
