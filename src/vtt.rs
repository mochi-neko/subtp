use std::fmt::Display;
use std::ops::{Add, Sub};

/// The WebVTT format (`.vtt`).
#[derive(Debug, Clone, PartialEq)]
pub struct WebVtt {
    /// The header.
    pub header: VttHeader,
    /// The blocks of the WebVTT.
    pub blocks: Vec<VttBlock>,
}

impl Default for WebVtt {
    fn default() -> Self {
        Self {
            header: VttHeader::default(),
            blocks: vec![],
        }
    }
}

impl Display for WebVtt {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}\n", self.header)?;

        for block in &self.blocks {
            write!(f, "{}\n", block)?;
        }

        Ok(())
    }
}

/// The header block.
#[derive(Debug, Clone, PartialEq)]
pub struct VttHeader {
    /// The description of this file.
    pub description: Option<VttDescription>,
}

impl Default for VttHeader {
    fn default() -> Self {
        Self { description: None }
    }
}

impl Display for VttHeader {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(description) = &self.description {
            write!(f, "WEBVTT{}\n", description)
        } else {
            write!(f, "WEBVTT\n")
        }
    }
}

/// The description of the WebVTT.
#[derive(Debug, Clone, PartialEq)]
pub enum VttDescription {
    /// From side with "WEBVTT".
    Side(String),
    /// From below with "WEBVTT".
    Below(String),
}

impl Default for VttDescription {
    fn default() -> Self {
        Self::Side(String::new())
    }
}

impl Display for VttDescription {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::Side(description) => {
                write!(f, " {}", description)
            },
            | Self::Below(description) => {
                write!(f, "\n{}", description)
            },
        }
    }
}

/// The block of WebVTT.
#[derive(Debug, Clone, PartialEq)]
pub enum VttBlock {
    /// The cue block.
    Que(VttQue),
    /// The comment block.
    Comment(VttComment),
    /// The style block.
    Style(VttStyle),
    /// The region block.
    Region(VttRegion),
}

impl From<VttQue> for VttBlock {
    fn from(value: VttQue) -> Self {
        VttBlock::Que(value)
    }
}

impl From<VttComment> for VttBlock {
    fn from(value: VttComment) -> Self {
        VttBlock::Comment(value)
    }
}

impl From<VttStyle> for VttBlock {
    fn from(value: VttStyle) -> Self {
        VttBlock::Style(value)
    }
}

impl From<VttRegion> for VttBlock {
    fn from(value: VttRegion) -> Self {
        VttBlock::Region(value)
    }
}

impl Display for VttBlock {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::Que(que) => {
                write!(f, "{}", que)
            },
            | Self::Comment(comment) => {
                write!(f, "{}", comment)
            },
            | Self::Style(style) => {
                write!(f, "{}", style)
            },
            | Self::Region(region) => {
                write!(f, "{}", region)
            },
        }
    }
}

/// The region block.
#[derive(Debug, Clone, PartialEq)]
pub struct VttRegion {
    /// The identifier.
    pub id: Option<RegionId>,
    /// The width.
    pub width: Option<Percentage>,
    /// The lines.
    pub lines: Option<u32>,
    /// The region anchor.
    pub region_anchor: Option<Anchor>,
    /// The viewport anchor.
    pub viewport_anchor: Option<Anchor>,
    /// The scroll.
    pub scroll: Option<Scroll>,
}

pub type RegionId = String;

impl Default for VttRegion {
    fn default() -> Self {
        Self {
            id: None,
            width: None,
            lines: None,
            region_anchor: None,
            viewport_anchor: None,
            scroll: None,
        }
    }
}

impl Display for VttRegion {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "REGION\n")?;

        if let Some(id) = &self.id {
            write!(f, "id:{}\n", id)?;
        }

        if let Some(width) = self.width {
            write!(f, "width:{}\n", width)?;
        }

        if let Some(lines) = self.lines {
            write!(f, "lines:{}\n", lines)?;
        }

        if let Some(region_anchor) = self.region_anchor {
            write!(f, "regionanchor:{}\n", region_anchor)?;
        }

        if let Some(viewport_anchor) = self.viewport_anchor {
            write!(f, "viewportanchor:{}\n", viewport_anchor)?;
        }

        if let Some(scroll) = self.scroll {
            write!(f, "scroll:{}\n", scroll)?;
        }

        Ok(())
    }
}

/// The comment block.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum VttComment {
    /// Side with "NOTE".
    Side(String),
    /// Below with "NOTE".
    Below(String),
}

impl Default for VttComment {
    fn default() -> Self {
        Self::Side(String::new())
    }
}

impl Display for VttComment {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::Side(comment) => {
                write!(f, "NOTE {}\n", comment)
            },
            | Self::Below(comment) => {
                write!(f, "NOTE\n{}\n", comment)
            },
        }
    }
}

/// The style block.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct VttStyle {
    pub style: String,
}

impl Default for VttStyle {
    fn default() -> Self {
        Self { style: String::new() }
    }
}

impl Display for VttStyle {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "STYLE\n{}\n", self.style)
    }
}

/// The cue block.
#[derive(Debug, Clone, PartialEq)]
pub struct VttQue {
    /// The identifier.
    pub identifier: Option<String>,
    /// The timings.
    pub timings: VttTimings,
    /// The settings.
    pub settings: Option<CueSettings>,
    /// The payload of subtitle text.
    pub payload: Vec<String>,
}

impl Default for VttQue {
    fn default() -> Self {
        Self {
            identifier: None,
            timings: VttTimings::default(),
            settings: None,
            payload: vec![],
        }
    }
}

impl Display for VttQue {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(identifier) = &self.identifier {
            write!(f, "{}\n", identifier)?;
        }

        write!(f, "{}", self.timings)?;

        if let Some(settings) = &self.settings {
            write!(f, " {}", settings)?;
        }

        write!(f, "\n{}\n", self.payload.join("\n"))
    }
}

/// The timings.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct VttTimings {
    /// The start timestamp.
    pub start: VttTimestamp,
    /// The end timestamp.
    pub end: VttTimestamp,
}

impl Default for VttTimings {
    fn default() -> Self {
        Self {
            start: VttTimestamp::default(),
            end: VttTimestamp::default(),
        }
    }
}

impl Display for VttTimings {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{} --> {}", self.start, self.end)
    }
}

/// The timestamp.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct VttTimestamp {
    /// The hours.
    pub hours: u8,
    /// The minutes.
    pub minutes: u8,
    /// The seconds.
    pub seconds: u8,
    /// The milliseconds.
    pub milliseconds: u16,
}

impl Default for VttTimestamp {
    fn default() -> Self {
        Self {
            hours: 0,
            minutes: 0,
            seconds: 0,
            milliseconds: 0,
        }
    }
}

impl Display for VttTimestamp {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{:02}:{:02}:{:02}.{:03}",
            self.hours, self.minutes, self.seconds, self.milliseconds
        )
    }
}

impl Add for VttTimestamp {
    type Output = Self;

    fn add(
        self,
        rhs: Self,
    ) -> Self::Output {
        let mut milliseconds = self.milliseconds + rhs.milliseconds;
        let mut seconds = self.seconds + rhs.seconds;
        let mut minutes = self.minutes + rhs.minutes;
        let mut hours = self.hours + rhs.hours;

        if milliseconds >= 1000 {
            milliseconds -= 1000;
            seconds += 1;
        }

        if seconds >= 60 {
            seconds -= 60;
            minutes += 1;
        }

        if minutes >= 60 {
            minutes -= 60;
            hours += 1;
        }

        Self {
            hours,
            minutes,
            seconds,
            milliseconds,
        }
    }
}

impl Sub for VttTimestamp {
    type Output = Self;

    fn sub(
        self,
        rhs: Self,
    ) -> Self::Output {
        let mut milliseconds =
            self.milliseconds as i16 - rhs.milliseconds as i16;
        let mut seconds = self.seconds as i16 - rhs.seconds as i16;
        let mut minutes = self.minutes as i16 - rhs.minutes as i16;
        let mut hours = self.hours as i16 - rhs.hours as i16;

        if milliseconds < 0 {
            milliseconds += 1000;
            seconds -= 1;
        }

        if seconds < 0 {
            seconds += 60;
            minutes -= 1;
        }

        if minutes < 0 {
            minutes += 60;
            hours -= 1;
        }

        Self {
            hours: hours as u8,
            minutes: minutes as u8,
            seconds: seconds as u8,
            milliseconds: milliseconds as u16,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CueSettings {
    /// The vertical setting.
    pub vertical: Option<Vertical>,
    /// The line setting.
    pub line: Option<Line>,
    /// The position setting.
    pub position: Option<Position>,
    /// The size setting.
    pub size: Option<Percentage>,
    /// The alignment setting.
    pub align: Option<Alignment>,
    /// The region setting.
    pub region: Option<RegionId>,
}

impl Default for CueSettings {
    fn default() -> Self {
        Self {
            vertical: None,
            line: None,
            position: None,
            size: None,
            align: None,
            region: None,
        }
    }
}

impl Display for CueSettings {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let mut settings = Vec::new();

        if let Some(vertical) = self.vertical {
            settings.push(format!("vertical:{}", vertical));
        }

        if let Some(line) = self.line {
            settings.push(format!("line:{}", line));
        }

        if let Some(position) = self.position {
            settings.push(format!("position:{}", position));
        }

        if let Some(size) = self.size {
            settings.push(format!("size:{}", size));
        }

        if let Some(align) = self.align {
            settings.push(format!("align:{}", align));
        }

        if let Some(region) = &self.region {
            settings.push(format!("region:{}", region));
        }

        write!(f, "{}", settings.join(" "))
    }
}

/// The percentage in range 0.0 to 100.0, inclusive.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Percentage {
    pub value: f32,
}

impl Default for Percentage {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}

impl Display for Percentage {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if self.value.fract() == 0.0 {
            write!(f, "{}%", self.value as i32)
        } else {
            write!(f, "{}%", self.value)
        }
    }
}

/// The anchor by percentages.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Anchor {
    /// The horizontal setting.
    pub x: Percentage,
    /// The vertical setting.
    pub y: Percentage,
}

impl Default for Anchor {
    fn default() -> Self {
        Self {
            x: Percentage { value: 0.0 },
            y: Percentage { value: 100.0 },
        }
    }
}

impl Display for Anchor {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

/// The scroll setting of region.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Scroll {
    /// The scroll up.
    Up,
}

impl Default for Scroll {
    fn default() -> Self {
        Self::Up
    }
}

impl Display for Scroll {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::Up => {
                write!(f, "up")
            },
        }
    }
}

/// The vertical setting of cue.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Vertical {
    /// From right to left.
    Rl,
    /// From left to right.
    Lr,
}

impl Default for Vertical {
    fn default() -> Self {
        Self::Rl
    }
}

impl Display for Vertical {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::Rl => {
                write!(f, "rl")
            },
            | Self::Lr => {
                write!(f, "lr")
            },
        }
    }
}

/// The line setting of cue.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Line {
    /// The percentage.
    Percentage(Percentage, Option<LineAlignment>),
    /// The line number.
    LineNumber(i32, Option<LineAlignment>),
}

impl Default for Line {
    fn default() -> Self {
        Self::Percentage(Percentage::default(), None)
    }
}

impl Display for Line {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::Percentage(percentage, align) => {
                if let Some(align) = align {
                    write!(f, "{},{}", percentage, align)
                } else {
                    write!(f, "{}", percentage)
                }
            },
            | Self::LineNumber(line_number, align) => {
                if let Some(align) = align {
                    write!(f, "{},{}", line_number, align)
                } else {
                    write!(f, "{}", line_number)
                }
            },
        }
    }
}

/// The alignment setting of line.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum LineAlignment {
    /// The start alignment.
    Start,
    /// The center alignment.
    Center,
    /// The end alignment.
    End,
}

impl Default for LineAlignment {
    fn default() -> Self {
        Self::Start
    }
}

impl Display for LineAlignment {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::Start => {
                write!(f, "start")
            },
            | Self::Center => {
                write!(f, "center")
            },
            | Self::End => {
                write!(f, "end")
            },
        }
    }
}

/// The position setting of cue.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Position {
    /// The position value.
    pub value: Percentage,
    /// The alignment setting.
    pub alignment: Option<PositionAlignment>,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            value: Percentage::default(),
            alignment: None,
        }
    }
}

impl Display for Position {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        if let Some(alignment) = self.alignment {
            write!(f, "{},{}", self.value, alignment)
        } else {
            write!(f, "{}", self.value)
        }
    }
}

/// The alignment setting of position.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum PositionAlignment {
    /// The line left alignment.
    LineLeft,
    /// The line center alignment.
    Center,
    /// The line right alignment.
    LineRight,
}

impl Default for PositionAlignment {
    fn default() -> Self {
        Self::LineLeft
    }
}

impl Display for PositionAlignment {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::LineLeft => {
                write!(f, "line-left")
            },
            | Self::Center => {
                write!(f, "center")
            },
            | Self::LineRight => {
                write!(f, "line-right")
            },
        }
    }
}

/// The alignment setting.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum Alignment {
    /// The start alignment.
    Start,
    /// The center alignment.
    Center,
    /// The end alignment.
    End,
    /// The left alignment.
    Left,
    /// The right alignment.
    Right,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Start
    }
}

impl Display for Alignment {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            | Self::Start => {
                write!(f, "start")
            },
            | Self::Center => {
                write!(f, "center")
            },
            | Self::End => {
                write!(f, "end")
            },
            | Self::Left => {
                write!(f, "left")
            },
            | Self::Right => {
                write!(f, "right")
            },
        }
    }
}
