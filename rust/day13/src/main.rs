use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
enum Value {
    Interger(i64),
    List(Vec<Value>),
}

type ParseIter<'a> = Peekable<Chars<'a>>;

fn parse_integer(buf: &mut ParseIter) -> Option<Value> {
    let mut xs = Vec::new();

    while let Some(chr) = buf.peek() {
        match chr {
            '0'..='9'         => xs.push(buf.next().unwrap()),
            _                 => break,
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

    if list.is_empty() {
        None
    } else {
        Some(Value::List(list))
    }
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

fn main() {
    println!("Hello, world!");
    let mut xs = "123,321,111".chars().peekable();
    println!("{:?}", parse(&mut xs));
    println!("{:?}", parse(&mut xs));
    println!("{:?}", parse(&mut xs));

    let mut xs = "[1,2,[2,[3,[4,[5,6,7]]]],8,9]".chars().peekable();
    println!("{:?}", parse(&mut xs));

    let mut xs = "[[4,4],4,4,4]".chars().peekable();
    println!("{:?}", parse(&mut xs));
}
