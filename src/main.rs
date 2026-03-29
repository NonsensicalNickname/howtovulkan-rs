mod model;
mod window;

use window::AppWindow;

use winit::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
    raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle},
    window::WindowAttributes,
};

use std::{
    ffi::{CString, c_char, c_void},
    mem::{MaybeUninit, transmute},
    num::NonZeroU32,
    ptr::copy_nonoverlapping,
    sync::Arc,
};

use vk_mem::{Alloc, Allocation, AllocationCreateInfo, AllocationInfo, Allocator};

use ash::{
    Device, Entry, Instance, khr,
    prelude::VkResult,
    vk::{
        self, API_VERSION_1_3, Buffer, CommandBuffer, DeviceAddress, Extent2D, Extent3D, Format,
        Image, ImageSubresourceRange, ImageView, PhysicalDevice, StructureType,
        SurfaceCapabilitiesKHR, SurfaceKHR, SwapchainKHR, TRUE,
    },
};

// check with show_physical_device_names
const PHYSICAL_DEVICE_IDX: usize = 0;
const MAKE_PRE_VK_SURFACE: bool = false;
const DISPLAY_SCALING: f64 = 1.0;
const MAX_FRAMES_IN_FLIGHT: usize = 2;

struct ShaderData {
    proj: glm::Mat4,
    view: glm::Mat4,
    model: [glm::Mat4; 3],
    light_pos: glm::Vec4,
    selected: u32,
}

struct ShaderDataBuffer {
    alloc: Allocation,
    alloc_info: AllocationInfo,
    buffer: Buffer,
    device_address: DeviceAddress,
}

fn main() {
    let entry = unsafe { Entry::load().expect("Wuh oh, no vulkan sdk and such") };

    let evl = EventLoop::new().unwrap();
    evl.set_control_flow(ControlFlow::Poll);

    let mut raw_display_handle = evl.raw_display_handle().unwrap();

    println!("Creating instance...");
    let instance = create_instance(&entry, raw_display_handle).expect("Error creating instance");

    println!("Creating physical device...");
    let physical_device = get_physical_device(&instance);

    println!("Creating queue...");
    let (device_queue, qf_idx) = create_queue(&instance, &physical_device);

    println!("Creating logical device...");
    let logical_device = get_logical_device(&instance, physical_device, &device_queue)
        .expect("Error creating logical device");

    let queue = unsafe { logical_device.get_device_queue(qf_idx, 0) };

    println!("Creating vulkan allocator...");
    let mut alloc_create_info =
        vk_mem::AllocatorCreateInfo::new(&instance, &logical_device, physical_device);

    alloc_create_info.flags = vk_mem::AllocatorCreateFlags::BUFFER_DEVICE_ADDRESS;

    let vk_alloc = unsafe { vk_mem::Allocator::new(alloc_create_info).unwrap() };

    // lettuce begin

    println!("Creating window...");
    let window = Arc::new(
        evl.create_window(
            WindowAttributes::default()
                .with_inner_size(winit::dpi::Size::Logical(LogicalSize::new(480.0, 480.0))),
        )
        .unwrap(),
    );

    // may or may not need to make and draw to a surface before the vulkan surface...
    if MAKE_PRE_VK_SURFACE {
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

    println!("Creating surface...");
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

    let win_size = {
        let ls = window.inner_size().to_logical::<u32>(DISPLAY_SCALING);
        (ls.width, ls.height)
    };

    println!("Creating swapchain...");
    let swapchain_loader = khr::swapchain::Device::new(&instance, &logical_device);
    let swapchain = create_swapchain(&swapchain_loader, surface, &sf_caps, win_size)
        .expect("Error creating the swapchain:");

    let swapchain_images = unsafe { swapchain_loader.get_swapchain_images(swapchain).unwrap() };

    let (depth_image, mut depth_image_alloc, depth_image_view) = get_depth_image(
        &instance,
        &logical_device,
        physical_device,
        win_size,
        &vk_alloc,
    )
    .expect("Error creating an image");

    let (model_vertices, model_indices) = model::load();

    let v_buf_size = size_of::<model::Vertex>() * model_vertices.len();
    let i_buf_size = size_of::<u16>() * model_indices.len();

    let (buffer, mut buffer_alloc) = get_buffer(&vk_alloc, (v_buf_size + i_buf_size) as u64)
        .expect("Error creating the buffer:");

    let buffer_mapped_ptr = vk_alloc.get_allocation_info(&buffer_alloc).mapped_data;

    println!("Copying mesh into VRAM...");
    let vertices_ptr = &model_vertices as *const _ as *const c_void;
    let indices_ptr = &model_indices as *const _ as *const c_void;
    unsafe {
        copy_nonoverlapping(vertices_ptr, buffer_mapped_ptr, model_vertices.len());
        copy_nonoverlapping(
            indices_ptr,
            buffer_mapped_ptr.add(model_vertices.len()),
            model_indices.len(),
        );
        //vk_alloc.unmap_memory(&mut buffer_alloc);
    };

    let mut shader_data_buffers = init_shader_data_buffers(&vk_alloc, &logical_device);
    let mut command_buffers: [CommandBuffer; MAX_FRAMES_IN_FLIGHT];

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
    (width, height): (u32, u32),
) -> VkResult<SwapchainKHR> {
    let image_format = vk::Format::B8G8R8A8_SRGB;

    let swapchain_info = vk::SwapchainCreateInfoKHR {
        s_type: StructureType::SWAPCHAIN_CREATE_INFO_KHR,
        surface,
        min_image_count: sf_caps.min_image_count,
        image_format,
        image_color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
        image_extent: Extent2D { width, height },
        image_array_layers: 1_u32,
        image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        pre_transform: vk::SurfaceTransformFlagsKHR::IDENTITY,
        composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
        present_mode: vk::PresentModeKHR::FIFO,
        ..Default::default()
    };

    unsafe { swapchain_loader.create_swapchain(&swapchain_info, None) }
}

fn get_depth_image(
    instance: &Instance,
    logical_device: &Device,
    physical_device: PhysicalDevice,
    (width, height): (u32, u32),
    vk_alloc: &Allocator,
) -> VkResult<(Image, vk_mem::Allocation, ImageView)> {
    let depth_formats = [
        vk::Format::D32_SFLOAT_S8_UINT,
        vk::Format::D24_UNORM_S8_UINT,
    ];

    let mut depth_format = vk::Format::UNDEFINED;

    for format in depth_formats {
        let mut format_props = vk::FormatProperties2 {
            s_type: StructureType::FORMAT_PROPERTIES_2,
            ..Default::default()
        };
        unsafe {
            instance.get_physical_device_format_properties2(
                physical_device,
                format,
                &mut format_props,
            );
        };
        if (format_props.format_properties.optimal_tiling_features
            & vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT)
            .as_raw()
            != 0_u32
        {
            depth_format = format;
            break;
        }
    }

    let depth_image_info = vk::ImageCreateInfo {
        s_type: StructureType::IMAGE_CREATE_INFO,
        image_type: vk::ImageType::TYPE_2D,
        format: depth_format,
        extent: Extent3D {
            width,
            height,
            depth: 1,
        },
        mip_levels: 1_u32,
        array_layers: 1_u32,
        samples: vk::SampleCountFlags::TYPE_1,
        tiling: vk::ImageTiling::OPTIMAL,
        usage: vk::ImageUsageFlags::DEPTH_STENCIL_ATTACHMENT,
        initial_layout: vk::ImageLayout::UNDEFINED,
        ..Default::default()
    };

    let alloc_info = vk_mem::AllocationCreateInfo {
        usage: vk_mem::MemoryUsage::Auto,
        flags: vk_mem::AllocationCreateFlags::DEDICATED_MEMORY,
        ..Default::default()
    };

    let depth_image = unsafe { vk_alloc.create_image(&depth_image_info, &alloc_info)? };

    let view_info = vk::ImageViewCreateInfo {
        s_type: StructureType::IMAGE_VIEW_CREATE_INFO,
        image: depth_image.0,
        view_type: vk::ImageViewType::TYPE_2D,
        format: depth_format,
        subresource_range: ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::DEPTH,
            level_count: 1_u32,
            layer_count: 1_u32,
            ..Default::default()
        },
        ..Default::default()
    };

    let image_view = unsafe { logical_device.create_image_view(&view_info, None)? };

    Ok((depth_image.0, depth_image.1, image_view))
}

fn get_buffer(vk_alloc: &Allocator, size: u64) -> VkResult<(Buffer, vk_mem::Allocation)> {
    let buffer_info = vk::BufferCreateInfo {
        s_type: StructureType::BUFFER_CREATE_INFO,
        size,
        usage: vk::BufferUsageFlags::VERTEX_BUFFER | vk::BufferUsageFlags::INDEX_BUFFER,
        ..Default::default()
    };

    let alloc_info = vk_mem::AllocationCreateInfo {
        usage: vk_mem::MemoryUsage::Auto,
        flags: vk_mem::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE
            | vk_mem::AllocationCreateFlags::HOST_ACCESS_ALLOW_TRANSFER_INSTEAD
            | vk_mem::AllocationCreateFlags::MAPPED,
        ..Default::default()
    };

    Ok(unsafe { vk_alloc.create_buffer(&buffer_info, &alloc_info)? })
}

fn init_shader_data_buffers(
    vk_alloc: &Allocator,
    logical_device: &Device,
) -> [ShaderDataBuffer; MAX_FRAMES_IN_FLIGHT] {
    let mut shader_data_buffers: [MaybeUninit<ShaderDataBuffer>; MAX_FRAMES_IN_FLIGHT] =
        [const { MaybeUninit::uninit() }; MAX_FRAMES_IN_FLIGHT];

    for elem in &mut shader_data_buffers[..] {
        let buffer_info = vk::BufferCreateInfo {
            s_type: StructureType::BUFFER_CREATE_INFO,
            size: size_of::<ShaderData>() as u64,
            usage: vk::BufferUsageFlags::SHADER_DEVICE_ADDRESS,
            ..Default::default()
        };

        let alloc_info = vk_mem::AllocationCreateInfo {
            usage: vk_mem::MemoryUsage::Auto,
            flags: vk_mem::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE
                | vk_mem::AllocationCreateFlags::HOST_ACCESS_ALLOW_TRANSFER_INSTEAD
                | vk_mem::AllocationCreateFlags::MAPPED,
            ..Default::default()
        };

        let (buffer, alloc) = unsafe { vk_alloc.create_buffer(&buffer_info, &alloc_info).unwrap() };

        let buffer_device_address_info = vk::BufferDeviceAddressInfo {
            s_type: StructureType::BUFFER_DEVICE_ADDRESS_INFO,
            buffer,
            ..Default::default()
        };

        let device_address =
            unsafe { logical_device.get_buffer_device_address(&buffer_device_address_info) };

        elem.write(ShaderDataBuffer {
            alloc,
            alloc_info: vk_alloc.get_allocation_info(&alloc),
            buffer,
            device_address,
        });
    }

    unsafe { transmute::<_, [ShaderDataBuffer; MAX_FRAMES_IN_FLIGHT]>(shader_data_buffers) }
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
