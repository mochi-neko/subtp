//! An example of rendering a WebVTT Subtitle (.vtt) format text.
//!
//! ```shell
//! $ cargo run --example render_vtt
//! ```

use subtp::vtt::VttComment;
use subtp::vtt::VttCue;
use subtp::vtt::VttDescription;
use subtp::vtt::VttHeader;
use subtp::vtt::VttRegion;
use subtp::vtt::VttTimestamp;
use subtp::vtt::VttTimings;
use subtp::vtt::WebVtt;
use subtp::vtt::{Alignment, Position, PositionAlignment, Scroll};
use subtp::vtt::{Anchor, VttStyle};
use subtp::vtt::{CueSettings, Line, LineAlignment, Percentage, Vertical};

fn main() -> anyhow::Result<()> {
    // Prepare the `WebVtt` struct.
    let mut webvtt = WebVtt {
        ..Default::default()
    };

    // Render the `WebVtt` struct to the WebVTT Subtitle (.vtt) format text.
    let rendered = webvtt.render();
    println!("Rendered vtt:\n{}", rendered);

    // Set description of the file.
    webvtt.header = VttHeader {
        description: Some(VttDescription::Side(
            "This is a description.".to_string(),
        )),
    };
    println!("Rendered vtt:\n{}", webvtt.render());

    // Add region block.
    webvtt.blocks.push(
        VttRegion {
            id: Some("region_id".to_string()),
            width: Some(Percentage {
                value: 100.0,
            }),
            lines: Some(3),
            region_anchor: Some(Anchor {
                x: Percentage {
                    value: 10.0,
                },
                y: Percentage {
                    value: 90.0,
                },
            }),
            viewport_anchor: Some(Anchor {
                x: Percentage {
                    value: 20.0,
                },
                y: Percentage {
                    value: 80.0,
                },
            }),
            scroll: Some(Scroll::Up),
        }
        .into(),
    );
    println!("Rendered vtt:\n{}", webvtt.render());

    // Add style block.
    webvtt.blocks.push(
        VttStyle {
            style: r#"video::cue {
  background-image: linear-gradient(to bottom, dimgray, lightgray);
  color: papayawhip;
}"#
            .to_string(),
        }
        .into(),
    );

    // Add comment block.
    webvtt.blocks.push(
        VttComment::Below(
            "You can add a comment between other blocks.".to_string(),
        )
        .into(),
    );
    println!("Rendered vtt:\n{}", webvtt.render());

    // Add simple cue block.
    webvtt.blocks.push(
        VttCue {
            timings: VttTimings {
                start: VttTimestamp {
                    seconds: 1,
                    ..Default::default()
                },
                end: VttTimestamp {
                    seconds: 2,
                    ..Default::default()
                },
            },
            payload: vec!["A standard subtitle cue.".to_string()],
            ..Default::default()
        }
        .into(),
    );
    println!("Rendered vtt:\n{}", webvtt.render());

    // Add comment block.
    webvtt.blocks.push(
        VttComment::Below(
            "You can add a comment between other blocks.".to_string(),
        )
        .into(),
    );

    // Add cue block with identifier and settings.
    webvtt.blocks.push(
        VttCue {
            identifier: Some("cue_id".to_string()),
            timings: VttTimings {
                start: VttTimestamp {
                    seconds: 3,
                    ..Default::default()
                },
                end: VttTimestamp {
                    seconds: 4,
                    ..Default::default()
                },
            },
            settings: Some(CueSettings {
                vertical: Some(Vertical::Lr),
                line: Some(Line::Percentage(
                    Percentage {
                        value: 100.0,
                    },
                    Some(LineAlignment::Center),
                )),
                position: Some(Position {
                    value: Percentage {
                        value: 50.0,
                    },
                    alignment: Some(PositionAlignment::LineLeft),
                }),
                size: Some(Percentage {
                    value: 50.0,
                }),
                align: Some(Alignment::Left),
                region: Some("region_id".to_string()),
            }),
            payload: vec![
                "A subtitle with cue identifier and cue settings.".to_string(),
                "A subtitle can span multiple lines.".to_string(),
            ],
        }
        .into(),
    );
    println!("Rendered vtt:\n{}", webvtt.render());

    Ok(())
}
