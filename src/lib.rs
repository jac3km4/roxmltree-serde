mod de;
mod error;

use std::fmt;

pub use de::{from_doc, Deserializer};
pub use error::{Error, Result};
pub use roxmltree::{Document, Error as ParseError};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    Text,
    Elem,
}

impl fmt::Display for NodeKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NodeKind::Text => f.write_str("text node"),
            NodeKind::Elem => f.write_str("xml node"),
        }
    }
}
