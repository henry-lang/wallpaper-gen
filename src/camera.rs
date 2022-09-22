use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2, Vec3};

pub const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array(&[
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.0, 0.0, 0.0, 0.5, 1.0,
]);

#[derive(Debug)]
pub struct Camera {
    position: Vec2,
    size: Vec2,
}

impl Camera {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self { position, size }
    }

    pub fn projection_matrix(&self) -> Mat4 {
        let view = Mat4::look_at_rh(
            (self.position.x, self.position.y, 1.0).into(),
            (self.position.x, self.position.y, 0.0).into(),
            Vec3::Y,
        );
        let proj = Mat4::orthographic_rh(0.0, self.size.x, 0.0, self.size.y, 0.01, 100.0);

        proj * view
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone, Pod, Zeroable)]
pub struct CameraUniform {
    view_proj: [[f32; 4]; 4],
}

impl CameraUniform {
    pub fn new() -> Self {
        Self {
            view_proj: Mat4::IDENTITY.to_cols_array_2d(),
        }
    }

    pub fn update_view_proj(&mut self, camera: &Camera) {
        self.view_proj = (OPENGL_TO_WGPU_MATRIX * camera.projection_matrix()).to_cols_array_2d();
    }
}
