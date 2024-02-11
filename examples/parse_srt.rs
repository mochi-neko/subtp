//! An example of parsing a SubRip Subtitle (.srt) format text.
//!
//! ```shell
//! $ cargo run --example parse_srt
//! ```

use subtp::srt::SubRip;

fn main() -> anyhow::Result<()> {
    // Prepare the SubRip Subtitle (.srt) format text.
    let text = r#"
1
00:00:00,000 --> 00:00:02,000
This is the first subtitle.

2
00:00:02,000 --> 00:00:04,000
This is the second subtitle.
Subtitle text can span multiple lines.
"#;

    // Parse the SubRip Subtitle (.srt) format text to the `SubRip` struct.
    let subrip = SubRip::parse(text)?;
    println!("Parsed srt:\n{:?}", subrip);

    // Render the `SubRip` struct to the SubRip Subtitle (.srt) format text.
    let rendered = subrip.render();
    println!("Rendered srt:\n{}", rendered);

    // Get each subtitle by iterator.
    println!("Iterate subtitles:");
    for subtitle in subrip.into_iter() {
        println!("Subtitle:\n{:?}", subtitle);
    }

    Ok(())
}
