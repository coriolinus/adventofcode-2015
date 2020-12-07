//! Simple parser for relatively fixed parsing tasks.
//!
//! For dirt-simple parsing, you can go straight to `parse()`. For more complex
//! settings, the best point of entry is to config a `Parser` by calling the appropriate builder
//! functions and then call its
//! `.parse()` method.

use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};

lazy_static! {
    static ref LETTERS: HashSet<char> = {
        let mut letters = HashSet::new();
        for letter in "abcdefghijklmnopqrstuvwxyz".chars() {
            letters.insert(letter);
        }
        letters
    };
    static ref NUMBERS: HashSet<char> = {
        let mut numbers = HashSet::new();
        for number in "1234567890".chars() {
            numbers.insert(number);
        }
        numbers
    };
    static ref PUNCTUATION: HashSet<char> = {
        let mut punctuation = HashSet::new();
        for p in ".,:?!;%".chars() {
            punctuation.insert(p);
        }
        punctuation
    };
}

/// True if the input is nothing but lowercase ASCII letters.
pub fn is_just_letters(s: &str) -> bool {
    for c in s.chars() {
        if !LETTERS.contains(&c) {
            return false;
        }
    }
    true
}

/// True if the input is nothing but ASCII digits.
pub fn is_just_numbers(s: &str) -> bool {
    for c in s.chars() {
        if !NUMBERS.contains(&c) {
            return false;
        }
    }
    true
}

/// A parser following English reading order is a Right parser.
///
/// Note: A Left parser reverses the normal sequence of tokens. That is, if you are parsing
/// strings like `x -> y b` to the Left, the arrow is now token 2.
#[derive(Clone)]
pub enum ParseDirection {
    Left,
    Right,
}

/// Line-based parser for relatively fixed inputs.
#[derive(Clone)]
pub struct Parser {
    direction: ParseDirection,
    tokenizer_split: String,
    fixed_tokens: HashMap<usize, String>,
    consume_only: Option<usize>,
    force_lowercase: bool,
    require_at_least: Option<usize>,
    require_fewer_than: Option<usize>,
    clear_trailing_punctuation: bool,
}

impl Default for Parser {
    fn default() -> Parser {
        Parser {
            direction: ParseDirection::Right,
            tokenizer_split: " ".to_string(),
            fixed_tokens: HashMap::new(),
            consume_only: None,
            force_lowercase: true,
            require_at_least: None,
            require_fewer_than: None,
            clear_trailing_punctuation: false,
        }
    }
}

impl Parser {
    /// The direction toward which parsing should be attempted.
    ///
    /// A parser following English reading order is a Right parser.
    ///
    /// Note: A Left parser reverses the normal sequence of tokens. That is, if you are parsing
    /// strings like `x -> y b` to the Left, the arrow is now token 2.
    ///
    /// Default: `Right`
    pub fn direction(&self, pd: ParseDirection) -> Parser {
        Parser {
            direction: pd,
            ..self.to_owned()
        }
    }

    /// This is the substr searched for to tokenize the input.
    ///
    /// Default: `" "`
    pub fn tokenizer_split(&self, ts: &str) -> Parser {
        Parser {
            tokenizer_split: ts.to_string(),
            ..self.to_owned()
        }
    }

    /// These tokens must be present at the indicated position or the parse will fail.
    ///
    /// For example:
    ///
    /// If you are parsing the following using a Right parser:
    ///
    /// - `x -> y b`
    /// - `z -> a c`
    ///
    /// `fixed_tokens` should contain only the pair `1 : "->"`.
    /// This indicates that the second token must be an arrow for this parse to be valid.
    ///
    /// Note that the fixed tokens are *not* included in `ParseResult::tokens`. A successful parse
    /// of the first example string would result in `ParseResult::tokens == Vec!["x", "y", "b"]`.
    ///
    /// Default: `HashMap::new()`
    pub fn fixed_tokens(&self, ft: HashMap<usize, String>) -> Parser {
        Parser {
            fixed_tokens: ft,
            ..self.to_owned()
        }
    }

    /// Convert every token to lowercase when true.
    ///
    /// Default: `true`.
    pub fn force_lowercase(&self, fl: bool) -> Parser {
        Parser {
            force_lowercase: fl,
            ..self.to_owned()
        }
    }

    /// Consume only `N` tokens if it is not `None`.
    /// Useful if there is a portion which might match a variety of parsing options,
    /// and therefore needs a more specialized parser.
    ///
    /// The rest of the tokens are returned with the key `rest`.
    ///
    /// Default: `None`.
    pub fn consume_only(&self, n: Option<usize>) -> Parser {
        Parser {
            consume_only: n,
            ..self.to_owned()
        }
    }

    /// Require at least `N` tokens if it is not `None`.
    /// This includes fixed tokens: if you want to parse `x -> y b` but not `c -> d`,
    /// this should be `4`.
    ///
    /// Default: `None`.
    pub fn require_at_least(&self, n: Option<usize>) -> Parser {
        Parser {
            require_at_least: n,
            ..self.to_owned()
        }
    }

    /// Require fewer than `N` tokens if it is not `None`.
    /// Fail fast if there exist `N` or more tokens.
    /// Useful to guarantee correct inputs when not using `consume_only`.
    ///
    /// Default: `None`.
    pub fn require_fewer_than(&self, n: Option<usize>) -> Parser {
        Parser {
            require_fewer_than: n,
            ..self.to_owned()
        }
    }

    /// If `true`, check every token's last character. If it's punctuation, drop it.
    ///
    /// Punctuation defined as ".,:?!;%".
    ///
    /// Note: doesn't affect un-consumed tokens in `rest`.
    ///
    /// Default: `false`.
    pub fn clear_trailing_punctuation(&self, c: bool) -> Parser {
        Parser {
            clear_trailing_punctuation: c,
            ..self.to_owned()
        }
    }

    /// Parse a string using these options, if you only care about a few of the tokens.
    ///
    /// `names` maps token position to a key in the output mapping.
    ///
    /// Returns a mapping of the assigned name to the token found for every item in `names`.
    ///
    /// Implies `self.require_at_least >= n`, where `n` is the largest item in `names.keys()`.
    ///
    /// # Example
    /// ```
    /// # use std::collections::HashMap;
    /// use util::parse::Parser;
    ///
    /// let input = "Dancer can fly 37 km/s for 1 seconds, but then must rest for 36 seconds.";
    /// let parser = Parser::default().force_lowercase(false);
    /// let result = parser.parse_named({
    ///     let mut h = HashMap::new();
    ///     h.insert(0, "name".to_string());
    ///     h.insert(3, "speed".to_string());
    ///     h.insert(6, "fly".to_string());
    ///     h.insert(13, "rest".to_string());
    ///     h
    /// }, input).unwrap();
    ///
    /// let name = result.get(&"name".to_string()).unwrap();
    /// let speed = result.get(&"speed".to_string()).unwrap();
    /// let fly = result.get(&"fly".to_string()).unwrap();
    /// let rest = result.get(&"rest".to_string()).unwrap();
    ///
    /// assert_eq!(name, "Dancer");
    /// assert_eq!(speed, "37");
    /// assert_eq!(fly, "1");
    /// assert_eq!(rest, "36");
    /// ```
    pub fn parse_named(
        &self,
        names: HashMap<usize, String>,
        input: &str,
    ) -> Result<HashMap<String, String>, ParseError> {
        if names.is_empty() {
            return Err(ParseError::InputIsEmpty);
        }

        let max_key = names.keys().max().unwrap();
        let p = if self.require_at_least.is_some() && &self.require_at_least.unwrap() >= max_key {
            self.clone()
        } else {
            self.require_at_least(Some(*max_key))
        };

        match p.parse(input) {
            Ok(v) => {
                let mut ret = HashMap::new();
                for (position, key) in &names {
                    let ref val = v.tokens[*position];
                    ret.insert(key.clone(), val.clone());
                }
                Ok(ret)
            }
            Err(e) => Err(e),
        }
    }

    /// Parse a string using these options
    pub fn parse(&self, input: &str) -> Result<ParseResult, ParseError> {
        let input = input.trim();
        if input.is_empty() {
            return Err(ParseError::InputIsEmpty);
        }

        let input = if self.force_lowercase {
            input.to_lowercase() // automatically converts to String
        } else {
            input.to_string()
        };

        let mut tokens: Vec<&str> = input.split(&self.tokenizer_split).collect();
        match self.direction {
            ParseDirection::Left => tokens.reverse(),
            _ => {}
        }

        if self.require_at_least.is_some() && tokens.len() < self.require_at_least.unwrap() {
            return Err(ParseError::TooFewTokens);
        }

        if self.require_fewer_than.is_some() && tokens.len() >= self.require_fewer_than.unwrap() {
            return Err(ParseError::TooManyTokens);
        }

        let mut pr = ParseResult {
            tokens: Vec::new(),
            rest: None,
        };

        for (i, tok) in tokens.iter().enumerate() {
            // have consumed enough tokens
            if self.consume_only.is_some() && i >= self.consume_only.unwrap() {
                pr.rest = Some(tokens.iter().skip(i).map(|&s| s.to_string()).collect());
                return Ok(pr);
            }
            // check fixed tokens
            if self.fixed_tokens.contains_key(&i) {
                if tok == self.fixed_tokens.get(&i).unwrap() {
                    // discard it
                    continue;
                } else {
                    // token mismatch on fixed key
                    return Err(ParseError::TokenMismatchOnFixedKey);
                }
            }
            let mut tok = tok.to_string();
            // check if we're eliminating punctuation
            if self.clear_trailing_punctuation {
                let trail = tok.pop().unwrap();
                if !PUNCTUATION.contains(&trail) {
                    // oops, it wasn't punctuation
                    tok.push(trail)
                }
            }

            // we must be ready to add the current token and move on!
            pr.tokens.push(tok);
        }
        Ok(pr)
    }
}

pub struct ParseResult {
    pub tokens: Vec<String>,
    pub rest: Option<Vec<String>>,
}

#[derive(PartialEq, Eq, Debug)]
pub enum ParseError {
    InputIsEmpty,
    TooFewTokens,
    TooManyTokens,
    TokenMismatchOnFixedKey,
    /// Never returned by `.parse()`, this error is a catch-all so that consumer code can return
    /// a `Result<_, ParseError>` instead of `Result<_, Option<ParseError>>` if it needs to trigger
    /// a parse error based on otherwise valid input.
    ConsumerError,
}

/// Parse a string using `Parser::default()`.
///
/// Roughly equivalent to `input.to_lowercase().split(' ').collect()`, but it returns
/// an `Err` when the input string is empty.
pub fn parse(input: &str) -> Result<ParseResult, ParseError> {
    Parser::default().parse(input)
}

#[cfg(test)]
mod tests {
    use super::{parse, ParseDirection, ParseError, Parser};
    use std::collections::HashMap;

    #[test]
    fn test_parse_wires() {
        let wires = vec!["123 -> x", "NOT x -> h", "x LSHIFT 2 -> f"];
        let results = vec![vec!["x"], vec!["h"], vec!["f"]];
        let rests = vec![vec!["123"], vec!["x", "not"], vec!["2", "lshift", "x"]];

        let results: Vec<Vec<String>> = results
            .iter()
            .map(|v| v.iter().map(|&s| s.to_string()).collect())
            .collect();
        let rests: Vec<Vec<String>> = rests
            .iter()
            .map(|v| v.iter().map(|&s| s.to_string()).collect())
            .collect();

        let expected = results.iter().zip(rests);

        let po = Parser::default()
            .direction(ParseDirection::Left)
            .require_at_least(Some(3))
            .require_fewer_than(Some(6))
            .consume_only(Some(2))
            .fixed_tokens({
                let mut h = HashMap::new();
                h.insert(1, "->".to_string());
                h
            });

        for (wire, (result, rest)) in wires.iter().zip(expected) {
            let pr = po.parse(wire);
            assert!(pr.is_ok()); //these should all be parseable
            let pr = pr.ok().unwrap();
            assert_eq!(&pr.tokens, result);
            assert_eq!(pr.rest, Some(rest));
        }
    }

    #[test]
    fn test_parsing_empty_string_fails() {
        let pr = parse("");
        match pr {
            Err(ParseError::InputIsEmpty) => {}
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_too_many_fails() {
        let pr = Parser::default()
            .require_fewer_than(Some(2))
            .parse("foo bar");
        match pr {
            Err(ParseError::TooManyTokens) => {}
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_too_few_fails() {
        let pr = Parser::default().require_at_least(Some(2)).parse("foo");
        match pr {
            Err(ParseError::TooFewTokens) => {}
            _ => panic!(),
        }
    }

    #[test]
    fn test_parse_fixed_token_not_found() {
        let pr = Parser::default()
            .fixed_tokens({
                let mut h = HashMap::new();
                h.insert(0, "->".to_string());
                h
            })
            .parse("<-");
        match pr {
            Err(ParseError::TokenMismatchOnFixedKey) => {}
            _ => panic!(),
        }
    }
}
