// File automatically generated by build.rs.
// Changes made to this file will not be saved.
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, bytemuck :: Pod, bytemuck :: Zeroable)]
pub struct VertexInput {
    pub position: glam::Vec3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, encase :: ShaderType)]
pub struct Uniforms {
    pub color_rgb: glam::Vec3,
}
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, encase :: ShaderType)]
pub struct PushConstants {
    pub color_matrix: glam::Mat4,
}
pub struct OverrideConstants {
    pub force_black: bool,
    pub scale: Option<f32>,
}
impl OverrideConstants {
    pub fn constants(&self) -> std::collections::HashMap<String, f64> {
        let mut entries = std::collections::HashMap::from([(
            "force_black".to_owned(),
            if self.force_black { 1.0 } else { 0.0 },
        )]);
        if let Some(value) = self.scale {
            entries.insert("scale".to_owned(), value as f64);
        }
        entries
    }
}
pub mod bind_groups {
    #[derive(Debug)]
    pub struct BindGroup0(wgpu::BindGroup);
    #[derive(Debug)]
    pub struct BindGroupLayout0<'a> {
        pub color_texture: &'a wgpu::TextureView,
        pub color_sampler: &'a wgpu::Sampler,
    }
    const LAYOUT_DESCRIPTOR0: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: Some("LayoutDescriptor0"),
        entries: &[
            wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Texture {
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                    view_dimension: wgpu::TextureViewDimension::D2,
                    multisampled: false,
                },
                count: None,
            },
            wgpu::BindGroupLayoutEntry {
                binding: 1,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                count: None,
            },
        ],
    };
    impl BindGroup0 {
        pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&LAYOUT_DESCRIPTOR0)
        }
        pub fn from_bindings(device: &wgpu::Device, bindings: BindGroupLayout0) -> Self {
            let bind_group_layout = device.create_bind_group_layout(&LAYOUT_DESCRIPTOR0);
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: wgpu::BindingResource::TextureView(bindings.color_texture),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: wgpu::BindingResource::Sampler(bindings.color_sampler),
                    },
                ],
                label: Some("BindGroup0"),
            });
            Self(bind_group)
        }
        pub fn set<P: SetBindGroup>(&self, pass: &mut P) {
            pass.set_bind_group(0, &self.0, &[]);
        }
    }
    #[derive(Debug)]
    pub struct BindGroup1(wgpu::BindGroup);
    #[derive(Debug)]
    pub struct BindGroupLayout1<'a> {
        pub uniforms: wgpu::BufferBinding<'a>,
    }
    const LAYOUT_DESCRIPTOR1: wgpu::BindGroupLayoutDescriptor = wgpu::BindGroupLayoutDescriptor {
        label: Some("LayoutDescriptor1"),
        entries: &[wgpu::BindGroupLayoutEntry {
            binding: 0,
            visibility: wgpu::ShaderStages::FRAGMENT,
            ty: wgpu::BindingType::Buffer {
                ty: wgpu::BufferBindingType::Uniform,
                has_dynamic_offset: false,
                min_binding_size: None,
            },
            count: None,
        }],
    };
    impl BindGroup1 {
        pub fn get_bind_group_layout(device: &wgpu::Device) -> wgpu::BindGroupLayout {
            device.create_bind_group_layout(&LAYOUT_DESCRIPTOR1)
        }
        pub fn from_bindings(device: &wgpu::Device, bindings: BindGroupLayout1) -> Self {
            let bind_group_layout = device.create_bind_group_layout(&LAYOUT_DESCRIPTOR1);
            let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
                layout: &bind_group_layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer(bindings.uniforms),
                }],
                label: Some("BindGroup1"),
            });
            Self(bind_group)
        }
        pub fn set<P: SetBindGroup>(&self, pass: &mut P) {
            pass.set_bind_group(1, &self.0, &[]);
        }
    }
    #[derive(Debug, Copy, Clone)]
    pub struct BindGroups<'a> {
        pub bind_group0: &'a BindGroup0,
        pub bind_group1: &'a BindGroup1,
    }
    impl BindGroups<'_> {
        pub fn set<P: SetBindGroup>(&self, pass: &mut P) {
            self.bind_group0.set(pass);
            self.bind_group1.set(pass);
        }
    }
    pub trait SetBindGroup {
        fn set_bind_group(
            &mut self,
            index: u32,
            bind_group: &wgpu::BindGroup,
            offsets: &[wgpu::DynamicOffset],
        );
    }
    impl SetBindGroup for wgpu::ComputePass<'_> {
        fn set_bind_group(
            &mut self,
            index: u32,
            bind_group: &wgpu::BindGroup,
            offsets: &[wgpu::DynamicOffset],
        ) {
            self.set_bind_group(index, bind_group, offsets);
        }
    }
    impl SetBindGroup for wgpu::RenderPass<'_> {
        fn set_bind_group(
            &mut self,
            index: u32,
            bind_group: &wgpu::BindGroup,
            offsets: &[wgpu::DynamicOffset],
        ) {
            self.set_bind_group(index, bind_group, offsets);
        }
    }
}
pub fn set_bind_groups<P: bind_groups::SetBindGroup>(
    pass: &mut P,
    bind_group0: &bind_groups::BindGroup0,
    bind_group1: &bind_groups::BindGroup1,
) {
    bind_group0.set(pass);
    bind_group1.set(pass);
}
impl VertexInput {
    pub const VERTEX_ATTRIBUTES: [wgpu::VertexAttribute; 1] = [wgpu::VertexAttribute {
        format: wgpu::VertexFormat::Float32x3,
        offset: std::mem::offset_of!(VertexInput, position) as u64,
        shader_location: 0,
    }];
    pub const fn vertex_buffer_layout(
        step_mode: wgpu::VertexStepMode,
    ) -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<VertexInput>() as u64,
            step_mode,
            attributes: &VertexInput::VERTEX_ATTRIBUTES,
        }
    }
}
pub const ENTRY_VS_MAIN: &str = "vs_main";
pub const ENTRY_FS_MAIN: &str = "fs_main";
#[derive(Debug)]
pub struct VertexEntry<const N: usize> {
    pub entry_point: &'static str,
    pub buffers: [wgpu::VertexBufferLayout<'static>; N],
    pub constants: std::collections::HashMap<String, f64>,
}
pub fn vertex_state<'a, const N: usize>(
    module: &'a wgpu::ShaderModule,
    entry: &'a VertexEntry<N>,
) -> wgpu::VertexState<'a> {
    wgpu::VertexState {
        module,
        entry_point: Some(entry.entry_point),
        buffers: &entry.buffers,
        compilation_options: wgpu::PipelineCompilationOptions {
            constants: &entry.constants,
            ..Default::default()
        },
    }
}
pub fn vs_main_entry(
    vertex_input: wgpu::VertexStepMode,
    overrides: &OverrideConstants,
) -> VertexEntry<1> {
    VertexEntry {
        entry_point: ENTRY_VS_MAIN,
        buffers: [VertexInput::vertex_buffer_layout(vertex_input)],
        constants: overrides.constants(),
    }
}
#[derive(Debug)]
pub struct FragmentEntry<const N: usize> {
    pub entry_point: &'static str,
    pub targets: [Option<wgpu::ColorTargetState>; N],
    pub constants: std::collections::HashMap<String, f64>,
}
pub fn fragment_state<'a, const N: usize>(
    module: &'a wgpu::ShaderModule,
    entry: &'a FragmentEntry<N>,
) -> wgpu::FragmentState<'a> {
    wgpu::FragmentState {
        module,
        entry_point: Some(entry.entry_point),
        targets: &entry.targets,
        compilation_options: wgpu::PipelineCompilationOptions {
            constants: &entry.constants,
            ..Default::default()
        },
    }
}
pub fn fs_main_entry(
    targets: [Option<wgpu::ColorTargetState>; 1],
    overrides: &OverrideConstants,
) -> FragmentEntry<1> {
    FragmentEntry {
        entry_point: ENTRY_FS_MAIN,
        targets,
        constants: overrides.constants(),
    }
}
pub const SOURCE: &str = include_str!("shader.wgsl");
pub fn create_shader_module(device: &wgpu::Device) -> wgpu::ShaderModule {
    let source = std::borrow::Cow::Borrowed(SOURCE);
    device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: None,
        source: wgpu::ShaderSource::Wgsl(source),
    })
}
pub const PUSH_CONSTANT_STAGES: wgpu::ShaderStages = wgpu::ShaderStages::FRAGMENT;
pub fn create_pipeline_layout(device: &wgpu::Device) -> wgpu::PipelineLayout {
    device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: None,
        bind_group_layouts: &[
            &bind_groups::BindGroup0::get_bind_group_layout(device),
            &bind_groups::BindGroup1::get_bind_group_layout(device),
        ],
        push_constant_ranges: &[wgpu::PushConstantRange {
            stages: PUSH_CONSTANT_STAGES,
            range: 0..64,
        }],
    })
}
