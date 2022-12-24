use std::{iter::Peekable, str::Chars, cmp::Ordering};

#[derive(Debug)]
#[derive(Eq)]
enum Value {
    Interger(i64),
    List(Vec<Value>),
}

impl Value {
    fn wrap(&self) -> Self {
        match self {
            Value::Interger(i) => Value::List(vec![Value::Interger(*i)]),
            Value::List(of) => panic!("can't wrap list: {:?}", of),
        }
    }
}

type ParseIter<'a> = Peekable<Chars<'a>>;

fn parse_integer(buf: &mut ParseIter) -> Option<Value> {
    let mut xs = Vec::new();

    while let Some(chr) = buf.peek() {
        match chr {
            '0'..='9' => xs.push(buf.next().unwrap()),
            _         => break,
        };
    }

    let val = xs.iter().collect::<String>();
    match val.parse::<i64>() {
        Ok(x) => Some(Value::Interger(x)),
        _          => panic!("can't parse ingeger: {}", val),
    }
}

fn parse_list(buf: &mut ParseIter) -> Option<Value> {
    let mut list = Vec::new();

    if let Some(chr) = buf.next() {
        if chr != '[' {
            panic!("can't parse list: {:?}", chr);
        }
    }

    while let Some(chr) = buf.peek() {
        match chr {
            ']'     => { buf.next(); break; },
            '[' | _ => parse(buf).map(|val| list.push(val)),
        };
    }

    Some(Value::List(list))
}

fn parse(buf: &mut ParseIter) -> Option<Value> {
    while let Some(chr) = buf.peek() {
        match chr {
            ' ' | ',' => buf.next(),
            _         => break,
        };
    }

    if let Some(chr) = buf.peek() {
        match chr {
            '[' => parse_list(buf),
            _   => parse_integer(buf),
        }
    } else {
        None
    }
}

impl Ord for Value {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Value::Interger(l), Value::Interger(r))       => l.cmp(r),
            (Value::List(l), Value::List(r)) => {
                for i in 0..std::cmp::min(r.len(), l.len()) {
                    match l[i].cmp(&r[i]) {
                        Ordering::Equal => continue,
                        pred @ Ordering::Less | pred @ Ordering::Greater => return pred,
                    }
                }

                return l.len().cmp(&r.len());
            },
            (i @ Value::Interger(_), xs) => i.wrap().cmp(xs),
            (xs, i @ Value::Interger(_)) => xs.cmp(&i.wrap()),
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

fn main() {
    let content = std::fs::read_to_string(std::env::args().nth(1).expect("pass path to input")).expect("can't read input");

    let mut result = 0;
    for (i, xs) in content.split("\n\n").enumerate() {
        let splitted: Vec<&str> = xs.split('\n').collect();
        let mut lhs = splitted[0].chars().peekable();
        let mut rhs = splitted[1].chars().peekable();
        if parse(&mut lhs).unwrap() < parse(&mut rhs).unwrap() {
            result += i + 1;
        }
    }
    println!("{}", result);
}
