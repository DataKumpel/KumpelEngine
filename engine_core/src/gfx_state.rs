//===== IMPORTS =====//
use std::sync::Arc;

use winit::window::Window;
//===== IMPORTS =====//


//===== GFX STATE STRUCTURE =====//
pub struct GfxState {
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    window: Arc<Window>,
}

impl GfxState {
    async fn new(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::default();

        // ---> Surface is our painting canvas in the window:
        let surface = instance.create_surface(window.clone()).unwrap();

        // ---> Adapter is our physical graphics card:
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptionsBase { 
                power_preference: wgpu::PowerPreference::HighPerformance, 
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            }).await.expect("No WGPU adapter was found...");

        // ---> Device (logical connection) and queue (for commands):
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await.expect("WGPU device could not be created...");

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_fmt = surface_caps.formats.iter().copied()
            .find(|fmt| fmt.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_fmt,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::AutoNoVsync,
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        Self { surface, device, queue, config, size, window }
    }

    fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {

    }
}
//===== GFX STATE STRUCTURE =====//

