use std::collections::HashMap;
use std::iter::Peekable;
use std::str::CharIndices;

mod error;
pub use error::ParseError;

mod expressions;
pub use expressions::Expression;

pub mod tokens;
pub use tokens::Token;
use tokens::Concat;

#[derive(Debug)]
pub struct Pattern<'a> {
  key:   &'a str,
  token: Box<dyn Token<'a> + 'a>,
}

impl<'a> Pattern<'a> {
  pub fn parse<'b, 'c>(
    &'c self,
    patterns: &'c Patterns<'a>,
    module:   &'b str,
    iter:     &mut Peekable<CharIndices<'b>>
  ) -> Result<Expression<'a, 'b>, ParseError> {
    if let Some((start, _)) = iter.peek() {
      let start = *start;
      let mut result = Expression::new(module.get(start..).unwrap(), self);
      self.token.parse(patterns, module, iter, &mut result)?;
      if let Some((end, _)) = iter.peek() {
        result.truncate(*end-start);
      }
      Ok(result)
    } else {
      Err(ParseError::new(
        format!("unexpected end of file, expected {}", self.key)
      ))
    }
  }
}

#[derive(Debug)]
pub struct Patterns<'a>{
  entry:    &'a str,
  patterns: HashMap<&'a str, Pattern<'a>>,
}

impl<'a> Patterns<'a> {
  pub fn new(syntax: &'a str) -> Self {
    let mut patterns = HashMap::new();
    let mut iter = syntax.char_indices().peekable();
    let mut key = get_key(&syntax, &mut iter).expect("no pattern found");
    let entry = key;
    loop {
      let token = Concat::parse(syntax, &mut iter);
      if patterns.insert(key, Pattern{ key, token }).is_some() {
        panic!("pattern key \"{}\" is used several times", key);
      }
      let key_opt = get_key(&syntax, &mut iter);
      if key_opt.is_none() {
        break
      }
      key = key_opt.unwrap();
    }
    Self{ entry, patterns }
  }

  pub fn get(&self, key: &str) -> Option<&Pattern<'a>> {
    self.patterns.get(key)
  }

  pub fn parse<'b, 'c>(
    &'c self, module: &'b str,
  ) -> Result<Expression<'a, 'b>, ParseError> {
    let mut iter = module.char_indices().peekable();
    let result = self.patterns[self.entry].parse(self, module, &mut iter);
    if let Some((i, _)) = iter.peek() {
      let remaining = module.get(*i..).unwrap();
      Err(ParseError::new(format!("remaining tokens: \"{}\"", remaining)))
    } else {
      result
    }
  }

}

fn get_key<'a>(
  syntax: &'a str,
  iter: &mut Peekable<CharIndices<'a>>,
) -> Option<&'a str> {
  let mut pattern_start: Option<usize> = None;
  let mut result: Option<&str> = None;
  while let Some((i, c)) = iter.next() {
    if !c.is_whitespace() {
      pattern_start = Some(i);
      break;
    }
  }
  if pattern_start.is_none() {
    return None;
  }
  while let Some((i, c)) = iter.next() {
    if c.is_whitespace() {
      result = Some(syntax.get(pattern_start.unwrap()..i).unwrap());
      break;
    }
  }
  result
}

