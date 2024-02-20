//! A parser implementation for the SubRip Subtitle format.

pub(crate) use srt_parser::srt;

peg::parser! {
    /// The parser for SubRip Subtitle format.
    grammar srt_parser() for str {
        use crate::srt::SrtTimestamp;
        use crate::srt::SubRip;
        use crate::srt::SrtSubtitle;

        /// Whitespace.
        rule whitespace() = [' ' | '\t']

        /// Newline.
        rule newline() = "\r\n" / "\n" / "\r"

        /// Whitespaces and/or newline without two or more newlines.
        rule separator() = !(newline() newline()) (whitespace() / newline())+

        /// Any-digit number.
        rule number() -> u32
            = n:$(['0'..='9']+) {?
                n.parse().or(Err("number in u32"))
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

        /// Multiple lines block of text.
        rule multiline() -> Vec<String>
            = !(whitespace() / newline()) lines:$(!(whitespace()+ newline()) (!newline() [_])+ newline()) ++ ()
            {
                lines
                    .iter()
                    .map(|l| l.to_string().trim().to_string())
                    .collect()
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

        /// Single subtitle entry.
        pub(crate) rule subtitle() -> SrtSubtitle
            = sequence:number() separator()
                start:timestamp() separator()* "-->" separator()* end:timestamp() separator()
                text:multiline()
            {
                SrtSubtitle { sequence, start, end, text }
            }

        /// The entire SRT.
        pub(crate) rule srt() -> SubRip
            = (whitespace() / newline())*
                subtitles:subtitle() ** (newline()+)
                (whitespace() / newline())*
            {
                SubRip { subtitles, }
            }
    }
}

#[cfg(test)]
mod test {
    use super::srt_parser;
    use crate::srt::*;

    #[test]
    fn parse_timestamp() {
        assert_eq!(
            srt_parser::timestamp("00:00:00,000").unwrap(),
            SrtTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: 0,
            }
        );
        assert_eq!(
            srt_parser::timestamp("00:00:01,000").unwrap(),
            SrtTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 1,
                milliseconds: 0,
            }
        );
        assert_eq!(
            srt_parser::timestamp("00:01:00,000").unwrap(),
            SrtTimestamp {
                hours: 0,
                minutes: 1,
                seconds: 0,
                milliseconds: 0,
            }
        );
        assert_eq!(
            srt_parser::timestamp("01:00:00,000").unwrap(),
            SrtTimestamp {
                hours: 1,
                minutes: 0,
                seconds: 0,
                milliseconds: 0,
            }
        );
        assert_eq!(
            srt_parser::timestamp("00:00:00,001").unwrap(),
            SrtTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: 1,
            }
        );

        // Invalid digits.
        assert!(srt_parser::timestamp("000:00:00,000").is_err());
        assert!(srt_parser::timestamp("00:000:00,000").is_err());
        assert!(srt_parser::timestamp("00:00:000,000").is_err());
        assert!(srt_parser::timestamp("00:00:00,0000").is_err());
        assert!(srt_parser::timestamp("00:00:00,00").is_err());
        // Invalid formats.
        assert!(srt_parser::timestamp("00:00:00,").is_err());
        assert!(srt_parser::timestamp("00:00:00").is_err());
        assert!(srt_parser::timestamp("00:00,000").is_err());
        // Invalid separators. (like WebVTT)
        assert!(srt_parser::timestamp("00:00:00.000").is_err());
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
            srt_parser::subtitle(
                "1\n00:00:00,000 --> 00:00:01,000\nHello, world!\n"
            )
            .unwrap(),
            subtitle
        );

        // Allow leading and trailing whitespaces.
        assert_eq!(
            srt_parser::subtitle(
                "1 \n00:00:00,000 --> 00:00:01,000 \nHello, world!  \n"
            )
            .unwrap(),
            subtitle
        );

        // Allow whitespaces.
        assert_eq!(
            srt_parser::subtitle(
                "1 \n 00:00:00,000  -->  00:00:01,000 \n \tHello, world! \n"
            )
            .unwrap(),
            subtitle
        );

        // Allow no whitespaces between sequence and timestamp.
        assert_eq!(
            srt_parser::subtitle(
                "1\n00:00:00,000-->00:00:01,000\nHello, world!\n"
            )
            .unwrap(),
            subtitle
        );

        // Allow separator with whitespaces.
        assert_eq!(
            srt_parser::subtitle(
                "1 00:00:00,000 --> 00:00:01,000 Hello, world!\n"
            )
            .unwrap(),
            subtitle
        );

        // Allow newline between timestamps.
        assert_eq!(
            srt_parser::subtitle(
                "1\n00:00:00,000\n-->\n00:00:01,000\nHello, world!\n"
            )
            .unwrap(),
            subtitle
        );

        // Prohibit spaces or new lines in header.
        assert!(srt_parser::subtitle(
            "\n1\n00:00:00,000 --> 00:00:01,000\nHello, world!\n"
        )
        .is_err());
        // Prohibit two or more newlines.
        assert!(srt_parser::subtitle(
            "1\n\n00:00:00,000 --> 00:00:01,000\nHello, world!\n"
        )
        .is_err());
        assert!(srt_parser::subtitle(
            "1\n00:00:00,000 --> 00:00:01,000\n\nHello, world!\n"
        )
        .is_err());
        assert!(srt_parser::subtitle(
            "1\n00:00:00,000 --> 00:00:01,000\nHello, world!\n\n"
        )
        .is_err());
        // Prohibit empty text.
        assert!(
            srt_parser::subtitle("1\n00:00:00,000 --> 00:00:01,000\n\n")
                .is_err()
        );
    }

    #[test]
    fn parse_srt() {
        let srt = SubRip {
            subtitles: vec![SrtSubtitle {
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
            }],
        };

        let text = r#"1
00:00:00,000 --> 00:00:01,000
Hello, world!
"#;
        assert_eq!(srt_parser::srt(text).unwrap(), srt);

        let text = r#"1
00:00:00,000 --> 00:00:01,000
Hello, world!"#;
        assert!(srt_parser::srt(text).is_err());

        let text = r#"
1
00:00:00,000 --> 00:00:01,000
Hello, world!
"#;
        assert_eq!(srt_parser::srt(text).unwrap(), srt);

        let text = r#"1
00:00:00,000 --> 00:00:01,000
Hello, world!

"#;
        assert_eq!(srt_parser::srt(text).unwrap(), srt);

        let text = r#"

1
00:00:00,000 --> 00:00:01,000
Hello, world!


"#;
        assert_eq!(srt_parser::srt(text).unwrap(), srt);

        let text = "1 \n00:00:00,000 --> 00:00:01,000 \nHello, world!   \n   ";
        assert_eq!(srt_parser::srt(text).unwrap(), srt);

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
        assert_eq!(srt_parser::srt(text).unwrap(), srt);

        let text = r#"
1
00:00:00,000 --> 00:00:01,000
Hello, world!

2
00:00:01,000 --> 00:00:02,000
This is a test.
"#;
        assert_eq!(srt_parser::srt(text).unwrap(), srt);

        let text = r#"1
00:00:00,000 --> 00:00:01,000
Hello, world!

2
00:00:01,000 --> 00:00:02,000
This is a test.

"#;
        assert_eq!(srt_parser::srt(text).unwrap(), srt);

        let text = r#"
1
00:00:00,000 --> 00:00:01,000
Hello, world!

2
00:00:01,000 --> 00:00:02,000
This is a test.

"#;
        assert_eq!(srt_parser::srt(text).unwrap(), srt);

        let text = r#"1
00:00:00,000 --> 00:00:01,000
Hello, world!


2
00:00:01,000 --> 00:00:02,000
This is a test.


"#;
        assert_eq!(srt_parser::srt(text).unwrap(), srt);
    }
}
