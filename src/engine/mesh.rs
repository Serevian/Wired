use anyhow::{Context, Result};
use ash::vk;

use crate::engine::{buffer::Buffer, device::Device, vertex::Vertex};

pub struct GpuMesh {
    pub name: String,
    pub vertex_buffer: Buffer,
    pub index_buffer: Buffer,
    pub index_count: u32,
}

impl GpuMesh {
    pub fn destroy(&mut self, allocator: &vk_mem::Allocator) {
        self.vertex_buffer.destroy(allocator);
        self.index_buffer.destroy(allocator);
    }
}

pub struct Mesh {
    pub name: String,
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn upload(&self, device: &Device, allocator: &vk_mem::Allocator) -> Result<GpuMesh> {
        let vertex_bytes = bytemuck::cast_slice(&self.vertices);
        let vertex_buffer = Buffer::create_device_local_and_write(
            device,
            allocator,
            vertex_bytes,
            vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
        )?;
        let index_bytes = bytemuck::cast_slice(&self.indices);
        let index_buffer = Buffer::create_device_local_and_write(
            device,
            allocator,
            index_bytes,
            vk::BufferUsageFlags::INDEX_BUFFER,
        )?;

        Ok(GpuMesh {
            name: self.name.clone(),
            vertex_buffer,
            index_buffer,
            index_count: self.indices.len() as u32,
        })
    }

    pub fn load_meshes(path: &str) -> Result<Vec<Self>> {
        let (document, buffers, _) = gltf::import(path)?;
        let mut meshes = Vec::new();

        for mesh in document.meshes() {
            for primitive in mesh.primitives() {
                // Let's handle triangles only for now
                if primitive.mode() != gltf::mesh::Mode::Triangles {
                    continue;
                }

                let reader = primitive.reader(|b| Some(&buffers[b.index()]));

                let positions: Vec<[f32; 3]> = match reader.read_positions() {
                    Some(p) => p.collect(),
                    None => continue,
                };

                let normals: Vec<[f32; 3]> = reader
                    .read_normals()
                    .map(|n| n.collect())
                    .unwrap_or_else(|| vec![[0.0, 0.0, 1.0]; positions.len()]);

                let uvs: Vec<[f32; 2]> = reader
                    .read_tex_coords(0)
                    .map(|t| t.into_f32().collect())
                    .unwrap_or_else(|| vec![[0.0, 0.0]; positions.len()]);

                let vertices = positions
                    .iter()
                    .zip(&normals)
                    .zip(&uvs)
                    .map(|((&position, &normal), &uv)| Vertex::new(&position, &normal, &uv))
                    .collect();

                let indices = match reader.read_indices() {
                    Some(i) => i.into_u32().collect(),
                    None => (0..positions.len() as u32).collect(),
                };

                meshes.push(Self {
                    name: mesh
                        .name()
                        .map(String::from)
                        .context("Error getting name from gtlf mesh")?,
                    vertices,
                    indices,
                });
            }
        }

        tracing::info!("Loaded {} mesh primitives", meshes.len());
        Ok(meshes)
    }
}
