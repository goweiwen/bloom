use num_enum::TryFromPrimitive;
use strum_macros::{EnumCount, EnumIter};

use super::Error;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, TryFromPrimitive, EnumCount, EnumIter)]
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

    pub fn name(self) -> &'static str {
        use Color::*;
        match self {
            White => "White",
            LightGray => "Light Gray",
            Gray => "Gray",
            Black => "Black",
            Yellow => "Yellow",
            Orange => "Orange",
            Red => "Red",
            Brown => "Brown",
            Lime => "Lime",
            Green => "Green",
            LightBlue => "Light Blue",
            Cyan => "Cyan",
            Blue => "Blue",
            Pink => "Pink",
            Magenta => "Magenta",
            Purple => "Purple",
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
