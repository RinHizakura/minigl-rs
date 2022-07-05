use crate::config::*;
use crate::err::MGLError;
use crate::mgl;

type Result<T> = std::result::Result<T, MGLError>;

pub fn op_begin(begin_type: u8) -> Result<()> {
    let ctx = mgl::ctx()?;

    /* Initial */
    ctx.vertex_begin_type = begin_type;
    ctx.vertex_cnt = 0;

    if ctx.matrix_model_projection_updated {
        /* TODO: consider when lighting is enabled */
        let m1 = ctx.matrix_stack[MGLMatrixMode::ModeModelView.idx()]
            .top()
            .ok_or(MGLError::EINVALID)?;
        let m2 = ctx.matrix_stack[MGLMatrixMode::ModeProjection.idx()]
            .top()
            .ok_or(MGLError::EINVALID)?;
        ctx.matrix_model_projection = Some(m1 * m2);
    }


    /* TODO: consider polygon mode */

    Ok(())
}

pub fn op_color(a:u8, r:u8, g:u8, b:u8) -> Result<()> {
    let ctx = mgl::ctx()?;

    ctx.current_color.x = a;
    ctx.current_color.y = r;
    ctx.current_color.z = g;
    ctx.current_color.w = b;

    Ok(())
}
