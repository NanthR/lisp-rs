use crate::types::Types;

pub fn pr_str(fin: Types, print_readably: bool) -> String {
    let mut res = vec![];
    match fin {
        Types::Simple(x) => {
            if print_readably {
                let mut fin = String::new();
                for i in x.chars() {
                    if i == '\n' {
                        fin += "\\";
                        fin += "n";
                    } else if i == '\\' {
                        fin += "\\\\";
                    } else if i == '\"' {
                        fin += "\\\"";
                    } else {
                        fin.push(i);
                    }
                }
                fin = fin + "\"";
                fin = "\"".to_string() + &fin;
                res.push(fin);
            } else {
                res.push(x);
            }
        }
        Types::Bool(x) => {
            if x {
                res.push("true".to_string());
            } else {
                res.push("false".to_string());
            }
        }
        Types::KeyWord(x) => {
            res.push(x);
        }
        Types::Dict(x) => {
            let mut temp = vec![];
            for (key, val) in x {
                if &key[0..1] == ":" {
                    temp.push(key);
                } else {
                    let fin = key + "\"";
                    let fin = "\"".to_string() + &fin;
                    temp.push(fin);
                }
                temp.push(pr_str(val, print_readably));
            }
            let temp = temp.join(" ");
            let temp = temp + "}";
            let temp = "{".to_string() + &temp;
            res.push(temp);
        }
        Types::Nil() => res.push("nil".to_string()),
        Types::Symbol(x) => match x.as_str() {
            "'" => res.push("quote".to_string()),
            "`" => res.push("quasiquote".to_string()),
            "~" => res.push("unquote".to_string()),
            _ => res.push(x),
        },
        Types::List(x) => {
            let fin = x
                .iter()
                .map(|i| pr_str(i.clone(), print_readably))
                .collect::<Vec<String>>();
            let val = "(".to_string() + &fin.join(" ");
            let val = val + ")";
            res.push(val);
        }
        Types::Vector(x) => {
            let fin = x
                .iter()
                .map(|i| pr_str(i.clone(), print_readably))
                .collect::<Vec<String>>();
            let val = "[".to_string() + &fin.join(" ");
            let val = val + "]";
            res.push(val);
        }
        Types::Int(x) => res.push(x.to_string()),
        Types::Error(x) => res = vec![x],
        Types::Func(_) => res.push("#<function>".to_string()),
        Types::Float(x) => res.push(x.to_string()),
        Types::UserFunc { .. } => res.push("#<function>".to_string()),
        Types::Atom(x) => {
            let fin = "(atom ".to_string() + &(pr_str((*x.borrow()).clone(), print_readably));
            res.push(fin + ")");
        }
    }
    // println!("{:?}", res);
    res.join(" ")
}
