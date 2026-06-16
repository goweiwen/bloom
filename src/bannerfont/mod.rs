mod banner_code;
mod banner_font;
mod color;
mod pattern;

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
}

/// The compact binary encoding of a banner: two bytes per layer — the pattern's
/// discriminant followed by the color's. Stored in IndexedDB as a `Uint8Array`;
/// the inverse is [`TryFrom<&[u8]>`](Banner).
impl From<&Banner> for Vec<u8> {
    fn from(banner: &Banner) -> Vec<u8> {
        banner
            .layers
            .iter()
            .flat_map(|layer| [layer.pattern as u8, layer.color as u8])
            .collect()
    }
}

impl TryFrom<&[u8]> for Banner {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self, Error> {
        if !bytes.len().is_multiple_of(2) {
            return Err(Error::UnexpectedEndOfCode);
        }
        let layers = bytes
            .chunks_exact(2)
            .map(|pair| {
                let pattern = Pattern::try_from(pair[0])
                    .map_err(|_| Error::InvalidPattern(pair[0].to_string()))?;
                let color = Color::try_from(pair[1])
                    .map_err(|_| Error::InvalidColor(pair[1].to_string()))?;
                Ok(Layer::new(pattern, color))
            })
            .collect::<Result<Vec<_>, Error>>()?;

        Ok(Banner::new(layers))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn banner_bytes_roundtrips() {
        let banner = Banner::new(vec![
            Layer::new(Pattern::Base, Color::White),
            Layer::new(Pattern::Creeper, Color::Black),
            Layer::new(Pattern::Guster, Color::Purple),
        ]);
        let bytes = Vec::<u8>::from(&banner);
        assert_eq!(bytes, vec![0, 0, 4, 3, 42, 15]);
        assert_eq!(Banner::try_from(bytes.as_slice()).unwrap(), banner);
    }

    #[test]
    fn banner_bytes_rejects_odd_length() {
        assert!(Banner::try_from([0, 0, 4].as_slice()).is_err());
    }

    #[test]
    fn banner_bytes_rejects_out_of_range() {
        assert!(Banner::try_from([43, 0].as_slice()).is_err());
        assert!(Banner::try_from([0, 16].as_slice()).is_err());
    }
}
