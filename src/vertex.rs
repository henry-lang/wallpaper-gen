use bytemuck::{Pod, Zeroable};
use wgpu::{VertexBufferLayout, VertexStepMode};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 2],
    color: [f32; 4],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x4];

    pub fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub const COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
pub const TRANSPARENT: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [200.0, 250.0],
        color: COLOR,
    },
    Vertex {
        position: [200.0, 200.0],
        color: COLOR,
    },
    Vertex {
        position: [250.0, 250.0],
        color: TRANSPARENT,
    },
    Vertex {
        position: [250.0, 200.0],
        color: TRANSPARENT,
    },
];
