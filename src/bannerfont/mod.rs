use num_enum::TryFromPrimitive;
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
}
