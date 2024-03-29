peg::parser! {
    grammar rules() for str {
        /// Whitespace.
        pub(crate) rule whitespace() = [' ' | '\t']

        /// Newline.
        pub(crate) rule newline() = "\r\n" / "\n" / "\r"

        /// Any-digit number.
        pub(crate) rule number() -> u32
            = n:$(['0'..='9']+) {?
                n.parse().or(Err("number in u32"))
            }

        /// Signed integer.
        pub(crate) rule int() -> i32
            = n:$(['+' | '-']? ['0'..='9']+) {?
                n.parse().or(Err("signed number"))
            }

        /// Two-digit number.
        pub(crate) rule two_number() -> u8
            = n:$(['0'..='9']['0'..='9']) {?
                n.parse().or(Err("two-digit number"))
            }

        /// Three-digit number.
        pub(crate) rule three_number() -> u16
            = n:$(['0'..='9']['0'..='9']['0'..='9']) {?
                n.parse().or(Err("three-digit number"))
            }

        /// Floating number.
        pub(crate) rule float() -> f32
            = n:$(['0'..='9']+ "." ['0'..='9']+) {?
                n.parse().or(Err("Invalid float"))
            }

        /// Percentage of integer number.
        pub(crate) rule percentage_int() -> u32
            = n:number() "%" {?
                if n <= 100 {
                    Ok(n)
                } else {
                    Err("Number out of range")
                }
            }

        /// Percentage of floating number.
        pub(crate) rule percentage_float() -> f32
            = f:float() "%" {?
                if f >= 0.0 && f <= 100.0 {
                    Ok(f)
                } else {
                    Err("Number out of range")
                }
            }

        /// Percentage.
        pub(crate) rule percentage() -> f32
            = p:percentage_int() { p as f32 }
            / p:percentage_float() { p }

        /// Anchor.
        pub(crate) rule anchor() -> (f32, f32)
            = x:percentage() "," y:percentage()
            {
                (x, y)
            }

        /// Sequential text without space of newline.
        pub(crate) rule sequence() -> String
            = t:$((!(whitespace() / newline()) [_])+)
            {
                t.to_string()
            }

        /// Block of text.
        pub(crate) rule text_block() -> String
            = !newline() lines:$((!newline() [_])+ newline()) ++ ()
            {
                lines.join("").to_string()
            }

        /// Single text with newline.
        pub(crate) rule line() -> String
            = !(whitespace() / newline()) t:$((!newline() [_])+) newline()
            {
                t.to_string().trim().to_string()
            }

        /// Multiple lines block of text.
        pub(crate) rule multiline() -> Vec<String>
            = !(whitespace() / newline()) lines:$(!(whitespace()+ newline()) (!newline() [_])+ newline()) ++ ()
            {
                lines
                    .iter()
                    .map(|l| l.to_string().trim().to_string())
                    .collect()
            }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn whitespace() {
        assert!(super::rules::whitespace(" ").is_ok());
        assert!(super::rules::whitespace("\t").is_ok());
        assert!(super::rules::whitespace("a").is_err());
    }

    #[test]
    fn newline() {
        assert!(super::rules::newline("\n").is_ok());
        assert!(super::rules::newline("\r").is_ok());
        assert!(super::rules::newline("\r\n").is_ok());
        assert!(super::rules::newline("").is_err());
        assert!(super::rules::newline("\n\r").is_err());
        assert!(super::rules::newline("\n\n").is_err());
        assert!(super::rules::newline("a").is_err());
    }

    #[test]
    fn number() {
        assert_eq!(super::rules::number("0").unwrap(), 0);
        assert_eq!(super::rules::number("1").unwrap(), 1);
        assert_eq!(super::rules::number("9").unwrap(), 9);
        assert_eq!(super::rules::number("10").unwrap(), 10);
        assert_eq!(
            super::rules::number("123").unwrap(),
            123
        );
        assert!(super::rules::number("a").is_err());
        assert!(super::rules::number(" ").is_err());
    }

    #[test]
    fn signed_number() {
        assert_eq!(super::rules::int("0").unwrap(), 0);
        assert_eq!(super::rules::int("1").unwrap(), 1);
        assert_eq!(super::rules::int("9").unwrap(), 9);
        assert_eq!(super::rules::int("10").unwrap(), 10);
        assert_eq!(super::rules::int("123").unwrap(), 123);
        assert_eq!(super::rules::int("+0").unwrap(), 0);
        assert_eq!(super::rules::int("+1").unwrap(), 1);
        assert_eq!(super::rules::int("+9").unwrap(), 9);
        assert_eq!(super::rules::int("+10").unwrap(), 10);
        assert_eq!(super::rules::int("+123").unwrap(), 123);
        assert_eq!(super::rules::int("-0").unwrap(), 0);
        assert_eq!(super::rules::int("-1").unwrap(), -1);
        assert_eq!(super::rules::int("-9").unwrap(), -9);
        assert_eq!(super::rules::int("-10").unwrap(), -10);
        assert_eq!(super::rules::int("-123").unwrap(), -123);
        assert!(super::rules::int("a").is_err());
        assert!(super::rules::int(" ").is_err());
    }

    #[test]
    fn two_number() {
        assert_eq!(
            super::rules::two_number("00").unwrap(),
            0
        );
        assert_eq!(
            super::rules::two_number("01").unwrap(),
            1
        );
        assert_eq!(
            super::rules::two_number("09").unwrap(),
            9
        );
        assert_eq!(
            super::rules::two_number("10").unwrap(),
            10
        );
        assert_eq!(
            super::rules::two_number("99").unwrap(),
            99
        );
        assert!(super::rules::two_number("0").is_err());
        assert!(super::rules::two_number("000").is_err());
        assert!(super::rules::two_number("a").is_err());
        assert!(super::rules::two_number(" ").is_err());
    }

    #[test]
    fn three_number() {
        assert_eq!(
            super::rules::three_number("000").unwrap(),
            0
        );
        assert_eq!(
            super::rules::three_number("001").unwrap(),
            1
        );
        assert_eq!(
            super::rules::three_number("009").unwrap(),
            9
        );
        assert_eq!(
            super::rules::three_number("010").unwrap(),
            10
        );
        assert_eq!(
            super::rules::three_number("099").unwrap(),
            99
        );
        assert_eq!(
            super::rules::three_number("100").unwrap(),
            100
        );
        assert_eq!(
            super::rules::three_number("999").unwrap(),
            999
        );
        assert!(super::rules::three_number("00").is_err());
        assert!(super::rules::three_number("0000").is_err());
        assert!(super::rules::three_number("a").is_err());
        assert!(super::rules::three_number(" ").is_err());
    }

    #[test]
    fn float() {
        assert_eq!(super::rules::float("0.0").unwrap(), 0.0);
        assert_eq!(super::rules::float("1.0").unwrap(), 1.0);
        assert_eq!(super::rules::float("9.0").unwrap(), 9.0);
        assert_eq!(
            super::rules::float("10.01").unwrap(),
            10.01
        );
        assert_eq!(
            super::rules::float("99.0").unwrap(),
            99.0
        );
        assert!(super::rules::float("0").is_err());
        assert!(super::rules::float("1").is_err());
        assert!(super::rules::float("10").is_err());
        assert!(super::rules::float("a").is_err());
        assert!(super::rules::float(" ").is_err());
    }

    #[test]
    fn percentage_int() {
        assert_eq!(
            super::rules::percentage_int("0%").unwrap(),
            0
        );
        assert_eq!(
            super::rules::percentage_int("1%").unwrap(),
            1
        );
        assert_eq!(
            super::rules::percentage_int("9%").unwrap(),
            9
        );
        assert_eq!(
            super::rules::percentage_int("10%").unwrap(),
            10
        );
        assert_eq!(
            super::rules::percentage_int("99%").unwrap(),
            99
        );
        assert_eq!(
            super::rules::percentage_int("100%").unwrap(),
            100
        );
        assert_eq!(
            super::rules::percentage_int("000%").unwrap(),
            0
        );
        assert!(super::rules::percentage_int("10.0%").is_err());
        assert!(super::rules::percentage_int("100.1%").is_err());
        assert!(super::rules::percentage_int("100.9%").is_err());
        assert!(super::rules::percentage_int("101%").is_err());
        assert!(super::rules::percentage_int("999%").is_err());
        assert!(super::rules::percentage_int("0").is_err());
        assert!(super::rules::percentage_int("a").is_err());
        assert!(super::rules::percentage_int(" ").is_err());
    }

    #[test]
    fn percentage_float() {
        assert_eq!(
            super::rules::percentage_float("0.0%").unwrap(),
            0.0
        );
        assert_eq!(
            super::rules::percentage_float("1.0%").unwrap(),
            1.0
        );
        assert_eq!(
            super::rules::percentage_float("9.0%").unwrap(),
            9.0
        );
        assert_eq!(
            super::rules::percentage_float("10.0%").unwrap(),
            10.0
        );
        assert_eq!(
            super::rules::percentage_float("99.0%").unwrap(),
            99.0
        );
        assert_eq!(
            super::rules::percentage_float("100.0%").unwrap(),
            100.0
        );
        assert_eq!(
            super::rules::percentage_float("99.9%").unwrap(),
            99.9
        );
        assert_eq!(
            super::rules::percentage_float("0.1%").unwrap(),
            0.1
        );
        assert_eq!(
            super::rules::percentage_float("0.9%").unwrap(),
            0.9
        );
        assert!(super::rules::percentage_float("100.1%").is_err());
        assert!(super::rules::percentage_float("100.9%").is_err());
        assert!(super::rules::percentage_float("100").is_err());
        assert!(super::rules::percentage_float("0").is_err());
        assert!(super::rules::percentage_float("a").is_err());
        assert!(super::rules::percentage_float(" ").is_err());
    }

    #[test]
    fn percentage() {
        assert_eq!(
            super::rules::percentage("0%").unwrap(),
            0.0
        );
        assert_eq!(
            super::rules::percentage("1%").unwrap(),
            1.0
        );
        assert_eq!(
            super::rules::percentage("9%").unwrap(),
            9.0
        );
        assert_eq!(
            super::rules::percentage("10%").unwrap(),
            10.0
        );
        assert_eq!(
            super::rules::percentage("99%").unwrap(),
            99.0
        );
        assert_eq!(
            super::rules::percentage("100%").unwrap(),
            100.0
        );
        assert_eq!(
            super::rules::percentage("100.0%").unwrap(),
            100.0
        );
        assert_eq!(
            super::rules::percentage("000%").unwrap(),
            0.0
        );
        assert!(super::rules::percentage("100.1%").is_err());
        assert!(super::rules::percentage("100.9%").is_err());
        assert!(super::rules::percentage("101%").is_err());
        assert!(super::rules::percentage("999%").is_err());
        assert!(super::rules::percentage("0").is_err());
        assert!(super::rules::percentage("a").is_err());
        assert!(super::rules::percentage(" ").is_err());
    }

    #[test]
    fn anchor() {
        assert_eq!(
            super::rules::anchor("0%,0%").unwrap(),
            (0.0, 0.0)
        );
        assert_eq!(
            super::rules::anchor("1%,1%").unwrap(),
            (1.0, 1.0)
        );
        assert_eq!(
            super::rules::anchor("9%,9%").unwrap(),
            (9.0, 9.0)
        );
        assert_eq!(
            super::rules::anchor("10%,10%").unwrap(),
            (10.0, 10.0)
        );
        assert_eq!(
            super::rules::anchor("99%,99%").unwrap(),
            (99.0, 99.0)
        );
        assert_eq!(
            super::rules::anchor("100%,100%").unwrap(),
            (100.0, 100.0)
        );
        assert_eq!(
            super::rules::anchor("100.0%,100.0%").unwrap(),
            (100.0, 100.0)
        );
        assert_eq!(
            super::rules::anchor("000%,000%").unwrap(),
            (0.0, 0.0)
        );
        assert!(super::rules::anchor("100.1%,100.1%").is_err());
        assert!(super::rules::anchor("100.9%,100.9%").is_err());
        assert!(super::rules::anchor("101%,101%").is_err());
        assert!(super::rules::anchor("999%,999%").is_err());
        assert!(super::rules::anchor("0,0").is_err());
        assert!(super::rules::anchor("a").is_err());
        assert!(super::rules::anchor(" ").is_err());
    }

    #[test]
    fn sequence() {
        assert_eq!(
            super::rules::sequence("Hello,world!").unwrap(),
            "Hello,world!".to_string()
        );
        assert!(super::rules::sequence(" Hello,world!").is_err());
        assert!(super::rules::sequence("Hello, world!").is_err());
        assert!(super::rules::sequence("Hello,world! ").is_err());
        assert!(super::rules::sequence("\nHello,world!").is_err());
        assert!(super::rules::sequence("Hello,\nworld!").is_err());
        assert!(super::rules::sequence("Hello,world!\n").is_err());
        assert!(super::rules::sequence(" Hello,world!  \n").is_err());
    }

    #[test]
    fn text_block() {
        assert_eq!(
            super::rules::text_block("Hello, world!\n").unwrap(),
            "Hello, world!\n".to_string()
        );
        assert_eq!(
            super::rules::text_block("Hello, world!\nThis is a test.\n")
                .unwrap(),
            "Hello, world!\nThis is a test.\n".to_string()
        );
        assert_eq!(
            super::rules::text_block(
                "Hello, world!\nThis is a test.\nHow are you?\n"
            )
            .unwrap(),
            "Hello, world!\nThis is a test.\nHow are you?\n".to_string()
        );
        assert!(super::rules::text_block("").is_err());
        assert!(super::rules::text_block("Hello, world!").is_err());
        assert!(super::rules::text_block("\nHello, world!\n").is_err());
        assert!(
            super::rules::text_block("Hello, world!\nThis is a test.\n\n")
                .is_err()
        );
        assert!(super::rules::text_block("some\ntext\n\nover\nline").is_err());
    }

    #[test]
    fn line() {
        assert_eq!(
            super::rules::line("Hello, world!\n").unwrap(),
            "Hello, world!".to_string()
        );
        assert_eq!(
            super::rules::line("Hello, world! \n").unwrap(),
            "Hello, world!".to_string()
        );
        assert!(super::rules::line(" Hello, world!\n").is_err());
        assert!(super::rules::line("Hello, world!").is_err());
        assert!(super::rules::line("\nHello, world!").is_err());
        assert!(super::rules::line("Hello, world!\nThis is a test.").is_err());
    }

    #[test]
    fn multiline() {
        assert_eq!(
            super::rules::multiline("Hello, world!\n").unwrap(),
            vec!["Hello, world!".to_string()]
        );
        assert_eq!(
            super::rules::multiline("Hello, world!\nThis is a test.\n")
                .unwrap(),
            vec![
                "Hello, world!".to_string(),
                "This is a test.".to_string(),
            ]
        );
        assert_eq!(
            super::rules::multiline(
                "Hello, world!\nThis is a test.\nHow are you?\n"
            )
            .unwrap(),
            vec![
                "Hello, world!".to_string(),
                "This is a test.".to_string(),
                "How are you?".to_string(),
            ]
        );

        assert!(super::rules::multiline("").is_err());
        assert!(super::rules::multiline("Hello, world!").is_err());
        assert!(super::rules::multiline(" Hello, world!\n").is_err());
        assert!(super::rules::multiline("\nHello, world!\n").is_err());
        assert!(
            super::rules::multiline("Hello, world!\nThis is a test.\n\n")
                .is_err()
        );
        assert!(
            super::rules::multiline("Hello, world!\nThis is a test.\n \n")
                .is_err()
        );
        assert!(super::rules::multiline("some\ntext\n\nover\nline").is_err());
    }
}
