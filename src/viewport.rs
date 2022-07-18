use cgmath::Vector3;

/* FIXME: Suppor generic types instead of limited on f32 */
pub struct ViewPort {
    scale: Vector3<f32>,
    trans: Vector3<f32>,
}

impl ViewPort {
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        /*
         * x_w = (x_nd + 1) * width/2 + x
         *     = (x_nd * width/2 + (x + width/2))
         *               ^^scale   ^^tran
         */

        /* TODO: viewport of Z axis is incorrect, but we will resolve this
         * until we really need it */
        ViewPort {
            scale: Vector3::new(width / 2.0, height / 2.0, 0.0),
            trans: Vector3::new(width / 2.0 + x, height / 2.0 + y, 0.0),
        }
    }
}

impl Default for ViewPort {
    fn default() -> Self {
        let t = 0.0;
        ViewPort {
            scale: Vector3::new(t, t, t),
            trans: Vector3::new(t, t, t),
        }
    }
}
