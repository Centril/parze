#![cfg(feature = "macros")]
#![feature(proc_macro_hygiene)]

use parze::prelude::*;

#[derive(Debug)]
enum Expr {
    Literal(i64),
    Neg(Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
    Rem(Box<Expr>, Box<Expr>),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
}

impl Expr {
    fn eval(&self) -> i64 {
        match self {
            Expr::Literal(a) => *a,
            Expr::Neg(a) => -a.eval(),
            Expr::Mul(a, b) => a.eval() * b.eval(),
            Expr::Div(a, b) => a.eval() / b.eval(),
            Expr::Rem(a, b) => a.eval() % b.eval(),
            Expr::Add(a, b) => a.eval() + b.eval(),
            Expr::Sub(a, b) => a.eval() - b.eval(),
        }
    }
}

fn main() {
    parsers! {
        number = {
            { one_of("0123456789".chars()) }+ => { |s| Expr::Literal(s.collect::<String>().parse().unwrap()) }
        }

        atom = {
            ( number | '(' -& expr &- ')')~
        }

        unary = {
            '-'~* & atom <: { |_, e| Expr::Neg(e.into()) }
        }

        product = {
            unary & (('*' | '/' | '%')~ & unary)* :> { |a, (op, b)| match op {
                '*' => Expr::Mul(a.into(), b.into()),
                '/' => Expr::Div(a.into(), b.into()),
                '%' => Expr::Rem(a.into(), b.into()),
                _ => unreachable!(),
            }}
        }

        sum = {
            product & (('+' | '-')~ & product)* :> {|a, (op, b)| match op {
                '+' => Expr::Add(a.into(), b.into()),
                '-' => Expr::Sub(a.into(), b.into()),
                _ => unreachable!(),
            }}
        }

        expr: Parser<_, _> = { ' '* -& sum }
    }

    assert_eq!(
        expr.parse_str("14 + 3 / 1 * (2 + 4) + -1").unwrap().eval(),
        31,
    );
}
