//! Result types for parsing.

use crate::ParseError;

/// The result of parsing.
pub type ParseResult<T> = Result<T, ParseError>;
