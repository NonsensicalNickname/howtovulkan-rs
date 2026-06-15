use std::{cell::RefCell, rc::Rc, sync::Arc};

use ash::{khr, vk};

use winit::{
    application::ApplicationHandler,
    event::{
        self,
        WindowEvent::{self, *},
    },
    event_loop::ActiveEventLoop,
    keyboard::{KeyCode, PhysicalKey},
    window::Window,
};

use crate::AppState;

pub struct AppWindow<'a> {
    pub window: Arc<Window>,
    pub surface: Arc<vk::SurfaceKHR>,
    pub surface_loader: Arc<khr::surface::Instance>,
    pub state: Rc<RefCell<AppState<'a>>>,
    mouse_down: bool,
}

impl<'a> AppWindow<'a> {
    pub fn new(
        window: Arc<Window>,
        surface: Arc<vk::SurfaceKHR>,
        surface_loader: Arc<khr::surface::Instance>,
        state: Rc<RefCell<AppState<'a>>>,
    ) -> Self {
        AppWindow {
            window,
            surface,
            surface_loader,
            state,
            mouse_down: false,
        }
    }
}

impl AppWindow<'_> {
    fn rotate(&mut self, rhs: nalgebra_glm::Vec3) {
        let selected = self.state.borrow().selected;
        let mut rot = &mut self.state.borrow_mut().obj_rotations[selected as usize];
        *rot += rhs;
    }
}

impl ApplicationHandler for AppWindow<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {}

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            CloseRequested => {
                println!("Application closed");
                event_loop.exit();
            }
            KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => match (event.physical_key, event.state) {
                (PhysicalKey::Code(KeyCode::ArrowLeft), event::ElementState::Pressed) => {
                    let selected = &mut self.state.borrow_mut().selected;
                    if *selected == 0 {
                        *selected = 2;
                    } else {
                        *selected -= 1;
                    }
                }
                (PhysicalKey::Code(KeyCode::ArrowRight), event::ElementState::Pressed) => {
                    let selected = &mut self.state.borrow_mut().selected;
                    if *selected == 2 {
                        *selected = 0;
                    } else {
                        *selected += 1;
                    }
                }
                (PhysicalKey::Code(KeyCode::Minus), event::ElementState::Pressed) => {
                    self.state.borrow_mut().cam_pos.z -= 0.1;
                }
                (PhysicalKey::Code(KeyCode::Equal), event::ElementState::Pressed) => {
                    self.state.borrow_mut().cam_pos.z += 0.1;
                }
                _ => (),
            },
            MouseWheel {
                device_id,
                delta,
                phase,
            } => {
                if let event::MouseScrollDelta::LineDelta(_, y) = delta {
                    let factor = self.state.borrow().frame_time * 10.0;
                    self.state.borrow_mut().cam_pos.z += y * factor;
                }
            }
            MouseInput {
                device_id,
                state,
                button,
            } => {
                if state == event::ElementState::Pressed && button == event::MouseButton::Left {
                    self.mouse_down = true;
                } else if state == event::ElementState::Released
                    && button == event::MouseButton::Left
                {
                    self.mouse_down = false;
                }
            }
            _ => (),
        }
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        device_id: event::DeviceId,
        event: event::DeviceEvent,
    ) {
        if self.mouse_down
            && let event::DeviceEvent::MouseMotion { delta } = event
        {
            let factor = self.state.borrow().frame_time;
            self.rotate(nalgebra_glm::vec3(
                -delta.1 as f32 * factor,
                delta.0 as f32 * factor,
                0.0,
            ));
        }
    }
}
