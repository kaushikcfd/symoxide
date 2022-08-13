use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n, take_while},
    sequence::tuple,
    Err::Error,
    error::{ParseError, ErrorKind},
};
use crate::Expression;

/// {{{ Precedence

#[derive(PartialEq, Eq, PartialOrd, Ord)]
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

/// }}}

impl std::ops::Sub<i32> for Precedence {
    type Output = Precedence;

    fn sub(self, other: i32)  -> Self::Output {
        return Precedence::from_i32(self as i32 - (other as i32));
    }
}





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


// {{{

fn is_first_character_of_id(x: char) -> bool{
    x.is_ascii_alphabetic() || x == '_'
}


fn is_second_character_onwards_of_id(x: char) -> bool{
    x.is_ascii_alphanumeric() || x == '_'
}

// }}}



fn _parse_expr(input: &str, prec: Precedence) -> IResult<&str, Expression, CustomError<&str>> {

    if prec >= Precedence::Parens {
        let prec = Precedence::Parens;
        println!("Trying parens on {}", input);
        let inner_expr = tuple((tag("("),
                                |x| _parse_expr(x, Precedence::Parens) ,
                                tag(")")))(input);
        let result = match inner_expr {
            Ok((input, (_, expr, _))) => Ok((input, expr)),
            _                 => _parse_expr(input, prec-1),
        };
        return result;
    }
    if prec >= Precedence::Identifier {
        let prec = Precedence::Identifier;
        println!("Trying identifier on {}", input);
        // take the first character
        let first_char_match: IResult<&str, &str> = take_while_m_n(1, 1, is_first_character_of_id)(input);
        match first_char_match {
            Ok((input_minus_first, first_char)) => {
                let second_char_onwards_match: IResult<&str, &str>  =
                    take_while(is_second_character_onwards_of_id)(input_minus_first);
                match second_char_onwards_match {
                    Ok((input_minus_word, second_char_onwards)) => {
                        let name = format!("{}{}", first_char, second_char_onwards);
                        return Ok((input_minus_word, Expression::Variable(name)));
                    },
                    _ => {return _parse_expr(input, prec-1)},
                }
            },
            _                       =>  {return _parse_expr(input, prec-1)},
        }
    }

    return Err(Error(CustomError::MyError));
}

pub fn parse_expr(input: &str) -> Expression {
    match _parse_expr(input, Precedence::Parens) {
        Ok((_, expr)) => expr,
        _                 => panic!(),
    }
}
