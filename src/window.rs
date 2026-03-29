use std::sync::Arc;

use ash::{khr, vk};

use winit::{
    application::ApplicationHandler, event::WindowEvent::*, event_loop::ActiveEventLoop,
    window::Window,
};

pub struct AppWindow {
    pub window: Arc<Window>,
    pub surface: Arc<vk::SurfaceKHR>,
    pub surface_loader: Arc<khr::surface::Instance>,
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
