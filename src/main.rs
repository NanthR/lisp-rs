use rustyline::error::ReadlineError;
use rustyline::Editor;
use types::Types;
pub mod core;
pub mod env;
pub mod evaluator;
pub mod printer;
mod reader;
pub mod types;
use clap::{App, Arg, SubCommand};
use env::{Env, EnvFunc};
use evaluator::{eval, load_file};

fn setup_command_line_args(args: &Vec<&str>, env: &Env) -> Result<(), String> {
    if args.len() % 2 != 0 {
        Err("Odd number of arguements".to_string())
    } else {
        let mut binds = vec![];
        let mut exprs = vec![];
        let mut m = args.chunks(2);
        while let Some(x) = m.next() {
            binds.push(read(x[0]));
            exprs.push(read(x[1]));
        }
        let mut x = binds.iter().zip(exprs.iter());
        let mut res = vec![];
        while let Some((i, j)) = x.next() {
            res.push(i.clone());
            res.push(j.clone());
        }
        env.env_bind(Types::List(binds), exprs)?;
        env.env_set("*ARGV*", Types::List(res));
        Ok(())
    }
}

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

fn main() -> Result<(), String> {
    let matches = App::new("Rust-Lisp")
        .arg(Arg::with_name("file").short("f").value_name("file"))
        .arg(
            Arg::with_name("args")
                .short("a")
                .multiple(true)
                .value_name("args"),
        )
        .get_matches();

    let mut rl = Editor::<()>::new();
    if rl.load_history(".rlisp.history").is_err() {
        println!("No previous history");
    };

    let mut env: Env = EnvFunc::new(None);
    // Set up core functions
    env.set_core_func();
    rep("(def! not (fn* (a) (if a false true)))", &env).unwrap();
    env.env_set("*ARGV*", Types::List(vec![]));

    if let Some(args) = matches.values_of("args") {
        let args = args.collect::<Vec<&str>>();
        if let Err(x) = setup_command_line_args(&args, &env) {
            return Err(x);
        }
    }

    if let Some(filename) = matches.value_of("file") {
        match load_file(filename.to_string(), &mut env) {
            Ok(_) => {},
            Err(x) => {
                println!("{}", x);
            }
        }
    }

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
    Ok(())
}
