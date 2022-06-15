use crate::opcode;
use crate::opcode::MGLOp;
use crate::zbuffer::ZBuffer;
use std::sync::Once;

struct MGLContext {
    zb: ZBuffer,
}

impl MGLContext {
    pub fn new(zb: ZBuffer) -> Self {
        MGLContext { zb: zb }
    }
}

static mut MGL_CONTEXT: Option<MGLContext> = None;
static MGL_CONTEXT_SET: Once = Once::new();

pub fn init(zb: ZBuffer) {
    MGL_CONTEXT_SET.call_once(|| {
        let ctx = MGLContext::new(zb);
        unsafe {
            MGL_CONTEXT.get_or_insert(ctx);
        }
    });
}

pub fn clear(mask: u32) {
    let mut op = MGLOp::new(opcode::OP_CLEAR_COLOR);
    op.add_param_u32(mask);
    op.run_op();
}
