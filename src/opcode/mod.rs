#[macro_use]
mod rgb;
mod clear;
mod matrix;
mod plot;

use crate::err::MGLError;

type Result<T> = std::result::Result<T, MGLError>;

pub const OP_CLEAR: usize = 1;
pub const OP_PLOT_PIXEL: usize = 2;
pub const OP_MATRIX_MODE: usize = 3;
pub const OP_LOAD_IDENTITY: usize = 4;
pub const OP_PUSH_MATRIX: usize = 5;

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

    pub fn add_param_u(&mut self, u: usize) {
        self.p.push(MGLParam { u: u });
    }

    pub fn run_op(&self) -> Result<()> {
        /* FIXME: We may want to pipeline our OP better instead of
         * executing it directly */
        let op = unsafe { self.p[0].op };
        match op {
            OP_CLEAR => {
                let p = unsafe { self.p[1].u };
                clear::op_clear(p)?
            }
            OP_PLOT_PIXEL => {
                let x = unsafe { self.p[1].u };
                let argb = unsafe { self.p[2].u } as u32;
                plot::op_plot_pixel(x, argb)?
            }
            OP_MATRIX_MODE => {
                let mode = unsafe { self.p[1].u } as u8;
                matrix::op_matrix_mode(mode)?
            }
            OP_LOAD_IDENTITY => matrix::op_load_identity()?,
            OP_PUSH_MATRIX => matrix::op_push_matrix()?,
            _ => todo!(),
        }

        Ok(())
    }
}
