use hex::ToHex;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Color {
    /// The original name of the color.
    /// Is either the name of a color, like "red", or the Hex Color code for that color
    pub name: String,
    /// Represents a Hex color value
    /// contains values for (red, green, blue).
    pub code: (u8, u8, u8),
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Color {
            name: [r, g, b].encode_hex_upper::<String>(),
            code: (r, g, b),
        }
    }
}

/// Convenience Enum for making named colors
pub enum ColorName {
    Purple,
    Orange,
    Pink,
    Red,
    Green,
    Blue,
    Yellow,
    White,
    Black,
}

/// Converts from a `ColorName` enum to the corresponding `Color`
impl From<ColorName> for Color {
    fn from(cn: ColorName) -> Self {
        match cn {
            ColorName::Purple => Color {
                name: "purple".to_string(),
                code: (128, 0, 128),
            },
            ColorName::Orange => Color {
                name: "orange".to_string(),
                code: (255, 165, 0),
            },
            ColorName::Pink => Color {
                name: "pink".to_string(),
                code: (255, 192, 203),
            },
            ColorName::Red => Color {
                name: "red".to_string(),
                code: (255, 0, 0),
            },
            ColorName::Green => Color {
                name: "green".to_string(),
                code: (0, 255, 0),
            },
            ColorName::Blue => Color {
                name: "blue".to_string(),
                code: (0, 0, 255),
            },
            ColorName::Yellow => Color {
                name: "yellow".to_string(),
                code: (255, 255, 0),
            },
            ColorName::White => Color {
                name: "white".to_string(),
                code: (255, 255, 255),
            },
            ColorName::Black => Color {
                name: "black".to_string(),
                code: (0, 0, 0),
            },
        }
    }
}
