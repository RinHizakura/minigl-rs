use crate::config::*;
use crate::err::MGLError;
use crate::opcode;
use crate::opcode::MGLOp;
use crate::zbuffer::ZBuffer;
use std::sync::Once;

type Result<T> = std::result::Result<T, MGLError>;

/* Let's use cgmath for the vector operation instead of
 * reinventing the wheel. If we want to learn about the
 * implementation of cgmath(e.g. SIMD optimization), just
 * dive into the source code! */
use cgmath::*;

pub struct MGLContext {
    pub zb: ZBuffer,
    pub clear_color: Vector4<u8>,
    pub textsize: MGLTextSize,
}

impl MGLContext {
    pub fn new(zb: ZBuffer) -> Self {
        MGLContext {
            zb: zb,
            clear_color: Vector4 {
                x: 0xff, // a
                y: 0xff, // r
                z: 0x00, // g
                w: 0x00, // b
            },
            textsize: MGLTextSize::TextSize8X8,
        }
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

pub fn ctx() -> Result<&'static mut MGLContext> {
    unsafe {
        match &mut MGL_CONTEXT {
            Some(x) => Ok(x),
            None => Err(MGLError::EFAULT),
        }
    }
}

pub fn pbuffer() -> Result<Vec<u8>> {
    unsafe {
        match &mut MGL_CONTEXT {
            /* FIXME: Will this be too inefficient? */
            Some(x) => Ok(x
                .zb
                .get_pbuf()
                .iter()
                .flat_map(|val| val.to_be_bytes())
                .collect()),
            None => Err(MGLError::EFAULT),
        }
    }
}

pub fn clear(mask: MGLBit) -> Result<()> {
    let mut op = MGLOp::new(opcode::OP_CLEAR);
    op.add_param_u32(mask.bits() as u32);
    op.run_op()?;

    Ok(())
}

pub fn draw_text(text: &str, xbase: usize, ybase: usize, color: MGLColor) -> Result<()> {
    let (w, h);
    unsafe {
        (w, h) = match &MGL_CONTEXT {
            Some(x) => (x.zb.xsize, x.zb.ysize),
            None => return Err(MGLError::EFAULT),
        };
    }

    Ok(())
}
