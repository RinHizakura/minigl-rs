pub const PIXEL_BYTES: usize = 4;

bitflags! {
    pub struct MGLBit: u32 {
        const COLOR     = 0b00000001;
        const DEPTH     = 0b00000010;
    }
}

bitflags! {
    pub struct MGLColor: u32 {
        const RED      = 0xff0000;
    }
}

pub enum MGLTextSize {
    TextSize8X8,
}

impl MGLTextSize {
    pub fn sz(&self) -> usize {
        match self {
            MGLTextSize::TextSize8X8 => 1,
        }
    }
}
