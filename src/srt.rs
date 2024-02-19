//! A parser for the SubRip Subtitle (`.srt`) format provided by [`subtp::srt::SubRip`](crate::srt::SubRip).
//!
//! ## Example
//! ```
//! use subtp::srt::SubRip;
//! use subtp::srt::SrtSubtitle;
//! use subtp::srt::SrtTimestamp;
//!
//! let text = r#"1
//! 00:00:01,000 --> 00:00:02,000
//! Hello, world!
//!
//! 2
//! 00:00:03,000 --> 00:00:04,000
//! This is a sample.
//! Thank you for your reading.
//! "#;
//!
//! let srt = SubRip::parse(text).unwrap();
//! assert_eq!(srt, SubRip {
//!     subtitles: vec![
//!         SrtSubtitle {
//!             sequence: 1,
//!             start: SrtTimestamp {
//!                 hours: 0,
//!                 minutes: 0,
//!                 seconds: 1,
//!                 milliseconds: 0,
//!             },
//!             end: SrtTimestamp {
//!                 hours: 0,
//!                 minutes: 0,
//!                 seconds: 2,
//!                 milliseconds: 0,
//!             },
//!             text: vec!["Hello, world!".to_string()],
//!         },
//!         SrtSubtitle {
//!             sequence: 2,
//!             start: SrtTimestamp {
//!                 hours: 0,
//!                 minutes: 0,
//!                 seconds: 3,
//!                 milliseconds: 0,
//!             },
//!             end: SrtTimestamp {
//!                 hours: 0,
//!                 minutes: 0,
//!                 seconds: 4,
//!                 milliseconds: 0,
//!             },
//!             text: vec![
//!                 "This is a sample.".to_string(),
//!                 "Thank you for your reading.".to_string()
//!             ],
//!         },
//!     ],
//! });
//!
//! let rendered = srt.render();
//! assert_eq!(rendered, text);
//! ```

use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops::{Add, Sub};
use std::time::Duration;

use crate::str_parser;
use crate::ParseResult;

/// The SubRip Subtitle (`.srt`) format.
///
/// Parses from text by [`SubRip::parse`](crate::srt::SubRip::parse)
/// and renders to text by [`SubRip::render`](crate::srt::SubRip::render).
///
/// ## Example
/// ```
/// use subtp::srt::SubRip;
/// use subtp::srt::SrtSubtitle;
/// use subtp::srt::SrtTimestamp;
///
/// let subrip = SubRip {
///     subtitles: vec![
///         SrtSubtitle {
///             sequence: 1,
///             start: SrtTimestamp {
///                 hours: 0,
///                 minutes: 0,
///                 seconds: 1,
///                 milliseconds: 0,
///             },
///             end: SrtTimestamp {
///                 hours: 0,
///                 minutes: 0,
///                 seconds: 2,
///                 milliseconds: 0,
///             },
///             text: vec!["Hello, world!".to_string()],
///         }
///     ],
/// };
///
/// assert_eq!(
///     subrip.render(),
///     "1\n00:00:01,000 --> 00:00:02,000\nHello, world!\n".to_string()
/// );
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubRip {
    /// The collection of subtitles.
    pub subtitles: Vec<SrtSubtitle>,
}

impl SubRip {
    /// Parses the SubRip Subtitle format from the given text.
    ///
    /// ## Example
    /// ```
    /// use subtp::srt::SubRip;
    ///
    /// let text = r#"1
    /// 00:00:01,000 --> 00:00:02,000
    /// Hello, world!
    ///
    /// 2
    /// 00:00:03,000 --> 00:00:04,000
    /// This is a sample.
    /// Thank you for your reading.
    /// "#;
    ///
    /// let srt = SubRip::parse(text).unwrap();
    /// ```
    pub fn parse(text: &str) -> ParseResult<Self> {
        str_parser::srt(text).map_err(|err| err.into())
    }

    /// Renders the text from the SubRip Subtitle format.
    ///
    /// ## Example
    /// ```
    /// use subtp::srt::SubRip;
    /// use subtp::srt::SrtSubtitle;
    /// use subtp::srt::SrtTimestamp;
    ///
    /// let subrip = SubRip {
    ///     subtitles: vec![
    ///         SrtSubtitle {
    ///             sequence: 1,
    ///             start: SrtTimestamp {
    ///                 hours: 0,
    ///                 minutes: 0,
    ///                 seconds: 1,
    ///                 milliseconds: 0,
    ///             },
    ///             end: SrtTimestamp {
    ///                 hours: 0,
    ///                 minutes: 0,
    ///                 seconds: 2,
    ///                 milliseconds: 0,
    ///             },
    ///             text: vec!["Hello, world!".to_string()],
    ///         }
    ///     ],
    /// };
    ///
    /// let rendered = subrip.render();
    /// ```
    pub fn render(&self) -> String {
        self.to_string()
    }
}

impl Default for SubRip {
    fn default() -> Self {
        Self {
            subtitles: vec![],
        }
    }
}

impl Display for SubRip {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        let length = self.subtitles.len();
        for (i, subtitle) in self
            .subtitles
            .iter()
            .enumerate()
        {
            if i + 1 < length {
                write!(f, "{}\n", subtitle)?;
            } else {
                write!(f, "{}", subtitle)?;
            }
        }

        Ok(())
    }
}

impl Iterator for SubRip {
    type Item = SrtSubtitle;

    fn next(&mut self) -> Option<Self::Item> {
        if self.subtitles.is_empty() {
            None
        } else {
            Some(self.subtitles.remove(0))
        }
    }
}

/// The subtitle entry.
///
/// ## Example
/// ```
/// use subtp::srt::SrtSubtitle;
/// use subtp::srt::SrtTimestamp;
///
/// let subtitle = SrtSubtitle {
///     sequence: 1,
///     start: SrtTimestamp {
///         hours: 0,
///         minutes: 0,
///         seconds: 1,
///         milliseconds: 0,
///     },
///     end: SrtTimestamp {
///         hours: 0,
///         minutes: 0,
///         seconds: 2,
///         milliseconds: 0,
///     },
///     text: vec!["Hello, world!".to_string()],
/// };
///
/// assert_eq!(
///     subtitle.to_string(),
///     "1\n00:00:01,000 --> 00:00:02,000\nHello, world!\n".to_string()
/// );
/// ```
///
/// or using `Default` as follows:
///
/// ```
/// use subtp::srt::SrtSubtitle;
/// use subtp::srt::SrtTimestamp;
///
/// let subtitle = SrtSubtitle {
///     sequence: 1,
///     start: SrtTimestamp {
///         seconds: 1,
///         ..Default::default()
///     },
///     end: SrtTimestamp {
///         seconds: 2,
///         ..Default::default()
///     },
///     text: vec!["Hello, world!".to_string()],
/// };
/// ```
#[derive(Debug, Clone, Eq, Hash)]
pub struct SrtSubtitle {
    /// The sequence number.
    pub sequence: u32,
    /// The start timestamp.
    pub start: SrtTimestamp,
    /// The end timestamp.
    pub end: SrtTimestamp,
    /// The subtitle text.
    pub text: Vec<String>,
}

impl PartialEq<Self> for SrtSubtitle {
    fn eq(
        &self,
        other: &Self,
    ) -> bool {
        self.sequence == other.sequence
    }
}

impl PartialOrd<Self> for SrtSubtitle {
    fn partial_cmp(
        &self,
        other: &Self,
    ) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SrtSubtitle {
    fn cmp(
        &self,
        other: &Self,
    ) -> Ordering {
        self.sequence
            .cmp(&other.sequence)
    }
}

impl Default for SrtSubtitle {
    fn default() -> Self {
        Self {
            sequence: 0,
            start: SrtTimestamp::default(),
            end: SrtTimestamp::default(),
            text: vec![],
        }
    }
}

impl Display for SrtSubtitle {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}\n{} --> {}\n{}\n",
            self.sequence,
            self.start,
            self.end,
            self.text.join("\n"),
        )
    }
}

/// The timestamp.
///
/// ## Example
/// ```
/// use subtp::srt::SrtTimestamp;
///
/// let timestamp = SrtTimestamp {
///     hours: 0,
///     minutes: 0,
///     seconds: 1,
///     milliseconds: 0,
/// };
///
/// assert_eq!(
///     timestamp.to_string(),
///     "00:00:01,000".to_string()
/// );
/// ```
///
/// or using `Default` as follows:
///
/// ```
/// use subtp::srt::SrtTimestamp;
///
/// let timestamp = SrtTimestamp {
///     seconds: 1,
///     ..Default::default()
/// };
///
/// assert_eq!(
///     timestamp.to_string(),
///     "00:00:01,000".to_string()
/// );
/// ```
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SrtTimestamp {
    /// The hours.
    pub hours: u8,
    /// The minutes.
    pub minutes: u8,
    /// The seconds.
    pub seconds: u8,
    /// The milliseconds.
    pub milliseconds: u16,
}

impl Default for SrtTimestamp {
    fn default() -> Self {
        Self {
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
        }
    }
}

impl Display for SrtTimestamp {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02},{:03}",
            self.hours, self.minutes, self.seconds, self.milliseconds
        )
    }
}

impl Add for SrtTimestamp {
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        let mut milliseconds = self.milliseconds + rhs.milliseconds;
        let mut seconds = self.seconds + rhs.seconds;
        let mut minutes = self.minutes + rhs.minutes;
        let mut hours = self.hours + rhs.hours;

        if milliseconds >= 1000 {
            milliseconds -= 1000;
            seconds += 1;
        }

        if seconds >= 60 {
            seconds -= 60;
            minutes += 1;
        }

        if minutes >= 60 {
            minutes -= 60;
            hours += 1;
        }

        Self {
            hours,
            minutes,
            seconds,
            milliseconds,
        }
    }
}

impl Sub for SrtTimestamp {
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        let mut milliseconds =
            self.milliseconds as i32 - rhs.milliseconds as i32;
        let mut seconds = self.seconds as i32 - rhs.seconds as i32;
        let mut minutes = self.minutes as i32 - rhs.minutes as i32;
        let mut hours = self.hours as i32 - rhs.hours as i32;

        if milliseconds < 0 {
            milliseconds += 1000;
            seconds -= 1;
        }

        if seconds < 0 {
            seconds += 60;
            minutes -= 1;
        }

        if minutes < 0 {
            minutes += 60;
            hours -= 1;
        }

        Self {
            hours: hours as u8,
            minutes: minutes as u8,
            seconds: seconds as u8,
            milliseconds: milliseconds as u16,
        }
    }
}

impl From<Duration> for SrtTimestamp {
    fn from(duration: Duration) -> Self {
        let seconds = duration.as_secs();
        let milliseconds = duration.subsec_millis() as u16;

        let hours = (seconds / 3600) as u8;
        let minutes = ((seconds % 3600) / 60) as u8;
        let seconds = (seconds % 60) as u8;

        Self {
            hours,
            minutes,
            seconds,
            milliseconds,
        }
    }
}

impl Into<Duration> for SrtTimestamp {
    fn into(self) -> Duration {
        Duration::new(
            u64::from(self.hours) * 3600
                + u64::from(self.minutes) * 60
                + u64::from(self.seconds),
            self.milliseconds as u32 * 1_000_000,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let srt_text = r#"
1
00:00:01,000 --> 00:00:02,000
Hello, world!

2
00:00:03,000 --> 00:00:04,000
This is a test.

"#;

        let expected = SubRip {
            subtitles: vec![
                SrtSubtitle {
                    sequence: 1,
                    start: SrtTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 1,
                        milliseconds: 0,
                    },
                    end: SrtTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 2,
                        milliseconds: 0,
                    },
                    text: vec!["Hello, world!".to_string()],
                },
                SrtSubtitle {
                    sequence: 2,
                    start: SrtTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 3,
                        milliseconds: 0,
                    },
                    end: SrtTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 4,
                        milliseconds: 0,
                    },
                    text: vec!["This is a test.".to_string()],
                },
            ],
        };

        assert_eq!(
            SubRip::parse(srt_text).unwrap(),
            expected
        );
    }

    #[test]
    fn render() {
        let srt = SubRip {
            subtitles: vec![SrtSubtitle {
                sequence: 1,
                start: SrtTimestamp {
                    hours: 0,
                    minutes: 0,
                    seconds: 1,
                    milliseconds: 0,
                },
                end: SrtTimestamp {
                    hours: 0,
                    minutes: 0,
                    seconds: 2,
                    milliseconds: 0,
                },
                text: vec!["Hello, world!".to_string()],
            }],
        };
        let expected = r#"1
00:00:01,000 --> 00:00:02,000
Hello, world!
"#;
        assert_eq!(srt.render(), expected);

        let srt = SubRip {
            subtitles: vec![
                SrtSubtitle {
                    sequence: 1,
                    start: SrtTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 1,
                        milliseconds: 0,
                    },
                    end: SrtTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 2,
                        milliseconds: 0,
                    },
                    text: vec!["Hello, world!".to_string()],
                },
                SrtSubtitle {
                    sequence: 2,
                    start: SrtTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 3,
                        milliseconds: 0,
                    },
                    end: SrtTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 4,
                        milliseconds: 0,
                    },
                    text: vec!["This is a test.".to_string()],
                },
            ],
        };
        let expected = r#"1
00:00:01,000 --> 00:00:02,000
Hello, world!

2
00:00:03,000 --> 00:00:04,000
This is a test.
"#;
        assert_eq!(srt.render(), expected);
    }

    #[test]
    fn iterator() {
        let srt = SubRip {
            subtitles: vec![
                SrtSubtitle {
                    sequence: 1,
                    start: SrtTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 1,
                        milliseconds: 0,
                    },
                    end: SrtTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 2,
                        milliseconds: 0,
                    },
                    text: vec!["Hello, world!".to_string()],
                },
                SrtSubtitle {
                    sequence: 2,
                    start: SrtTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 3,
                        milliseconds: 0,
                    },
                    end: SrtTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 4,
                        milliseconds: 0,
                    },
                    text: vec!["This is a test.".to_string()],
                },
            ],
        };

        let mut iter = srt.into_iter();

        assert_eq!(
            iter.next(),
            Some(SrtSubtitle {
                sequence: 1,
                start: SrtTimestamp {
                    hours: 0,
                    minutes: 0,
                    seconds: 1,
                    milliseconds: 0,
                },
                end: SrtTimestamp {
                    hours: 0,
                    minutes: 0,
                    seconds: 2,
                    milliseconds: 0,
                },
                text: vec!["Hello, world!".to_string()],
            })
        );

        assert_eq!(
            iter.next(),
            Some(SrtSubtitle {
                sequence: 2,
                start: SrtTimestamp {
                    hours: 0,
                    minutes: 0,
                    seconds: 3,
                    milliseconds: 0,
                },
                end: SrtTimestamp {
                    hours: 0,
                    minutes: 0,
                    seconds: 4,
                    milliseconds: 0,
                },
                text: vec!["This is a test.".to_string()],
            })
        );

        assert_eq!(iter.next(), None);
    }

    #[test]
    fn display_subtitle() {
        let subtitle = SrtSubtitle {
            sequence: 1,
            start: SrtTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 1,
                milliseconds: 0,
            },
            end: SrtTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 2,
                milliseconds: 0,
            },
            text: vec!["Hello, world!".to_string()],
        };
        let displayed = format!("{}", subtitle);
        let expected = "1\n00:00:01,000 --> 00:00:02,000\nHello, world!\n";
        assert_eq!(displayed, expected);

        let subtitle = SrtSubtitle {
            sequence: 1,
            start: SrtTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 1,
                milliseconds: 0,
            },
            end: SrtTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 2,
                milliseconds: 0,
            },
            text: vec![
                "Hello, world!".to_string(),
                "This is the test.".to_string(),
            ],
        };
        let displayed = format!("{}", subtitle);
        let expected = "1\n00:00:01,000 --> 00:00:02,000\nHello, world!\nThis is the test.\n";
        assert_eq!(displayed, expected);
    }

    #[test]
    fn ordering_subtitle() {
        let subtitle1 = SrtSubtitle {
            sequence: 1,
            start: SrtTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 1,
                milliseconds: 0,
            },
            end: SrtTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 2,
                milliseconds: 0,
            },
            text: vec!["First".to_string()],
        };
        let subtitle2 = SrtSubtitle {
            sequence: 2,
            start: SrtTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 3,
                milliseconds: 0,
            },
            end: SrtTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 4,
                milliseconds: 0,
            },
            text: vec!["Second".to_string()],
        };
        assert!(subtitle1 < subtitle2);
    }

    #[test]
    fn display_timestamp() {
        let timestamp = SrtTimestamp {
            hours: 0,
            minutes: 0,
            seconds: 1,
            milliseconds: 0,
        };
        let displayed = format!("{}", timestamp);
        let expected = "00:00:01,000";
        assert_eq!(displayed, expected);
    }

    #[test]
    fn from_duration_to_timestamp() {
        let duration = Duration::new(1, 0);
        let timestamp: SrtTimestamp = duration.into();
        assert_eq!(
            timestamp,
            SrtTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 1,
                milliseconds: 0,
            }
        );

        let duration = Duration::new(3661, 0);
        let timestamp: SrtTimestamp = duration.into();
        assert_eq!(
            timestamp,
            SrtTimestamp {
                hours: 1,
                minutes: 1,
                seconds: 1,
                milliseconds: 0,
            }
        );

        let duration = Duration::new(3661, 500 * 1_000_000);
        let timestamp: SrtTimestamp = duration.into();
        assert_eq!(
            timestamp,
            SrtTimestamp {
                hours: 1,
                minutes: 1,
                seconds: 1,
                milliseconds: 500,
            }
        );
    }

    #[test]
    fn from_timestamp_to_duration() {
        let timestamp = SrtTimestamp {
            hours: 0,
            minutes: 0,
            seconds: 1,
            milliseconds: 0,
        };
        let duration: Duration = timestamp.into();
        assert_eq!(duration, Duration::new(1, 0));

        let timestamp = SrtTimestamp {
            hours: 1,
            minutes: 1,
            seconds: 1,
            milliseconds: 0,
        };
        let duration: Duration = timestamp.into();
        assert_eq!(duration, Duration::new(3661, 0));

        let timestamp = SrtTimestamp {
            hours: 1,
            minutes: 1,
            seconds: 1,
            milliseconds: 500,
        };
        let duration: Duration = timestamp.into();
        assert_eq!(
            duration,
            Duration::new(3661, 500 * 1_000_000)
        );
    }
}
