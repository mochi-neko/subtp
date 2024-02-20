//! A parser implementation for the WebVTT format.

pub(crate) use vtt_parser::vtt;

peg::parser! {
    /// The parser for the WebVTT format.
    grammar vtt_parser() for str {
        use crate::vtt::WebVtt;
        use crate::vtt::VttHeader;
        use crate::vtt::VttRegion;
        use crate::vtt::VttBlock;
        use crate::vtt::VttCue;
        use crate::vtt::VttComment;
        use crate::vtt::VttStyle;
        use crate::vtt::VttTimings;
        use crate::vtt::VttTimestamp;
        use crate::vtt::CueSettings;
        use crate::vtt::Vertical;
        use crate::vtt::Percentage;
        use crate::vtt::Line;
        use crate::vtt::Alignment;
        use crate::vtt::Anchor;
        use crate::vtt::Scroll;
        use crate::vtt::LineAlignment;
        use crate::vtt::PositionAlignment;
        use crate::vtt::Position;
        use crate::vtt::VttDescription;

        /// Whitespace.
        rule whitespace() = [' ' | '\t']

        /// Newline.
        rule newline() = "\r\n" / "\n" / "\r"

        /// Any-digit number.
        rule number() -> u32
            = n:$(['0'..='9']+) {?
                n.parse().or(Err("number in u32"))
            }

        /// Signed integer.
        rule int() -> i32
            = n:$(['+' | '-']? ['0'..='9']+) {?
                n.parse().or(Err("signed number"))
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

        /// Floating number.
        rule float() -> f32
            = n:$(['0'..='9']+ "." ['0'..='9']+) {?
                n.parse().or(Err("float"))
            }

        /// Percentage of integer number.
        rule percentage_int() -> u32
            = n:number() "%" {?
                if n <= 100 {
                    Ok(n)
                } else {
                    Err("Number out of range")
                }
            }

        /// Percentage of floating number.
        rule percentage_float() -> f32
            = f:float() "%" {?
                if f >= 0.0 && f <= 100.0 {
                    Ok(f)
                } else {
                    Err("Number out of range")
                }
            }

        /// Percentage.
        rule percentage() -> Percentage
            = p:percentage_int() { Percentage { value: p as f32 } }
                / p:percentage_float() { Percentage { value: p } }

        /// Anchor.
        rule anchor() -> Anchor
            = x:percentage() "," y:percentage()
            {
                Anchor {
                    x,
                    y,
                }
            }

        /// Sequential text.
        rule sequence() -> String
            = t:$((!(whitespace() / newline()) [_])+)
            {
                t.to_string()
            }

        /// Block of text.
        rule text_block() -> String
            = !newline() lines:$((!newline() [_])+ newline()) ++ ()
            {
                lines.join("").to_string()
            }

        /// Single text with newline.
        rule line() -> String
            = !(whitespace() / newline()) t:$((!newline() [_])+) newline()
            {
                t.to_string().trim().to_string()
            }

        /// Multiple lines.
        rule multiline() -> Vec<String>
            = !(whitespace() / newline()) lines:$((!newline() [_])+ newline()) ++ ()
            {
                lines
                    .iter()
                    .map(|l| l.to_string().trim().to_string())
                    .collect()
            }

        /// Timestamp.
        pub(crate) rule timestamp() -> VttTimestamp
            = timestamp_with_hours()
                / timestamp_without_hours()

        /// Timestamp with hours.
        rule timestamp_with_hours() -> VttTimestamp
            = hours:two_number() ":" minutes:two_number() ":" seconds:two_number() "." milliseconds:three_number()
            {
                VttTimestamp {
                    hours,
                    minutes,
                    seconds,
                    milliseconds,
                }
            }

        /// timestamp without hours.
        rule timestamp_without_hours() -> VttTimestamp
            = minutes:two_number() ":" seconds:two_number() "." milliseconds:three_number()
            {
                VttTimestamp {
                    hours: 0,
                    minutes,
                    seconds,
                    milliseconds,
                }
            }

        /// Timings
        pub(crate) rule timings() -> VttTimings
            = start:timestamp() whitespace()* "-->" whitespace()* end:timestamp()
            {
                VttTimings { start, end }
            }

        /// Cue settings
        pub(crate) rule cue_settings() -> CueSettings
            = options:sequence() ** (whitespace()+) {?
                let mut settings = CueSettings::default();
                for option in options {
                    if let Ok(region) = cue_region(option.as_str()) {
                        settings.region = Some(region);
                    } else if let Ok(vertical) = cue_vertical(option.as_str()) {
                        settings.vertical = Some(vertical);
                    } else if let Ok(line) = cue_line(option.as_str()) {
                        settings.line = Some(line);
                    } else if let Ok(position) = cue_position(option.as_str()) {
                        settings.position = Some(position);
                    } else if let Ok(size) = cue_size(option.as_str()) {
                        settings.size = Some(size);
                    } else if let Ok(align) = cue_align(option.as_str()) {
                        settings.align = Some(align);
                    } else {
                        return Err("Invalid cue setting");
                    }
                }
                Ok(settings)
            }

        /// Cue vertical setting.
        pub(crate) rule cue_vertical() -> Vertical
            = "vertical:rl" { Vertical::Rl }
                / "vertical:lr" { Vertical::Lr }

        /// Cue line setting.
        pub(crate) rule cue_line() -> Line
            = cue_line_percentage_with_aligment()
                / cue_line_number_with_alignment()
                / cue_line_percentage()
                / cue_line_number()

        rule cue_line_number() -> Line
            = "line:" i:int() {
                Line::LineNumber(i, None)
            }

        rule cue_line_number_with_alignment() -> Line
            = "line:" i:int() "," align:cue_line_alignment() {
                Line::LineNumber(i, Some(align))
            }

        rule cue_line_percentage() -> Line
            = "line:" p:percentage() {
                Line::Percentage(p, None)
            }

        rule cue_line_percentage_with_aligment() -> Line
            = "line:" p:percentage() "," align:cue_line_alignment() {
                Line::Percentage(p, Some(align))
            }

        rule cue_line_alignment() -> LineAlignment
            = align:sequence() {?
                match align.as_str() {
                    "start" => Ok(LineAlignment::Start),
                    "center" => Ok(LineAlignment::Center),
                    "end" => Ok(LineAlignment::End),
                    _ => Err("Invalid align"),
                }
            }

        /// Cue position setting.
        pub(crate) rule cue_position() -> Position
            = cue_position_with_alignment()
                / cue_position_without_alignment()

        rule cue_position_without_alignment() -> Position
            = "position:" p:percentage()
            {
                Position {
                    value: p,
                    alignment: None,
                }
            }

        rule cue_position_with_alignment() -> Position
            = "position:" p:percentage() "," align:cue_position_alignment()
            {?
                Ok(Position {
                    value: p,
                    alignment: Some(align),
                })
            }

        rule cue_position_alignment() -> PositionAlignment
            = align:sequence() {?
                match align.as_str() {
                    "line-left" => Ok(PositionAlignment::LineLeft),
                    "center" => Ok(PositionAlignment::Center),
                    "line-right" => Ok(PositionAlignment::LineRight),
                    _ => Err("Invalid align"),
                }
            }

        /// Cue size setting.
        pub(crate) rule cue_size() -> Percentage
            = "size:" p:percentage() { p }

        /// Cue align setting.
        pub(crate) rule cue_align() -> Alignment
            = "align:" t:sequence() {?
                match t.as_str() {
                    "start" => Ok(Alignment::Start),
                    "center" => Ok(Alignment::Center),
                    "end" => Ok(Alignment::End),
                    "left" => Ok(Alignment::Left),
                    "right" => Ok(Alignment::Right),
                    _ => Err("Invalid align"),
                }
            }

        /// Cue region setting.
        pub(crate) rule cue_region() -> String
            = "region:" t:sequence() { t }

        /// Cue block
        pub(crate) rule cue() -> VttCue
            = cue_with_identifier_and_settings()
                / cue_with_identifier()
                / cue_with_settings()
                / cue_minimal()

        /// Minimal cue block
        rule cue_minimal() -> VttCue
            = whitespace()* timings:timings() whitespace()* newline()
                whitespace()* payload:multiline()
            {
                VttCue {
                    identifier: None,
                    timings,
                    settings: None,
                    payload,
                }
            }

        /// Cue block with an identifier.
        rule cue_with_identifier() -> VttCue
            = whitespace()* identifier:line()
                whitespace()* timings:timings() whitespace()* newline()
                whitespace()* payload:multiline()
            {
                VttCue {
                    identifier: Some(identifier),
                    timings,
                    settings: None,
                    payload,
                }
            }

        /// Cue block with settings.
        rule cue_with_settings() -> VttCue
            = whitespace()* timings:timings() whitespace()+ settings:cue_settings() whitespace()* newline()
                whitespace()* payload:multiline()
            {
                VttCue {
                    identifier: None,
                    timings,
                    settings: Some(settings),
                    payload,
                }
            }

        /// Cue block with an identifier and settings.
        rule cue_with_identifier_and_settings() -> VttCue
            = whitespace()* identifier:line()
                whitespace()* timings:timings() whitespace()+ settings:cue_settings() whitespace()* newline()
                whitespace()* payload:multiline()
            {
                VttCue {
                    identifier: Some(identifier),
                    timings,
                    settings: Some(settings),
                    payload,
                }
            }

        /// Comment block.
        pub(crate) rule comment() -> VttComment
            = comment_below() / comment_across() / comment_side()

        /// Single line comment.
        rule comment_side() -> VttComment
            = "NOTE" whitespace()+ comment:line()
            {
                VttComment::Side(comment)
            }

        /// Multiple lines comment block.
        rule comment_below() -> VttComment
            = "NOTE" whitespace()* newline()
                comments:multiline()
            {
                VttComment::Below(comments.join("\n"))
            }

        /// Multiple lines comment block across a line.
        rule comment_across() -> VttComment
            = "NOTE" whitespace()+ comments:multiline()
            {
                VttComment::Side(comments.join("\n"))
            }

        /// Style block.
        pub(crate) rule style() -> VttStyle
            = "STYLE" whitespace()* newline()
                style:text_block()
            {
                VttStyle { style }
            }

        /// Parses a region block.
        pub(crate) rule region() -> VttRegion
            = "REGION" whitespace()* newline()
                options:sequence() ** newline() newline()
            {?
                let mut region = VttRegion::default();
                for option in options {
                    if let Ok(id) = region_id(option.as_str()) {
                        region.id = Some(id);
                    } else if let Ok(width) = region_width(option.as_str()) {
                        region.width = Some(width);
                    } else if let Ok(lines) = region_lines(option.as_str()) {
                        region.lines = Some(lines);
                    } else if let Ok(region_anchor) = region_region_anchor(option.as_str()) {
                        region.region_anchor = Some(region_anchor);
                    } else if let Ok(viewport_anchor) = region_viewport_anchor(option.as_str()) {
                        region.viewport_anchor = Some(viewport_anchor);
                    } else if let Ok(scroll) = region_scroll(option.as_str()) {
                        region.scroll = Some(scroll);
                    } else {
                        return Err("Invalid region setting");
                    }
                }
                Ok(region)
            }

        pub(crate) rule region_id() -> String
            = "id:" id:sequence() { id }

        pub(crate) rule region_width() -> Percentage
            = "width:" width:percentage() { width }

        pub(crate) rule region_lines() -> u32
            = "lines:" lines:number() { lines }

        pub(crate) rule region_region_anchor() -> Anchor
            = "regionanchor:" region_anchor:anchor() { region_anchor }

        pub(crate) rule region_viewport_anchor() -> Anchor
            = "viewportanchor:" viewport_anchor:anchor() { viewport_anchor }

        pub(crate) rule region_scroll() -> Scroll
            = "scroll:up" { Scroll::Up }

         rule cue_block() -> VttBlock
            = cue:cue() { cue.into() }

        rule comment_block() -> VttBlock
            = comment:comment() { comment.into() }

        rule style_block() -> VttBlock
            = style:style() { style.into() }

        rule region_block() -> VttBlock
            = region:region() { region.into() }

        /// Any block (cue, comment, or style)
        pub(crate) rule block() -> VttBlock
            = cue_block()
                / comment_block()
                / style_block()
                / region_block()

        /// Header
        pub(crate) rule header() -> VttHeader
            = header_with_below_description()
                / header_with_side_descruption()
                / header_minimal()

        rule header_minimal() -> VttHeader
            = "WEBVTT" whitespace()* newline()
            {
                VttHeader { description: None }
            }

        rule header_with_side_descruption() -> VttHeader
            = "WEBVTT" whitespace()* description:text_block()
            {
                VttHeader { description: Some(VttDescription::Side(description)) }
            }

        rule header_with_below_description() -> VttHeader
            = "WEBVTT" whitespace()* newline()
                description:text_block()
            {
                VttHeader { description: Some(VttDescription::Below(description)) }
            }

        /// The entire WebVTT file.
        pub(crate) rule vtt() -> WebVtt
            = header:header() newline()
                (whitespace() / newline())*
                blocks:block() ** (newline()+)
                (whitespace() / newline())*
            {
                WebVtt {
                    header,
                    blocks,
                }
            }
    }
}

#[cfg(test)]
mod test {
    use super::vtt_parser;
    use crate::vtt::*;

    #[test]
    fn timestamp() {
        assert_eq!(
            vtt_parser::timestamp("00:00:00.000").unwrap(),
            VttTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: 0,
            }
        );
        assert_eq!(
            vtt_parser::timestamp("00:00:01.000").unwrap(),
            VttTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 1,
                milliseconds: 0,
            }
        );
        assert_eq!(
            vtt_parser::timestamp("00:01:00.000").unwrap(),
            VttTimestamp {
                hours: 0,
                minutes: 1,
                seconds: 0,
                milliseconds: 0,
            }
        );
        assert_eq!(
            vtt_parser::timestamp("01:00:00.000").unwrap(),
            VttTimestamp {
                hours: 1,
                minutes: 0,
                seconds: 0,
                milliseconds: 0,
            }
        );

        // Allow without hours.
        assert_eq!(
            vtt_parser::timestamp("00:01.000").unwrap(),
            VttTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 1,
                milliseconds: 0,
            }
        );

        // Invalid digits.
        assert!(vtt_parser::timestamp("000:00:00.000").is_err());
        assert!(vtt_parser::timestamp("00:000:00.000").is_err());
        assert!(vtt_parser::timestamp("00:00:000.000").is_err());
        assert!(vtt_parser::timestamp("00:00:00.0000").is_err());
        assert!(vtt_parser::timestamp("00:00:00.00").is_err());
        // Invalid formats.
        assert!(vtt_parser::timestamp("00:00:00.").is_err());
        assert!(vtt_parser::timestamp("00:00:00").is_err());
        // Invalid separators. (like SubRip Subtitle)
        assert!(vtt_parser::timestamp("00:00:00,000").is_err());
    }

    #[test]
    fn timings() {
        let expected = VttTimings {
            start: VttTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 0,
                milliseconds: 0,
            },
            end: VttTimestamp {
                hours: 0,
                minutes: 0,
                seconds: 1,
                milliseconds: 0,
            },
        };

        assert_eq!(
            vtt_parser::timings("00:00:00.000 --> 00:00:01.000").unwrap(),
            expected
        );

        assert_eq!(
            vtt_parser::timings("00:00.000 --> 00:01.000").unwrap(),
            expected
        );

        assert_eq!(
            vtt_parser::timings("00:00:00.000 --> 00:01.000").unwrap(),
            expected
        );

        assert_eq!(
            vtt_parser::timings("00:00.000 --> 00:00:01.000").unwrap(),
            expected
        );

        assert_eq!(
            vtt_parser::timings("00:00.000-->00:00:01.000").unwrap(),
            expected
        );

        assert_eq!(
            vtt_parser::timings("00:00.000 -->00:00:01.000").unwrap(),
            expected
        );

        assert_eq!(
            vtt_parser::timings("00:00.000--> 00:00:01.000").unwrap(),
            expected
        );

        assert_eq!(
            vtt_parser::timings("00:00.000  -->  00:00:01.000").unwrap(),
            expected
        );

        assert!(vtt_parser::timings("00:00.000 -->\n00:00:01.000").is_err());
    }

    #[test]
    fn cue_region() {
        assert_eq!(
            vtt_parser::cue_region("region:id").unwrap(),
            "id".to_string()
        );
        assert!(vtt_parser::cue_region("region: id").is_err());
    }

    #[test]
    fn cue_vertical() {
        assert_eq!(
            vtt_parser::cue_vertical("vertical:rl").unwrap(),
            Vertical::Rl
        );
        assert_eq!(
            vtt_parser::cue_vertical("vertical:lr").unwrap(),
            Vertical::Lr
        );
        assert!(vtt_parser::cue_vertical("vertical:rr").is_err());
        assert!(vtt_parser::cue_vertical("vertical: rl").is_err());
    }

    #[test]
    fn cue_line() {
        assert_eq!(
            vtt_parser::cue_line("line:1").unwrap(),
            Line::LineNumber(1, None)
        );
        assert_eq!(
            vtt_parser::cue_line("line:1,center").unwrap(),
            Line::LineNumber(1, Some(LineAlignment::Center))
        );
        assert_eq!(
            vtt_parser::cue_line("line:-1").unwrap(),
            Line::LineNumber(-1, None)
        );
        assert_eq!(
            vtt_parser::cue_line("line:-1,start").unwrap(),
            Line::LineNumber(-1, Some(LineAlignment::Start))
        );
        assert_eq!(
            vtt_parser::cue_line("line:10%").unwrap(),
            Line::Percentage(
                Percentage {
                    value: 10.0,
                },
                None,
            )
        );
        assert_eq!(
            vtt_parser::cue_line("line:10%,end").unwrap(),
            Line::Percentage(
                Percentage {
                    value: 10.0,
                },
                Some(LineAlignment::End),
            )
        );

        assert!(vtt_parser::cue_line("line:10.0").is_err());
        assert!(vtt_parser::cue_line("line:101%").is_err());
        assert!(vtt_parser::cue_line("line: 1").is_err());
    }

    #[test]
    fn cue_position() {
        assert_eq!(
            vtt_parser::cue_position("position:10%").unwrap(),
            Position {
                value: Percentage {
                    value: 10.0
                },
                alignment: None,
            }
        );
        assert_eq!(
            vtt_parser::cue_position("position:1%,line-left").unwrap(),
            Position {
                value: Percentage {
                    value: 1.0
                },
                alignment: Some(PositionAlignment::LineLeft),
            }
        );
        assert_eq!(
            vtt_parser::cue_position("position:100%,line-right").unwrap(),
            Position {
                value: Percentage {
                    value: 100.0
                },
                alignment: Some(PositionAlignment::LineRight),
            }
        );
        assert!(vtt_parser::cue_position("position:10.0").is_err());
        assert!(vtt_parser::cue_position("position:101%").is_err());
        assert!(vtt_parser::cue_position("position: 10%").is_err());
    }

    #[test]
    fn cue_size() {
        assert_eq!(
            vtt_parser::cue_size("size:10%").unwrap(),
            Percentage {
                value: 10.0
            }
        );
        assert!(vtt_parser::cue_size("size:10.0").is_err());
        assert!(vtt_parser::cue_size("size:101%").is_err());
        assert!(vtt_parser::cue_size("size: 10%").is_err());
    }

    #[test]
    fn cue_align() {
        assert_eq!(
            vtt_parser::cue_align("align:start").unwrap(),
            Alignment::Start
        );
        assert_eq!(
            vtt_parser::cue_align("align:center").unwrap(),
            Alignment::Center
        );
        assert_eq!(
            vtt_parser::cue_align("align:end").unwrap(),
            Alignment::End
        );
        assert_eq!(
            vtt_parser::cue_align("align:left").unwrap(),
            Alignment::Left
        );
        assert_eq!(
            vtt_parser::cue_align("align:right").unwrap(),
            Alignment::Right
        );
        assert!(vtt_parser::cue_align("align:middle").is_err());
        assert!(vtt_parser::cue_align("align: start").is_err());
    }

    #[test]
    fn cue_settings() {
        assert_eq!(
            vtt_parser::cue_settings("region:id").unwrap(),
            CueSettings {
                region: Some("id".to_string()),
                vertical: None,
                line: None,
                position: None,
                size: None,
                align: None,
            }
        );

        assert_eq!(
            vtt_parser::cue_settings("vertical:rl").unwrap(),
            CueSettings {
                region: None,
                vertical: Some(Vertical::Rl),
                line: None,
                position: None,
                size: None,
                align: None,
            }
        );

        assert_eq!(
            vtt_parser::cue_settings("line:1").unwrap(),
            CueSettings {
                region: None,
                vertical: None,
                line: Some(Line::LineNumber(1, None)),
                position: None,
                size: None,
                align: None,
            }
        );

        assert_eq!(
            vtt_parser::cue_settings("position:10%").unwrap(),
            CueSettings {
                region: None,
                vertical: None,
                line: None,
                position: Some(Position {
                    value: Percentage {
                        value: 10.0,
                    },
                    alignment: None,
                }),
                size: None,
                align: None,
            }
        );

        assert_eq!(
            vtt_parser::cue_settings("size:10%").unwrap(),
            CueSettings {
                region: None,
                vertical: None,
                line: None,
                position: None,
                size: Some(Percentage {
                    value: 10.0
                }),
                align: None,
            }
        );

        assert_eq!(
            vtt_parser::cue_settings("align:start").unwrap(),
            CueSettings {
                region: None,
                vertical: None,
                line: None,
                position: None,
                size: None,
                align: Some(Alignment::Start),
            }
        );

        let settings = CueSettings {
            region: Some("id".to_string()),
            vertical: Some(Vertical::Rl),
            line: Some(Line::LineNumber(1, None)),
            position: Some(Position {
                value: Percentage {
                    value: 10.0,
                },
                alignment: None,
            }),
            size: Some(Percentage {
                value: 10.0,
            }),
            align: Some(Alignment::Start),
        };

        assert_eq!(
            vtt_parser::cue_settings("region:id vertical:rl line:1 position:10% size:10% align:start").unwrap(),
            settings
        );
        assert_eq!(
            vtt_parser::cue_settings("vertical:rl line:1 position:10% size:10% align:start region:id").unwrap(),
            settings
        );
        assert_eq!(
            vtt_parser::cue_settings("line:1 position:10% size:10% align:start region:id vertical:rl").unwrap(),
            settings
        );
    }

    #[test]
    fn cue() {
        // Minimal
        assert_eq!(
            vtt_parser::cue("00:00:00.000 --> 00:00:01.000\nHello, world!\n")
                .unwrap(),
            VttCue {
                identifier: None,
                timings: VttTimings {
                    start: VttTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 0,
                        milliseconds: 0,
                    },
                    end: VttTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 1,
                        milliseconds: 0,
                    },
                },
                settings: None,
                payload: vec!["Hello, world!".to_string()],
            }
        );

        // With identifier
        assert_eq!(
            vtt_parser::cue(
                "id\n00:00:00.000 --> 00:00:01.000\nHello, world!\n"
            )
            .unwrap(),
            VttCue {
                identifier: Some("id".to_string()),
                timings: VttTimings {
                    start: VttTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 0,
                        milliseconds: 0,
                    },
                    end: VttTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 1,
                        milliseconds: 0,
                    },
                },
                settings: None,
                payload: vec!["Hello, world!".to_string()],
            }
        );

        // With settings
        assert_eq!(
            vtt_parser::cue(
                "00:00:00.000 --> 00:00:01.000 line:1 position:50%\nHello, world!\n"
            )
                .unwrap(),
            VttCue {
                identifier: None,
                timings: VttTimings {
                    start: VttTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 0,
                        milliseconds: 0,
                    },
                    end: VttTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 1,
                        milliseconds: 0,
                    },
                },
                settings: Some(CueSettings {
                    line: Some(Line::LineNumber(1, None)),
                    position: Some(Position {
                        value: Percentage { value: 50.0 },
                        alignment: None,
                    }),
                    ..Default::default()
                }),
                payload: vec!["Hello, world!".to_string()],
            }
        );

        // With identifier and settings
        assert_eq!(
            vtt_parser::cue(
                "id\n00:00:00.000 --> 00:00:01.000 line:1 position:50%\nHello, world!\n"
            )
                .unwrap(),
            VttCue {
                identifier: Some("id".to_string()),
                timings: VttTimings {
                    start: VttTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 0,
                        milliseconds: 0,
                    },
                    end: VttTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 1,
                        milliseconds: 0,
                    },
                },
                settings: Some(CueSettings {
                    line: Some(Line::LineNumber(1, None)),
                    position: Some(Position {
                        value: Percentage { value: 50.0 },
                        alignment: None,
                    }),
                    ..Default::default()
                }),
                payload: vec!["Hello, world!".to_string()],
            }
        );

        // Allow whitespaces
        assert_eq!(
            vtt_parser::cue(
                " id \n 00:00:00.000 --> 00:00:01.000  line:1  position:50%  \n Hello, world! \n"
            )
                .unwrap(),
            VttCue {
                identifier: Some("id".to_string()),
                timings: VttTimings {
                    start: VttTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 0,
                        milliseconds: 0,
                    },
                    end: VttTimestamp {
                        hours: 0,
                        minutes: 0,
                        seconds: 1,
                        milliseconds: 0,
                    },
                },
                settings: Some(CueSettings {
                    line: Some(Line::LineNumber(1, None)),
                    position: Some(Position {
                        value: Percentage { value: 50.0 },
                        alignment: None,
                    }),
                    ..Default::default()
                }),
                payload: vec!["Hello, world!".to_string()],
            }
        );

        // Prohibit two or more newlines
        assert!(vtt_parser::cue("id\n\n00:00:00.000 --> 00:00:01.000 line:1 position:50%\nHello, world!\n").is_err());
        assert!(vtt_parser::cue("id\n00:00:00.000 --> 00:00:01.000 line:1 position:50%\n\nHello, world!\n").is_err());
        assert!(vtt_parser::cue("id\n00:00:00.000 --> 00:00:01.000 line:1 position:50%\nHello, world!\n\n").is_err());
    }

    #[test]
    fn comment() {
        assert_eq!(
            vtt_parser::comment("NOTE comment\n").unwrap(),
            VttComment::Side("comment".to_string())
        );

        assert_eq!(
            vtt_parser::comment("NOTE\ncomment\nmultiline\n").unwrap(),
            VttComment::Below("comment\nmultiline".to_string())
        );

        assert_eq!(
            vtt_parser::comment("NOTE comment\nacross line\n").unwrap(),
            VttComment::Side("comment\nacross line".to_string())
        );

        assert!(vtt_parser::comment("NOTE").is_err());
        assert!(vtt_parser::comment("NOTE \n").is_err());
        assert!(vtt_parser::comment("NOTE_Comment\n").is_err());
        assert!(vtt_parser::comment("NOTE\n\n").is_err());
    }

    #[test]
    fn header() {
        assert_eq!(
            vtt_parser::header("WEBVTT\n").unwrap(),
            VttHeader {
                description: None
            }
        );

        assert_eq!(
            vtt_parser::header("WEBVTT\ndescription\n").unwrap(),
            VttHeader {
                description: Some(VttDescription::Below(
                    "description\n".to_string()
                ))
            }
        );

        assert_eq!(
            vtt_parser::header("WEBVTT\nfirst\nsecond\n").unwrap(),
            VttHeader {
                description: Some(VttDescription::Below(
                    "first\nsecond\n".to_string()
                ))
            }
        );

        assert_eq!(
            vtt_parser::header("WEBVTT description\n").unwrap(),
            VttHeader {
                description: Some(VttDescription::Side(
                    "description\n".to_string()
                ))
            }
        );

        assert!(vtt_parser::header("WEBVTT").is_err());
        assert!(vtt_parser::header("WEBVTT\n\n").is_err());
        assert!(vtt_parser::header(" WEBVTT\n").is_err());
        assert!(vtt_parser::header("webvtt\n").is_err());
    }

    #[test]
    fn style() {
        assert_eq!(
            vtt_parser::style("STYLE\nfirst\nsecond\n").unwrap(),
            VttStyle {
                style: "first\nsecond\n".to_string()
            }
        );

        assert!(vtt_parser::style("STYLE first\n").is_err());
        assert!(vtt_parser::style("STYLE\n").is_err());
        assert!(vtt_parser::style("STYLE").is_err());
        assert!(vtt_parser::style("STYLE\n\n").is_err());
        assert!(vtt_parser::style(" STYLE\n").is_err());
        assert!(vtt_parser::style("style\n").is_err());
    }

    #[test]
    fn region_id() {
        assert_eq!(
            vtt_parser::region_id("id:region").unwrap(),
            "region".to_string()
        );
        assert!(vtt_parser::region_id("id: region").is_err());
    }

    #[test]
    fn region_width() {
        assert_eq!(
            vtt_parser::region_width("width:10%").unwrap(),
            Percentage {
                value: 10.0
            }
        );
        assert!(vtt_parser::region_width("width:10.0").is_err());
        assert!(vtt_parser::region_width("width:101%").is_err());
        assert!(vtt_parser::region_width("width: 10%").is_err());
    }

    #[test]
    fn region_lines() {
        assert_eq!(
            vtt_parser::region_lines("lines:10").unwrap(),
            10
        );
        assert!(vtt_parser::region_lines("lines:10.0").is_err());
        assert!(vtt_parser::region_lines("lines: 10").is_err());
    }

    #[test]
    fn region_region_anchor() {
        assert_eq!(
            vtt_parser::region_region_anchor("regionanchor:10%,10%").unwrap(),
            Anchor {
                x: Percentage {
                    value: 10.0
                },
                y: Percentage {
                    value: 10.0
                },
            }
        );
        assert_eq!(
            vtt_parser::region_region_anchor("regionanchor:10.1%,10%").unwrap(),
            Anchor {
                x: Percentage {
                    value: 10.1
                },
                y: Percentage {
                    value: 10.0
                },
            }
        );
        assert!(
            vtt_parser::region_region_anchor("regionanchor:10,10").is_err()
        );
        assert!(
            vtt_parser::region_region_anchor("regionanchor:101%,10%").is_err()
        );
        assert!(
            vtt_parser::region_region_anchor("regionanchor: 10%,10%").is_err()
        );
    }

    #[test]
    fn region_viewport_anchor() {
        assert_eq!(
            vtt_parser::region_viewport_anchor("viewportanchor:10%,10%")
                .unwrap(),
            Anchor {
                x: Percentage {
                    value: 10.0
                },
                y: Percentage {
                    value: 10.0
                },
            }
        );
        assert_eq!(
            vtt_parser::region_viewport_anchor("viewportanchor:10.1%,10%")
                .unwrap(),
            Anchor {
                x: Percentage {
                    value: 10.1
                },
                y: Percentage {
                    value: 10.0
                },
            }
        );
        assert!(
            vtt_parser::region_viewport_anchor("viewportanchor:10,10").is_err()
        );
        assert!(
            vtt_parser::region_viewport_anchor("viewportanchor:101%,10%")
                .is_err()
        );
        assert!(
            vtt_parser::region_viewport_anchor("viewportanchor: 10%,10%")
                .is_err()
        );
    }

    #[test]
    fn region_scroll() {
        assert_eq!(
            vtt_parser::region_scroll("scroll:up").unwrap(),
            Scroll::Up
        );
        assert!(vtt_parser::region_scroll("scroll:down").is_err());
        assert!(vtt_parser::region_scroll("scroll: up").is_err());
    }

    #[test]
    fn region() {
        assert!(vtt_parser::region("REGION\n").is_err());

        assert_eq!(
            vtt_parser::region("REGION\nid:region\n").unwrap(),
            VttRegion {
                id: Some("region".to_string()),
                ..Default::default()
            }
        );

        assert_eq!(
            vtt_parser::region("REGION\nwidth:10%\n").unwrap(),
            VttRegion {
                width: Some(Percentage {
                    value: 10.0
                }),
                ..Default::default()
            }
        );

        assert_eq!(
            vtt_parser::region("REGION\nlines:10\n").unwrap(),
            VttRegion {
                lines: Some(10),
                ..Default::default()
            }
        );

        assert_eq!(
            vtt_parser::region("REGION\nregionanchor:10%,10%\n").unwrap(),
            VttRegion {
                region_anchor: Some(Anchor {
                    x: Percentage {
                        value: 10.0
                    },
                    y: Percentage {
                        value: 10.0
                    },
                }),
                ..Default::default()
            }
        );

        assert_eq!(
            vtt_parser::region("REGION\nviewportanchor:10%,10%\n").unwrap(),
            VttRegion {
                viewport_anchor: Some(Anchor {
                    x: Percentage {
                        value: 10.0
                    },
                    y: Percentage {
                        value: 10.0
                    },
                }),
                ..Default::default()
            }
        );

        assert_eq!(
            vtt_parser::region("REGION\nscroll:up\n").unwrap(),
            VttRegion {
                scroll: Some(Scroll::Up),
                ..Default::default()
            }
        );

        assert_eq!(
            vtt_parser::region("REGION\nid:region\nwidth:10%\nlines:10\nregionanchor:10%,10%\nviewportanchor:10%,10%\nscroll:up\n").unwrap(),
            VttRegion {
                id: Some("region".to_string()),
                width: Some(Percentage { value: 10.0 }),
                lines: Some(10),
                region_anchor: Some(Anchor {
                    x: Percentage { value: 10.0 },
                    y: Percentage { value: 10.0 },
                }),
                viewport_anchor: Some(Anchor {
                    x: Percentage { value: 10.0 },
                    y: Percentage { value: 10.0 },
                }),
                scroll: Some(Scroll::Up),

            });

        assert_eq!(
            vtt_parser::region("REGION\nwidth:10%\nlines:10\nregionanchor:10%,10%\nviewportanchor:10%,10%\nscroll:up\nid:region\n").unwrap(),
            VttRegion {
                id: Some("region".to_string()),
                width: Some(Percentage { value: 10.0 }),
                lines: Some(10),
                region_anchor: Some(Anchor {
                    x: Percentage { value: 10.0 },
                    y: Percentage { value: 10.0 },
                }),
                viewport_anchor: Some(Anchor {
                    x: Percentage { value: 10.0 },
                    y: Percentage { value: 10.0 },
                }),
                scroll: Some(Scroll::Up),

            });
    }

    #[test]
    fn vtt() {
        let text = r#"WEBVTT

00:01.000 --> 00:04.000
- Never drink liquid nitrogen.

00:05.000 --> 00:09.000
- It will perforate your stomach.
- You could die.
"#;

        let expected = WebVtt {
            header: VttHeader {
                description: None,
            },
            blocks: vec![
                VttCue {
                    identifier: None,
                    timings: VttTimings {
                        start: VttTimestamp {
                            hours: 0,
                            minutes: 0,
                            seconds: 1,
                            milliseconds: 0,
                        },
                        end: VttTimestamp {
                            hours: 0,
                            minutes: 0,
                            seconds: 4,
                            milliseconds: 0,
                        },
                    },
                    settings: None,
                    payload: vec!["- Never drink liquid nitrogen.".to_string()],
                }
                .into(),
                VttCue {
                    identifier: None,
                    timings: VttTimings {
                        start: VttTimestamp {
                            hours: 0,
                            minutes: 0,
                            seconds: 5,
                            milliseconds: 0,
                        },
                        end: VttTimestamp {
                            hours: 0,
                            minutes: 0,
                            seconds: 9,
                            milliseconds: 0,
                        },
                    },
                    settings: None,
                    payload: vec![
                        "- It will perforate your stomach.".to_string(),
                        "- You could die.".to_string(),
                    ],
                }
                .into(),
            ],
        };

        assert_eq!(vtt_parser::vtt(text).unwrap(), expected);

        let text = r#"WEBVTT - This file has cues.

14
00:01:14.815 --> 00:01:18.114
- What?
- Where are we now?

15
00:01:18.171 --> 00:01:20.991
- This is big bat country.

16
00:01:21.058 --> 00:01:23.868
- [ Bats Screeching ]
- They won't get in your hair. They're after the bugs.
"#;

        let expected = WebVtt {
            header: VttHeader {
                description: Some(VttDescription::Side("- This file has cues.\n".to_string()))
            },
            blocks: vec![
                VttCue {
                    identifier: Some("14".to_string()),
                    timings: VttTimings {
                        start: VttTimestamp {
                            hours: 0,
                            minutes: 1,
                            seconds: 14,
                            milliseconds: 815,
                        },
                        end: VttTimestamp {
                            hours: 0,
                            minutes: 1,
                            seconds: 18,
                            milliseconds: 114,
                        },
                    },
                    settings: None,
                    payload: vec![
                        "- What?".to_string(),
                        "- Where are we now?".to_string(),
                    ],
                }.into(),
                VttCue {
                    identifier: Some("15".to_string()),
                    timings: VttTimings {
                        start: VttTimestamp {
                            hours: 0,
                            minutes: 1,
                            seconds: 18,
                            milliseconds: 171,
                        },
                        end: VttTimestamp {
                            hours: 0,
                            minutes: 1,
                            seconds: 20,
                            milliseconds: 991,
                        },
                    },
                    settings: None,
                    payload: vec![
                        "- This is big bat country.".to_string(),
                    ],
                }.into(),
                VttCue {
                    identifier: Some("16".to_string()),
                    timings: VttTimings {
                        start: VttTimestamp {
                            hours: 0,
                            minutes: 1,
                            seconds: 21,
                            milliseconds: 58,
                        },
                        end: VttTimestamp {
                            hours: 0,
                            minutes: 1,
                            seconds: 23,
                            milliseconds: 868,
                        },
                    },
                    settings: None,
                    payload: vec![
                        "- [ Bats Screeching ]".to_string(),
                        "- They won't get in your hair. They're after the bugs.".to_string(),
                    ],
                }.into(),
            ],
        };

        assert_eq!(vtt_parser::vtt(text).unwrap(), expected);

        let text = r#"WEBVTT - Translation of that film I like

NOTE
This translation was done by Kyle so that
some friends can watch it with their parents.

1
00:02:15.000 --> 00:02:20.000
- Ta en kopp varmt te.
- Det är inte varmt.

2
00:02:20.000 --> 00:02:25.000
- Har en kopp te.
- Det smakar som te.

NOTE This last line may not translate well.

3
00:02:25.000 --> 00:02:30.000
- Ta en kopp
"#;

        let expected = WebVtt {
            header: VttHeader {
                description: Some(VttDescription::Side("- Translation of that film I like\n".to_string()))
            },
            blocks: vec![
                VttComment::Below("This translation was done by Kyle so that\nsome friends can watch it with their parents.".to_string()).into(),
                VttCue {
                    identifier: Some("1".to_string()),
                    timings: VttTimings {
                        start: VttTimestamp {
                            hours: 0,
                            minutes: 2,
                            seconds: 15,
                            milliseconds: 0,
                        },
                        end: VttTimestamp {
                            hours: 0,
                            minutes: 2,
                            seconds: 20,
                            milliseconds: 0,
                        },
                    },
                    settings: None,
                    payload: vec![
                        "- Ta en kopp varmt te.".to_string(),
                        "- Det är inte varmt.".to_string(),
                    ],
                }.into(),
                VttCue {
                    identifier: Some("2".to_string()),
                    timings: VttTimings {
                        start: VttTimestamp {
                            hours: 0,
                            minutes: 2,
                            seconds: 20,
                            milliseconds: 0,
                        },
                        end: VttTimestamp {
                            hours: 0,
                            minutes: 2,
                            seconds: 25,
                            milliseconds: 0,
                        },
                    },
                    settings: None,
                    payload: vec![
                        "- Har en kopp te.".to_string(),
                        "- Det smakar som te.".to_string(),
                    ],
                }.into(),
                VttComment::Side("This last line may not translate well.".to_string()).into(),
                VttCue {
                    identifier: Some("3".to_string()),
                    timings: VttTimings {
                        start: VttTimestamp {
                            hours: 0,
                            minutes: 2,
                            seconds: 25,
                            milliseconds: 0,
                        },
                        end: VttTimestamp {
                            hours: 0,
                            minutes: 2,
                            seconds: 30,
                            milliseconds: 0,
                        },
                    },
                    settings: None,
                    payload: vec!["- Ta en kopp".to_string()],
                }.into(),
            ],
        };

        assert_eq!(vtt_parser::vtt(text).unwrap(), expected);

        let text = r#"WEBVTT

STYLE
::cue {
  background-image: linear-gradient(to bottom, dimgray, lightgray);
  color: papayawhip;
}
/* Style blocks cannot use blank lines nor "dash dash greater than" */

NOTE comment blocks can be used between style blocks.

STYLE
::cue(b) {
  color: peachpuff;
}

00:00:00.000 --> 00:00:10.000
- Hello <b>world</b>.

NOTE style blocks cannot appear after the first cue.
"#;

        let expected = WebVtt {
            header: VttHeader {
                description: None
            },
            blocks: vec![
                VttStyle {
                    style: "::cue {\n  background-image: linear-gradient(to bottom, dimgray, lightgray);\n  color: papayawhip;\n}\n/* Style blocks cannot use blank lines nor \"dash dash greater than\" */\n".to_string()
                }.into(),
                VttComment::Side("comment blocks can be used between style blocks.".to_string()).into(),
                VttStyle {
                    style: "::cue(b) {\n  color: peachpuff;\n}\n".to_string()
                }.into(),
                VttCue {
                    identifier: None,
                    timings: VttTimings {
                        start: VttTimestamp {
                            hours: 0,
                            minutes: 0,
                            seconds: 0,
                            milliseconds: 0,
                        },
                        end: VttTimestamp {
                            hours: 0,
                            minutes: 0,
                            seconds: 10,
                            milliseconds: 0,
                        },
                    },
                    settings: None,
                    payload: vec!["- Hello <b>world</b>.".to_string()],
                }.into(),
                VttComment::Side("style blocks cannot appear after the first cue.".to_string()).into(),
            ],
        };

        assert_eq!(vtt_parser::vtt(text).unwrap(), expected);

        let text = r#"WEBVTT

00:00:00.000 --> 00:00:04.000 position:10%,line-left align:left size:35%
Where did he go?

00:00:03.000 --> 00:00:06.500 position:90% align:right size:35%
I think he went down this lane.

00:00:04.000 --> 00:00:06.500 position:45%,line-right align:center size:35%
What are you waiting for?
"#;

        let expected = WebVtt {
            header: VttHeader {
                description: None,
            },
            blocks: vec![
                VttCue {
                    identifier: None,
                    timings: VttTimings {
                        start: VttTimestamp {
                            hours: 0,
                            minutes: 0,
                            seconds: 0,
                            milliseconds: 0,
                        },
                        end: VttTimestamp {
                            hours: 0,
                            minutes: 0,
                            seconds: 4,
                            milliseconds: 0,
                        },
                    },
                    settings: Some(CueSettings {
                        position: Some(Position {
                            value: Percentage {
                                value: 10.0,
                            },
                            alignment: Some(PositionAlignment::LineLeft),
                        }),
                        align: Some(Alignment::Left),
                        size: Some(Percentage {
                            value: 35.0,
                        }),
                        ..Default::default()
                    }),
                    payload: vec!["Where did he go?".to_string()],
                }
                .into(),
                VttCue {
                    identifier: None,
                    timings: VttTimings {
                        start: VttTimestamp {
                            hours: 0,
                            minutes: 0,
                            seconds: 3,
                            milliseconds: 0,
                        },
                        end: VttTimestamp {
                            hours: 0,
                            minutes: 0,
                            seconds: 6,
                            milliseconds: 500,
                        },
                    },
                    settings: Some(CueSettings {
                        position: Some(Position {
                            value: Percentage {
                                value: 90.0,
                            },
                            alignment: None,
                        }),
                        align: Some(Alignment::Right),
                        size: Some(Percentage {
                            value: 35.0,
                        }),
                        ..Default::default()
                    }),
                    payload: vec![
                        "I think he went down this lane.".to_string(),
                    ],
                }
                .into(),
                VttCue {
                    identifier: None,
                    timings: VttTimings {
                        start: VttTimestamp {
                            hours: 0,
                            minutes: 0,
                            seconds: 4,
                            milliseconds: 0,
                        },
                        end: VttTimestamp {
                            hours: 0,
                            minutes: 0,
                            seconds: 6,
                            milliseconds: 500,
                        },
                    },
                    settings: Some(CueSettings {
                        position: Some(Position {
                            value: Percentage {
                                value: 45.0,
                            },
                            alignment: Some(PositionAlignment::LineRight),
                        }),
                        align: Some(Alignment::Center),
                        size: Some(Percentage {
                            value: 35.0,
                        }),
                        ..Default::default()
                    }),
                    payload: vec!["What are you waiting for?".to_string()],
                }
                .into(),
            ],
        };

        assert_eq!(vtt_parser::vtt(text).unwrap(), expected);
    }
}
