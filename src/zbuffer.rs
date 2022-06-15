use crate::config::PIXEL_BYTES;
use crate::err::MglError;

pub struct ZBuffer {
    xsize: usize,
    ysize: usize,

    zbuf: Vec<u16>,
    pbuf: Vec<u32>,
}

impl ZBuffer {
    pub fn new(xsize: usize, ysize: usize) -> Self {
        let size = xsize * ysize * 2;
        let zbuf = Vec::with_capacity(size);
        let pbuf = Vec::with_capacity(ysize * xsize * PIXEL_BYTES);

        ZBuffer {
            xsize: xsize,
            ysize: ysize,
            zbuf: zbuf,
            pbuf: pbuf,
        }
    }
}
