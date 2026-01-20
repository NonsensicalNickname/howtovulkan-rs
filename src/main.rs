use std::{
    ffi::{c_void, CString},
    sync::Arc,
};

use winit::{
    application::ApplicationHandler,
    event::WindowEvent::*,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    raw_window_handle::{HasRawDisplayHandle, RawDisplayHandle},
    window::{Window, WindowAttributes},
};

use ash::{
    khr,
    prelude::VkResult,
    vk::{self, StructureType, API_VERSION_1_3, TRUE},
    Device, Entry, Instance,
};

use vk_mem;

// check with show_physical_device_names
const PHYSICAL_DEVICE_IDX: usize = 0;

struct AppWindow {
    window: Arc<Window>,
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
                println!("redraw pls");
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

    let instance = create_instance(&entry, raw_display_handle)
        .expect("Error creating instance");

    let physical_device = get_physical_device(&instance);

    let (device_queue, qf_idx) = create_queue(&instance, &physical_device);

    let logical_device = get_logical_device(&instance, physical_device, &device_queue)
        .expect("Error creating logical device");

    let queue = unsafe { logical_device.get_device_queue(qf_idx, 0) };

    let alloc_create_info = vk_mem::AllocationCreateInfo {
        usage: vk_mem::MemoryUsage::Auto,
        ..Default::default()
    };

    // peruse https://github.com/ash-rs/ash/blob/master/ash-window/examples/winit.rs
    // let sf = khr::surface::Instance::new(&entry, &instance);
    // let surface = vk::WaylandSurfaceCreateInfoKHR {
    //     s_type: StructureType::WAYLAND_SURFACE_CREATE_INFO_KHR,
    //     display: &mut raw_display_handle as *mut _ as *mut c_void,
    //     surface:,
    //     ..Default::default()
    // };
    // let surface_capabilities = unsafe {
    //     khr::surface::Instance::get_physical_device_surface_capabilities(&sf, physical_device, surface)
    // };
    // WaylandSurface

    // println!("{queue:?}");

    let window = Arc::new(evl.create_window(WindowAttributes::default()).unwrap());

    let mut app = AppWindow {
        window: window.clone(),
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

    let instance_info = vk::InstanceCreateInfo {
        s_type: StructureType::INSTANCE_CREATE_INFO,
        p_application_info: &app_info,
        enabled_extension_count: extensions.len() as u32,
        pp_enabled_extension_names: extensions.as_ptr(),
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

	// VkPhysicalDeviceVulkan12Features enabledVk12Features{ 
    //     .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_2_FEATURES,
    //     .descriptorIndexing = true,
    //     .descriptorBindingVariableDescriptorCount = true,
    //     .runtimeDescriptorArray = true,
    //     .bufferDeviceAddress = true 
    // };
	// VkPhysicalDeviceVulkan13Features enabledVk13Features{
    //     .sType = VK_STRUCTURE_TYPE_PHYSICAL_DEVICE_VULKAN_1_3_FEATURES,
    //     .pNext = &enabledVk12Features,
    //     .synchronization2 = true,
    //     .dynamicRendering = true 
    // };
	// const std::vector<const char*> deviceExtensions{ VK_KHR_SWAPCHAIN_EXTENSION_NAME };
	// const VkPhysicalDeviceFeatures enabledVk10Features{ .samplerAnisotropy = VK_TRUE };
	// VkDeviceCreateInfo deviceCI{
	// 	.sType = VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO,
	// 	.pNext = &enabledVk13Features,
	// 	.queueCreateInfoCount = 1,
	// 	.pQueueCreateInfos = &queueCI,
	// 	.enabledExtensionCount = static_cast<uint32_t>(deviceExtensions.size()),
	// 	.ppEnabledExtensionNames = deviceExtensions.data(),
	// 	.pEnabledFeatures = &enabledVk10Features
	// };
fn get_logical_device(
    instance: &Instance,
    physical_device: vk::PhysicalDevice,
    queue_create_info: &vk::DeviceQueueCreateInfo
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
        p_queue_create_infos: queue_create_info,
        enabled_extension_count: logical_device_extensions.len() as u32,
        pp_enabled_extension_names: logical_device_extensions.as_ptr() as *const *const i8,
        p_enabled_features: &enabled_vk10_features,
        ..Default::default()
    };

    unsafe { instance.create_device(physical_device, &logical_device_info, None) }
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
}
