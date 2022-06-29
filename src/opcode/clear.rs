use crate::config::MGLBit;
use crate::err::MGLError;
use crate::mgl;

type Result<T> = std::result::Result<T, MGLError>;

pub fn op_clear(p: usize) -> Result<()> {
    let ctx = mgl::ctx()?;

    /* FIXME: let user declares their clear color instead of using the default */
    let argb = RGB_TO_PIXEL!(ctx.clear_color.y, ctx.clear_color.z, ctx.clear_color.w);
    ctx.zb.clear(
        (p & MGLBit::COLOR.bits()) != 0,
        (p & MGLBit::DEPTH.bits()) != 0,
        argb,
    );

    Ok(())
}
