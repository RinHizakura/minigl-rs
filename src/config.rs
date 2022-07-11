pub const PIXEL_BYTES: usize = 4;

bitflags! {
    pub struct MGLBit: usize {
        const COLOR     = 0b00000001;
        const DEPTH     = 0b00000010;
    }
}

bitflags! {
    pub struct MGLColor: usize {
        const RED      = 0xff0000;
    }
}

pub enum MGLTextSize {
    TextSize8X8,
    TextSize32x32,
    TextSizeMAX,
}

impl MGLTextSize {
    pub fn sz(&self) -> usize {
        match self {
            MGLTextSize::TextSize8X8 => 1,
            MGLTextSize::TextSize32x32 => 4,
            MGLTextSize::TextSizeMAX => 16,
        }
    }
}

pub enum MGLMatrixMode {
    ModeModelView,
    ModeProjection,
    ModeTexture,
}

impl MGLMatrixMode {
    pub fn idx(&self) -> usize {
        match self {
            MGLMatrixMode::ModeModelView => 0,
            MGLMatrixMode::ModeProjection => 1,
            MGLMatrixMode::ModeTexture => 2,
        }
    }
}

#[repr(u8)]
pub enum MGLVertexMode {
    ModeNone = 0, // this is used to represent when we are not in the begin scope
    ModeTriangles = 1,
}

impl MGLVertexMode {
    pub fn idx(&self) -> usize {
        match self {
            MGLVertexMode::ModeNone => 0,
            MGLVertexMode::ModeTriangles => 1,
        }
    }
}

impl From<usize> for MGLVertexMode {
    fn from(v: usize) -> Self {
        match v {
            1 => return MGLVertexMode::ModeTriangles,
            _ => return MGLVertexMode::ModeNone,
        };
    }
}
