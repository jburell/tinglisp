#[macro_use]
extern crate nom;

use nom::alphanumeric;

type Variable = String;
type Filter = String;

named! (plain_expression (&str) -> (Variable, Filter), do_parse! (
    tag_s! ("(") >>
    variable: alphanumeric >>
    tag_s! ("|") >>
    filter: alphanumeric >>
    tag_s! (")") >>
    ((variable.into(), filter.into()))));

#[derive(Debug)]
enum Expression {
    Plain(Variable, Filter),
    Recursive(Box<Expression>, Box<Expression>),
    Left(Variable, Box<Expression>),
    Right(Box<Expression>, Filter),
}

//named! (recursive_expression (&str) -> Expression,
//  alt_complete! (
//    map! (plain_expression, |(v, f)| Expression::Plain (v, f)) |
//    do_parse! (
//      tag_s! ("(") >>
//      sub: recursive_expression >>
//      tag_s! ("|") >>
//      filter: alphanumeric >>
//      tag_s! (")") >>
//      (Expression::Recursive (Box::new (sub), filter.into())))));

named! (recursive_expression (&str) -> Expression,
    alt_complete! (
        map! (plain_expression, |(v, f)| Expression::Plain (v, f)) |
        do_parse! (
            tag_s! ("(") >>
            sub: recursive_expression >>
            tag_s! ("|") >>
            other: recursive_expression >>
            tag_s! (")") >>
            (Expression::Recursive (Box::new (sub), Box::new (other)))
        ) |
        do_parse! (
            tag_s! ("(") >>
            sub: alphanumeric >>
            tag_s! ("|") >>
            other: recursive_expression >>
            tag_s! (")") >>
            (Expression::Left (sub.into(), Box::new (other)))
        ) |
        do_parse! (
            tag_s! ("(") >>
            sub: recursive_expression >>
            tag_s! ("|") >>
            other: alphanumeric >>
            tag_s! (")") >>
            (Expression::Right (Box::new (sub), other.into()))
        )
    )
);

fn main() {
    let plain = "(var|fil)";
    //let recursive = "(((var1|fil1)|fil2)|fil3)";
    let recursive = "(((var1|fil1)|(var2|fil2))|(var3|fil3))";
    let recursive2 = "((var1|(var2|fil2))|(var3|fil3))";
    let recursive3 = "(((var1|fil1)|fil2)|(var3|fil3))";
    println!("{:?}", plain_expression(plain));
    println!("{:?}", recursive_expression(recursive));
    println!("{:?}", recursive_expression(recursive2));
    println!("{:?}", recursive_expression(recursive3));
}
