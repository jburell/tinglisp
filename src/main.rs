#![feature(trace_macros)]
#[macro_use]
extern crate nom;
extern crate either;

use nom::{alphanumeric, digit};
use std::str::{self, FromStr};
use std::collections::{HashMap, LinkedList};

#[derive(Debug, PartialEq)]
pub enum TingValue {
    Str(String),
    Num(f32),
    Array(Vec<TingValue>),
    Object(HashMap<String, TingValue>),
    List(LinkedList<TingValue>),
    Value(),
}

// FIXME: since we already parsed a serie of digits and dots,
// we know it is correct UTF-8. no need to use from_utf8 to
// verify it is correct
// FIXME: use alt_complete (implement ws for alt_complete)
named!(
    unsigned_float<f32>,
    map_res!(
        map_res!(
            recognize!(alt_complete!(
                delimited!(digit, tag!("."), opt!(complete!(digit)))
                    | delimited!(opt!(digit), tag!("."), digit) | digit
            )),
            str::from_utf8
        ),
        FromStr::from_str
    )
);

named!(
    float<f32>,
    map!(
        pair!(opt!(alt!(tag!("+") | tag!("-"))), unsigned_float),
        |(sign, value): (Option<&[u8]>, f32)| {
            sign.and_then(|s| {
                if s[0] == ('-' as u8) {
                    Some(-1f32)
                } else {
                    None
                }
            }).unwrap_or(1f32) * value
        }
    )
);

//FIXME: verify how json strings are formatted
named!(
    string<&str>,
    delimited!(
        tag!("\""),
        map_res!(
            escaped!(call!(alphanumeric), '\\', is_a!("\"n\\")),
            str::from_utf8
        ),
        tag!("\"")
    )
);

named!(
    array<Vec<TingValue>>,
    ws!(delimited!(
        tag!("["),
        separated_list!(tag!(","), value),
        tag!("]")
    ))
);

named!(
    key_value<(&str, TingValue)>,
    ws!(separated_pair!(string, tag!(":"), value))
);

named!(
    hash<HashMap<String, TingValue>>,
    ws!(map!(
        delimited!(tag!("{"), separated_list!(tag!(","), key_value), tag!("}")),
        |tuple_vec| {
            let mut h: HashMap<String, TingValue> = HashMap::new();
            for (k, v) in tuple_vec {
                h.insert(String::from(k), v);
            }
            h
        }
    ))
);

named!(
    list<LinkedList<TingValue>>,
    ws!(map!(
        delimited!(tag!("("), separated_list!(tag!(" "), key_value), tag!(")")),
        |tuple_vec| {
            let mut h: LinkedList<TingValue> = LinkedList::new();
            for (k, v) in tuple_vec {
                h.push_back(v);
            }
            h
        }
        ))
);

named!(
    list_or_value<TingValue>,
    ws!(
        alt_complete!(
            list  => { |l| list(l)   } |
            value => { |v| value(v)  }
        )
    )
);

named!(
    value<TingValue>,
    ws!(dbg_dmp!(alt_complete!(
      hash   => { |h|   TingValue::Object(h)            } |
      array  => { |v|   TingValue::Array(v)             } |
      string => { |s|   TingValue::Str(String::from(s)) } |
      float  => { |num| TingValue::Num(num)             } |
      list   => { |val| TingValue::List(val)            }
    )))
);

fn main() {
    let code = &b"(conj (conj #{1 2 3} 4) 4)";
    let result = value(&code[..]);
    println!("{:?}", result);
}
