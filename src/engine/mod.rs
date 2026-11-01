use anyhow::{Ok, Result};
use ash::vk;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

use crate::engine::{
    buffer::Buffer,
    context::Context,
    device::{Device, Queue},
    frame::Frame,
    image::Image,
    mesh::{GpuMesh, Mesh},
    surface::Surface,
    swapchain::Swapchain,
};

mod buffer;
mod context;
mod device;
mod frame;
mod image;
mod mesh;
mod shader_data;
mod surface;
mod swapchain;
mod vertex;

const FRAMES_IN_FLIGHT: usize = 2;

pub struct Engine {
    frames: Vec<Frame>,
    render_complete_semaphores: Vec<vk::Semaphore>,
    meshes: Vec<GpuMesh>,
    depth: Image,
    swapchain: Swapchain,
    surface: Surface,
    allocator: vk_mem::Allocator,
    queue: Queue,
    device: Device,
    context: Context,
}

impl Engine {
    pub fn new(
        display_handle: RawDisplayHandle,
        window_handle: RawWindowHandle,
        window_size: (u32, u32),
    ) -> Result<Self> {
        let context = Context::new(&display_handle)?;
        let (device, queue) = Device::new(&context)?;

        let mut allocator_info =
            vk_mem::AllocatorCreateInfo::new(&context.instance, &device.logical, device.physical);
        allocator_info.vulkan_api_version = vk::API_VERSION_1_4;
        allocator_info.flags = vk_mem::AllocatorCreateFlags::BUFFER_DEVICE_ADDRESS;
        let allocator = unsafe { vk_mem::Allocator::new(allocator_info)? };

        let surface = Surface::new(&context, &device, &display_handle, &window_handle)?;
        let swapchain = Swapchain::new(&context, &device, &surface, window_size)?;
        let depth = Image::depth(
            &device,
            &allocator,
            vk::Extent3D::default()
                .width(swapchain.size.width)
                .height(swapchain.size.height)
                .depth(1),
        )?;

        let cpu_meshes = Mesh::load_meshes("assets/FlightHelmet/FlightHelmet.gltf")?;
        let mut gpu_meshes = Vec::with_capacity(cpu_meshes.len());
        for mesh in &cpu_meshes {
            gpu_meshes.push(mesh.upload(&device, &allocator)?);
        }
        tracing::info!("Finished uploading {} meshes to GPU", gpu_meshes.len());

        let mut render_complete_semaphores = Vec::new();
        let semaphore_info = vk::SemaphoreCreateInfo::default();
        for _ in 0..swapchain.images.len() {
            let semaphore = unsafe { device.logical.create_semaphore(&semaphore_info, None)? };
            render_complete_semaphores.push(semaphore);
        }

        let mut frames = Vec::new();
        for _ in 0..FRAMES_IN_FLIGHT {
            let frame = Frame::new(&device, &allocator)?;
            frames.push(frame);
        }

        Ok(Self {
            frames,
            render_complete_semaphores,
            meshes: gpu_meshes,
            depth,
            swapchain,
            surface,
            allocator,
            queue,
            device,
            context,
        })
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        for frame in &mut self.frames {
            frame.destroy(&self.device, &self.allocator);
        }
        unsafe {
            for semaphore in &self.render_complete_semaphores {
                self.device.logical.destroy_semaphore(*semaphore, None);
            }
        }
        for mesh in &mut self.meshes {
            mesh.destroy(&self.allocator);
        }
        self.depth.destroy(&self.device, &self.allocator);
    }
}
