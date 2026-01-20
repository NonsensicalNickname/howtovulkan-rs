use std::{
    ffi::{CString}, 
    sync::Arc,
};

use winit::{
    application::ApplicationHandler, 
    event::WindowEvent::*, 
    event_loop::{
        ActiveEventLoop, ControlFlow, EventLoop
    }, 
    raw_window_handle::{HasRawDisplayHandle, RawDisplayHandle}, 
    window::{Window, WindowAttributes}
};

use ash::{
    Entry, Instance, prelude::VkResult, 
    vk::{ self, API_VERSION_1_3, StructureType }
};

// check with show_device_names
const PHYSICAL_DEVICE_IDX: usize = 0;

struct AppWindow {
    window: Arc<Window>,
}

impl ApplicationHandler for AppWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    }

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

    let instance = instance(&entry, evl.raw_display_handle().unwrap())
        .expect("Error creating instance");
    get_device(&instance);

    let window = Arc::new(evl.create_window(
        WindowAttributes::default()).unwrap());

    let mut app = AppWindow {
        window: window.clone(),
    };

    evl.run_app(&mut app).unwrap();
}

fn instance(entry: &Entry, display_handle: RawDisplayHandle) -> VkResult<Instance> {
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

    let extensions = ash_window::enumerate_required_extensions(display_handle).expect("Failed to enumerate required extensions");

    let instance_info = vk::InstanceCreateInfo {
        s_type: StructureType::INSTANCE_CREATE_INFO,
        p_application_info: &app_info,
        enabled_extension_count: extensions.len() as u32,
        pp_enabled_extension_names: extensions.as_ptr(),
        ..Default::default()
    };

    unsafe { entry.create_instance(&instance_info, None) }
}

fn get_device(instance: &Instance) -> vk::PhysicalDevice {
    let devices = unsafe { instance.enumerate_physical_devices()
        .expect("Failed to enumerate physical devices") };
    devices[PHYSICAL_DEVICE_IDX]
}

// TODO: alongside a flag for this, add option to manually set device
fn show_device_names(instance: &Instance) {
    let devices = unsafe { instance.enumerate_physical_devices()
        .expect("Failed to enumerate physical devices") };
    for (i, dev) in devices.iter().enumerate() {
        let mut prop = vk::PhysicalDeviceProperties2 {
            s_type: vk::StructureType::PHYSICAL_DEVICE_PROPERTIES_2,
            ..Default::default()
        };
        unsafe { instance.get_physical_device_properties2(*dev, &mut prop) };
        println!("{i}: {:?} is a {:?} called {:?}",
            dev, prop.properties.device_type, prop.properties.device_name
                .map(|c| c as u8 as char)
                .iter().collect::<String>()
                .trim_matches(char::from(0))
            );
    }
}
