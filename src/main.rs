use std::{
    ffi::{CString, c_char, c_void}, num::NonZeroU32, sync::Arc
};

use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle},
    window::{Window, WindowAttributes},
};

use ash::{
    Device, Entry, Instance, khr,
    prelude::VkResult,
    vk::{
        self, API_VERSION_1_3, Extent2D, StructureType, SurfaceCapabilitiesKHR, SurfaceKHR, SwapchainKHR, TRUE
    },
};

// check with show_physical_device_names
const PHYSICAL_DEVICE_IDX: usize = 0;
const MAKE_PRE_VK_SURFACE: bool = false;

struct AppWindow {
    window: Arc<Window>,
    surface: Arc<vk::SurfaceKHR>,
    surface_loader: Arc<khr::surface::Instance>,
}

impl ApplicationHandler for AppWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            CloseRequested => {
                println!("Application closed");
                event_loop.exit();
            }
            RedrawRequested => {
                // println!("redraw pls");
                // call for another redraw immediately
                // self.window.as_ref().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() {
    let entry = unsafe { Entry::load().expect("Wuh oh, no vulkan sdk and such") };

    let evl = EventLoop::new().unwrap();
    evl.set_control_flow(ControlFlow::Poll);

    let mut raw_display_handle = evl.raw_display_handle().unwrap();

    let instance = create_instance(&entry, raw_display_handle).expect("Error creating instance");

    let physical_device = get_physical_device(&instance);

    let (device_queue, qf_idx) = create_queue(&instance, &physical_device);

    let logical_device = get_logical_device(&instance, physical_device, &device_queue)
        .expect("Error creating logical device");

    let queue = unsafe { logical_device.get_device_queue(qf_idx, 0) };

    let alloc_create_info = vk_mem::AllocationCreateInfo {
        usage: vk_mem::MemoryUsage::Auto,
        ..Default::default()
    };

    let window = Arc::new(
        evl.create_window(
            WindowAttributes::default()
                .with_inner_size(winit::dpi::Size::Logical(LogicalSize::new(480.0, 480.0)))
                ,
        )
        .unwrap(),
    );

    // make a softbuffer surface and draw to it HERE so that the window has a size and so forth
    if MAKE_PRE_VK_SURFACE
    {
        let sbuf_ctx = softbuffer::Context::new(evl.owned_display_handle()).unwrap();
        let mut sbuf_surface = softbuffer::Surface::new(&sbuf_ctx, window.clone()).unwrap();
        let win_size = window.inner_size();
        sbuf_surface
            .resize(
                NonZeroU32::new(win_size.width).unwrap(),
                NonZeroU32::new(win_size.height).unwrap(),
            )
            .unwrap();
        let mut buffer = sbuf_surface.buffer_mut().unwrap();
        buffer.fill_with(|| 255 | 255 << 8 | 255 << 16); 

        buffer.present().unwrap();
    }

    let surface = unsafe {
        ash_window::create_surface(
            &entry,
            &instance,
            window.raw_display_handle().unwrap(),
            window.raw_window_handle().unwrap(),
            None,
        )
        .unwrap()
    };

    let surface_loader = khr::surface::Instance::new(&entry, &instance);

    let sf_caps = unsafe {
        surface_loader
            .get_physical_device_surface_capabilities(physical_device, surface)
            .unwrap()
    };

    let win_size = window.inner_size().to_logical::<u32>(1.0);

    let swapchain_loader = khr::swapchain::Device::new(&instance, &logical_device);
    let swapchain = create_swapchain(&swapchain_loader, surface, &sf_caps, (win_size.width, win_size.height))
        .expect("Error creating the swapchain:");

    let swapchain_images = unsafe { swapchain_loader.get_swapchain_images(swapchain).unwrap() };

    // depth stuff
    // let depth_formats = [
    //     vk::Format::D32_SFLOAT_S8_UINT,
    //     vk::Format::D24_UNORM_S8_UINT,
    // ];
    //
    // let depth_format = vk::Format::UNDEFINED;
    //
    // let swapchain = vk::SwapchainCreateInfoKHR {
    //
    //     ..Default::default()
    // };

    let mut app = AppWindow {
        window: window.clone(),
        surface: Arc::new(surface),
        surface_loader: Arc::new(surface_loader),
    };

    evl.run_app(&mut app).unwrap();
}

fn create_instance(entry: &Entry, display_handle: RawDisplayHandle) -> VkResult<Instance> {
    let app_info;
    if let Ok(s) = CString::new("bingus") {
        app_info = vk::ApplicationInfo {
            s_type: StructureType::APPLICATION_INFO,
            p_application_name: s.as_ptr(),
            api_version: API_VERSION_1_3,
            ..Default::default()
        }
    } else {
        panic!();
    };

    let extensions = ash_window::enumerate_required_extensions(display_handle)
        .expect("Failed to enumerate required extensions");

    let layers = [c"VK_LAYER_KHRONOS_validation"];
    let layers_raw: Vec<*const c_char> = layers.iter().map(|raw_name| raw_name.as_ptr()).collect();

    let instance_info = vk::InstanceCreateInfo {
        s_type: StructureType::INSTANCE_CREATE_INFO,
        p_application_info: &app_info,
        enabled_extension_count: extensions.len() as u32,
        pp_enabled_extension_names: extensions.as_ptr(),
        enabled_layer_count: layers_raw.len() as u32,
        pp_enabled_layer_names: layers_raw.as_ptr(),
        ..Default::default()
    };

    unsafe { entry.create_instance(&instance_info, None) }
}

fn get_physical_device(instance: &Instance) -> vk::PhysicalDevice {
    let devices = unsafe {
        instance
            .enumerate_physical_devices()
            .expect("Failed to enumerate physical devices")
    };
    println!("Using: {PHYSICAL_DEVICE_IDX} from:");
    show_physical_device_names(instance);

    devices[PHYSICAL_DEVICE_IDX]
}

fn create_queue<'a>(
    instance: &'a Instance,
    device: &'a vk::PhysicalDevice,
) -> (vk::DeviceQueueCreateInfo<'a>, u32) {
    let mut qf_idx: u32 = 0;
    let qf_priorities: f32 = 1.0;

    let mut queue_families = unsafe {
        vec![
            vk::QueueFamilyProperties2 {
                ..Default::default()
            };
            instance.get_physical_device_queue_family_properties2_len(*device)
        ]
    };

    unsafe {
        instance.get_physical_device_queue_family_properties2(*device, &mut queue_families);
    }

    for (idx, qf) in queue_families.iter().enumerate() {
        if qf
            .queue_family_properties
            .queue_flags
            .contains(vk::QueueFlags::GRAPHICS)
        {
            qf_idx = idx as u32;
        }
    }

    (
        vk::DeviceQueueCreateInfo {
            s_type: StructureType::DEVICE_QUEUE_CREATE_INFO,
            queue_family_index: qf_idx,
            queue_count: 1,
            p_queue_priorities: &qf_priorities,
            ..Default::default()
        },
        qf_idx,
    )
}

fn get_logical_device(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    queue_create_info: &vk::DeviceQueueCreateInfo,
) -> VkResult<Device> {
    let logical_device_extensions = [khr::swapchain::NAME];

    let mut enabled_vk12_features = vk::PhysicalDeviceVulkan12Features {
        s_type: StructureType::PHYSICAL_DEVICE_VULKAN_1_2_FEATURES,
        descriptor_indexing: TRUE,
        descriptor_binding_variable_descriptor_count: TRUE,
        runtime_descriptor_array: TRUE,
        buffer_device_address: TRUE,
        ..Default::default()
    };

    let mut enabled_vk13_features = vk::PhysicalDeviceVulkan13Features {
        s_type: StructureType::PHYSICAL_DEVICE_VULKAN_1_3_FEATURES,
        p_next: &mut enabled_vk12_features as *mut _ as *mut c_void,
        synchronization2: TRUE,
        dynamic_rendering: TRUE,
        ..Default::default()
    };

    let enabled_vk10_features = vk::PhysicalDeviceFeatures {
        sampler_anisotropy: TRUE,
        ..Default::default()
    };

    let logical_device_info = vk::DeviceCreateInfo {
        s_type: StructureType::DEVICE_CREATE_INFO,
        p_next: &mut enabled_vk13_features as *mut _ as *mut c_void,
        queue_create_info_count: 1_u32,
        p_queue_create_infos: queue_create_info, // the bingus
        enabled_extension_count: logical_device_extensions.len() as u32,
        pp_enabled_extension_names: logical_device_extensions.as_ptr() as *const *const i8,
        p_enabled_features: &enabled_vk10_features,
        ..Default::default()
    };

    unsafe { instance.create_device(physical_device, &logical_device_info, None) }
}

fn create_swapchain(
    swapchain_loader: &khr::swapchain::Device,
    surface: SurfaceKHR,
    sf_caps: &SurfaceCapabilitiesKHR,
    win_size: (u32, u32),
) -> VkResult<SwapchainKHR> {
    let image_format = vk::Format::B8G8R8A8_SRGB;

    let swapchain_info = vk::SwapchainCreateInfoKHR {
        s_type: StructureType::SWAPCHAIN_CREATE_INFO_KHR,
        surface,
        min_image_count: sf_caps.min_image_count,
        image_format,
        image_color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
        // wayland making me cry
        image_extent: Extent2D {width: win_size.0, height: win_size.1},//sf_caps.current_extent,
        image_array_layers: 1_u32,
        image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        pre_transform: vk::SurfaceTransformFlagsKHR::IDENTITY,
        composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
        present_mode: vk::PresentModeKHR::FIFO,
        ..Default::default()
    };

    unsafe { swapchain_loader.create_swapchain(&swapchain_info, None) }
}

// TODO: alongside a flag for this, add option to manually set device
fn show_physical_device_names(instance: &Instance) {
    let devices = unsafe {
        instance
            .enumerate_physical_devices()
            .expect("Failed to enumerate physical devices")
    };
    for (i, dev) in devices.iter().enumerate() {
        let mut prop = vk::PhysicalDeviceProperties2 {
            s_type: StructureType::PHYSICAL_DEVICE_PROPERTIES_2,
            ..Default::default()
        };
        unsafe { instance.get_physical_device_properties2(*dev, &mut prop) };
        println!(
            "{i}: {:?} is a {:?} called {:?}",
            dev,
            prop.properties.device_type,
            prop.properties
                .device_name
                .map(|c| c as u8 as char)
                .iter()
                .collect::<String>()
                .trim_matches(char::from(0))
        );
    }
    println!();
}
