//! Simple parser for relatively fixed parsing tasks.

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
}

pub fn is_just_letters(s: &str) -> bool {
    for c in s.chars() {
        if !LETTERS.contains(&c) {
            return false;
        }
    }
    true
}

pub fn is_just_numbers(s: &str) -> bool {
    for c in s.chars() {
        if !NUMBERS.contains(&c) {
            return false;
        }
    }
    true
}

#[derive(Clone)]
pub enum ParseDirection {
    Left,
    Right,
}

#[derive(Clone)]
pub struct ParseOptions {
    direction: ParseDirection,
    tokenizer_split: String,
    fixed_tokens: HashMap<usize, String>,
    consume_only: Option<usize>,
    force_lowercase: bool,
    require_at_least: Option<usize>,
    require_fewer_than: Option<usize>,
}

impl Default for ParseOptions {
    fn default() -> ParseOptions {
        ParseOptions {
            direction: ParseDirection::Right,
            tokenizer_split: " ".to_string(),
            fixed_tokens: HashMap::new(),
            consume_only: None,
            force_lowercase: true,
            require_at_least: None,
            require_fewer_than: None,
        }
    }
}

impl ParseOptions {
    /// The direction toward which parsing should be attempted.
    ///
    /// A parser following English reading order is a Right parser.
    ///
    /// Note: A Left parser reverses the normal sequence of tokens. That is, if you are parsing
    /// strings like `x -> y b` to the Left, the arrow is now token 2.
    ///
    /// Default: Right
    pub fn direction(&self, pd: ParseDirection) -> ParseOptions {
        ParseOptions { direction: pd, ..self.to_owned() }
    }

    /// This is the substr searched for to tokenize the input.
    ///
    /// Default: " "
    pub fn tokenizer_split(&self, ts: &str) -> ParseOptions {
        ParseOptions { tokenizer_split: ts.to_string(), ..self.to_owned() }
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
    pub fn fixed_tokens(&self, ft: HashMap<usize, String>) -> ParseOptions {
        ParseOptions { fixed_tokens: ft, ..self.to_owned() }
    }

    /// Consume only `N` tokens if it is not `None`.
    /// Useful if there is a portion which might match a variety of parsing options,
    /// and therefore needs a more specialized parser.
    ///
    /// The rest of the tokens are returned with the key `rest`.
    ///
    /// Default: None.
    pub fn consume_only(&self, n: Option<usize>) -> ParseOptions {
        ParseOptions { consume_only: n, ..self.to_owned() }
    }

    /// Require at least `N` tokens if it is not `None`.
    /// This includes fixed tokens: if you want to parse `x -> y b` but not `c -> d`,
    /// this should be `4`.
    ///
    /// Default: None.
    pub fn require_at_least(&self, n: Option<usize>) -> ParseOptions {
        ParseOptions { require_at_least: n, ..self.to_owned() }
    }

    /// Require fewer than `N` tokens if it is not `None`.
    /// Fail fast if there exist `N` or more tokens.
    /// Useful to guarantee correct inputs when not using `consume_only`.
    ///
    /// Default: None.
    pub fn require_fewer_than(&self, n: Option<usize>) -> ParseOptions {
        ParseOptions { require_fewer_than: n, ..self.to_owned() }
    }

    /// Parse a string using these options
    pub fn parse(&self, input: &str) -> Result<ParseResult, ParseError> {
        Parser::parse_with_options(input, self.to_owned())
    }
}


pub struct ParseResult {
    pub tokens: Vec<String>,
    pub rest: Option<Vec<String>>,
}

pub enum ParseError {
    InputIsEmpty,
    TooFewTokens,
    TooManyTokens,
    TokenMismatchOnFixedKey,
}

#[derive(Default)]
pub struct Parser;

impl Parser {
    pub fn parse(input: &str) -> Result<ParseResult, ParseError> {
        Parser::parse_with_options(input, ParseOptions::default())
    }

    pub fn parse_with_options(input: &str,
                              options: ParseOptions)
                              -> Result<ParseResult, ParseError> {
        let input = input.trim();
        if input.is_empty() {
            return Err(ParseError::InputIsEmpty);
        }

        let input = if options.force_lowercase {
            input.to_lowercase() // automatically converts to String
        } else {
            input.to_string()
        };

        let mut tokens: Vec<&str> = input.split(&options.tokenizer_split).collect();
        match options.direction {
            ParseDirection::Left => tokens.reverse(),
            _ => {}
        }

        if options.require_at_least.is_some() && tokens.len() < options.require_at_least.unwrap() {
            return Err(ParseError::TooFewTokens);
        }

        if options.require_fewer_than.is_some() &&
           tokens.len() >= options.require_fewer_than.unwrap() {
            return Err(ParseError::TooManyTokens);
        }

        let mut pr = ParseResult {
            tokens: Vec::new(),
            rest: None,
        };

        for (i, tok) in tokens.iter().enumerate() {
            // have consumed enough tokens
            if options.consume_only.is_some() && i > options.consume_only.unwrap() {
                pr.rest = Some(tokens.iter().skip(i).map(|&s| s.to_string()).collect());
                return Ok(pr);
            }
            // check fixed tokens
            if options.fixed_tokens.contains_key(&i) {
                if tok == options.fixed_tokens.get(&i).unwrap() {
                    // discard it
                    continue;
                } else {
                    // token mismatch on fixed key
                    return Err(ParseError::TokenMismatchOnFixedKey);
                }
            }
            // we must be ready to add the current token and move on!
            pr.tokens.push(tok.to_string());
        }
        Ok(pr)
    }
}
