//! # subtp
//! A parser for subtitle files such as the SubRip Subtitle (.srt) and the WebVTT (.vtt).
//!
//! - [SubRip Subtitle (.srt)](`crate::srt::SubRip`)
//! - [WebVTT (.vtt)](`crate::vtt::WebVtt`)

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
