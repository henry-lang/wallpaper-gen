use bytemuck::{Pod, Zeroable};
use glam::{Mat4, Vec2};
use winit::dpi::{PhysicalSize, Pixel};

#[derive(Debug)]
pub struct Camera {
    position: Vec2,
    size: PhysicalSize<f32>,
}

impl Camera {
    pub fn new(position: Vec2, size: PhysicalSize<impl Pixel>) -> Self {
        Self {
            position,
            size: size.cast::<f32>(),
        }
    }

    pub fn update_size(&mut self, new_size: PhysicalSize<impl Pixel>) {
        self.size = new_size.cast::<f32>();
    }

    pub fn projection_matrix(&self) -> Mat4 {
        let left = self.position.x - self.size.width * 0.5;
        let right = self.position.x + self.size.width * 0.5;
        let top = self.position.y - self.size.height * 0.5;
        let bottom = self.position.y + self.size.height * 0.5;

        Mat4::orthographic_lh(left, right, bottom, top, 0.01, 100.0)
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
        println!("Updated: {:?}", camera);
        self.view_proj = camera.projection_matrix().to_cols_array_2d();
        println!("{:?}", self.view_proj);
    }
}
