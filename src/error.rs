/// The error of parsing.
#[derive(Debug, thiserror::Error)]
#[error("Parse error at {location}: expected {expected}")]
pub struct ParseError {
    /// The furthest position the parser reached in the input before failing.
    pub location: String,
    /// The set of literals that failed to match at that position.
    pub expected: String,
}

impl From<peg::error::ParseError<peg::str::LineCol>> for ParseError {
    fn from(err: peg::error::ParseError<peg::str::LineCol>) -> Self {
        ParseError {
            location: format!("{}", err.location),
            expected: format!("{}", err.expected),
        }
    }
}