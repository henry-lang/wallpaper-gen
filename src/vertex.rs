use bytemuck::{Pod, Zeroable};
use wgpu::{VertexBufferLayout, VertexStepMode};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
    position: [f32; 3],
    color: [f32; 4],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x4];

    pub fn layout() -> VertexBufferLayout<'static> {
        VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

pub const GREEN: [f32; 4] = [0.3392, 0.3992, 0.2718, 1.0];
pub const TRANSPARENT: [f32; 4] = [0.3392, 0.3992, 0.2718, 0.0];

pub const VERTICES: &[Vertex] = &[
    Vertex {
        position: [0.1, 0.1, 0.0],
        color: GREEN,
    },
    Vertex {
        position: [0.4, 0.4, 0.0],
        color: TRANSPARENT,
    },
    Vertex {
        position: [0.1, 0.4, 0.0],
        color: GREEN,
    },
    Vertex {
        position: [0.4, 0.1, 0.0],
        color: TRANSPARENT,
    },
    Vertex {
        position: [0.4, 0.4, 0.0],
        color: TRANSPARENT,
    },
    Vertex {
        position: [0.1, 0.1, 0.0],
        color: GREEN,
    },
];
