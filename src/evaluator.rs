use crate::env::Environment;
use crate::types::Types;

fn eval_ast(ast: Types, env: &mut Environment) -> Result<Types, String> {
    match ast {
        Types::Symbol(_) => env.env_get(ast),
        Types::List(x) => {
            let mut res = vec![];
            for i in x {
                res.push(eval(i, env)?);
            }
            Ok(Types::List(res))
        }
        _ => Ok(ast),
    }
}

pub fn eval(ast: Types, env: &mut Environment) -> Result<Types, String> {
    match ast {
        Types::List(ref res) => {
            if res.len() == 0 {
                Ok(ast)
            } else {
                match &res[0] {
                    Types::Symbol(x) if x == "def!" => {
                        if res.len() != 3 {
                            Err("Invalid definition".to_string())
                        } else {
                            match &res[1] {
                                Types::Symbol(z) => {
                                    let fin = eval(res[2].clone(), env)?;
                                    env.env_set(&z, fin.clone());
                                    // env.env_set(&z, eval(x[2].clone(), env)?);
                                    Ok(fin)
                                }
                                _ => Err("Cannot bind to non-symbol".to_string()),
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
                                        let mut cur_env = Environment::new();
                                        cur_env.outer = Some(Box::new(env.clone()));
                                        let mut cur = 0;
                                        while cur < y.len() {
                                            let key = &y[cur];
                                            match key {
                                                Types::Symbol(sym) => {
                                                    let val = y[cur + 1].clone();
                                                    let res = eval(val, &mut cur_env)?;
                                                    cur_env.env_set(&sym, res);
                                                }
                                                _ => {
                                                    return Err(
                                                        "Cannot bind to non-symbols".to_string()
                                                    );
                                                }
                                            }
                                            cur += 2;
                                        }
                                        eval(res[2].clone(), &mut cur_env)
                                    }
                                }
                                _ => Err("Expected a list for let bindings".to_string()),
                            }
                        }
                    }
                    Types::Symbol(x) if x == "do" => {
                        match eval_ast(Types::List(res[1..].to_vec()), env)? {

                            Types::List(mut x) => Ok(x.remove(x.len() - 1)),
                            _ => Err("Invalid case".to_string())
                        }
                    }
                    Types::Symbol(x) if x == "if" => {
                        if res.len() < 3 {
                            Err("Not enough arguements provided".to_string())
                        } else {
                            let result_bool = eval(res[1].clone(), env)?;
                            match result_bool {
                                Types::Bool(false) | Types::Nil() => {
                                    if res.len() >= 4 {
                                        eval(res[3].clone(), env)
                                    } else {
                                        Ok(Types::Nil())
                                    }
                                }
                                _ => {
                                    eval(res[2].clone(), env)
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
                            println!("{:?}", ast_here);
                            Ok(Types::UserFunc {
                                ast: Box::new(ast_here),
                                params: Box::new(params),
                            })
                        }
                    }
                    _ => {
                        let res = eval_ast(ast, env)?;
                        if let Types::List(res) = res {
                            let ref f = res[0].clone();
                            Ok(f.apply(&mut (res[1..].to_vec()), env)?)
                        } else {
                            Err("Expected a list".to_string())
                        }
                    }
                }
            }
        }
        Types::Vector(x) => {
            let mut res = vec![];
            for i in x {
                res.push(eval(i, env)?);
            }
            Ok(Types::Vector(res))
        }
        Types::Dict(mut x) => {
            for (_, val) in x.iter_mut() {
                *val = eval(val.clone(), env)?;
            }
            Ok(Types::Dict(x))
        }
        _ => eval_ast(ast, env),
    }
}
