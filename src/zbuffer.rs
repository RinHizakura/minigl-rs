pub struct ZBuffer {
    xsize: usize,
    ysize: usize,

    zbuf: Vec<u16>,
    pub pbuf: Vec<u32>,
}

impl ZBuffer {
    pub fn new(xsize: usize, ysize: usize) -> Self {
        let size = xsize * ysize;
        /* FIXME: Is this a good way to allocate those buffer? */
        let zbuf = vec![0; size];
        let pbuf = vec![0; size];

        ZBuffer {
            xsize: xsize,
            ysize: ysize,
            zbuf: zbuf,
            pbuf: pbuf,
        }
    }

    pub fn clear(&mut self, do_color: bool, do_depth: bool, argb: u32) {
        if do_depth {
            todo!();
        }

        println!("argb {:x}", argb);
        if do_color {
            let pbuf = &mut self.pbuf;
            pbuf[0..(self.xsize * self.ysize)].fill(argb);
        }
    }
}
