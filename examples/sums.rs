extern crate gobble;
use gobble::*;

#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Div,
    Mul,
}

#[derive(Debug)]
pub enum Expr {
    Val(isize),
    Parenth(Box<Expr>),
    Oper(Op, Box<Expr>, Box<Expr>),
    //Bracket(Box<Expr>),
}

fn parse_op() -> impl Parser<Op> {
    //s_ allows whitespace either side of a parser
    s_(one_char("+-*/")).try_map(|o| match o {
        '+' => Ok(Op::Add),
        '-' => Ok(Op::Sub),
        '*' => Ok(Op::Mul),
        '/' => Ok(Op::Div),
        _ => Err(ECode::Never("Op not in list")),
    })
}

fn parse_expr_l() -> impl Parser<Expr> {
    or(
        middle("(", parse_expr, ")").map(|e| Expr::Parenth(Box::new(e))),
        common_int.map(|i| Expr::Val(i)),
    )
}

/// Resolve a simple mathematical expression
pub fn parse_expr<'a>(i: &LCChars<'a>) -> ParseRes<'a, Expr> {
    let p = (parse_expr_l(), maybe((parse_op(), parse_expr))).map(|(l, opt)| match opt {
        //Note this cares nothing for operation priority except for brackets
        Some((oper, r)) => Expr::Oper(oper, Box::new(l), Box::new(r)),
        None => l,
    });
    p.parse(i)
}

/// This loops asking the user for input of simple sum type "4 + 6 * (3-1)"
/// and will simply output the parsed expression
fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    loop {
        let mut s = String::new();
        match stdin.read_line(&mut s) {
            Ok(0) => return Ok(()),
            Err(e) => return Err(e),
            _ => {}
        }
        let e = parse_expr.parse_s(&s);
        println!("{:?}", e);
    }
    //Ok(())
}