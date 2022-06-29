use crate::err::MGLError;
use crate::mgl;

type Result<T> = std::result::Result<T, MGLError>;

pub fn op_plot_pixel(x: usize, argb: u32) -> Result<()> {
    let ctx = mgl::ctx()?;
    ctx.zb.set(x, argb);

    Ok(())
}
