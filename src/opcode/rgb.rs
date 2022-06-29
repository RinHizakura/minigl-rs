#[macro_export]
macro_rules! RGB_TO_PIXEL {
    ($r:expr, $g:expr, $b:expr) => {
        (($r as u32 & 0xff) << 16) | (($g as u32 & 0xff) << 8) | ($b as u32 & 0xff)
    };
}
