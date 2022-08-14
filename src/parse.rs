use nom::{
    IResult,
    bytes::complete::{tag, take_while_m_n, take_while},
    sequence::tuple,
    Err::Error,
    error::{ParseError, ErrorKind},
};
use std::rc::Rc;
use crate::{Expression, BinaryOpType};
use log;

/// {{{ Precedence

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
pub enum Precedence{
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
    Identifier,
    Literal,
}

impl Precedence {
    fn from_i32(x: i32) -> Precedence {
        let result = match x {
             0 => Precedence::Or,
             1 => Precedence::And,
             2 => Precedence::Not,
             3 => Precedence::Eq,
             4 => Precedence::Neq,
             5 => Precedence::Gt,
             6 => Precedence::Lt,
             7 => Precedence::Ge,
             8 => Precedence::Le,
             9 => Precedence::BitwiseOr,
            10 => Precedence::BitwiseXor,
            11 => Precedence::BitwiseAnd,
            12 => Precedence::Lshift,
            13 => Precedence::Rshift,
            14 => Precedence::Sum,
            15 => Precedence::Minus,
            16 => Precedence::Product,
            17 => Precedence::Div,
            18 => Precedence::FloorDiv,
            19 => Precedence::Modulo,
            20 => Precedence::Exponent,
            21 => Precedence::Parens,
            22 => Precedence::Subscript,
            23 => Precedence::Call,
            24 => Precedence::Identifier,
            25 => Precedence::Literal,
            _  => panic!("Unexpected Precedence constant {}.", x),
        };

        assert_eq!(result.clone() as i32, x);
        return result;
    }
}


impl std::ops::Sub<i32> for Precedence {
    type Output = Self;

    fn sub(self, other: i32)  -> Self {
        return Precedence::from_i32(self as i32 - (other as i32));
    }
}

impl std::ops::Add<i32> for Precedence {
    type Output = Self;

    fn add(self, other: i32)  -> Self {
        return Precedence::from_i32(self as i32 + (other as i32));
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
        |x| _parse_expr(x, Precedence::from_i32(0)) ,
        take_while(is_whitespace),
        tag(")"),
        take_while(is_whitespace),
    ))(input);




    match inner_expr {
        Ok((input, (_, _, _, expr, _, _, _))) => Ok((input, expr)),
        _                                     => Err(Error(CustomError::MyError)),
    }
}


fn parse_left_assoc_binary_op(input: &str, bin_op_type: BinaryOpType) -> IResult<&str, Expression, CustomError<&str>> {
    let operator_str = match bin_op_type {
        BinaryOpType::Product  => "*",
        BinaryOpType::Sum      => "+",
        BinaryOpType::Divide   => "/",
        BinaryOpType::FloorDiv => "//",
    };

    // TODO: Can this be fixed somehow
    // Yucky, but closure support is dubios, yields compiler error
    // as: only FnMut allowed, but closure only implemented FnOnce
    let acc = match bin_op_type {
        BinaryOpType::Product => tuple((
            take_while(is_whitespace),
            |x| _parse_expr(x, Precedence::Product+1),
            take_while(is_whitespace),
            tag(operator_str),
            take_while(is_whitespace),
            |x| _parse_expr(x, Precedence::Product),
            take_while(is_whitespace),
        ))(input),

        BinaryOpType::Sum => tuple((
            take_while(is_whitespace),
            |x| _parse_expr(x, Precedence::Sum+1),
            take_while(is_whitespace),
            tag(operator_str),
            take_while(is_whitespace),
            |x| _parse_expr(x, Precedence::Sum),
            take_while(is_whitespace),
        ))(input),

        BinaryOpType::Divide => tuple((
            take_while(is_whitespace),
            |x| _parse_expr(x, Precedence::Div+1),
            take_while(is_whitespace),
            tag(operator_str),
            take_while(is_whitespace),
            |x| _parse_expr(x, Precedence::Div),
            take_while(is_whitespace),
        ))(input),

        BinaryOpType::FloorDiv => tuple((
            take_while(is_whitespace),
            |x| _parse_expr(x, Precedence::FloorDiv+1),
            take_while(is_whitespace),
            tag(operator_str),
            take_while(is_whitespace),
            |x| _parse_expr(x, Precedence::FloorDiv),
            take_while(is_whitespace),
        ))(input),
    };

    match acc {
        Ok((input, (_, left, _, _, _, right, _))) => Ok((input,
                                                         Expression::BinaryOp(Rc::new(left),
                                                                              bin_op_type,
                                                                              Rc::new(right)))),
        _                                        => Err(Error(CustomError::MyError)),
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
        Ok((input, (_, head, tail, _))) if head != "" => Ok((input,
                                                             Expression::Variable(format!("{}{}",
                                                                            head, tail)))),
        _                               => Err(Error(CustomError::MyError)),
    }
}


// }}}


fn _parse_expr(input: &str, min_prec: Precedence) -> IResult<&str, Expression, CustomError<&str>> {

    if Precedence::Sum >= min_prec {
        log::debug!("Parsing '{}' as Sum.", input);
        let min_prec = Precedence::Sum;
        let input_as_sum = parse_left_assoc_binary_op(input, BinaryOpType::Sum);

        let result = match input_as_sum {
            Ok(_)  => input_as_sum,
            Err(_) => {
                log::debug!("Couldn't parse '{}' as Sum.", input);
                _parse_expr(input, min_prec+1)
            }
        };
        return result;
    }
    else if Precedence::Product >= min_prec {
        log::debug!("Parsing '{}' as Product.", input);
        let min_prec = Precedence::Product;
        let input_as_product = parse_left_assoc_binary_op(input, BinaryOpType::Product);

        let result = match input_as_product {
            Ok(_)  => input_as_product,
            Err(_) => {
                log::debug!("Couldn't parse '{}' as Product.", input);
                _parse_expr(input, min_prec+1)
            }
        };
        return result;
    }
    else if Precedence::Parens >= min_prec {
        log::debug!("Parsing '{}' as Parens.", input);
        let min_prec = Precedence::Parens;
        let input_as_parens = _parse_parens(input);

        let result = match input_as_parens {
            Ok(_)  => input_as_parens,
            Err(_) => {
                log::debug!("Couldn't parse '{}' as Parens.", input);
                _parse_expr(input, min_prec+1)
            }
        };
        return result;
    }
    else if Precedence::Identifier >= min_prec {
        log::debug!("Parsing '{}' as Identifier.", input);
        let input_as_iden = parse_identifier(input);

        let result = match input_as_iden {
            Ok(_)  => {
                log::debug!("Parsed '{}' as Identifier.", input);
                input_as_iden
            },
            Err(_) => {
                log::debug!("Couldn't parse '{}' as Identifier.", input);
                Err(Error(CustomError::MyError))
            }
        };
        return result;
    } else {
        unimplemented!("Precedence {:?}.", min_prec);
    }
}


pub fn parse_expr(input: &str) -> Expression {
    match _parse_expr(input, Precedence::from_i32(0)) {
        Ok(("", expr)) => expr,
        _              => panic!(),
    }
}

// vim: fdm=marker
