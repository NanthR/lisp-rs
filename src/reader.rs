use crate::types::Types;
use std::str::Chars;
use std::{collections::HashMap, iter::Peekable};

pub struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
    tokens: Vec<String>,
    cur_token: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(input: &'a str) -> Self {
        Tokenizer {
            chars: input.chars().peekable(),
            tokens: vec![],
            cur_token: 0,
        }
    }

    pub fn read_str(&mut self) -> Types {
        self.tokenize();
        // println!("{:?}", self.tokens);
        let mut res = vec![];
        while self.peek().is_some() {
            let val = self.read_form();
            match val {
                Types::Error(val) => {
                    res = vec![Types::Error(val)];
                    break;
                }
                x => {
                    res.push(x);
                }
            }
        }
        if res.len() == 1 {
            res.into_iter().nth(0).unwrap()
        } else {
            Types::List(res)
        }
    }

    fn tokenize(&mut self) {
        let mut res = vec![];
        while self.chars.peek().is_some() {
            let val = self.chars.next().unwrap();
            match val {
                ' ' | '\t' | '\n' | ',' => {}
                '~' => {
                    let x = self.chars.peek();
                    if x.is_some() && x == Some(&'@') {
                        res.push("~@".to_string());
                        self.chars.next().unwrap();
                    } else {
                        res.push("~".to_string());
                    }
                }
                '[' | ']' | '{' | '}' | '(' | ')' | '`' | '\'' | '^' | '@' => {
                    res.push(val.to_string());
                }
                '"' => {
                    let mut cur = '"';
                    let mut fin = String::from('"');
                    let mut done = false;
                    while self.chars.peek().is_some() {
                        let next = self.chars.next().unwrap();
                        match next {
                            '"' => {
                                if cur == '\\' {
                                    fin.push('"');
                                    cur = '"'
                                } else {
                                    fin.push('"');
                                    done = true;
                                    break;
                                }
                            }
                            'n' => {
                                if cur == '\\' {
                                    fin.push('\n');
                                    cur = 'a';
                                } else {
                                    fin.push('n');
                                    cur = 'n';
                                }
                            }
                            '\\' => {
                                if cur == '\\' {
                                    fin.push('\\');
                                    cur = 'a';
                                } else {
                                    cur = '\\';
                                }
                            }
                            x => {
                                fin.push(x);
                                cur = x;
                            }
                        }
                    }
                    if done {
                        res.push(fin);
                    } else {
                        res.push("EOF".to_string());
                    }
                }
                ';' => {
                    let mut fin = String::new();
                    while self.chars.peek().is_some() {
                        let c = self.chars.next().unwrap();
                        if c == '\n' {
                            break;
                        } else {
                            fin.push(c);
                        }
                    }
                    // res.push(fin);
                }
                x => {
                    let mut fin = String::from(x);
                    while self.chars.peek().is_some() {
                        let val = self.chars.peek().unwrap();
                        match val {
                            '[' | ']' | '{' | '}' | '(' | ')' | '`' | '\'' | '^' | '@' | ' '
                            | '\"' | ',' | ';' | '\n' | '\t' => {
                                break;
                            }
                            _ => {
                                fin.push(self.chars.next().unwrap());
                            }
                        }
                    }
                    res.push(fin);
                }
            }
        }
        self.tokens = res;
        self.cur_token = 0;
    }

    fn read_form(&mut self) -> Types {
        match self.peek().unwrap() {
            "[" => {
                self.next().unwrap();
                self.read_list("[")
            }
            "(" => {
                self.next().unwrap();
                self.read_list("(")
            }
            "{" => {
                self.next().unwrap();
                self.read_list("{")
            }
            "@" => {
                self.next().unwrap();
                Types::List(vec![Types::Symbol("deref".to_string()), self.read_form()])
            }
            _ => self.read_atom(),
        }
    }

    fn read_list(&mut self, x: &str) -> Types {
        let mut done = false;
        let mut res = vec![];
        let end = match x {
            "[" => "]",
            "(" => ")",
            "{" => "}",
            _ => "",
        };
        while self.peek().is_some() {
            let val = self.peek().unwrap();
            if val == end {
                done = true;
                self.next().unwrap();
                break;
            } else {
                res.push(self.read_form());
            }
        }
        if done {
            match x {
                "[" => Types::Vector(res),
                "{" => {
                    if res.len() % 2 != 0 {
                        Types::Error("EOF".to_string())
                    } else {
                        let mut hash_map = HashMap::<String, Types>::new();
                        while res.len() > 0 {
                            let key = res.remove(0);
                            let val = res.remove(0);
                            match key {
                                Types::KeyWord(x) => {
                                    hash_map.insert(x, val);
                                }
                                Types::Simple(x) => {
                                    hash_map.insert(x, val);
                                }
                                _ => {
                                    return Types::Error("EOF".to_string());
                                }
                            }
                        }
                        Types::Dict(hash_map)
                    }
                }
                _ => Types::List(res),
            }
        } else {
            Types::Error("EOF".to_string())
        }
    }

    fn read_atom(&mut self) -> Types {
        let val = self.next().unwrap().to_string();
        match val.parse::<i64>() {
            Ok(x) => Types::Int(x),
            Err(_) => match val.parse::<f64>() {
                Ok(x) => Types::Float(x),
                Err(_) => match val.as_str() {
                    "+" | "-" | "*" | "/" | "**" | "'" | "`" | "~" => Types::Symbol(val),
                    "true" => Types::Bool(true),
                    "false" => Types::Bool(false),
                    "nil" => Types::Nil(),
                    _ => {
                        if &val[0..1] == "\"" {
                            Types::Simple(val[1..val.len() - 1].to_string())
                        } else if &val[0..1] == ":" {
                            Types::KeyWord(val)
                        } else {
                            Types::Symbol(val)
                        }
                    }
                },
            },
        }
    }

    fn next(&mut self) -> Option<&str> {
        if self.cur_token < self.tokens.len() {
            self.cur_token += 1;
            Some(&self.tokens[self.cur_token - 1])
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&str> {
        if self.cur_token < self.tokens.len() {
            Some(&self.tokens[self.cur_token])
        } else {
            None
        }
    }
}
