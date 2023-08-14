use winit::window::Window;

pub struct Surface {
    window: Window,
    surface: wgpu::Surface,
    surface_config: wgpu::SurfaceConfiguration,
    pub surface_size: winit::dpi::PhysicalSize<u32>,
}

pub struct App {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pub surface: Surface,
    pub render_pipeline: crate::pipeline::RenderPipeline,
    pub compute_pipeline: crate::pipeline::ComputePipeline,
    pub grid_size: u16,
    pub flip: bool,
}

impl App {
    pub async fn new(window: Window, grid_size: u16) -> Self {
        let window_size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let surface = unsafe { instance.create_surface(&window) }.unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::VERTEX_WRITABLE_STORAGE,
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window_size.width,
            height: window_size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &surface_config);

        let render_pipeline =
            crate::pipeline::RenderPipeline::new(&device, surface_config.format, grid_size as f32);

        let compute_pipeline = crate::pipeline::ComputePipeline::new(&device, grid_size as f32);

        let app_surface = Surface {
            window,
            surface,
            surface_config,
            surface_size: window_size,
        };

        Self {
            device,
            queue,
            surface: app_surface,
            render_pipeline,
            compute_pipeline,
            grid_size,
            flip: true,
        }
    }

    pub fn window(&self) -> &Window {
        &self.surface.window
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.surface.surface_size = new_size;
            self.surface.surface_config.width = new_size.width;
            self.surface.surface_config.height = new_size.height;
            self.surface
                .surface
                .configure(&self.device, &self.surface.surface_config);
        }
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let texture = self.surface.surface.get_current_texture()?;
        let view = texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            render_pass.set_pipeline(&self.render_pipeline.pipeline);
            render_pass.set_vertex_buffer(0, self.render_pipeline.vertex_buffer.slice(..));
            render_pass.set_index_buffer(
                self.render_pipeline.index_buffer.slice(..),
                wgpu::IndexFormat::Uint16,
            );
            render_pass.set_bind_group(
                0,
                &self.render_pipeline.binding_groups[self.flip as usize],
                &[],
            );
            render_pass.draw_indexed(0..6, 0, 0..(self.grid_size * self.grid_size) as u32);
        }
        {
            let mut compute_pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: Some("Compute Pass"),
            });
            compute_pass.set_pipeline(&self.compute_pipeline.pipeline);
            compute_pass.set_bind_group(
                0,
                &self.render_pipeline.binding_groups[self.flip as usize],
                &[],
            );
            let workgroup_count = (self.grid_size as f32 / 8.0).ceil() as u32;
            compute_pass.dispatch_workgroups(workgroup_count, workgroup_count, 1)
        }

        self.queue.submit(std::iter::once(encoder.finish()));
        texture.present();
        self.flip = !self.flip;

        Ok(())
    }
}
