pub const PIXEL_BYTES: usize = 4;

bitflags! {
    pub struct MGLBit: u32 {
        const COLOR     = 0b00000001;
        const DEPTH     = 0b00000010;
    }
}
