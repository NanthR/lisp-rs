use crate::env::{Env, EnvFunc};
use crate::eval;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

pub type Args = Vec<Types>;

#[derive(Clone)]
pub enum Types {
    Simple(String),
    Symbol(String),
    KeyWord(String),
    Int(i64),
    Float(f64),
    List(Vec<Types>),
    Error(String),
    Nil(),
    Bool(bool),
    Vector(Vec<Types>),
    Dict(HashMap<String, Types>),
    Func(fn(&mut Args) -> Result<Types, String>),
    Atom(Rc<RefCell<Types>>),
    UserFunc { ast: Box<Types>, params: Box<Types> },
}

impl PartialEq for Types {
    fn eq(&self, other: &Types) -> bool {
        match (&self, &other) {
            (&Types::Simple(a), &Types::Simple(b)) => a == b,
            (&Types::Symbol(a), &Types::Symbol(b)) => a == b,
            (&Types::KeyWord(a), &Types::KeyWord(b)) => a == b,
            (&Types::Int(a), &Types::Int(b)) => a == b,
            (&Types::Float(a), &Types::Float(b)) => a == b,
            (&Types::List(a), &Types::List(b)) => a == b,
            (&Types::Vector(a), &Types::List(b)) => a == b,
            (&Types::List(a), &Types::Vector(b)) => a == b,
            (&Types::Bool(a), &Types::Bool(b)) => a == b,
            (&Types::Vector(a), &Types::Vector(b)) => a == b,
            (&Types::Dict(a), &Types::Dict(b)) => a == b,
            (&Types::Nil(), &Types::Nil()) => true,
            _ => false,
        }
    }
}

impl fmt::Debug for Types {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &*self {
            Types::Simple(x) => write!(f, "{}", format!("Simple({})", x)),
            Types::Symbol(x) => write!(f, "{}", format!("Sym({})", x)),
            Types::KeyWord(x) => write!(f, "{}", format!("Key({})", x)),
            Types::Int(x) => write!(f, "{}", format!("Int({})", x)),
            Types::Float(x) => write!(f, "{}", format!("Float({})", x)),
            Types::List(x) => write!(f, "{}", format!("List({:?})", x)),
            Types::Bool(x) => write!(f, "{}", format!("Bool({})", x)),
            Types::Vector(x) => write!(f, "{}", format!("Vect({:?})", x)),
            Types::Dict(x) => write!(f, "{}", format!("Dict({:?})", x)),
            Types::Func(_) => write!(f, "Fn"),
            Types::UserFunc { .. } => write!(f, "UserFunc"),
            Types::Atom(x) => write!(f, "Atom({:?})", x),
            Types::Error(x) => write!(f, "Error ({})", x),
            Types::Nil() => write!(f, "Nil"),
        }
    }
}

impl Types {
    pub fn apply(&self, args: &mut Args, env: &mut Env) -> Result<Types, String> {
        match &*self {
            Types::Func(f) => f(args),
            Types::UserFunc {
                ref ast,
                ref params,
                ..
            } => {
                let a = &**ast;
                let b = &**params;
                // println!("AST {:?}", a);
                // println!("Param {:?}", b);
                // println!("Args {:?}", args);
                let new_env: Env = EnvFunc::new(Some(env.clone()));
                new_env.env_bind(b.clone(), args.to_vec())?;
                *env = new_env.clone();
                Ok(eval(a.clone(), new_env.clone())?)
            }
            _ => Err("Attempted to call a non-function".to_string()),
        }
    }
}
