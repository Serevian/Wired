use glam::Mat4;

#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct ShaderData {
    pub projection: glam::Mat4,
    pub view: glam::Mat4,
    pub model: [glam::Mat4; 3],
    pub light_position: glam::Vec4,
    pub selected: glam::Vec4, // Read only x. This is for padding
}

impl ShaderData {
    pub fn new() -> Self {
        Self {
            projection: Mat4::default(),
            view: Mat4::default(),
            model: [Mat4::default(); 3],
            light_position: glam::Vec4::from_slice(&[0.0, -10.0, 10.0, 0.0]),
            selected: glam::Vec4::from_slice(&[1.0, 0.0, 0.0, 0.0]),
        }
    }
}
