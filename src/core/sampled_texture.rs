use super::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, Instance, Sampler, SamplerDescriptor,
    ShaderStage, Texture, TextureComponentType, TextureView, TextureViewDescriptor,
    TextureViewDimension,
};

pub struct SampledTexture {
    texture_view: TextureView,
    sampler: Sampler,
    bind_group: BindGroup,
}

impl SampledTexture {
    pub fn bind_group_layout(instance: &Instance) -> BindGroupLayout {
        BindGroupLayout::new(
            instance,
            &BindGroupLayoutDescriptor {
                label: None,
                entries: &[
                    BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStage::FRAGMENT,
                        ty: BindingType::SampledTexture {
                            multisampled: false,
                            component_type: TextureComponentType::Float,
                            dimension: TextureViewDimension::D2,
                        },
                        count: None,
                    },
                    BindGroupLayoutEntry {
                        binding: 1,
                        visibility: ShaderStage::FRAGMENT,
                        ty: BindingType::Sampler { comparison: false },
                        count: None,
                    },
                ],
            },
        )
    }

    pub fn new(instance: &Instance, texture: Texture, sampler_desc: &SamplerDescriptor) -> Self {
        let texture_view = texture.create_view(&TextureViewDescriptor::default());
        let sampler = Sampler::new(instance, sampler_desc);
        let bind_group_layout = Self::bind_group_layout(instance);
        let bind_group = BindGroup::new(
            instance,
            &BindGroupDescriptor {
                label: None,
                layout: &bind_group_layout,
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(&texture_view),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(&sampler),
                    },
                ],
            },
        );
        Self {
            texture_view,
            sampler,
            bind_group,
        }
    }

    pub fn texture_view(&self) -> &TextureView {
        &self.texture_view
    }

    pub fn sampler(&self) -> &Sampler {
        &self.sampler
    }

    pub fn bind_group(&self) -> &BindGroup {
        &self.bind_group
    }
}
