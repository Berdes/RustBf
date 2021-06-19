extern crate nom;
use nom::{
  IResult,
  branch::alt,
  bytes::complete::tag,
  character::complete::char,
  combinator::eof,
  multi::{many0, many_till},
  sequence::delimited,
};

#[derive(Debug,PartialEq)]
enum Bf {
    NextCell,
    PrevCell,
    Incr,
    Decr,
    Out,
    In,
    Loop(Vec<Bf>),
}

fn parse_next(input: &str) -> IResult<&str, Bf> {
    let (input, _) = tag(">")(input)?;
    Ok((input, Bf::NextCell))
}

fn parse_prev(input: &str) -> IResult<&str, Bf> {
    let (input, _) = tag("<")(input)?;
    Ok((input, Bf::PrevCell))
}

fn parse_incr(input: &str) -> IResult<&str, Bf> {
    let (input, _) = tag("+")(input)?;
    Ok((input, Bf::Incr))
}

fn parse_decr(input: &str) -> IResult<&str, Bf> {
    let (input, _) = tag("-")(input)?;
    Ok((input, Bf::Decr))
}

fn parse_out(input: &str) -> IResult<&str, Bf> {
    let (input, _) = tag(".")(input)?;
    Ok((input, Bf::Out))
}

fn parse_in(input: &str) -> IResult<&str, Bf> {
    let (input, _) = tag(",")(input)?;
    Ok((input, Bf::In))
}

fn parse_loop(input: &str) -> IResult<&str, Bf> {
    let (input, loop_inside) =
        delimited(char('['), parse_inside_loop, char(']'))(input)?;
    Ok((input, Bf::Loop(loop_inside)))
}

fn parse_inside_loop(input: &str) -> IResult<&str, Vec<Bf>> {
    many0(alt((parse_next,
               parse_prev,
               parse_incr,
               parse_decr,
               parse_out,
               parse_in,
               parse_loop)))(input)
}

fn parse_bf(input: &str) -> IResult<&str, Vec<Bf>> {
    let (input, (parsed, _)) =
        many_till(alt((parse_next,
                       parse_prev,
                       parse_incr,
                       parse_decr,
                       parse_out,
                       parse_in,
                       parse_loop)), eof)(input)?;
    Ok((input, parsed))
}

fn main() {
  println!("Hello, world!");
}

#[test]
fn parse_single() {
    assert_eq!(parse_bf(">"), Ok(("", vec![Bf::NextCell])));
    assert_eq!(parse_bf("<"), Ok(("", vec![Bf::PrevCell])));
    assert_eq!(parse_bf("+"), Ok(("", vec![Bf::Incr])));
    assert_eq!(parse_bf("-"), Ok(("", vec![Bf::Decr])));
    assert_eq!(parse_bf("."), Ok(("", vec![Bf::Out])));
    assert_eq!(parse_bf(","), Ok(("", vec![Bf::In])));
}

#[test]
fn parse_empty_loop() {
    assert_eq!(parse_bf("[]"), Ok(("", vec![Bf::Loop(vec![])])));
}

#[test]
fn parse_simple_loop() {
    assert_eq!(parse_bf("[-]"), Ok(("", vec![Bf::Loop(vec![Bf::Decr])])));
}

#[test]
fn parse_square_prog() {
    assert_eq!(
        parse_bf(",[>+>+<<-]>[>[>+<<<+>>-]>[<+>-]<<-]<."),
        Ok(("", vec![
            // Read value
            Bf::In,  // cell 0
            // Move value to cells 1 and 2
            Bf::Loop(vec![
                     Bf::NextCell,
                     Bf::Incr,  // cell 1
                     Bf::NextCell,
                     Bf::Incr,  // cell 2
                     Bf::PrevCell,
                     Bf::PrevCell,
                     Bf::Decr  // cell 0
            ]),
            // Multiply cells 1 and 2 with result in cell 0, using cell 3 as tmp
            Bf::NextCell,
            Bf::Loop(vec![
                     // Move cell 2 to cells 3 and 0
                     Bf::NextCell,
                     Bf::Loop(vec![
                              Bf::NextCell,
                              Bf::Incr,  // cell 3
                              Bf::PrevCell,
                              Bf::PrevCell,
                              Bf::PrevCell,
                              Bf::Incr,  // cell 0
                              Bf::NextCell,
                              Bf::NextCell,
                              Bf::Decr,  // cell 2
                     ]),
                     // Move cell 3 to cell 2
                     Bf::NextCell,
                     Bf::Loop(vec![
                              Bf::PrevCell,
                              Bf::Incr,  // cell 2
                              Bf::NextCell,
                              Bf::Decr,  // cell 3
                     ]),
                     Bf::PrevCell,
                     Bf::PrevCell,
                     Bf::Decr,  // cell 1
            ]),
            Bf::PrevCell,
            Bf::Out  // cell 0
        ])));
}

#[test]
fn bad_char() {
    assert!(matches!(parse_bf("a"), Err(..)));
}

#[test]
fn bad_char_in_loop() {
    assert!(matches!(parse_bf("[a]"), Err(..)));
}

#[test]
fn unmatched_loop() {
    assert!(matches!(parse_bf("["), Err(..)));
    assert!(matches!(parse_bf("[[]"), Err(..)));
    assert!(matches!(parse_bf("[]["), Err(..)));
    assert!(matches!(parse_bf("[]]"), Err(..)));
    assert!(matches!(parse_bf("[]][[]"), Err(..)));
}
