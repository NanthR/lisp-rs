use rustyline::error::ReadlineError;
use rustyline::Editor;
use types::Types;
pub mod core;
pub mod env;
pub mod evaluator;
pub mod printer;
mod reader;
pub mod types;
use env::{Env, EnvFunc};
use evaluator::eval;

pub fn read(line: &str) -> Types {
    let mut tokenizer = reader::Tokenizer::new(line);
    tokenizer.read_str()
}

fn print(ast: Types) -> String {
    printer::pr_str(ast, true)
}

pub fn rep(line: &str, env: &Env) -> Result<Types, String> {
    let ast = read(line);
    eval(ast, env.clone())
}

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history(".rlisp.history").is_err() {
        println!("No previous history");
    };

    let env: Env = EnvFunc::new(None);
    // Set up core functions
    env.set_core_func();
    rep("(def! not (fn* (a) (if a false true)))", &env).unwrap();

    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                let x = rep(&line, &env);
                match x {
                    Ok(x) => println!("{}", print(x)),
                    Err(x) => println!("{}", x),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Exiting");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Exiting");
                break;
            }
            Err(err) => {
                println!("{:?}", err);
                break;
            }
        }
    }
    if rl.save_history(".rlisp.history").is_err() {
        println!("Error saving history");
    } else {
        println!("Suc");
    }
}
