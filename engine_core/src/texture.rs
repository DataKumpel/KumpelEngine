use image::{DynamicImage, GenericImageView};


pub const DEPTH_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Depth32Float;


//***** DEPTH TEXTURE *****************************************************************************
pub fn create_depth_texture(
    device: &wgpu::Device, 
    config: &wgpu::SurfaceConfiguration, 
    label: &str,
) -> wgpu::TextureView {
    let size = wgpu::Extent3d {
        width: config.width.max(1),
        height: config.height.max(1),
        depth_or_array_layers: 1,
    };

    let desc = wgpu::TextureDescriptor {
        label: Some(label),
        size,
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: DEPTH_FORMAT,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    };

    let texture = device.create_texture(&desc);
    texture.create_view(&wgpu::TextureViewDescriptor::default())
}
//***** DEPTH TEXTURE *****************************************************************************


//***** DIFFUSE TEXTURE STRUCTURE *****************************************************************
pub struct DiffuseTexture {
    pub texture: wgpu::Texture,
    pub view: wgpu::TextureView,
    pub sampler: wgpu::Sampler,
    pub bind_group: wgpu::BindGroup,
}

impl DiffuseTexture {
    pub fn from_assets(
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        texture_filename: &str,
        label: &str,
        layout: &wgpu::BindGroupLayout,
    ) -> Result<Self, image::ImageError> {
        let asset_path = format!("./assets/textures/{texture_filename}");
        Ok(Self::from_path(device, queue, &asset_path, label, layout)?)
    }

    pub fn from_path(
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        path: &str, 
        label: &str, 
        layout: &wgpu::BindGroupLayout,
    ) -> Result<Self, image::ImageError> {
        let img = image::open(path)?;
        Ok(Self::from_bytes(img, device, queue, label, layout))
    }


    pub fn from_memory(
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        bytes: &[u8], 
        label: &str, 
        layout: &wgpu::BindGroupLayout,
    ) -> Result<Self, image::ImageError> {
        let img = image::load_from_memory(bytes)?;
        Ok(DiffuseTexture::from_bytes(img, device, queue, label, layout))
    }

    fn from_bytes(
        img: DynamicImage, 
        device: &wgpu::Device, 
        queue: &wgpu::Queue, 
        label: &str, 
        layout: &wgpu::BindGroupLayout,
    ) -> Self {
        
        let rgba = img.to_rgba8();
        let dimensions = img.dimensions();
        let size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        // ---> Transfer pixel data to GPU:
        queue.write_texture(
            wgpu::ImageCopyTexture {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            }, 
            &rgba, 
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: Some(4 * dimensions.0),
                rows_per_image: Some(dimensions.1),
            }, 
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor { 
            label: Some("Texture Sampler"), 
            address_mode_u: wgpu::AddressMode::Repeat, 
            address_mode_v: wgpu::AddressMode::Repeat, 
            address_mode_w: wgpu::AddressMode::Repeat, 
            mag_filter: wgpu::FilterMode::Nearest, 
            min_filter: wgpu::FilterMode::Nearest, 
            mipmap_filter: wgpu::FilterMode::Nearest, 
            ..Default::default()
        });

        // ---> Bind group bundles view and sampler for our shader:
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor { 
            label: Some("Diffuse Bind Group"), 
            layout, 
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
        });

        Self { texture, view, sampler, bind_group }

    }
}
//***** DIFFUSE TEXTURE STRUCTURE *****************************************************************
