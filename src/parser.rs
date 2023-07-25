use std::rc::Rc;

pub trait SemiGroup<T> {
    fn append(&self, x: T) -> Self;
}

pub trait Monoid<T>: SemiGroup<T> {
    fn pure() -> Self;
}

impl SemiGroup<String> for String {
    fn append(&self, x: String) -> String {
        return self.clone() + &*x;
    }
}

impl Monoid<String> for String {
    fn pure() -> Self {
        return "".to_string();
    }
}

pub type Parser = dyn Fn(String) -> (Option<Vec<String>>, String);

pub fn space(s: String) -> (Option<Vec<String>>, String) {
    let bytes = s.as_bytes();
    if bytes.len() > 0 && bytes[0] as char == ' ' {
        // println!("space");
        return (Some(vec![" ".to_string()]), s[1..].to_string());
    }
    return (None, s);
}

pub fn end(s: String) -> (Option<Vec<String>>, String) {
    if s.is_empty() {
        return (Some(vec![]), s);
    }
    return (None, s);
}

pub fn string(str: String) -> Box<Parser> {
    return Box::new(move |s: String| {
        if s.starts_with(&str) {
            return (Some(vec![str.clone()]), s[str.len()..].to_string());
        }
        return (None, s);
    });
}

pub fn ident(s: String) -> (Option<Vec<String>>, String) {
    if let Some((s1, s2)) = (s.clone() + " ").split_once(" ") {
        return (Some(vec![s1.to_string()]), s2.to_string());
    }
    return (None, s);
}

pub fn ignore(p: Box<Parser>) -> Box<Parser> {
    return Box::new(move |s: String| {
        let (parsed, rest) = p(s);
        return match parsed {
            Some(_) => (Some(vec![]), rest),
            None => (None, rest),
        };
    });
}

#[allow(dead_code)]
pub fn tryp(p: Box<Parser>) -> Box<Parser> {
    return Box::new(move |s: String| {
        let (parsed, rest) = p(s.clone());
        return match parsed {
            Some(_) => (parsed, rest),
            None => (None, s),
        };
    });
}

pub fn combine(p1: Box<Parser>, p2: Box<Parser>) -> Box<Parser> {
    return Box::new(move |s: String| {
        let (parsed1, rest1) = p1(s);
        return if let Some(mut v) = parsed1 {
            let (parsed2, rest2) = p2(rest1);
            if let Some(mut v1) = parsed2 {
                v.append(&mut v1);
                (Some(v), rest2)
            } else {
                (None, rest2)
            }
        } else {
            (None, rest1)
        };
    });
}

pub fn or(p1: Box<Parser>, p2: Box<Parser>) -> Box<Parser> {
    return Box::new(move |s: String| {
        let (parsed, rest) = p1(s);
        return if let Some(_) = parsed {
            (parsed, rest)
        } else {
            p2(rest)
        };
    });
}

pub fn many(p: Rc<Parser>) -> Box<Parser> {
    return Box::new(move |s: String| {
        let (parsed, rest) = p(s);
        if let Some(mut v) = parsed {
            let (parsed, rest) = many(p.clone())(rest);
            if let Some(mut v1) = parsed {
                v.append(&mut v1);
                return (Some(v), rest);
            } else {
                return (Some(vec![]), rest);
            }
        } else {
            return (Some(vec![]), rest);
        }
    });
}
