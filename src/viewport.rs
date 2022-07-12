use cgmath::Vector3;

pub struct ViewPort<T> {
    scale: Vector3<T>,
    trans: Vector3<T>,
}

impl<T> ViewPort<T> {
    pub fn new(x: T, y: T, width: T, height: T) -> Self {
        /*
         * x_w = (x_nd + 1) * width/2 + x
         *     = (x_nd * width/2 + (x + width/2))
         *               ^^scale   ^^tran
         */
        todo!();
    }
}

impl<T: std::default::Default + Copy> Default for ViewPort<T> {
    fn default() -> Self {
        let t: T = Default::default();
        ViewPort {
            scale: Vector3::new(t, t, t),
            trans: Vector3::new(t, t, t),
        }
    }
}
