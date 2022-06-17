use crate::config::MGLBit;
use crate::mgl;

macro_rules! RGB_TO_PIXEL {
    ($r:expr, $g:expr, $b:expr) => {
        (($r as u32 & 0xff) << 16) | (($g as u32 & 0xff) << 8) | ($b as u32 & 0xff)
    };
}

pub const OP_CLEAR: usize = 1;

union MGLParam {
    pub op: usize,
    pub i: u32,
    pub f: f32,
}

pub struct MGLOp {
    p: Vec<MGLParam>,
}

impl MGLOp {
    pub fn new(op: usize) -> Self {
        let mut v = Vec::new();
        v.push(MGLParam { op: op });
        MGLOp { p: v }
    }

    fn op_clear(&self) {
        let ctx = mgl::ctx();
        let p = unsafe { self.p[1].i };

        /* FIXME: let user declares their clear color instead of using the default */
        let argb = RGB_TO_PIXEL!(ctx.clear_color.y, ctx.clear_color.z, ctx.clear_color.w);
        ctx.zb.clear(
            (p & MGLBit::COLOR.bits()) != 0,
            (p & MGLBit::DEPTH.bits()) != 0,
            argb,
        );
    }

    pub fn add_param_u32(&mut self, p: u32) {
        self.p.push(MGLParam { i: p });
    }

    pub fn run_op(&self) {
        /* FIXME: We may want to pipeline our OP better instead of
         * executing it directly */
        let op = unsafe { self.p[0].op };
        match op {
            OP_CLEAR => {
                self.op_clear();
            }

            _ => todo!(),
        }
    }
}
