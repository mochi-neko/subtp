# subtp
A parser in Rust for subtitle text formats such as the SubRip Subtitle (.srt) format and the WebVTT (.vtt) format.

## Installation

Run the following Cargo command in your project directory:

```shell
cargo add subtp
```

or add the following line to your Cargo.toml:

```toml
[dependencies]
subtp = "0.2.0"
```

## Features

- [x] [SubRip Subtitle (.srt)](#subrip-subtitle-srt) parser and renderer.
- [x] [WebVTT (.vtt)](#webvtt-vtt) parser and renderer.

## Usage

### SubRip Subtitle (.srt)

Parse a SubRip Subtitle (.srt) text into a `subtp::srt::SubRip` struct.

```rust
use subtp::srt::SubRip;

let text = r#"
1
00:00:00,000 --> 00:00:02,000
Hello, world!

2
00:00:02,000 --> 00:00:04,000
This is a subtitle.
"#;

let subrip = SubRip::parse(text)?;
```

Render a `subtp::srt::SubRip` struct into a SubRip Subtitle (.srt) text.

`subtp::srt::SubRip` is constructed with a vector of `subtp::srt::SrtSubtitle`.

```rust
use subtp::srt::{SubRip, SrtSubtitle, SrtTimestamp};

let subrip = SubRip {
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
 
let text = subrip.render();
```

### WebVTT (.vtt)

Parse a WebVTT (.vtt) text into a `subtp::vtt::WebVTT` struct.

```rust
use subtp::vtt::WebVtt;

let text = r#"WEBVTT

1
00:00:00.000 --> 00:00:02.000
Hello, world!

2
00:00:02.000 --> 00:00:04.000
This is a subtitle.
"#;

let webvtt = WebVtt::parse(text)?;
```

Render a `subtp::vtt::WebVTT` struct into a WebVTT (.vtt) text.

`subtp::vtt::WebVTT` is constructed with a header `subtp::vtt::VttHeader` and a vector of `subtp::vtt::VttBlcok`.

`subtp::vtt::VttBlcok` can be following types:
- `subtp::vtt::VttCue`
    - Cue block with an identifier (Optional), timings, settings (Optional) and a subtitle text. 
- `subtp::vtt::VttComment`
    - Comment block with a noting text. 
- `subtp::vtt::VttStyle`
    - Style block with a CSS style text. 
- `subtp::vtt::VttRegion`
    - Region block with a region definition. 

```rust
use subtp::vtt::{WebVtt, VttBlock, VttCue, VttHeader, VttTimings, VttTimestamp};

let webvtt = WebVtt {
    blocks: vec![
        VttCue {
            identifier: Some("1".to_string()),
            timings: VttTimings {
                start: VttTimestamp {
                    seconds: 0,
                    ..Default::default()
                },
                end: VttTimestamp {
                    seconds: 2,
                    ..Default::default()
                },
            },
            payload: vec!["Hello, world!".to_string()],
            ..Default::default()
        }.into(),
        VttCue {
            identifier: Some("2".to_string()),
            timings: VttTimings {
                start: VttTimestamp {
                    seconds: 2,
                    ..Default::default()
                },
                end: VttTimestamp {
                    seconds: 4,
                    ..Default::default()
                },
            },
            payload: vec!["This is a subtitle.".to_string()],
            ..Default::default()
        }.into(),
    ],
    ..Default::default()
};

let text = webvtt.render();
```

## Other examples

See the [./examples](./examples) directory.

## Changelog

See [CHANGELOG](./CHANGELOG.md).

## License

Licensed under either of the [Apache License, Version 2.0](./LICENSE-APACHE) or the [MIT](./LICENSE-MIT) license at your option.
