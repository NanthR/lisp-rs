use rustyline::error::ReadlineError;
use rustyline::Editor;
use types::Types;
pub mod core;
pub mod env;
pub mod evaluator;
pub mod printer;
mod reader;
pub mod types;
use env::Environment;
use evaluator::eval;

fn read(line: &str) -> Types {
    let mut tokenizer = reader::Tokenizer::new(line);
    tokenizer.read_str()
}

fn print(ast: Types) -> String {
    printer::pr_str(ast, true)
}

fn rep(line: &str, env: &mut Environment) -> Result<Types, String> {
    let ast = read(line);
    eval(ast, env)
}

fn main() {
    let mut rl = Editor::<()>::new();
    if rl.load_history(".rlisp.history").is_err() {
        println!("No previous history");
    };

    let mut env = Environment::new();
    env.set_core_func();
    rep("(def! not (fn* (a) (if a false true)))", &mut env).unwrap();

    loop {
        let readline = rl.readline("user> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(&line);
                let x = rep(&line, &mut env);
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
