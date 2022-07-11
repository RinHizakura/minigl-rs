#[macro_use]
mod rgb;
mod clear;
mod matrix;
mod plot;
mod vertex;

use crate::err::MGLError;

type Result<T> = std::result::Result<T, MGLError>;

pub const OP_CLEAR: usize = 1;
pub const OP_PLOT_PIXEL: usize = 2;
pub const OP_MATRIX_MODE: usize = 3;
pub const OP_LOAD_IDENTITY: usize = 4;
pub const OP_PUSH_MATRIX: usize = 5;
pub const OP_ROTATE: usize = 6;
pub const OP_BEGIN: usize = 7;
pub const OP_COLOR: usize = 8;
pub const OP_VERTEX: usize = 9;

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

    pub fn add_param_f(&mut self, f: f32) {
        self.p.push(MGLParam { f: f });
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
            OP_ROTATE => {
                let angle = unsafe { self.p[1].f };
                let x = unsafe { self.p[2].f };
                let y = unsafe { self.p[3].f };
                let z = unsafe { self.p[4].f };
                matrix::op_rotate(angle, x, y, z)?
            }
            OP_BEGIN => {
                let t = unsafe { self.p[1].u };
                vertex::op_begin(t)?;
            }
            OP_COLOR => {
                let a = unsafe { self.p[1].u } as u8;
                let r = unsafe { self.p[2].u } as u8;
                let g = unsafe { self.p[3].u } as u8;
                let b = unsafe { self.p[4].u } as u8;
                vertex::op_color(a, r, g, b)?;
            }
            OP_VERTEX => {
                let x = unsafe { self.p[1].f };
                let y = unsafe { self.p[2].f };
                let z = unsafe { self.p[3].f };
                let w = unsafe { self.p[4].f };
                vertex::op_vertex(x, y, z, w)?;
            }
            _ => todo!(),
        }

        Ok(())
    }
}
