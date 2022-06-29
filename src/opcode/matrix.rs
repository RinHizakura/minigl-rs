use crate::err::MGLError;
use crate::mgl;
use cgmath::*;

type Result<T> = std::result::Result<T, MGLError>;

pub fn op_matrix_mode(mode: u8) -> Result<()> {
    let ctx = mgl::ctx()?;

    ctx.matrix_mode = mode;

    Ok(())
}

pub fn op_load_identity() -> Result<()> {
    let ctx = mgl::ctx()?;
    let mode = ctx.matrix_mode as usize;

    let _m = ctx.matrix_stack[mode].pop().ok_or(MGLError::EINVALID)?;
    ctx.matrix_stack[mode].push(Matrix4::identity());

    ctx.matrix_model_projection_updated = ctx.matrix_mode <= 1;
    Ok(())
}

pub fn op_push_matrix() -> Result<()> {
    let ctx = mgl::ctx()?;
    let mode = ctx.matrix_mode as usize;
    let stack = &mut ctx.matrix_stack[mode];
    let m = stack.top().ok_or(MGLError::EINVALID)?;
    stack.push(m);

    ctx.matrix_model_projection_updated = ctx.matrix_mode <= 1;
    Ok(())
}
