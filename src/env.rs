use crate::core::namespace;
use crate::types::Types;
use std::collections::HashMap;

pub type Env = HashMap<String, Types>;

#[derive(Clone, Debug)]
pub struct Environment {
    data: HashMap<String, Types>,
    pub outer: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            data: Env::default(),
            outer: None,
        }
    }

    pub fn set_core_func(&mut self) {
        let functions = namespace();
        for (i, j) in functions {
            self.env_set(i, j);
        }
    }

    pub fn env_bind(&mut self, binds: Types, exprs: Vec<Types>) -> Result<(), String> {
        match binds {
            Types::List(x) | Types::Vector(x) => {
                println!("{}", x.len());
                for i in 0..x.len() {
                    match &x[i] {
                        Types::Symbol(y) if y == "&" => {
                            if i + 1 == x.len() {
                                return Err("Invalid parameter list".to_string());
                            } else {
                                match &x[i + 1] {
                                    Types::Symbol(z) => {
                                        self.env_set(&z, Types::List(exprs[i..].to_vec()))
                                    }
                                    _ => {
                                        return Err("Cannot bind non-symbols".to_string());
                                    }
                                }
                                break;
                            }
                        }
                        Types::Symbol(y) => {
                            self.env_set(&y, exprs[i].clone());
                        }
                        _ => {return Err("Cannot bind non-symbols".to_string())}
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }

    pub fn env_set(&mut self, key: &str, val: Types) {
        self.data.insert(key.to_string(), val);
    }

    pub fn env_get(&self, key: Types) -> Result<Types, String> {
        let mut cur_env = self;
        println!("{:?}", cur_env);
        loop {
            match key {
                Types::Symbol(ref s) => {
                    let x = cur_env.data.get(s);
                    match x {
                        Some(_) => {
                            return Ok(x.unwrap().clone());
                        }
                        None => match &cur_env.outer {
                            Some(x) => {
                                cur_env = &*x;
                            }
                            None => {
                                return Err(format!("{} not found", s));
                            }
                        },
                    }
                }
                _ => {
                    return Err("Cannot find".to_string());
                }
            }
        }
    }
}
