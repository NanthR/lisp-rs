use crate::printer::pr_str;
use crate::read;
use crate::types::Types;
use std::fs;
// use std::io::{BufRead, BufReader};
use std::cell::RefCell;
use std::rc::Rc;

type Args = Vec<Types>;

fn check_len(a: &Args, b: usize) -> Result<(), String> {
    if a.len() < b {
        Err("Not enough arguements".to_string())
    } else {
        Ok(())
    }
}

fn create_string(a: &Args, print_readably: bool) -> Vec<String> {
    a.iter()
        .map(|i| pr_str(i.clone(), print_readably))
        .collect::<Vec<String>>()
}

pub fn namespace() -> Vec<(&'static str, Types)> {
    vec![
        (
            "+",
            Types::Func(|a| {
                match &a[0] {
                    Types::Int(_) => {
                        let res = a.iter().try_fold(0, |acc, x| match x {
                            Types::Int(y) => Ok(acc + y),
                            _ => Err("Invalid arguements".to_string()),
                        });
                        match res {
                            Ok(x) => Ok(Types::Int(x)),
                            Err(_) => Err("Invalid".to_string()),
                        }
                    }
                    Types::Float(_) => {
                        let res = a.iter().try_fold(0., |acc, x| {
                            match x {
                                Types::Float(y) => Ok(acc + y),
                                // Types::Int(y) => Ok(acc + y),
                                _ => Err("Invalid arguements".to_string()),
                            }
                        });
                        match res {
                            Ok(x) => Ok(Types::Float(x)),
                            Err(_) => Err("Invalid".to_string()),
                        }
                    }
                    _ => Err("Invalid".to_string()),
                }
            }),
        ),
        (
            "-",
            Types::Func(|a| {
                let st = a.remove(0);
                match st {
                    Types::Int(z) => {
                        let res = a.iter().try_fold(z, |acc, x| match x {
                            Types::Int(y) => Ok(acc - y),
                            _ => Err("Invalid arguements".to_string()),
                        });
                        match res {
                            Ok(x) => Ok(Types::Int(x)),
                            Err(_) => Err("Invalid".to_string()),
                        }
                    }
                    Types::Float(z) => {
                        let res = a.iter().try_fold(z, |acc, x| match x {
                            Types::Float(y) => Ok(acc - y),
                            _ => Err("Invalid arguements".to_string()),
                        });
                        match res {
                            Ok(x) => Ok(Types::Float(x)),
                            Err(_) => Err("Invalid".to_string()),
                        }
                    }
                    _ => Err("Invalid".to_string()),
                }
            }),
        ),
        (
            "*",
            Types::Func(|a| {
                match &a[0] {
                    Types::Int(_) => {
                        let res = a.iter().try_fold(1, |acc, x| match x {
                            Types::Int(y) => Ok(acc * y),
                            _ => Err("Invalid arguements".to_string()),
                        });
                        match res {
                            Ok(x) => Ok(Types::Int(x)),
                            Err(_) => Err("Invalid".to_string()),
                        }
                    }
                    Types::Float(_) => {
                        let res = a.iter().try_fold(1., |acc, x| {
                            match x {
                                Types::Float(y) => Ok(acc * y),
                                // Types::Int(y) => Ok(acc + y),
                                _ => Err("Invalid arguements".to_string()),
                            }
                        });
                        match res {
                            Ok(x) => Ok(Types::Float(x)),
                            Err(_) => Err("Invalid".to_string()),
                        }
                    }
                    _ => Err("Invalid".to_string()),
                }
            }),
        ),
        (
            "/",
            Types::Func(|a| {
                match &a[0] {
                    Types::Int(_) => {
                        let res = a.iter().try_fold(0, |acc, x| match x {
                            Types::Int(y) => Ok(acc / y),
                            _ => Err("Invalid arguements".to_string()),
                        });
                        match res {
                            Ok(x) => Ok(Types::Int(x)),
                            Err(_) => Err("Invalid".to_string()),
                        }
                    }
                    Types::Float(_) => {
                        let res = a.iter().try_fold(0., |acc, x| {
                            match x {
                                Types::Float(y) => Ok(acc / y),
                                // Types::Int(y) => Ok(acc + y),
                                _ => Err("Invalid arguements".to_string()),
                            }
                        });
                        match res {
                            Ok(x) => Ok(Types::Float(x)),
                            Err(_) => Err("Invalid".to_string()),
                        }
                    }
                    _ => Err("Invalid".to_string()),
                }
            }),
        ),
        (
            "=",
            Types::Func(|a: &mut Args| {
                check_len(a, 2)?;
                Ok(Types::Bool(a[0] == a[1]))
            }),
        ),
        (
            "count",
            Types::Func(|a: &mut Args| match &a[0] {
                Types::List(x) | Types::Vector(x) => Ok(Types::Int(x.len() as i64)),
                Types::Nil() => Ok(Types::Int(0)),
                _ => Ok(Types::Int(1)),
            }),
        ),
        (
            "list",
            Types::Func(|a: &mut Args| Ok(Types::List(a.to_vec()))),
        ),
        (
            "list?",
            Types::Func(|a: &mut Args| {
                check_len(a, 1)?;
                match &a[0] {
                    Types::List(_) => Ok(Types::Bool(true)),
                    _ => Ok(Types::Bool(false)),
                }
            }),
        ),
        (
            "empty?",
            Types::Func(|a: &mut Args| {
                check_len(a, 1)?;
                match &a[0] {
                    Types::List(x) | Types::Vector(x) => Ok(Types::Bool(x.len() == 0)),
                    _ => Ok(Types::Bool(true)),
                }
            }),
        ),
        (
            "<",
            Types::Func(|a: &mut Args| {
                check_len(a, 2)?;
                match (&a[0], &a[1]) {
                    (Types::Int(x), Types::Int(y)) => Ok(Types::Bool(x < y)),
                    (Types::Float(x), Types::Float(y)) => Ok(Types::Bool(x < y)),
                    _ => Err("Cannot be compared".to_string()),
                }
            }),
        ),
        (
            ">",
            Types::Func(|a: &mut Args| {
                check_len(a, 2)?;
                match (&a[0], &a[1]) {
                    (Types::Int(x), Types::Int(y)) => Ok(Types::Bool(x > y)),
                    (Types::Float(x), Types::Float(y)) => Ok(Types::Bool(x > y)),
                    _ => Err("Cannot be compared".to_string()),
                }
            }),
        ),
        (
            ">=",
            Types::Func(|a: &mut Args| {
                check_len(a, 2)?;
                match (&a[0], &a[1]) {
                    (Types::Int(x), Types::Int(y)) => Ok(Types::Bool(x >= y)),
                    (Types::Float(x), Types::Float(y)) => Ok(Types::Bool(x >= y)),
                    _ => Err("Cannot be compared".to_string()),
                }
            }),
        ),
        (
            "<=",
            Types::Func(|a: &mut Args| {
                check_len(a, 2)?;
                match (&a[0], &a[1]) {
                    (Types::Int(x), Types::Int(y)) => Ok(Types::Bool(x <= y)),
                    (Types::Float(x), Types::Float(y)) => Ok(Types::Bool(x <= y)),
                    _ => Err("Cannot be compared".to_string()),
                }
            }),
        ),
        (
            "prn",
            Types::Func(|a: &mut Args| {
                println!("{}", create_string(a, true).join(" "));
                Ok(Types::Nil())
            }),
        ),
        (
            "str",
            Types::Func(|a| Ok(Types::Simple(create_string(a, false).join("")))),
        ),
        (
            "pr-str",
            Types::Func(|a| Ok(Types::Simple(create_string(a, true).join(" ")))),
        ),
        (
            "println",
            Types::Func(|a| {
                println!("{}", create_string(a, false).join(" "));
                Ok(Types::Nil())
            }),
        ),
        (
            "slurp",
            Types::Func(|a| {
                if let Types::Simple(x) = &a[0] {
                    let data = fs::read_to_string(x).expect("Unable to read file");
                    Ok(Types::Simple(data))
                    // if let Ok(file) = File::open(x) {
                    //     let reader = BufReader::new(file);
                    //     Ok(Types::List(reader.lines().map(|i| Types::Simple(i.unwrap())).collect::<Vec<Types>>()))
                    // } else {
                    //     Err("File cannot be opened".to_string())
                    // }
                } else {
                    Ok(Types::Nil())
                }
            }),
        ),
        (
            "read-string",
            Types::Func(|a| {
                println!("{:?}", a);
                check_len(a, 1)?;
                if let Types::Simple(x) = &a[0] {
                    Ok(read(&x))
                } else {
                    Err("Not provided a string".to_string())
                }
            }),
        ),
        (
            "atom",
            Types::Func(|a| {
                check_len(a, 1)?;
                Ok(Types::Atom(Rc::new(RefCell::new(a[0].clone()))))
            }),
        ),
        (
            "atom?",
            Types::Func(|a| {
                check_len(a, 1)?;
                if let Types::Atom(_) = &a[0] {
                    Ok(Types::Bool(true))
                } else {
                    Ok(Types::Bool(false))
                }
            }),
        ),
        (
            "deref",
            Types::Func(|a| {
                check_len(a, 1)?;
                if let Types::Atom(x) = &a[0] {
                    Ok((*x.borrow()).clone())
                } else {
                    Err("Not an atom".to_string())
                }
            }),
        ),
        (
            "reset!",
            Types::Func(|a| {
                check_len(a, 2)?;
                if let Types::Atom(x) = a[0].clone() {
                    *x.borrow_mut() = a[1].clone();
                    Ok((*x.borrow()).clone())
                } else {
                    Err("Not an atom".to_string())
                }
            }),
        ),
    ]
}
