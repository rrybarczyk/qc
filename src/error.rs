use crate::common::*;

#[derive(Debug, PartialEq)]
pub(crate) enum Error {
  StackUnderflow,
  ParseError(ParseIntError),
}

impl From<ParseIntError> for Error {
  fn from(error: ParseIntError) -> Self {
    Error::ParseError(error)
  }
}
