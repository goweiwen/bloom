use num_enum::TryFromPrimitive;
use std::str::FromStr;
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
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
pub enum Color {
    White = 0,
    LightGray,
    Gray,
    Black,
    Yellow,
    Orange,
    Red,
    Brown,
    Lime,
    Green,
    LightBlue,
    Cyan,
    Blue,
    Pink,
    Magenta,
    Purple,
}

impl Color {
    pub fn rgb(self) -> (u8, u8, u8) {
        use Color::*;
        match self {
            White => (249, 255, 254),
            LightGray => (157, 157, 151),
            Gray => (71, 79, 82),
            Black => (29, 29, 33),
            Yellow => (254, 216, 61),
            Orange => (249, 128, 29),
            Red => (176, 46, 38),
            Brown => (131, 84, 50),
            Lime => (128, 199, 31),
            Green => (94, 124, 22),
            LightBlue => (58, 179, 218),
            Cyan => (22, 156, 156),
            Blue => (60, 68, 170),
            Pink => (243, 139, 170),
            Magenta => (199, 78, 189),
            Purple => (137, 50, 184),
        }
    }
    pub fn code(self) -> &'static str {
        use Color::*;
        match self {
            White => "0",
            LightGray => "8",
            Gray => "7",
            Black => "15",
            Yellow => "4",
            Orange => "1",
            Red => "14",
            Brown => "12",
            Lime => "5",
            Green => "13",
            LightBlue => "3",
            Cyan => "9",
            Blue => "11",
            Pink => "6",
            Magenta => "2",
            Purple => "10",
        }
    }

    pub fn try_from_code(code: &str) -> Result<Self, Error> {
        use Color::*;
        Ok(match code {
            "0" => White,
            "8" => LightGray,
            "7" => Gray,
            "15" => Black,
            "4" => Yellow,
            "1" => Orange,
            "14" => Red,
            "12" => Brown,
            "5" => Lime,
            "13" => Green,
            "3" => LightBlue,
            "9" => Cyan,
            "11" => Blue,
            "6" => Pink,
            "2" => Magenta,
            "10" => Purple,
            _ => return Err(Error::InvalidColor(code.to_owned())),
        })
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive)]
pub enum Pattern {
    Base = 0,
    Border,
    Bricks,
    Circle,
    Creeper,
    Cross,
    CurlyBorder,
    DiagonalLeft,
    DiagonalUpRight,
    DiagonalUpLeft,
    DiagonalRight,
    Flower,
    Globe,
    Gradient,
    GradientUp,
    HalfHorizontal,
    HalfHorizontalBottom,
    HalfVertical,
    HalfVerticalRight,
    Mojang,
    Piglin,
    Rhombus,
    Skull,
    SmallStripes,
    SquareBottomLeft,
    SquareBottomRight,
    SquareTopLeft,
    SquareTopRight,
    StraightCross,
    StripeBottom,
    StripeCenter,
    StripeDownLeft,
    StripeDownRight,
    StripeLeft,
    StripeMiddle,
    StripeRight,
    StripeTop,
    TriangleBottom,
    TriangleTop,
    TrianglesBottom,
    TrianglesTop,
    Flow,
    Guster,
}

impl Pattern {
    pub fn code(self) -> &'static str {
        use Pattern::*;
        match self {
            Base => "b",
            Border => "bo",
            Bricks => "bri",
            Circle => "mc",
            Creeper => "cre",
            Cross => "cr",
            CurlyBorder => "cbo",
            DiagonalLeft => "ld",
            DiagonalUpRight => "rud",
            DiagonalUpLeft => "lud",
            DiagonalRight => "rd",
            Flower => "flo",
            Globe => "glb",
            Gradient => "gra",
            GradientUp => "gru",
            HalfHorizontal => "hh",
            HalfHorizontalBottom => "hhb",
            HalfVertical => "vh",
            HalfVerticalRight => "vhr",
            Mojang => "moj",
            Piglin => "pig",
            Rhombus => "mr",
            Skull => "sku",
            SmallStripes => "ss",
            SquareBottomLeft => "bl",
            SquareBottomRight => "br",
            SquareTopLeft => "tl",
            SquareTopRight => "tr",
            StraightCross => "sc",
            StripeBottom => "bs",
            StripeCenter => "cs",
            StripeDownLeft => "dls",
            StripeDownRight => "drs",
            StripeLeft => "ls",
            StripeMiddle => "ms",
            StripeRight => "rs",
            StripeTop => "ts",
            TriangleBottom => "bt",
            TriangleTop => "tt",
            TrianglesBottom => "bts",
            TrianglesTop => "tts",
            Flow => "flw",
            Guster => "gus",
        }
    }

    pub fn try_from_code(code: &str) -> Result<Self, Error> {
        use Pattern::*;
        Ok(match code {
            "b" => Base,
            "bo" => Border,
            "bri" => Bricks,
            "mc" => Circle,
            "cre" => Creeper,
            "cr" => Cross,
            "cbo" => CurlyBorder,
            "ld" => DiagonalLeft,
            "rud" => DiagonalUpRight,
            "lud" => DiagonalUpLeft,
            "rd" => DiagonalRight,
            "flo" => Flower,
            "glb" => Globe,
            "gra" => Gradient,
            "gru" => GradientUp,
            "hh" => HalfHorizontal,
            "hhb" => HalfHorizontalBottom,
            "vh" => HalfVertical,
            "vhr" => HalfVerticalRight,
            "moj" => Mojang,
            "pig" => Piglin,
            "mr" => Rhombus,
            "sku" => Skull,
            "ss" => SmallStripes,
            "bl" => SquareBottomLeft,
            "br" => SquareBottomRight,
            "tl" => SquareTopLeft,
            "tr" => SquareTopRight,
            "sc" => StraightCross,
            "bs" => StripeBottom,
            "cs" => StripeCenter,
            "dls" => StripeDownLeft,
            "drs" => StripeDownRight,
            "ls" => StripeLeft,
            "ms" => StripeMiddle,
            "rs" => StripeRight,
            "ts" => StripeTop,
            "bt" => TriangleBottom,
            "tt" => TriangleTop,
            "bts" => TrianglesBottom,
            "tts" => TrianglesTop,
            "flw" => Flow,
            "gus" => Guster,
            _ => return Err(Error::InvalidPattern(code.to_owned())),
        })
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
