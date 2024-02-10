//! An example of parsing a SubRip Subtitle (.srt) file.
//!
//! ```shell
//! $ cargo run --example srt
//! ```

use subtp::srt::SubRip;

fn main() -> anyhow::Result<()> {
    let text = r#"
1
00:00:00,000 --> 00:00:02,000
This is the first subtitle.

2
00:00:02,000 --> 00:00:04,000
This is the second subtitle.
Subtitle text can span multiple lines.
"#;

    let subrip = SubRip::parse(text)?;
    println!("{}", subrip);

    Ok(())
}
