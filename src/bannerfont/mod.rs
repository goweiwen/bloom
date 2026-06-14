mod color;
mod pattern;

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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
