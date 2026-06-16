mod banner_code;
mod banner_font;
mod color;
mod pattern;

pub use banner_code::BannerCode;
pub use banner_font::OptimizedBannerFont;
pub use color::Color;
pub use pattern::Pattern;

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

    /// Parse the compact binary encoding produced by [`BannerBytes`]: two bytes
    /// per layer, the pattern's discriminant then the color's.
    pub fn try_from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        if bytes.len() % 2 != 0 {
            return Err(Error::UnexpectedEndOfCode);
        }
        bytes
            .chunks_exact(2)
            .map(|pair| {
                let pattern = Pattern::try_from(pair[0])
                    .map_err(|_| Error::InvalidPattern(pair[0].to_string()))?;
                let color = Color::try_from(pair[1])
                    .map_err(|_| Error::InvalidColor(pair[1].to_string()))?;
                Ok(Layer::new(pattern, color))
            })
            .collect::<Result<Vec<_>, _>>()
            .map(Self::new)
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
