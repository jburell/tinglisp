#[macro_use]
extern crate nom;

use nom::{IResult, digit, alpha};

// Parser definition

use std::str;
//use std::str::FromStr;

// We parse any expr surrounded by parens, ignoring all whitespaces around those
named!(parens<&[u8]>, ws!(delimited!( tag!("("), expr, tag!(")") )) );

//named!(sym<&[u8], (&str, &[u8])>, 
//    is_alphanumeric
//    );

named!(expr<&[u8], &[u8]>, ws!( alt!(parens | digit | alpha) ) );

fn main() {
    assert_eq!(expr(b"(5)"), IResult::Done(&b""[..], &b"5"[..]));
    assert_eq!(expr(b"(A)"), IResult::Done(&b""[..], &b"A"[..]));
    assert_eq!(expr(b"((5)(B))"), IResult::Done(&b""[..], &b"5B"[..]));
}