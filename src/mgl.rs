use crate::config::*;
use crate::err::MGLError;
use crate::font8x8::*;
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
                x: 0x00, // a
                y: 0x00, // r
                z: 0x00, // g
                w: 0x00, // b
            },
            textsize: MGLTextSize::TextSize32x32,
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
    op.add_param_u(mask.bits());
    op.run_op()?;

    Ok(())
}

pub fn plot_pixel(x: usize, y: usize, color: MGLColor) -> Result<()> {
    let (w, h);
    unsafe {
        match &MGL_CONTEXT {
            Some(x) => {
                w = x.zb.xsize;
                h = x.zb.ysize;
            }
            None => return Err(MGLError::EFAULT),
        };
    }

    if x < w && y < h {
        let mut op = MGLOp::new(opcode::OP_PLOT_PIXEL);
        op.add_param_u(x + y * w);
        op.add_param_u(color.bits());
        op.run_op()?;
    }

    Ok(())
}

fn renderchar(
    ch_bitmap: &[u8],
    xoff: usize,
    yoff: usize,
    mult: usize,
    color: MGLColor,
) -> Result<()> {
    for x in 0..8 {
        for y in 0..8 {
            let set = ch_bitmap[y] & (1 << x);
            if set != 0 {
                for i in 0..mult {
                    for j in 0..mult {
                        plot_pixel(xoff + i + x * mult, yoff + j + y * mult, color)?;
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn draw_text(text: &str, xbase: usize, ybase: usize, color: MGLColor) -> Result<()> {
    let (w, h, mult);
    unsafe {
        match &MGL_CONTEXT {
            Some(x) => {
                w = x.zb.xsize;
                h = x.zb.ysize;
                mult = x.textsize.sz()
            }
            None => return Err(MGLError::EFAULT),
        };
    }

    let mut xoff = xbase;
    let mut yoff = ybase;
    let s: Vec<u8> = text.as_bytes().to_vec();
    for c in s {
        if yoff >= h {
            break;
        }

        if c == b'\n' {
            xoff = 0;
            yoff += 8 * mult;
        } else {
            if xoff < w {
                renderchar(&FONT8X8_BASIC[c as usize], xoff, yoff, mult, color)?;
                xoff += 8 * mult;
            }
        }
    }

    Ok(())
}
