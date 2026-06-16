mod banner_code;
mod color;
mod pattern;

use std::fmt::{self, Write};

pub use banner_code::BannerCode;
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
///
/// The alternate form (`{:#}`) instead lists every emitted character with its codepoint and meaning.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Optimized<'a>(pub &'a [Banner]);

/// One emitted glyph of an [`Optimized`] string.
#[derive(Clone, Copy)]
enum Glyph {
    Layer(Layer),
    /// Horizontal cursor move of `offset` banner widths (negative = backwards).
    Space(i32),
}

impl Glyph {
    fn char(&self) -> char {
        match *self {
            Glyph::Layer(layer) => layer.bannerfont_char(),
            Glyph::Space(offset) => bannerfont_space_char(offset),
        }
    }
}

impl fmt::Display for Glyph {
    /// Writes the font char, or under `{:#}` a `U+XXXX  meaning` explanation.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if !f.alternate() {
            return f.write_char(self.char());
        }

        let code = self.char() as u32;
        match self {
            Glyph::Layer(layer) => {
                write!(
                    f,
                    "U+{code:04X}  {} {}",
                    layer.color.name(),
                    layer.pattern.name()
                )
            }
            Glyph::Space(offset) => write!(f, "U+{code:04X}  space: {offset}"),
        }
    }
}

impl Optimized<'_> {
    /// The ordered glyphs that make up the rendered string.
    fn glyphs(&self) -> Vec<Glyph> {
        let banners = self.0;
        let n = banners.len();
        let mut glyphs = Vec::new();
        if n == 0 {
            return glyphs;
        }

        let max_layers = banners.iter().map(|b| b.layers.len()).max().unwrap_or(0);

        // Background row: each banner's first layer, or positive space if it has none.
        glyphs.extend(banners.iter().map(|b| match b.layers.first() {
            Some(&layer) => Glyph::Layer(layer),
            None => Glyph::Space(1),
        }));

        let mut cursor = n;
        for depth in 1..max_layers {
            let last_idx = banners
                .iter()
                .rposition(|b| b.layers.len() > depth)
                .unwrap();
            glyphs.push(Glyph::Space(-(cursor as i32)));
            glyphs.extend(
                banners[..=last_idx]
                    .iter()
                    .map(|b| match b.layers.get(depth) {
                        Some(&layer) => Glyph::Layer(layer),
                        None => Glyph::Space(1),
                    }),
            );
            cursor = last_idx + 1;
        }

        if cursor < n {
            glyphs.push(Glyph::Space((n - cursor) as i32));
        }

        let mut flattened: Vec<Glyph> = Vec::with_capacity(glyphs.len());
        for glyph in glyphs {
            match (flattened.last_mut(), glyph) {
                (Some(Glyph::Space(prev)), Glyph::Space(offset)) => *prev += offset,
                (Some(prev @ Glyph::Space(0)), glyph) => *prev = glyph,
                (_, glyph) => flattened.push(glyph),
            }
        }
        flattened
    }
}

impl fmt::Display for Optimized<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let glyphs = self.glyphs();

        if !f.alternate() {
            return glyphs.iter().try_for_each(|g| write!(f, "{g}"));
        }

        for (i, glyph) in glyphs.iter().enumerate() {
            if i > 0 {
                f.write_char('\n')?;
            }
            write!(f, "{glyph:#}")?;
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

#[cfg(test)]
mod tests {
    use super::*;
    use Color::*;
    use Pattern::*;
    use test_case::test_case;

    fn banner(layers: &[(Pattern, Color)]) -> Banner {
        Banner::new(layers.iter().map(|&(p, c)| Layer::new(p, c)).collect())
    }

    #[test_case(vec![] => ""; "empty")]
    #[test_case(vec![banner(&[(Base, White)])] => "\u{E000}"; "single base")]
    #[test_case(vec![banner(&[(Base, White), (Flower, Black)])] => "\u{E000}\u{F03F}\u{E311}"; "overlay backtracks one width")]
    #[test_case(vec![banner(&[(Base, White)]), banner(&[(Base, Red)])] => "\u{E000}\u{E600}"; "multiple bases")]
    #[test_case(
        vec![
            banner(&[(Base, Green)]),
            banner(&[(Base, Green)]),
            banner(&[(Base, White), (StripeCenter, Orange)]),
        ] => "\u{E900}\u{E900}\u{E000}\u{F03F}\u{E52A}";
        "flattens consecutive spaces"
    )]
    fn test_optimized_display(banners: Vec<Banner>) -> String {
        Optimized(&banners).to_string()
    }

    #[test]
    fn test_optimized_alternate_display() {
        let banners = [
            banner(&[(Base, Green)]),
            banner(&[(Base, Green)]),
            banner(&[(Base, White), (StripeCenter, Orange)]),
        ];
        let expected = "\
U+E900  Green Base
U+E900  Green Base
U+E000  White Base
U+F03F  space: -1
U+E52A  Orange Pale";
        assert_eq!(format!("{:#}", Optimized(&banners)), expected);
    }
}
