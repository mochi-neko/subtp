//! # subtp
//! A parser for subtitle files such as the SubRip Subtitle (.srt) and the WebVTT (.vtt).
//!
//! See [srt](`crate::srt`) and [vtt](`crate::vtt`) for more information.

// Re-exports.
pub use error::ParseError;
pub use result::ParseResult;

// Public modules.
pub mod srt;
pub mod vtt;

// Internal modules.
mod error;
mod general;
mod result;
mod str_parser;
mod vtt_parser;
