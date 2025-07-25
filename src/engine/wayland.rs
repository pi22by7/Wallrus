//! Native Wayland wallpaper implementation using wlr-layer-shell protocol

#[cfg(feature = "wayland")]
use crate::errors::{Result, WallrusError};
use std::path::Path;


#[cfg(feature = "wayland")]
use smithay_client_toolkit::{
    compositor::{CompositorHandler, CompositorState},
    delegate_compositor, delegate_layer, delegate_output, delegate_registry, delegate_shm,
    output::{OutputHandler, OutputState},
    registry::{ProvidesRegistryState, RegistryState},
    registry_handlers,
    shell::wlr_layer::{
        Anchor, KeyboardInteractivity, Layer, LayerShell, LayerShellHandler, LayerSurface,
    },
    shm::{raw::RawPool, Shm, ShmHandler},
};

#[cfg(feature = "wayland")]
use wayland_client::{
    globals::registry_queue_init,
    protocol::{wl_buffer, wl_output, wl_shm, wl_surface},
    Connection, Dispatch, QueueHandle,
};

#[cfg(feature = "wayland")]
struct WallpaperState {
    registry_state: RegistryState,
    output_state: OutputState,
    compositor_state: CompositorState,
    shm_state: Shm,
    layer_shell: LayerShell,
    
    exit: bool,
    surfaces: Vec<WallpaperSurface>,
}

#[cfg(feature = "wayland")]
struct WallpaperSurface {
    surface: wl_surface::WlSurface,
    layer_surface: LayerSurface,
    output: wl_output::WlOutput,
    width: u32,
    height: u32,
    configured: bool,
}

#[cfg(feature = "wayland")]
impl CompositorHandler for WallpaperState {
    fn scale_factor_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _new_factor: i32,
    ) {
    }

    fn transform_changed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _new_transform: wl_output::Transform,
    ) {
    }

    fn frame(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _time: u32,
    ) {
        // Frame callback - could be used for animations
    }

    fn surface_enter(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _output: &wl_output::WlOutput,
    ) {
    }

    fn surface_leave(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _surface: &wl_surface::WlSurface,
        _output: &wl_output::WlOutput,
    ) {
    }
}

#[cfg(feature = "wayland")]
impl OutputHandler for WallpaperState {
    fn output_state(&mut self) -> &mut OutputState {
        &mut self.output_state
    }

    fn new_output(
        &mut self,
        _conn: &Connection,
        qh: &QueueHandle<Self>,
        output: wl_output::WlOutput,
    ) {
        eprintln!("[WAYLAND DEBUG] New output detected, creating wallpaper surface...");
        // Create a wallpaper surface for each output
        let surface = self.compositor_state.create_surface(qh);
        
        let layer_surface = self.layer_shell.create_layer_surface(
            qh,
            surface.clone(),
            Layer::Background,
            Some("wallrus"),
            Some(&output),
        );

        // Configure layer surface for wallpaper
        eprintln!("[WAYLAND DEBUG] Configuring layer surface: Background layer, anchor=all, exclusive_zone=-1");
        layer_surface.set_anchor(Anchor::all());
        layer_surface.set_exclusive_zone(-1);
        layer_surface.set_keyboard_interactivity(KeyboardInteractivity::None);
        
        eprintln!("[WAYLAND DEBUG] Layer surface configured with anchor=all, exclusive_zone=-1");

        // CRITICAL: Commit the surface to trigger configure events
        surface.commit();
        eprintln!("[WAYLAND DEBUG] Surface committed, waiting for configure...");

        let wallpaper_surface = WallpaperSurface {
            surface,
            layer_surface,
            output,
            width: 0,
            height: 0,
            configured: false,
        };

        self.surfaces.push(wallpaper_surface);
        eprintln!("[WAYLAND DEBUG] Created surface, total surfaces: {}", self.surfaces.len());
    }

    fn update_output(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        _output: wl_output::WlOutput,
    ) {
    }

    fn output_destroyed(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        output: wl_output::WlOutput,
    ) {
        // Remove surface for destroyed output
        self.surfaces.retain(|s| s.output != output);
    }
}

#[cfg(feature = "wayland")]
impl LayerShellHandler for WallpaperState {
    fn closed(&mut self, _conn: &Connection, _qh: &QueueHandle<Self>, layer: &LayerSurface) {
        // Remove closed surface
        self.surfaces.retain(|s| &s.layer_surface != layer);
        if self.surfaces.is_empty() {
            self.exit = true;
        }
    }

    fn configure(
        &mut self,
        _conn: &Connection,
        _qh: &QueueHandle<Self>,
        layer: &LayerSurface,
        configure: smithay_client_toolkit::shell::wlr_layer::LayerSurfaceConfigure,
        _serial: u32,
    ) {
        eprintln!("[WAYLAND DEBUG] Layer surface configure event: {}x{}", configure.new_size.0, configure.new_size.1);
        // Update surface dimensions
        if let Some(surface) = self.surfaces.iter_mut().find(|s| &s.layer_surface == layer) {
            surface.width = configure.new_size.0;
            surface.height = configure.new_size.1;
            surface.configured = true;
            eprintln!("[WAYLAND DEBUG] Surface configured: {}x{}", surface.width, surface.height);
        } else {
            eprintln!("[WAYLAND DEBUG] WARNING: Configure event for unknown surface!");
        }
    }
}

#[cfg(feature = "wayland")]
impl ShmHandler for WallpaperState {
    fn shm_state(&mut self) -> &mut Shm {
        &mut self.shm_state
    }
}

#[cfg(feature = "wayland")]
impl ProvidesRegistryState for WallpaperState {
    fn registry(&mut self) -> &mut RegistryState {
        &mut self.registry_state
    }

    registry_handlers![OutputState];
}

#[cfg(feature = "wayland")]
delegate_compositor!(WallpaperState);
#[cfg(feature = "wayland")]
delegate_output!(WallpaperState);
#[cfg(feature = "wayland")]
delegate_shm!(WallpaperState);
#[cfg(feature = "wayland")]
delegate_layer!(WallpaperState);
#[cfg(feature = "wayland")]
delegate_registry!(WallpaperState);

// Add buffer dispatch handler
#[cfg(feature = "wayland")]
impl Dispatch<wl_buffer::WlBuffer, ()> for WallpaperState {
    fn event(
        _state: &mut Self,
        _proxy: &wl_buffer::WlBuffer,
        _event: <wl_buffer::WlBuffer as wayland_client::Proxy>::Event,
        _data: &(),
        _conn: &Connection,
        _qhandle: &QueueHandle<Self>,
    ) {
        // Handle buffer events if needed
    }
}


#[cfg(feature = "wayland")]
pub fn set_wayland_wallpaper(image_path: &Path) -> Result<()> {
    let _total_start = std::time::Instant::now();
    eprintln!("[WAYLAND DEBUG] Starting native wallpaper setting for: {:?}", image_path);
    
    // Load image
    let img_start = std::time::Instant::now();
    let img = image::open(image_path)
        .map_err(|e| WallrusError::ImageProcessing(format!("Failed to load image: {}", e)))?;
    let img = img.to_rgba8();
    let (width, height) = img.dimensions();
    let image_data = img.into_raw();
    eprintln!("[WAYLAND DEBUG] Loaded image: {}x{} pixels, {} bytes (took {:?})", 
              width, height, image_data.len(), img_start.elapsed());

    // Connect to Wayland
    let conn = Connection::connect_to_env()
        .map_err(|e| WallrusError::Config(format!("Failed to connect to Wayland: {}", e)))?;
    eprintln!("[WAYLAND DEBUG] Connected to Wayland successfully");

    let (globals, mut event_queue) = registry_queue_init(&conn)
        .map_err(|e| WallrusError::Config(format!("Failed to initialize registry: {}", e)))?;
    eprintln!("[WAYLAND DEBUG] Registry initialized");
    
    let qh = event_queue.handle();

    // Initialize state
    eprintln!("[WAYLAND DEBUG] Binding Wayland protocols...");
    let mut state = WallpaperState {
        registry_state: RegistryState::new(&globals),
        output_state: OutputState::new(&globals, &qh),
        compositor_state: CompositorState::bind(&globals, &qh)
            .map_err(|e| WallrusError::Config(format!("Failed to bind compositor: {}", e)))?,
        shm_state: Shm::bind(&globals, &qh)
            .map_err(|e| WallrusError::Config(format!("Failed to bind shm: {}", e)))?,
        layer_shell: LayerShell::bind(&globals, &qh)
            .map_err(|e| WallrusError::Config(format!("Failed to bind layer shell: {}", e)))?,
        exit: false,
        surfaces: Vec::new(),
    };
    eprintln!("[WAYLAND DEBUG] All protocols bound successfully");

    // Initial roundtrip to discover outputs
    eprintln!("[WAYLAND DEBUG] Starting initial roundtrip to discover outputs...");
    event_queue.roundtrip(&mut state)
        .map_err(|e| WallrusError::Config(format!("Failed initial roundtrip: {}", e)))?;
    eprintln!("[WAYLAND DEBUG] Initial roundtrip complete. Found {} surfaces", state.surfaces.len());

    // Wait for surfaces to be configured with timeout
    let mut configured_count = 0;
    let mut attempts = 0;
    const MAX_ATTEMPTS: usize = 10;
    
    eprintln!("[WAYLAND DEBUG] Waiting for surface configuration...");
    while configured_count < state.surfaces.len() && !state.exit && attempts < MAX_ATTEMPTS {
        eprintln!("[WAYLAND DEBUG] Attempt {}: dispatching pending events...", attempts + 1);
        match event_queue.dispatch_pending(&mut state) {
            Ok(_) => {},
            Err(e) => return Err(WallrusError::Config(format!("Event dispatch failed: {}", e))),
        }
        
        configured_count = state.surfaces.iter().filter(|s| s.configured).count();
        eprintln!("[WAYLAND DEBUG] Configured surfaces: {}/{}", configured_count, state.surfaces.len());
        attempts += 1;
        
        // If no progress, do a blocking dispatch once
        if configured_count == 0 && attempts < MAX_ATTEMPTS {
            eprintln!("[WAYLAND DEBUG] No progress, trying blocking dispatch...");
            match event_queue.blocking_dispatch(&mut state) {
                Ok(_) => {},
                Err(e) => return Err(WallrusError::Config(format!("Blocking dispatch failed: {}", e))),
            }
            configured_count = state.surfaces.iter().filter(|s| s.configured).count();
            eprintln!("[WAYLAND DEBUG] After blocking dispatch - configured surfaces: {}/{}", configured_count, state.surfaces.len());
        }
    }
    
    // If we couldn't configure surfaces, fail gracefully
    if configured_count == 0 {
        eprintln!("[WAYLAND DEBUG] ERROR: No surfaces configured after {} attempts", attempts);
        return Err(WallrusError::Config(
            "Failed to configure layer surfaces - compositor may not support wlr-layer-shell".into()
        ));
    }
    
    eprintln!("[WAYLAND DEBUG] Surface configuration complete! {} surfaces ready", configured_count);

    // Create and attach buffers for each surface
    eprintln!("[WAYLAND DEBUG] Creating buffers for {} surfaces", state.surfaces.len());
    for (i, surface) in state.surfaces.iter_mut().enumerate() {
        eprintln!("[WAYLAND DEBUG] Processing surface {}: configured={}, size={}x{}", 
                  i, surface.configured, surface.width, surface.height);
        if surface.configured && surface.width > 0 && surface.height > 0 {
            eprintln!("[WAYLAND DEBUG] Scaling image from {}x{} to {}x{}", width, height, surface.width, surface.height);
            // Scale image to surface size
            let scale_start = std::time::Instant::now();
            let _scaled_img = image::imageops::resize(
                &image::RgbaImage::from_raw(width, height, image_data.clone()).unwrap(),
                surface.width,
                surface.height,
                image::imageops::FilterType::Lanczos3,
            );
            eprintln!("[WAYLAND DEBUG] Image scaling took {:?}", scale_start.elapsed());

            // Create shared memory pool
            let stride = surface.width * 4;
            let size = stride * surface.height;
            eprintln!("[WAYLAND DEBUG] Creating SHM pool: {} bytes ({}x{} * 4)", size, surface.width, surface.height);
            
            // Create SHM pool
            let mut shm_pool = RawPool::new(size as usize, &state.shm_state)
                .map_err(|e| WallrusError::Config(format!("Failed to create SHM pool: {}", e)))?;
            eprintln!("[WAYLAND DEBUG] SHM pool created successfully");

            // Get mutable slice to write image data
            let pool_data = shm_pool.mmap();
            eprintln!("[WAYLAND DEBUG] Pool data length: {} bytes", pool_data.len());
            
            // Write scaled image data to buffer (convert RGBA to BGRA)
            let mut pixels_written = 0;
            for (i, chunk) in _scaled_img.chunks(4).enumerate() {
                if i >= (surface.width * surface.height) as usize {
                    break;
                }
                let offset = i * 4;
                if chunk.len() >= 4 && offset + 3 < pool_data.len() {
                    pool_data[offset] = chunk[2];     // B
                    pool_data[offset + 1] = chunk[1]; // G  
                    pool_data[offset + 2] = chunk[0]; // R
                    pool_data[offset + 3] = chunk[3]; // A
                    pixels_written += 1;
                }
            }
            eprintln!("[WAYLAND DEBUG] Wrote {} pixels to buffer", pixels_written);
            
            // Debug: Check first few pixels
            for i in 0..10 {
                let offset = i * 4;
                if offset + 3 < pool_data.len() {
                    eprintln!("[WAYLAND DEBUG] Pixel {}: B={}, G={}, R={}, A={}", 
                             i, pool_data[offset], pool_data[offset+1], 
                             pool_data[offset+2], pool_data[offset+3]);
                }
            }

            let buffer = shm_pool.create_buffer(
                0,
                surface.width as i32,
                surface.height as i32,
                stride as i32,
                wl_shm::Format::Argb8888,
                (),
                &qh,
            );

            // Attach buffer and commit
            eprintln!("[WAYLAND DEBUG] Attaching buffer and committing surface...");
            surface.surface.attach(Some(&buffer), 0, 0);
            surface.surface.damage_buffer(0, 0, surface.width as i32, surface.height as i32);
            surface.surface.commit();
            eprintln!("[WAYLAND DEBUG] Surface committed with buffer attached");
        } else {
            eprintln!("[WAYLAND DEBUG] Skipping surface {}: not configured or zero size", i);
        }
    }

    // Final roundtrip to ensure everything is submitted
    eprintln!("[WAYLAND DEBUG] Performing final roundtrip...");
    event_queue.roundtrip(&mut state)
        .map_err(|e| WallrusError::Config(format!("Failed final roundtrip: {}", e)))?;

    eprintln!("[WAYLAND DEBUG] Wallpaper surfaces created! Keeping process alive...");
    eprintln!("[WAYLAND DEBUG] Press Ctrl+C to exit and remove wallpaper");
    
    // Keep the process alive to maintain wallpaper surfaces
    // Layer surfaces only exist while the client process is running
    loop {
        match event_queue.dispatch_pending(&mut state) {
            Ok(_) => {},
            Err(e) => {
                eprintln!("[WAYLAND DEBUG] Event dispatch error: {}", e);
                break;
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    
    eprintln!("[WAYLAND DEBUG] Process exiting, wallpaper surfaces will be destroyed");
    Ok(())
}

#[cfg(not(feature = "wayland"))]
pub fn set_wayland_wallpaper(_image_path: &Path) -> Result<()> {
    Err(WallrusError::Config(
        "Wayland support not compiled in. Compile with --features wayland".into(),
    ))
}