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

pub fn op_rotate(angle: f32, x: f32, y:f32, z:f32) -> Result<()> {

    let dir_code = ((x != 0.0) as usize) << 2 |
                   ((y != 0.0) as usize) << 1 |
                   ((z != 0.0) as usize) << 0;

    // http://en.wikipedia.org/wiki/Rotation_matrix#Basic_rotations
    let (angle_deg, m);
    match dir_code {
        1 => {
            if z < 0.0 {
                angle_deg = Deg(-angle);
            }
            else
            {
                angle_deg = Deg(angle);
            }
            m = Matrix4::from_angle_z(angle_deg);
        }
        2 => {
            if y < 0.0 {
                angle_deg = Deg(-angle);
            }
            else
            {
                angle_deg = Deg(angle);
            }
            m = Matrix4::from_angle_y(angle_deg);
        }
        4 => {
            if x < 0.0 {
                angle_deg = Deg(-angle);
            }
            else
            {
                angle_deg = Deg(angle);
            }
            m = Matrix4::from_angle_x(angle_deg);
        }
        // TODO: only simple case of rotation is supported now
        _ => todo!(),
    }

    Ok(())
}
