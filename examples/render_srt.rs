//! An example of rendering a SubRip Subtitle (.srt) format text.
//!
//! ```shell
//! $ cargo run --example render_srt
//! ```

use subtp::srt::SrtSubtitle;
use subtp::srt::SrtTimestamp;
use subtp::srt::SubRip;

fn main() -> anyhow::Result<()> {
    // Prepare the `SubRip` struct.
    let mut subrip = SubRip {
        subtitles: vec![
            SrtSubtitle {
                sequence: 1,
                start: SrtTimestamp {
                    seconds: 1,
                    ..Default::default()
                },
                end: SrtTimestamp {
                    seconds: 2,
                    ..Default::default()
                },
                text: vec!["This is the first subtitle.".to_string()],
                ..Default::default()
            },
            SrtSubtitle {
                sequence: 2,
                start: SrtTimestamp {
                    seconds: 3,
                    ..Default::default()
                },
                end: SrtTimestamp {
                    seconds: 4,
                    ..Default::default()
                },
                text: vec![
                    "This is the second subtitle.".to_string(),
                    "Subtitle text can span multiple lines.".to_string(),
                ],
                ..Default::default()
            },
        ],
    };

    // Render the `SubRip` struct to the SubRip Subtitle (.srt) format text.
    let rendered = subrip.render();
    println!("Rendered srt:\n{}", rendered);

    subrip
        .subtitles
        .push(SrtSubtitle {
            sequence: 3,
            start: SrtTimestamp {
                seconds: 5,
                ..Default::default()
            },
            end: SrtTimestamp {
                seconds: 6,
                ..Default::default()
            },
            text: vec!["This is the third subtitle.".to_string()],
            ..Default::default()
        });
    println!("Rendered srt:\n{}", subrip.render());

    Ok(())
}
