//! An example of parsing a WebVTT Subtitle (.vtt) format text with full options.
//!
//! ```shell
//! $ cargo run --example parse_vtt_full
//! ```

use subtp::vtt::WebVtt;

fn main() -> anyhow::Result<()> {
    // Prepare the WebVTT Subtitle (.vtt) format text.
    let text = r#"WEBVTT This is a description.
Header can span multiple lines.

REGION
id:region_id
width:50%
lines:3
regionanchor:50%,50%
viewportanchor:50%,50%
scroll:up

NOTE You can define a region settings and specify it in a cue settings by region id.

STYLE
video::cue {
  background-image: linear-gradient(to bottom, dimgray, lightgray);
  color: papayawhip;
}

NOTE
You can define a CSS style settings.

00:00:01.000 --> 00:00:02.000
A standard subtitle cue.

NOTE You can add a comment
between other blocks.

cue_id
00:00:03.000 --> 00:00:04.000 vertical:lr line:100%,center position:50%,center size:50% align:center region:region_id
A subtitle with cue identifier and cue settings.
A subtitle can span multiple lines.

00:05.000 --> 00:06.000
A minimal time format subtitle.
You can omit the hour part.
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
