use crate::env::{Env, EnvFunc};
use crate::rep;
use crate::types::Types;
use std::fs;

fn eval_ast(ast: Types, env: &Env) -> Result<Types, String> {
    match ast {
        Types::Symbol(_) => env.env_get(ast),
        Types::List(x) | Types::Vector(x) => {
            let mut res = vec![];
            for i in x {
                res.push(eval(i, env.clone())?);
            }
            Ok(Types::List(res))
        }
        Types::Dict(mut x) => {
            for (_, val) in x.iter_mut() {
                *val = eval(val.clone(), env.clone())?;
            }
            Ok(Types::Dict(x))
        }
        _ => Ok(ast),
    }
}

pub fn eval(mut ast: Types, mut env: Env) -> Result<Types, String> {
    let ret: Result<Types, String>;
    'tco: loop {
        ret = match ast {
            Types::List(ref res) => {
                if res.len() == 0 {
                    Ok(ast)
                } else {
                    match &res[0] {
                        Types::Symbol(x) if x == "def!" => {
                            if res.len() != 3 {
                                Err("Invalid definition".to_string())
                            } else {
                                if let Types::Symbol(z) = &res[1] {
                                    let fin = eval(res[2].clone(), env.clone())?;
                                    // println!("{:?}", fin);
                                    env.env_set(&z, fin.clone());
                                    Ok(fin)
                                } else {
                                    Err("Cannot bind to non-symbol".to_string())
                                }
                            }
                        }
                        Types::Symbol(x) if x == "let*" => {
                            if res.len() != 3 {
                                Err("Invalid let usage".to_string())
                            } else {
                                match &res[1] {
                                    Types::List(y) | Types::Vector(y) => {
                                        if y.len() % 2 != 0 {
                                            Err("Expected even number of elements in let bindings list"
                                            .to_string())
                                        } else {
                                            env = EnvFunc::new(Some(env));
                                            let mut cur = 0;
                                            while cur < y.len() {
                                                let key = &y[cur];
                                                match key {
                                                    Types::Symbol(sym) => {
                                                        let val = y[cur + 1].clone();
                                                        let res = eval(val, env.clone())?;
                                                        env.env_set(&sym, res);
                                                    }
                                                    _ => {
                                                        return Err("Cannot bind to non-symbols"
                                                            .to_string());
                                                    }
                                                }
                                                cur += 2;
                                            }
                                            ast = res[2].clone();
                                            continue 'tco;
                                        }
                                    }
                                    _ => Err("Expected a list for let bindings".to_string()),
                                }
                            }
                        }
                        Types::Symbol(x) if x == "do" => {
                            if let Types::List(_) =
                                eval_ast(Types::List(res[1..res.len() - 1].to_vec()), &env)?
                            {
                                ast = res.last().unwrap_or(&Types::Nil()).clone();
                                continue 'tco;
                            } else {
                                Err("Invalid case".to_string())
                            }
                        }
                        Types::Symbol(x) if x == "if" => {
                            if res.len() < 3 {
                                Err("Not enough arguements provided".to_string())
                            } else {
                                let result_bool = eval(res[1].clone(), env.clone())?;
                                match result_bool {
                                    Types::Bool(false) | Types::Nil() => {
                                        if res.len() >= 4 {
                                            ast = res[3].clone();
                                            continue 'tco;
                                        } else {
                                            Ok(Types::Nil())
                                        }
                                    }
                                    _ => {
                                        ast = res[2].clone();
                                        continue 'tco;
                                    }
                                }
                            }
                        }
                        Types::Symbol(x) if x == "fn*" => {
                            if res.len() < 3 {
                                Err("Invalid number of arguments".to_string())
                            } else {
                                let params = res[1].clone();
                                let ast_here = res[2].clone();
                                Ok(Types::UserFunc {
                                    ast: Box::new(ast_here),
                                    params: Box::new(params),
                                })
                            }
                        }
                        Types::Symbol(x) if x == "eval" => {
                            ast = eval(res[1].clone(), env.clone())?;
                            // println!("{:?}", ast);
                            continue 'tco;
                        }
                        Types::Symbol(x) if x == "load-file" => {
                            if let Types::Simple(x) = &res[1] {
                                match fs::read_to_string(x) {
                                    Ok(y) => {
                                        let z = y + "\nnil)";
                                        let z = "(do ".to_string() + &z;
                                        // println!("{:?}", z);
                                        rep(&z, &env)
                                    }
                                    Err(_) => Err("File error".to_string()),
                                }
                            } else {
                                Err("Provide file name as string".to_string())
                            }
                        }
                        Types::Symbol(x) if x == "swap!" => {
                            if res.len() < 3 {
                                Err("Invalid number of arguements".to_string())
                            } else {
                                let res = eval_ast(Types::List(res[1..].to_vec()), &env)?;
                                // println!("{:?}", res);
                                if let Types::List(res) = res {
                                    if let Types::Atom(x) = res[0].clone() {
                                        let f = &res[1];
                                        let mut args = if res.len() > 2 {
                                            // println!("Hey");
                                            res[2..].to_vec()
                                        } else {
                                            vec![]
                                        };
                                        args.insert(0, (*x).borrow().clone());
                                        println!("{:?}", args);
                                        let res = f.apply(&mut args, &mut env)?;
                                        // println!("{:?}", x);
                                        // println!("{:?}", env);
                                        *x.borrow_mut() = res.clone();
                                        // println!("{:?}", env);
                                        // println!("{:?}", x);
                                        Ok((*x.borrow()).clone())
                                    } else {
                                        Err("Can only be applied on atoms".to_string())
                                    }
                                } else {
                                    Err("Expected a list".to_string())
                                }
                            }
                        }
                        _ => {
                            let res = eval_ast(ast, &env)?;
                            // println!("{:?}", res);
                            // println!("{:?}", env);
                            if let Types::List(res) = res {
                                let ref f = res[0].clone();
                                let res = f.apply(&mut (res[1..].to_vec()), &mut env)?;
                                Ok(res)
                                // match f {
                                //     Types::Func(_) => Ok(res),
                                //     Types::UserFunc {
                                //         ast: ref fn_ast, ..
                                //     } => {
                                //         ast = (&**fn_ast).clone();
                                //         continue 'tco;
                                //     }
                                //     _ => Err("Invalid state".to_string()),
                                // }
                            } else {
                                Err("Expected a list".to_string())
                            }
                        }
                    }
                }
            }
            _ => eval_ast(ast, &env),
        };
        break;
    }
    ret
}
