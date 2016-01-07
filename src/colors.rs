/// A simple ARGB color representation
#[derive(Debug, Copy, Clone)]
pub struct Color(pub u8, pub u8, pub u8, pub u8);

impl Color {
    pub fn from_rbg(r: u8, g: u8, b: u8) -> Self {
        Color(0, r, g, b)
    }

    pub fn from_int(i: u32) -> Self {
        let a = ((i >> 24) & 255) as u8;
        let r = ((i >> 16) & 255) as u8;
        let g = ((i >> 8) & 255) as u8;
        let b = (i & 255) as u8;
        Color(a, r, g, b)
    }

    pub fn to_int(&self) -> u32 {
        let &Color(a, r, g, b) = self;

        (b as u32)        |
        (g as u32) << 8   |
        (r as u32) << 16  |
        (a as u32) << 24
    }
}

pub const WHITE: Color      = Color(0,      255,    255,    255);
pub const BLACK: Color      = Color(0,      0,      0,      0);
pub const RED: Color        = Color(0,      255,    0,      0);
pub const GREEN: Color      = Color(0,      0,      255,    0);
pub const BLUE: Color       = Color(0,      0,      0,      255);
pub const DARK_GRAY: Color  = Color(0,      50,     50,     50);
pub const GRAY: Color       = Color(0,      100,    100,    100);
pub const LIGHT_GRAY: Color = Color(0,      200,    200,    200);
// TODO: more colors?
