//! An example of parsing a WebVTT Subtitle (.vtt) format text.
//!
//! ```shell
//! $ cargo run --example parse_vtt
//! ```

use subtp::vtt::WebVtt;

fn main() -> anyhow::Result<()> {
    // Prepare the WebVTT Subtitle (.vtt) format text.
    let text = r#"WEBVTT

00:00:01.000 --> 00:00:04.000
- Never drink liquid nitrogen.

00:00:05.000 --> 00:00:09.000
- It will perforate your stomach.
- You could die.
"#;

    // Parse the WebVTT Subtitle (.vtt) format text to the `WebVtt` struct.
    let webvtt = WebVtt::parse(text)?;
    println!("Parsed vtt:\n{:?}", webvtt);

    // Render the `WebVtt` struct to the WebVTT Subtitle (.vtt) format text.
    let rendered = webvtt.render();
    println!("Rendered vtt:\n{}", rendered);

    // Get each block by iterator.
    println!("Iterate blocks:");
    for subtitle in webvtt.into_iter() {
        println!("Block:\n{:?}", subtitle);
    }

    Ok(())
}
