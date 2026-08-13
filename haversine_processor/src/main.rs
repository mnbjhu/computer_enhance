use std::{fs, iter::Peekable, num::ParseFloatError, str::Chars};

use thiserror::Error;

#[derive(Debug)]
struct Pair {
    x0: f64,
    y0: f64,
    x1: f64,
    y1: f64,
}

impl Pair {
    fn new() -> Self {
        Pair {
            x0: f64::NAN,
            y0: f64::NAN,
            x1: f64::NAN,
            y1: f64::NAN,
        }
    }
}

#[derive(Debug, Error)]
enum ParseError<'a> {
    #[error("Expected {expected} at char {offset}")]
    ExpectedChar { expected: char, offset: usize },

    #[error("Failed to parse float at char {offset}: {err}")]
    FloatError { err: ParseFloatError, offset: usize },

    #[error("Found {found} but expected {expected} at char {offset}")]
    UnexpectedKey {
        found: &'a str,
        expected: &'static str,
        offset: usize,
    },

    #[error("Missing key {name} at char {offset}")]
    MissingKey { name: &'static str, offset: usize },
}

struct Input<'a> {
    text: &'a str,
    chars: Peekable<Chars<'a>>,
    offset: usize,
}

impl<'a> Input<'a> {
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }
    fn next(&mut self) -> Option<char> {
        self.offset += 1;
        self.chars.next()
    }
}

impl<'a> Iterator for Input<'a> {
    type Item = Pair;

    fn next(&mut self) -> Option<Self::Item> {
        let is_first = self.offset == 0;
        if is_first {
            self.skip_ws();
            self.parse_char('{').unwrap_or_else(|e| panic!("{e}"));
            self.skip_ws();
            let offset = self.offset;
            let pairs_key = self.parse_string().unwrap_or_else(|e| panic!("{e}"));
            if pairs_key != "pairs" {
                let err = ParseError::UnexpectedKey {
                    found: pairs_key,
                    expected: "pairs",
                    offset,
                };
                panic!("{err}")
            }
            self.skip_ws();
            self.parse_char(':').unwrap_or_else(|e| panic!("{e}"));
            self.skip_ws();
            self.parse_char('[').unwrap_or_else(|e| panic!("{e}"));
            self.skip_ws();
        }
        if matches!(self.parse_char(']'), Err(_)) {
            if !is_first {
                self.parse_char(',').unwrap_or_else(|e| panic!("{e}"));
                self.skip_ws();
            }
            let pair = self.parse_pair().unwrap_or_else(|e| panic!("{e}"));
            self.skip_ws();
            Some(pair)
        } else {
            self.skip_ws();
            self.parse_char('}').unwrap_or_else(|e| panic!("{e}"));
            None
        }
    }
}

fn main() {
    let text = fs::read_to_string("input.json").unwrap();
    let input = Input {
        text: &text,
        chars: text.chars().peekable(),
        offset: 0,
    };
    input.for_each(|p| println!("{p:#?}"));
}

impl<'a> Input<'a> {
    fn skip_ws(&mut self) {
        while let Some(next) = self.peek()
            && next.is_whitespace()
        {
            self.next().unwrap();
        }
    }

    fn parse_char(&mut self, c: char) -> Result<(), ParseError<'a>> {
        let Some(next) = self.peek() else {
            return Err(ParseError::ExpectedChar {
                expected: c,
                offset: self.offset,
            });
        };
        if *next == c {
            self.next();
            Ok(())
        } else {
            Err(ParseError::ExpectedChar {
                expected: c,
                offset: self.offset,
            })
        }
    }

    fn parse_string(&mut self) -> Result<&'a str, ParseError<'a>> {
        self.parse_char('"')?;
        let start = self.offset;
        while let Some(next) = self.next() {
            if next == '"' {
                let end = self.offset;
                return Ok(&self.text[start..end - 1]);
            }
        }
        Err(ParseError::ExpectedChar {
            expected: '"',
            offset: self.offset,
        })
    }

    fn parse_float(&mut self) -> Result<f64, ParseError<'a>> {
        let start = self.offset;
        while let Some(next) = self.peek()
            && (next.is_digit(10) || *next == '-' || *next == '.')
        {
            self.next().unwrap();
        }
        let end = self.offset;
        match self.text[start..end].parse::<f64>() {
            Ok(res) => Ok(res),
            Err(err) => Err(ParseError::FloatError { err, offset: start }),
        }
    }

    fn parse_pair(&mut self) -> Result<Pair, ParseError<'a>> {
        let mut res = Pair::new();
        let mut count = 0;
        let offset = self.offset;
        self.parse_char('{')?;
        self.skip_ws();
        while matches!(self.parse_char('}'), Err(_)) {
            let offset = self.offset;
            if count != 0 {
                self.parse_char(',')?;
                self.skip_ws();
            }
            let (key, value) = self.parse_kv()?;
            match key {
                "x0" => res.x0 = value,
                "y0" => res.y0 = value,
                "x1" => res.x1 = value,
                "y1" => res.y1 = value,
                _ => {
                    return Err(ParseError::UnexpectedKey {
                        found: key,
                        expected: "one of 'x0', 'y0', 'x1', 'y1'",
                        offset,
                    });
                }
            }
            self.skip_ws();
            count += 1;
        }
        if res.x0.is_nan() {
            return Err(ParseError::MissingKey { name: "x0", offset });
        }
        if res.y0.is_nan() {
            return Err(ParseError::MissingKey { name: "y0", offset });
        }
        if res.x1.is_nan() {
            return Err(ParseError::MissingKey { name: "x1", offset });
        }
        if res.y1.is_nan() {
            return Err(ParseError::MissingKey { name: "y1", offset });
        }
        Ok(res)
    }

    fn parse_kv(&mut self) -> Result<(&'a str, f64), ParseError<'a>> {
        let key = self.parse_string()?;
        self.skip_ws();
        self.parse_char(':')?;
        self.skip_ws();
        let value = self.parse_float()?;
        Ok((key, value))
    }
}
