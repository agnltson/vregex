use winnow::combinator::*;
use winnow::Result;
use winnow::token::one_of;
use winnow::Parser;

pub fn parse_regex(input: &str) -> Result<Regex, String> {
    let mut s = input;
    expr.parse_next(&mut s)
        .map_err(|e| format!("parse error: {:?}", e))
}

#[derive(Clone, Debug)]
pub enum Regex {
    Literal(char),
    Concat(Box<Regex>, Box<Regex>),
    Plus(Box<Regex>, Box<Regex>),
    Star(Box<Regex>),
}

fn literal(input: &mut &str) -> Result<Regex> {
    one_of('a'..='z')
        .map(Regex::Literal)
        .parse_next(input)
}

fn atom(input: &mut &str) -> Result<Regex> {
    alt((
        delimited('(', expr, ')'),
        literal,
        ))
        .parse_next(input)
}

fn star(input: &mut &str) -> Result<Regex> {
    let mut node = atom.parse_next(input)?;
    while opt('*').parse_next(input)?.is_some() {
        node = Regex::Star(Box::new(node));
    }
    Ok(node)
}

fn concat(input: &mut &str) -> Result<Regex> {
    let mut nodes = Vec::new();
    nodes.push(star.parse_next(input)?);
    while let Ok(next) = star.parse_next(input) {
        nodes.push(next);
    }
    let mut iter = nodes.iter();
    let mut result = iter.next().unwrap().clone();
    for n in iter {
        result = Regex::Concat(Box::new(result), Box::new(n.clone()));
    }
    Ok(result)
}

fn plus(input: &mut &str) -> Result<Regex> {
    let mut left = concat.parse_next(input)?;
    while opt('+').parse_next(input)?.is_some() {
        let right = concat.parse_next(input)?;
        left = Regex::Plus(Box::new(left), Box::new(right));
    }
    Ok(left)
}

fn expr(input: &mut &str) -> Result<Regex> {
    plus.parse_next(input)
}
