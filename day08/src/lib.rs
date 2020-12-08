use aoc2015::parse;

use std::{path::Path, unreachable};
use thiserror::Error;

fn unescape(s: &str) -> Result<String, Error> {
    let mut within_quotes = false;
    let mut escape = false;
    let mut hexescape = None;
    let mut hex_escape_data = String::with_capacity(2);

    let mut out = String::with_capacity(s.len());

    for (idx, ch) in s.chars().enumerate() {
        match (idx, within_quotes, escape, hexescape) {
            (0, false, _, _) if ch == '"' => within_quotes = true,
            (0, false, _, _) => return Err(Error::NoLeadingQuote),
            (_, false, _, _) => return Err(Error::NoTrailingQuote),
            (_, _, false, _) if ch == '"' => within_quotes = false,
            (_, _, false, _) if ch == '\\' => escape = true,
            (_, _, true, None) => match ch {
                '\\' => {
                    escape = false;
                    out.push('\\');
                }
                '"' => {
                    escape = false;
                    out.push('"')
                }
                'x' => hexescape = Some(2),
                other => return Err(Error::UnexpectedEscapedChar(other)),
            },
            (_, _, true, Some(2)) => {
                hexescape = Some(1);
                hex_escape_data.push(ch);
            }
            (_, _, true, Some(1)) => {
                escape = false;
                hexescape = None;
                hex_escape_data.push(ch);
                let n = u8::from_str_radix(&hex_escape_data, 16).map_err(|err| {
                    Error::BadHexEscape(err, hex_escape_data.clone(), out.clone())
                })?;
                out.push(n.into());
                hex_escape_data.clear();
            }
            (_, _, true, Some(_)) => unreachable!(),
            (_, true, false, _) => out.push(ch),
        }
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
        total_unescaped_len += s.len() - unescape(&s)?.len();
    }
    println!("total unescaped len: {}", total_unescaped_len);
    Ok(())
}

pub fn part2(input: &Path) -> Result<(), Error> {
    let mut total_escaped_len = 0;
    for s in parse::<String>(input)? {
        total_escaped_len += escape(&s).len() - s.len();
    }
    println!("total escaped len: {}", total_escaped_len);
    Ok(())
}

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("string did not begin with a quote char")]
    NoLeadingQuote,
    #[error("string contains chars past the trailing quote char")]
    NoTrailingQuote,
    #[error("unexpected escaped character '{0}'")]
    UnexpectedEscapedChar(char),
    #[error("failed to parse \"{1}\" as integer. Parsed so far: \"{2}\"")]
    BadHexEscape(#[source] std::num::ParseIntError, String, String),
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
