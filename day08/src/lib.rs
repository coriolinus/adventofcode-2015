use aoc2015::parse;

use std::{path::Path, unreachable};
use thiserror::Error;

#[derive(Clone, PartialEq, Eq, Debug)]
enum State {
    ExpectInitalQuote,
    Normal,
    Escape,
    CollectHex(String),
    OutsideQuotes,
}

fn unescape(s: &str) -> Result<String, Error> {
    let mut state = State::ExpectInitalQuote;
    let mut out = String::with_capacity(s.len());

    for ch in s.chars() {
        match (&mut state, ch) {
            (State::ExpectInitalQuote, '"') => state = State::Normal,
            (State::ExpectInitalQuote, _) => return Err(Error::NoLeadingQuote),
            (State::Normal, '"') => state = State::OutsideQuotes,
            (State::Normal, '\\') => state = State::Escape,
            (State::Normal, _) => out.push(ch),
            (State::Escape, _) => match ch {
                '\\' | '"' => {
                    out.push(ch);
                    state = State::Normal;
                }
                'x' => state = State::CollectHex(String::with_capacity(2)),
                _ => return Err(Error::UnexpectedEscapedChar(ch)),
            },
            (State::CollectHex(ref mut hex), _) => match hex.len() {
                0 => hex.push(ch),
                1 => {
                    hex.push(ch);
                    let n = u8::from_str_radix(hex, 16)
                        .map_err(|err| Error::BadHexEscape(err, hex.clone()))?;
                    out.push(n.into());
                    state = State::Normal;
                }
                _ => unreachable!(),
            },
            (State::OutsideQuotes, _) => return Err(Error::CharsAfterTrailingQuote),
        }
    }

    if state != State::OutsideQuotes {
        return Err(Error::NoTrailingQuote);
    }

    Ok(out)
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(2 + (2 * s.len()));
    out.push('"');
    for ch in s.chars() {
        match ch {
            '\\' | '"' => {
                out.push('\\');
                out.push(ch);
            }
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

pub fn part1(input: &Path) -> Result<(), Error> {
    let mut total_unescaped_len = 0;
    for s in parse::<String>(input)? {
        total_unescaped_len += s.chars().count() - unescape(&s)?.chars().count();
    }
    println!("total unescaped len: {}", total_unescaped_len);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut total_escaped_len = 0;
    for s in parse::<String>(input)? {
        total_escaped_len += escape(&s).chars().count() - s.chars().count();
    }
    println!("total escaped len: {}", total_escaped_len);
    Ok(())
}

pub fn roundtrip_input(input: &Path) -> Result<(), Error> {
    let mut output = false;
    for input in parse::<String>(input)? {
        let escaped = escape(&input);
        let unescaped = match unescape(&escaped) {
            Ok(unescaped) => unescaped,
            Err(err) => {
                eprintln!("unescaping {}; expect {}; got {}", escaped, input, err);
                return Err(err);
            }
        };
        let unescaped2 = match unescape(&unescaped) {
            Ok(u) => u,
            Err(err) => {
                eprintln!("double-unescaping {}; got {}", escaped, err);
                return Err(err);
            }
        };
        let reescaped = escape(&unescaped2);
        if input != unescaped {
            println!("{} E-> {} U-> {}", input, escaped, unescaped);
            output = true;
        }
        // the second condition here is required because otherwise there are too many false positives
        if input != reescaped && !input.contains("\\x") {
            println!("{} U-> {} E-> {}", input, unescaped2, reescaped);
            output = true;
        }
    }
    if !output {
        println!("roundtrip complete");
    }
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("string did not begin with a quote char")]
    NoLeadingQuote,
    #[error("string did not end with a quote char")]
    NoTrailingQuote,
    #[error("string contains chars past the final quote char")]
    CharsAfterTrailingQuote,
    #[error("unexpected escaped character '{0}'")]
    UnexpectedEscapedChar(char),
    #[error("failed to parse \"{1}\" as integer.")]
    BadHexEscape(#[source] std::num::ParseIntError, String),
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::rstest;

    #[rstest(
        input,
        expect,
        case(r#""m\x44gqbcppho\\b""#, r"mDgqbcppho\b"),
        case(r#""\\bo""#, r"\bo")
    )]
    fn test_unescape(input: &str, expect: &str) {
        assert_eq!(unescape(input).unwrap(), expect);
    }

    #[test]
    fn test_unescape_example() {
        let input = [r#""""#, r#""abc""#, r#""aaa\"aaa""#, r#""\x27""#];
        let mut tul = 0;
        for s in &input {
            tul += s.len() - unescape(s).unwrap().len();
        }
        assert_eq!(tul, 12);
    }
}
