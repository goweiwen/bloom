mod color;
mod pattern;

use std::fmt::{self, Write};

pub use color::Color;
pub use pattern::Pattern;

/// Horizontal-space glyph advancing `offset` banner widths (negative = backwards).
const fn bannerfont_space_char(offset: i32) -> char {
    char::from_u32((0xF040_i32 + offset) as u32).unwrap()
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid pattern code: {0}")]
    InvalidPattern(String),
    #[error("invalid color code: {0}")]
    InvalidColor(String),
    #[error("unexpected end of code")]
    UnexpectedEndOfCode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Layer {
    pub pattern: Pattern,
    pub color: Color,
}

impl Layer {
    pub fn new(pattern: Pattern, color: Color) -> Self {
        Self { pattern, color }
    }

    /// The banner-font glyph for this layer:
    /// `\u{Exyy}` where `x` is the color and `yy` the pattern's BCD index.
    fn bannerfont_char(self) -> char {
        char::from_u32(0xE000 + ((self.color as u32) << 8) + self.pattern.bcd() as u32).unwrap()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Banner {
    pub layers: Vec<Layer>,
}

impl Banner {
    pub fn new(layers: Vec<Layer>) -> Self {
        Self { layers }
    }

    pub fn base_color(&self) -> Option<Color> {
        let layer = self.layers.first()?;
        if layer.pattern == Pattern::Base {
            Some(layer.color)
        } else {
            None
        }
    }

    pub fn code(&self) -> String {
        self.layers
            .iter()
            .flat_map(|layer| [layer.pattern.code(), layer.color.code()])
            .collect()
    }

    pub fn try_from_code(mut code: &str) -> Result<Self, Error> {
        let mut layers = Vec::new();

        while !code.is_empty() {
            let pattern_end = code
                .find(|c: char| !c.is_ascii_lowercase())
                .unwrap_or(code.len());
            let pattern = &code[..pattern_end];
            if pattern.is_empty() {
                return Err(Error::UnexpectedEndOfCode);
            }
            let pattern = Pattern::try_from_code(pattern)?;
            code = &code[pattern_end..];

            let color_end = code
                .find(|c: char| !c.is_ascii_digit())
                .unwrap_or(code.len());
            let color = &code[..color_end];
            if color.is_empty() {
                return Err(Error::UnexpectedEndOfCode);
            }
            let color = Color::try_from_code(color)?;
            code = &code[color_end..];

            layers.push(Layer::new(pattern, color));
        }

        Ok(Self::new(layers))
    }
}

/// Renders a row of banners as the optimized banner-font string for Minecraft: every banner's first
/// layer, and each layer overlaid in place using negative-space chars to move the cursor back.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Optimized<'a>(pub &'a [Banner]);

impl fmt::Display for Optimized<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let banners = self.0;
        let n = banners.len();
        if n == 0 {
            return Ok(());
        }

        let max_layers = banners.iter().map(|b| b.layers.len()).max().unwrap_or(0);

        banners.iter().try_for_each(|b| {
            f.write_char(
                b.layers
                    .first()
                    .map_or(bannerfont_space_char(1), |l| l.bannerfont_char()),
            )
        })?;

        let cursor = (1..max_layers).try_fold(n, |cursor, depth| {
            let last_idx = banners
                .iter()
                .rposition(|b| b.layers.len() > depth)
                .unwrap();
            f.write_char(bannerfont_space_char(-(cursor as i32)))?;
            banners[..=last_idx].iter().try_for_each(|b| {
                f.write_char(
                    b.layers
                        .get(depth)
                        .map_or(bannerfont_space_char(1), |l| l.bannerfont_char()),
                )
            })?;
            Ok(last_idx + 1)
        })?;

        if cursor < n {
            f.write_char(bannerfont_space_char((n - cursor) as i32))?;
        }

        Ok(())
    }
}

#[derive(
    Default, Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum WritingDirection {
    #[default]
    LeftToRight,
    RightToLeft,
}

impl WritingDirection {
    pub fn class(self) -> &'static str {
        match self {
            WritingDirection::LeftToRight => "ltr",
            WritingDirection::RightToLeft => "rtl",
        }
    }
}
mod tests {
    use super::*;

    const NETHER_PORTAL: &str = "b10ss2bri10cbo2bo15";

    #[test]
    fn test_banner_code() {
        let code = NETHER_PORTAL;
        let banner = Banner::try_from_code(code).unwrap();
        let new_code = banner.code();
        assert_eq!(code, new_code);
    }
}
