use cgmath::*;

#[derive(Clone)]
pub struct Vertex<T> {
    coord: Vector4<T>,
    color: Vector4<u8>,
    pc: Vector4<T>, /* coordinates in the normalized volume */
}

impl<T: BaseFloat> Vertex<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        Vertex {
            coord: Vector4::new(x, y, z, w),
            color: Vector4::new(0, 0, 0, 0),
            pc: Vector4::new(x, y, z, w),
        }
    }

    pub fn set_color(&mut self, color: Vector4<u8>) {
        self.color = color;
    }

    pub fn to_viewpoint(&mut self) {
        todo!();
    }

    pub fn transform(&mut self, m: Matrix4<T>) {
        self.pc = m * self.coord;
    }
}

impl<T: Copy + std::default::Default> Default for Vertex<T> {
    fn default() -> Self {
        let v: T = Default::default();
        Vertex {
            coord: Vector4::new(v, v, v, v),
            color: Vector4::new(0, 0, 0, 0),
            pc: Vector4::new(v, v, v, v),
        }
    }
}
