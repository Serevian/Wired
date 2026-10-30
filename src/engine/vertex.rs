#[derive(Clone, Copy, PartialEq, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: glam::Vec3,
    pub normal: glam::Vec3,
    pub uv: glam::Vec2,
}

impl Vertex {
    pub fn new(position: &[f32], normal: &[f32], uv: &[f32]) -> Self {
        Self {
            position: glam::Vec3::from_slice(position),
            normal: glam::Vec3::from_slice(normal),
            uv: glam::Vec2::from_slice(uv),
        }
    }
}
