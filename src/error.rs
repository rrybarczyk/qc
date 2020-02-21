use crate::common::*;

#[derive(Debug, PartialEq)]
pub(crate) enum Error {
  StackUnderflow,
  ParseError(ParseIntError),
  EndianWidth { width: i64 },
}

impl From<ParseIntError> for Error {
  fn from(error: ParseIntError) -> Self {
    Error::ParseError(error)
  }
}
