use crate::config::*;
use crate::err::MGLError;
use crate::mgl;
use crate::vertex::*;

type Result<T> = std::result::Result<T, MGLError>;

pub fn op_begin(begin_type: usize) -> Result<()> {
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

pub fn op_color(a: u8, r: u8, g: u8, b: u8) -> Result<()> {
    let ctx = mgl::ctx()?;

    ctx.current_color.x = a;
    ctx.current_color.y = r;
    ctx.current_color.z = g;
    ctx.current_color.w = b;

    Ok(())
}

fn draw_triangle<T>(v1: Vertex<T>, v2: Vertex<T>, v3: Vertex<T>) -> Result<()> {
    // FIXME: Simply assume that no clipping is required
    todo!();
}

fn vertex_triangles() -> Result<()> {
    let ctx = mgl::ctx()?;

    if ctx.vertex_cnt == 3 {
        let v1 = ctx.vertex_stack.pop().ok_or(MGLError::EFAULT)?;
        let v2 = ctx.vertex_stack.pop().ok_or(MGLError::EFAULT)?;
        let v3 = ctx.vertex_stack.pop().ok_or(MGLError::EFAULT)?;
        draw_triangle(v1, v2, v3)?;
        ctx.vertex_cnt = 0;
    }

    Ok(())
}

pub fn op_vertex(x: f32, y: f32, z: f32, w: f32) -> Result<()> {
    let ctx = mgl::ctx()?;

    let begin_type = ctx.vertex_begin_type;
    /* should locate in the begin scope */
    if begin_type == MGLVertexMode::ModeNone.idx() {
        return Err(MGLError::EPERM);
    }

    // increase the counter for new vertex
    ctx.vertex_cnt += 1;

    // TODO: transfrom, should consider when lighting enabled
    let mut v = Vertex::new(x, y, z, w);
    let m = ctx.matrix_model_projection.ok_or(MGLError::EFAULT)?;
    v.transform(m);
    ctx.vertex_stack.push(v);

    /* draw the vertex if they form a new area */
    match begin_type.into() {
        MGLVertexMode::ModeTriangles => vertex_triangles()?,
        _ => todo!(),
    };

    Ok(())
}
