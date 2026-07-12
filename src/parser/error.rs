use crate::error::Error;

pub fn parse_error(msg: impl Into<String>) -> Error {
    Error::ParseError(msg.into())
}
