use crate::core::namespace;
use crate::types::Types;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub struct Environment {
    data: RefCell<HashMap<String, Types>>,
    pub outer: Option<Rc<Environment>>,
}

pub type Env = Rc<Environment>;

pub trait EnvFunc {
    fn env_set(&self, key: &str, val: Types);
    fn set_core_func(&self);
    fn env_get(&self, key: Types) -> Result<Types, String>;
    fn new(outer: Option<Rc<Environment>>) -> Self;
    fn env_bind(&self, binds: Types, exprs: Vec<Types>) -> Result<(), String>;
}

impl EnvFunc for Env {
    fn new(outer: Option<Rc<Environment>>) -> Env {
        Rc::new(Environment {
            data: RefCell::new(HashMap::new()),
            outer: outer,
        })
    }

    fn env_set(&self, key: &str, val: Types) {
        self.data.borrow_mut().insert(key.to_string(), val);
    }

    fn set_core_func(&self) {
        let functions = namespace();
        for (i, j) in functions {
            self.env_set(i, j);
        }
    }

    fn env_get(&self, key: Types) -> Result<Types, String> {
        let mut cur_env = self;
        loop {
            if let Types::Symbol(ref s) = key {
                if let Some(y) = cur_env.data.borrow().get(s) {
                    return Ok(y.clone());
                } else {
                    if let Some(z) = &cur_env.outer {
                        cur_env = z;
                    } else {
                        return Err(format!("{} not found", s));
                    }
                }
            } else {
                return Err("Cannot find".to_string());
            }
        }
    }

    fn env_bind(&self, binds: Types, exprs: Vec<Types>) -> Result<(), String> {
        match binds {
            Types::List(x) | Types::Vector(x) => {
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
                        _ => return Err("Cannot bind non-symbols".to_string()),
                    }
                }
            }
            _ => {}
        }
        Ok(())
    }
}
