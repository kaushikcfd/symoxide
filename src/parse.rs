use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n, take_while},
    sequence::tuple,
    Err::Error,
    error::{ParseError, ErrorKind},
};
use crate::Expression;

/// {{{ Precedence

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Precedence{
    Identifier,
    Literal,
    Or,
    And,
    Not,
    Eq,
    Neq,
    Gt,
    Lt,
    Ge,
    Le,
    BitwiseOr,
    BitwiseXor,
    BitwiseAnd,
    Lshift,
    Rshift,
    Sum,
    Minus,
    Product,
    Div,
    FloorDiv,
    Modulo,
    Exponent,
    Parens,
    Subscript,
    Call,
}

impl Precedence {
    fn from_i32(x: i32) -> Precedence {
        match x {
             0 => Precedence::Identifier,
             1 => Precedence::Literal,
             2 => Precedence::Or,
             3 => Precedence::And,
             4 => Precedence::Not,
             5 => Precedence::Eq,
             6 => Precedence::Neq,
             7 => Precedence::Gt,
             8 => Precedence::Lt,
             9 => Precedence::Ge,
            10 => Precedence::Le,
            11 => Precedence::BitwiseOr,
            12 => Precedence::BitwiseXor,
            13 => Precedence::BitwiseAnd,
            14 => Precedence::Lshift,
            15 => Precedence::Rshift,
            16 => Precedence::Sum,
            17 => Precedence::Minus,
            18 => Precedence::Product,
            19 => Precedence::Div,
            20 => Precedence::FloorDiv,
            21 => Precedence::Modulo,
            22 => Precedence::Exponent,
            23 => Precedence::Parens,
            24 => Precedence::Subscript,
            25 => Precedence::Call,
            _  => panic!("Unexpected Precedence constant {}.", x),
        }
    }
}


impl std::ops::Sub<i32> for Precedence {
    type Output = Precedence;

    fn sub(self, other: i32)  -> Self::Output {
        return Precedence::from_i32(self as i32 - (other as i32));
    }
}

/// }}}


#[derive(Debug, PartialEq)]
pub enum CustomError<I> {
    MyError,
    Nom(I, ErrorKind),
}

impl<I> ParseError<I> for CustomError<I> {
    fn from_error_kind(input: I, kind: ErrorKind) -> Self {
        CustomError::Nom(input, kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}


// {{{ parsers for individual nodes

fn is_first_character_of_id(x: char) -> bool{
    x.is_ascii_alphabetic() || x == '_'
}

fn is_second_character_onwards_of_id(x: char) -> bool{
    x.is_ascii_alphanumeric() || x == '_'
}

fn is_whitespace(x: char) -> bool {
    x.is_whitespace()
}


fn _parse_parens(input: &str) -> IResult<&str, Expression, CustomError<&str>> {
    let inner_expr = tuple((
        take_while(is_whitespace),
        tag("("),
        take_while(is_whitespace),
        |x| _parse_expr(x, Precedence::Parens) ,
        take_while(is_whitespace),
        tag(")"),
        take_while(is_whitespace),
    ))(input);

    match inner_expr {
        Ok((input, (_, _, _, expr, _, _, _))) => Ok((input, expr)),
        _                                     => Err(Error(CustomError::MyError)),
    }
}


fn parse_identifier(input: &str) -> IResult<&str, Expression, CustomError<&str>> {
    let acc: IResult<&str, (&str, &str, &str, &str)> = tuple((
        take_while(is_whitespace),
        take_while_m_n(1, 1, is_first_character_of_id),
        take_while(is_second_character_onwards_of_id),
        take_while(is_whitespace),
    ))(input);

    match acc {
        Ok((input, (_, head, tail, _))) => Ok((input,
                                               Expression::Variable(format!("{}{}",
                                                                            head, tail)))),
        _                               => Err(Error(CustomError::MyError)),
    }
}

// }}}


fn _parse_expr(input: &str, prec: Precedence) -> IResult<&str, Expression, CustomError<&str>> {

    if prec >= Precedence::Parens {
        let prec = Precedence::Parens;
        let input_as_parens = _parse_parens(input);

        match input_as_parens {
            Ok(_)  => input_as_parens,
            Err(_) => _parse_expr(input, prec-1)
        }
    }
    else if prec >= Precedence::Identifier {
        let input_as_iden = parse_identifier(input);

        match input_as_iden {
            Ok(_)  => input_as_iden,
            Err(_) => Err(Error(CustomError::MyError))
        }
    } else {
        unimplemented!("Precedence {:?}.", prec);
    }
}


pub fn parse_expr(input: &str) -> Expression {
    match _parse_expr(input, Precedence::Parens) {
        Ok(("", expr)) => expr,
        _                 => panic!(),
    }
}

// vim: fdm=marker
