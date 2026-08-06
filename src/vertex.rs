use bytemuck::{Pod, Zeroable};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

pub const TRIANGLE: &[Vertex] = &[
    Vertex {
        position: [-0.3, -0.3, 0.0],
        color: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [0.0, 0.5, 0.0],
        color: [1.0, 1.0, 1.0],
    },
    Vertex {
        position: [0.3, -0.3, 0.0],
        color: [1.0, 1.0, 1.0],
    },
];

pub fn create_vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer {
    let buffer_init_descriptor = BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(TRIANGLE),
        usage: wgpu::BufferUsages::VERTEX,
    };

    DeviceExt::create_buffer_init(device, &buffer_init_descriptor)
}
