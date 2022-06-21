use crate::config::MGLBit;
use crate::err::MGLError;
use crate::mgl;

type Result<T> = std::result::Result<T, MGLError>;

macro_rules! RGB_TO_PIXEL {
    ($r:expr, $g:expr, $b:expr) => {
        (($r as u32 & 0xff) << 16) | (($g as u32 & 0xff) << 8) | ($b as u32 & 0xff)
    };
}

pub const OP_CLEAR: usize = 1;
pub const OP_PLOT_PIXEL: usize = 2;

union MGLParam {
    pub op: usize,
    pub u: usize,
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

    fn op_clear(&self) -> Result<()> {
        let ctx = mgl::ctx()?;
        let p = unsafe { self.p[1].u };

        /* FIXME: let user declares their clear color instead of using the default */
        let argb = RGB_TO_PIXEL!(ctx.clear_color.y, ctx.clear_color.z, ctx.clear_color.w);
        ctx.zb.clear(
            (p & MGLBit::COLOR.bits()) != 0,
            (p & MGLBit::DEPTH.bits()) != 0,
            argb,
        );

        Ok(())
    }

    fn op_plot_pixel(&self) -> Result<()> {
        let ctx = mgl::ctx()?;
        let x = unsafe { self.p[1].u };
        let argb = unsafe { self.p[2].u } as u32;

        ctx.zb.set(x, argb);

        Ok(())
    }

    pub fn add_param_u(&mut self, u: usize) {
        self.p.push(MGLParam { u: u });
    }

    pub fn run_op(&self) -> Result<()> {
        /* FIXME: We may want to pipeline our OP better instead of
         * executing it directly */
        let op = unsafe { self.p[0].op };
        match op {
            OP_CLEAR => {
                self.op_clear()?;
            }
            OP_PLOT_PIXEL => {
                self.op_plot_pixel()?;
            }
            _ => todo!(),
        }

        Ok(())
    }
}
