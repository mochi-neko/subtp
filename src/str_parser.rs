pub(crate) use srt_parser::multiline;
pub(crate) use srt_parser::srt;
pub(crate) use srt_parser::subtitle;
pub(crate) use srt_parser::timestamp;

peg::parser! {
    /// The parser for SubRip Subtitle format.
    grammar srt_parser() for str {
        use crate::srt::SrtTimestamp;
        use crate::srt::SubRip;
        use crate::srt::SrtSubtitle;
        use regex::Regex;

        /// Zero or more whitespaces.
        rule whitespaces() = quiet!{ [' ' | '\t']* }

        /// Newline.
        rule newline() = "\r\n" / "\n" / "\r"

        /// Zero or more whitespaces or newlines.
        rule whitespaces_or_newlines() = quiet!{ [' ' | '\t' | '\r' | '\n']* }

        /// One or more whitespaces or newlines.
        rule some_whitespaces_or_newlines() = quiet!{ [' ' | '\t' | '\r' | '\n']+ }

        /// Any-digit number.
        rule number() -> u32
            = n:$(['0'..='9']+) {?
                n.parse().or(Err("number"))
            }

        /// Two-digit number.
        rule two_number() -> u8
            = n:$(['0'..='9']['0'..='9']) {?
                n.parse().or(Err("two-digit number"))
            }

        /// Three-digit number.
        rule three_number() -> u16
            = n:$(['0'..='9']['0'..='9']['0'..='9']) {?
                n.parse().or(Err("three-digit number"))
            }

        /// Timestamp.
        pub(crate) rule timestamp() -> SrtTimestamp
            = hours:two_number() ":" minutes:two_number() ":" seconds:two_number() "," milliseconds:three_number()
            {
                SrtTimestamp {
                    hours,
                    minutes,
                    seconds,
                    milliseconds,
                }
            }

        /// Multiple lines block of text.
        pub(crate) rule multiline() -> Vec<String>
            = !newline() t:$((!(newline() newline()) [_])*)
            {
                Regex::new(r"\r\n|\n|\r")
                    .unwrap()
                    .split(t)
                    .map(|line| line.trim().to_string())
                    .filter(|line| !line.is_empty())
                    .collect()
            }

        /// Single subtitle entry.
        pub(crate) rule subtitle() -> SrtSubtitle
            = whitespaces() sequence:number() whitespaces() newline()
                whitespaces() start:timestamp() " --> " end:timestamp() whitespaces() newline()
                whitespaces() text:multiline()
            {
                SrtSubtitle { sequence, start, end, text }
            }

        /// The entire SRT.
        pub(crate) rule srt() -> SubRip
            = whitespaces_or_newlines()
                subtitles:subtitle() ** some_whitespaces_or_newlines()
                whitespaces_or_newlines()
            {
                SubRip { subtitles, }
            }
    }
}

#[cfg(test)]
mod test {
    use crate::srt::*;

    #[test]
    fn parse_timestamp() {
        assert_eq!(
            super::timestamp("00:00:00,000").unwrap(),
            SrtTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: 0,
            }
        );
        assert_eq!(
            super::timestamp("00:00:01,000").unwrap(),
            SrtTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 1,
                milliseconds: 0,
            }
        );
        assert_eq!(
            super::timestamp("00:01:00,000").unwrap(),
            SrtTimestamp {
                hours: 0,
                minutes: 1,
                seconds: 0,
                milliseconds: 0,
            }
        );
        assert_eq!(
            super::timestamp("01:00:00,000").unwrap(),
            SrtTimestamp {
                hours: 1,
                minutes: 0,
                seconds: 0,
                milliseconds: 0,
            }
        );
        assert_eq!(
            super::timestamp("00:00:00,001").unwrap(),
            SrtTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: 1,
            }
        );

        // Invalid digits.
        assert!(super::timestamp("000:00:00,000").is_err());
        assert!(super::timestamp("00:000:00,000").is_err());
        assert!(super::timestamp("00:00:000,000").is_err());
        assert!(super::timestamp("00:00:00,0000").is_err());
        assert!(super::timestamp("00:00:00,00").is_err());
        // Invalid formats.
        assert!(super::timestamp("00:00:00,").is_err());
        assert!(super::timestamp("00:00:00").is_err());
        assert!(super::timestamp("00:00,000").is_err());
        // Invalid separators. (like WebVTT)
        assert!(super::timestamp("00:00:00.000").is_err());
    }

    #[test]
    fn parse_multiline() {
        // Allow single line.
        assert_eq!(
            super::multiline("Hello, world!").unwrap(),
            vec!["Hello, world!".to_string()]
        );
        // Allow leading whitespaces.
        assert_eq!(
            super::multiline(" Hello, world!").unwrap(),
            vec!["Hello, world!".to_string()]
        );
        // Allow trailing whitespaces.
        assert_eq!(
            super::multiline("Hello, world! ").unwrap(),
            vec!["Hello, world!".to_string()]
        );
        // Ignore empty lines.
        assert_eq!(
            super::multiline("Hello, world!\n").unwrap(),
            vec!["Hello, world!".to_string()]
        );
        // Allow multiple lines.
        assert_eq!(
            super::multiline("Hello, world!\nThis is a test.").unwrap(),
            vec![
                "Hello, world!".to_string(),
                "This is a test.".to_string(),
            ]
        );
        // Allow other newline codes.
        assert_eq!(
            super::multiline("Hello, world!\rThis is a test.").unwrap(),
            vec![
                "Hello, world!".to_string(),
                "This is a test.".to_string(),
            ]
        );
        assert_eq!(
            super::multiline("Hello, world!\r\nThis is a test.").unwrap(),
            vec![
                "Hello, world!".to_string(),
                "This is a test.".to_string(),
            ]
        );

        // Prohibit newlines in header.
        assert!(super::multiline("\nHello, world!").is_err());
        // Prohibit two or more newlines.
        assert!(super::multiline("Hello, world!\nThis is a test.\n\n").is_err());
        assert!(super::multiline("some\ntext\n\nover\nline").is_err());
    }

    #[test]
    fn parse_subtitle() {
        let subtitle = SrtSubtitle {
            sequence: 1,
            start: SrtTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: 0,
            },
            end: SrtTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 1,
                milliseconds: 0,
            },
            text: vec!["Hello, world!".to_string()],
        };

        assert_eq!(
            super::subtitle("1\n00:00:00,000 --> 00:00:01,000\nHello, world!").unwrap(),
            subtitle
        );

        // Allow trailing newline.
        assert_eq!(
            super::subtitle("1\n00:00:00,000 --> 00:00:01,000\nHello, world!\n").unwrap(),
            subtitle
        );

        // Allow leading and trailing whitespaces.
        assert_eq!(
            super::subtitle("1 \n00:00:00,000 --> 00:00:01,000 \nHello, world!  ").unwrap(),
            subtitle
        );

        // Allow leading and trailing whitespaces.
        assert_eq!(super::subtitle(" 1 \n 00:00:00,000 --> 00:00:01,000 \n \tHello, world! \n").unwrap(),
            subtitle
        );

        // Prohibit spaces or new lines in header.
        assert!(super::subtitle("\n1\n00:00:00,000 --> 00:00:01,000\nHello, world!\n").is_err());
        // Must be separated by newlines.
        assert!(super::subtitle("1 00:00:00,000 --> 00:00:01,000 Hello, world!\n").is_err());
        // Prohibit two or more newlines.
        assert!(super::subtitle("1\n\n00:00:00,000 --> 00:00:01,000\nHello, world!\n").is_err());
        assert!(super::subtitle("1\n00:00:00,000 --> 00:00:01,000\n\nHello, world!\n").is_err());
        assert!(super::subtitle("1\n00:00:00,000 --> 00:00:01,000\nHello, world!\n\n").is_err());
    }

    #[test]
    fn parse_srt() {
        let srt = SubRip {
            subtitles: vec![
                SrtSubtitle {
                    sequence: 1,
                    start: SrtTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 0,
                        milliseconds: 0,
                    },
                    end: SrtTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 1,
                        milliseconds: 0,
                    },
                    text: vec!["Hello, world!".to_string()],
                },
            ],
        };

        let text = r#"1
00:00:00,000 --> 00:00:01,000
Hello, world!
"#;
        assert_eq!(
            super::srt(text).unwrap(),
            srt
        );

        let text = r#"
1
00:00:00,000 --> 00:00:01,000
Hello, world!
"#;
        assert_eq!(
            super::srt(text).unwrap(),
            srt
        );

        let text = r#"1
00:00:00,000 --> 00:00:01,000
Hello, world!

"#;
        assert_eq!(
            super::srt(text).unwrap(),
            srt
        );

        let text = r#"

1
00:00:00,000 --> 00:00:01,000
Hello, world!


"#;
        assert_eq!(
            super::srt(text).unwrap(),
            srt
        );

        let text = "1 \n00:00:00,000 --> 00:00:01,000 \nHello, world!   \n   ";
        assert_eq!(
            super::srt(text).unwrap(),
            srt
        );

        let srt = SubRip {
            subtitles: vec![
                SrtSubtitle {
                    sequence: 1,
                    start: SrtTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 0,
                        milliseconds: 0,
                    },
                    end: SrtTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 1,
                        milliseconds: 0,
                    },
                    text: vec!["Hello, world!".to_string()],
                },
                SrtSubtitle {
                    sequence: 2,
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
                    text: vec!["This is a test.".to_string()],
                },
            ],
        };

        let text = r#"1
00:00:00,000 --> 00:00:01,000
Hello, world!

2
00:00:01,000 --> 00:00:02,000
This is a test.
"#;
        assert_eq!(
            super::srt(text).unwrap(),
            srt
        );

        let text = r#"
1
00:00:00,000 --> 00:00:01,000
Hello, world!

2
00:00:01,000 --> 00:00:02,000
This is a test.
"#;
        assert_eq!(
            super::srt(text).unwrap(),
            srt
        );

        let text = r#"1
00:00:00,000 --> 00:00:01,000
Hello, world!

2
00:00:01,000 --> 00:00:02,000
This is a test.

"#;
        assert_eq!(
            super::srt(text).unwrap(),
            srt
        );

        let text = r#"
1
00:00:00,000 --> 00:00:01,000
Hello, world!

2
00:00:01,000 --> 00:00:02,000
This is a test.

"#;
        assert_eq!(
            super::srt(text).unwrap(),
            srt
        );

        let text = r#"1
00:00:00,000 --> 00:00:01,000
Hello, world!


2
00:00:01,000 --> 00:00:02,000
This is a test.


"#;
        assert_eq!(
            super::srt(text).unwrap(),
            srt
        );
    }
}
