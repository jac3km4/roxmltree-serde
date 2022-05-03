use std::fmt;

use serde::{de, ser};

use crate::NodeKind;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Clone, Debug, PartialEq)]
pub enum Error {
    NodesExhausted,
    ExpectedUnit(NodeKind),
    ExpectedArray(NodeKind),
    ExpectedMap(NodeKind),
    ExpectedEnum(NodeKind),
    ExpectedText(NodeKind),
    ParseError(String),
    Other(String),
}

impl ser::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Other(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T: fmt::Display>(msg: T) -> Self {
        Error::Other(msg.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Other(msg) => formatter.write_str(msg),
            Error::NodesExhausted => formatter.write_str("nodes exhausted"),
            Error::ParseError(err) => write!(formatter, "parse error: {err}"),
            Error::ExpectedUnit(found) => write!(formatter, "expected unit, found {found}"),
            Error::ExpectedArray(found) => write!(formatter, "expected array, found {found}"),
            Error::ExpectedMap(found) => write!(formatter, "expected map, found {found}"),
            Error::ExpectedEnum(found) => write!(formatter, "expected enum, found {found}"),
            Error::ExpectedText(found) => write!(formatter, "expected text, found {found}"),
        }
    }
}

impl std::error::Error for Error {}
