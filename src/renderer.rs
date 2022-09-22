use std::{mem::size_of, num::NonZeroU32};

use crate::pipeline::{Instance, Pipeline};

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,

    texture: wgpu::Texture,
    texture_view: wgpu::TextureView,

    output_buffer: wgpu::Buffer,

    pipeline: Pipeline,
}

impl Renderer {
    pub async fn new(texture_size: (u32, u32)) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&Default::default(), None)
            .await
            .unwrap();

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Framebuffer"),
            size: wgpu::Extent3d {
                width: texture_size.0,
                height: texture_size.1,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::COPY_SRC | wgpu::TextureUsages::RENDER_ATTACHMENT,
        });

        let texture_view = texture.create_view(&Default::default());

        let output_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Output Buffer"),
            size: (size_of::<u32>() * texture_size.0 as usize * texture_size.1 as usize) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let pipeline = Pipeline::new(&device, wgpu::TextureFormat::Rgba8UnormSrgb);

        Self {
            device,
            queue,

            texture,
            texture_view,

            output_buffer,

            pipeline,
        }
    }

    pub async fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.texture_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: (64.0f64 / 255.0f64).powf(2.2),
                            g: (60.0f64 / 255.0f64).powf(2.2),
                            b: (60.0f64 / 255.0f64).powf(2.2),
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            self.pipeline.render(
                &mut render_pass,
                &mut self.queue,
                &[Instance {
                    transform: glam::Mat4::from_translation((0.0, 50.0, 0.0).into())
                        .to_cols_array_2d(),
                    color: [1.0, 0.0, 0.0],
                }],
            );
        }

        encoder.copy_texture_to_buffer(
            wgpu::ImageCopyTexture {
                aspect: wgpu::TextureAspect::All,
                texture: &self.texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            wgpu::ImageCopyBuffer {
                buffer: &self.output_buffer,
                layout: wgpu::ImageDataLayout {
                    offset: 0,
                    // TODO: FIX
                    bytes_per_row: NonZeroU32::new((size_of::<u32>() * 1920) as u32),
                    rows_per_image: NonZeroU32::new(1080),
                },
            },
            // TODO: FIX PT 2
            wgpu::Extent3d {
                width: 1920,
                height: 1080,
                depth_or_array_layers: 1,
            },
        );

        self.queue.submit(Some(encoder.finish()));
        println!("tf");
        {
            let buffer_slice = self.output_buffer.slice(..);

            // NOTE: We have to create the mapping THEN device.poll() before await
            // the future. Otherwise the application will freeze.
            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
            self.device.poll(wgpu::Maintain::Wait);
            rx.receive().await.unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            use image::{ImageBuffer, Rgba};
            let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(1920, 1080, data).unwrap();
            buffer.save("image.png").unwrap();
        }

        self.output_buffer.unmap();

        Ok(())
    }
}