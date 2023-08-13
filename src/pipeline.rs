use crate::vertex::{Vertex, VERTICES};
use rand::Rng;
use wgpu::util::DeviceExt;

pub struct RenderPipeline {
    pub pipeline: wgpu::RenderPipeline,
    pub vertex_buffer: wgpu::Buffer,
    pub uniform_buffer: wgpu::Buffer,
    pub binding_groups: Vec<wgpu::BindGroup>,
}

pub struct ComputePipeline {
    pub pipeline: wgpu::ComputePipeline,
}

struct SharedPipelineLayout {
    layout: wgpu::PipelineLayout,
    binding_groups: Vec<wgpu::BindGroup>,
    uniform_buffer: wgpu::Buffer,
}

fn pipeline_layout(device: &wgpu::Device, grid_size: f32) -> SharedPipelineLayout {
    let shader_binding_group_layout =
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: true },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::COMPUTE,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
            label: Some("shader binding group"),
        });

    let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Uniform Buffer"),
        contents: bytemuck::cast_slice(&[grid_size]),
        usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
    });

    // Storage buffers
    let mut rng = rand::thread_rng();
    let mut cell_state_1 = vec![0; (grid_size * grid_size) as usize];

    cell_state_1.iter_mut().for_each(|x| {
        if rng.gen::<f32>() > 0.5 {
            *x = 1;
        } else {
            *x = 0
        };
    });
    let storage_buffer_1 = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer 1"),
        contents: bytemuck::cast_slice(&cell_state_1),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });

    let cell_state_2 = vec![0; (grid_size * grid_size) as usize];
    let storage_buffer_2 = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Storage Buffer 2"),
        contents: bytemuck::cast_slice(&cell_state_2),
        usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
    });

    let storage_buffers = vec![storage_buffer_1, storage_buffer_2];

    let shader_binding_group_1 = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &shader_binding_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: storage_buffers[0].as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: storage_buffers[1].as_entire_binding(),
            },
        ],
        label: Some("shader binding group 1"),
    });

    let shader_binding_group_2 = device.create_bind_group(&wgpu::BindGroupDescriptor {
        layout: &shader_binding_group_layout,
        entries: &[
            wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: storage_buffers[1].as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 2,
                resource: storage_buffers[0].as_entire_binding(),
            },
        ],
        label: Some("shader binding group 2"),
    });

    let shader_binding_groups = vec![shader_binding_group_1, shader_binding_group_2];

    SharedPipelineLayout {
        layout: device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Pipeline Layout"),
            bind_group_layouts: &[&shader_binding_group_layout],
            push_constant_ranges: &[],
        }),
        binding_groups: shader_binding_groups,
        uniform_buffer,
    }
}

impl RenderPipeline {
    pub fn new(device: &wgpu::Device, surface_format: wgpu::TextureFormat, grid_size: f32) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Draw Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("draw.wgsl").into()),
        });

        // Vertex buffer
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        let shared_pipeline_layout = pipeline_layout(device, grid_size);

        let uniform_buffer = shared_pipeline_layout.uniform_buffer;

        let binding_groups = shared_pipeline_layout.binding_groups;

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&shared_pipeline_layout.layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        Self {
            pipeline,
            vertex_buffer,
            uniform_buffer,
            binding_groups,
        }
    }
}

impl ComputePipeline {
    pub fn new(device: &wgpu::Device, grid_size: f32) -> Self {
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Compute Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("compute.wgsl").into()),
        });

        let shared_pipeline_layout = pipeline_layout(device, grid_size);

        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("Compute Pipeline"),
            layout: Some(&shared_pipeline_layout.layout),
            module: &shader,
            entry_point: "cs_main",
        });

        Self { pipeline }
    }
}
