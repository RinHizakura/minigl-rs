pub const OP_CLEAR_COLOR: usize = 1;

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

    fn op_clear_color(&self) {
        let p = unsafe { self.p[1].i };
        println!("p2 = {}", p);
    }

    pub fn add_param_u32(&mut self, p: u32) {
        self.p.push(MGLParam { i: p });
    }

    pub fn run_op(&self) {
        /* FIXME: We may want to pipeline our OP better instead of
         * executing it directly */
        let op = unsafe { self.p[0].op };
        match op {
            OP_CLEAR_COLOR => {
                self.op_clear_color();
            }

            _ => todo!(),
        }
    }
}
