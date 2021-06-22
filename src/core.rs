use crate::types::Types;
use crate::printer::pr_str;

type Args = Vec<Types>;

pub fn namespace() -> Vec<(&'static str, Types)> {
    vec![
        ("+", Types::Func(|a| {
                match &a[0] {
                    Types::Int(_) => { 
                        let res = a.iter().try_fold(0, |acc, x| {
                            match x {
                                Types::Int(y) => Ok(acc + y),
                                _ => Err("Invalid arguements".to_string())
                            }
                        });
                        match res {
                            Ok(x) => Ok(Types::Int(x)),
                            Err(_) => Err("Invalid".to_string())
                        }
                    }
                    Types::Float(_) => { 
                        let res = a.iter().try_fold(0., |acc, x| {
                            match x {
                                Types::Float(y) => Ok(acc + y),
                                // Types::Int(y) => Ok(acc + y),
                                _ => Err("Invalid arguements".to_string())
                            }
                        });
                        match res {
                            Ok(x) => Ok(Types::Float(x)),
                            Err(_) => Err("Invalid".to_string())
                        }
                    }
                    _ => Err("Invalid".to_string())
                }
        })),
        ("-", Types::Func(|a| {
                match &a[0] {
                    Types::Int(_) => { 
                        let res = a.iter().try_fold(0, |acc, x| {
                            match x {
                                Types::Int(y) => Ok(acc - y),
                                _ => Err("Invalid arguements".to_string())
                            }
                        });
                        match res {
                            Ok(x) => Ok(Types::Int(x)),
                            Err(_) => Err("Invalid".to_string())
                        }
                    }
                    Types::Float(_) => { 
                        let res = a.iter().try_fold(0., |acc, x| {
                            match x {
                                Types::Float(y) => Ok(acc - y),
                                // Types::Int(y) => Ok(acc + y),
                                _ => Err("Invalid arguements".to_string())
                            }
                        });
                        match res {
                            Ok(x) => Ok(Types::Float(x)),
                            Err(_) => Err("Invalid".to_string())
                        }
                    }
                    _ => Err("Invalid".to_string())
                }
        })),
        ("*", Types::Func(|a| {
                match &a[0] {
                    Types::Int(_) => { 
                        let res = a.iter().try_fold(0, |acc, x| {
                            match x {
                                Types::Int(y) => Ok(acc * y),
                                _ => Err("Invalid arguements".to_string())
                            }
                        });
                        match res {
                            Ok(x) => Ok(Types::Int(x)),
                            Err(_) => Err("Invalid".to_string())
                        }
                    }
                    Types::Float(_) => { 
                        let res = a.iter().try_fold(0., |acc, x| {
                            match x {
                                Types::Float(y) => Ok(acc / y),
                                // Types::Int(y) => Ok(acc + y),
                                _ => Err("Invalid arguements".to_string())
                            }
                        });
                        match res {
                            Ok(x) => Ok(Types::Float(x)),
                            Err(_) => Err("Invalid".to_string())
                        }
                    }
                    _ => Err("Invalid".to_string())
                }
        })),

        ("-", Types::Func(|a| {
                match &a[0] {
                    Types::Int(_) => { 
                        let res = a.iter().try_fold(0, |acc, x| {
                            match x {
                                Types::Int(y) => Ok(acc / y),
                                _ => Err("Invalid arguements".to_string())
                            }
                        });
                        match res {
                            Ok(x) => Ok(Types::Int(x)),
                            Err(_) => Err("Invalid".to_string())
                        }
                    }
                    Types::Float(_) => { 
                        let res = a.iter().try_fold(0., |acc, x| {
                            match x {
                                Types::Float(y) => Ok(acc / y),
                                // Types::Int(y) => Ok(acc + y),
                                _ => Err("Invalid arguements".to_string())
                            }
                        });
                        match res {
                            Ok(x) => Ok(Types::Float(x)),
                            Err(_) => Err("Invalid".to_string())
                        }
                    }
                    _ => Err("Invalid".to_string())
                }
        })),
        (
            "=",
            Types::Func(|a: &mut Args| {
                if a.len() < 2 {
                    Err("Not enough arguements".to_string())
                } else {
                    Ok(Types::Bool(a[0] == a[1]))
                }
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
                if a.len() < 1 {
                    Err("Invalid number of arguements".to_string())
                } else {
                    match &a[0] {
                        Types::List(_) => Ok(Types::Bool(true)),
                        _ => Ok(Types::Bool(false)),
                    }
                }
            }),
        ),
        (
            "empty?",
            Types::Func(|a: &mut Args| {
                if a.len() < 1 {
                    Err("Invalid number of arguements".to_string())
                } else {
                    match &a[0] {
                        Types::List(x) | Types::Vector(x) => Ok(Types::Bool(x.len() == 0)),
                        _ => Ok(Types::Bool(true)),
                    }
                }
            }),
        ),
        (
            "<",
            Types::Func(|a: &mut Args| {
                if a.len() < 2 {
                    Err("Invalid number of arguements".to_string())
                } else {
                    match (&a[0], &a[1]) {
                        (Types::Int(x), Types::Int(y)) => Ok(Types::Bool(x < y)),
                        (Types::Float(x), Types::Float(y)) => Ok(Types::Bool(x < y)),
                        _ => Err("Cannot be compared".to_string()),
                    }
                }
            }),
        ),
        (
            ">",
            Types::Func(|a: &mut Args| {
                if a.len() < 2 {
                    Err("Invalid number of arguements".to_string())
                } else {
                    match (&a[0], &a[1]) {
                        (Types::Int(x), Types::Int(y)) => Ok(Types::Bool(x > y)),
                        (Types::Float(x), Types::Float(y)) => Ok(Types::Bool(x > y)),
                        _ => Err("Cannot be compared".to_string()),
                    }
                }
            }),
        ),
        (
            ">=",
            Types::Func(|a: &mut Args| {
                if a.len() < 2 {
                    Err("Invalid number of arguements".to_string())
                } else {
                    match (&a[0], &a[1]) {
                        (Types::Int(x), Types::Int(y)) => Ok(Types::Bool(x >= y)),
                        (Types::Float(x), Types::Float(y)) => Ok(Types::Bool(x >= y)),
                        _ => Err("Cannot be compared".to_string()),
                    }
                }
            }),
        ),
        (
            "<=",
            Types::Func(|a: &mut Args| {
                if a.len() < 2 {
                    Err("Invalid number of arguements".to_string())
                } else {
                    match (&a[0], &a[1]) {
                        (Types::Int(x), Types::Int(y)) => Ok(Types::Bool(x <= y)),
                        (Types::Float(x), Types::Float(y)) => Ok(Types::Bool(x <= y)),
                        _ => Err("Cannot be compared".to_string()),
                    }
                }
            }),
        ),
        (
            "prn",
            Types::Func(|a: &mut Args| {
                let mut res = vec![];
                for i in a {
                    res.push(pr_str(i.clone(), true));
                }
                println!("{}", res.join(" "));
                Ok(Types::Nil())
            })
        ),
        (
            "str",
            Types::Func(|a: &mut Args| {
                let mut res = vec![];
                for i in a {
                    res.push(pr_str(i.clone(), false));
                }
                Ok(Types::Simple(res.join("")))
            })
        ),
        (
            "pr-str",
            Types::Func(|a: &mut Args| {
                let mut res = vec![];
                for i in a {
                    res.push(pr_str(i.clone(), true));
                }
                Ok(Types::Simple(res.join(" ")))
            })
        ),
        (
            "println",
            Types::Func(|a: &mut Args| {
                let mut res = vec![];
                for i in a {
                    res.push(pr_str(i.clone(), false));
                }
                println!("{}", res.join(" "));
                Ok(Types::Nil())
            })
        )
    ]
}
