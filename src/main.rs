mod extra_ktx;
mod gl_format;
mod model;
mod shader;
mod vk_format;
mod window;

use model::Vertex;

use inline_spirv::include_spirv;
use window::AppWindow;

use winit::{
    dpi::LogicalSize,
    event_loop::{ControlFlow, EventLoop},
    platform::pump_events::{self, EventLoopExtPumpEvents},
    raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle},
    window::WindowAttributes,
};

use std::{
    array,
    cell::RefCell,
    ffi::{CString, c_char, c_void},
    mem::offset_of,
    rc::Rc,
    sync::Arc,
    time::Duration,
};

use ash::{
    Device, Entry, Instance, khr,
    prelude::VkResult,
    vk::{self, StructureType},
};

use vk_mem::Alloc;

use crate::extra_ktx::ktx_texture_get_offset;
use ktx::{Ktx, KtxInfo, include_ktx};

// check with show_physical_device_names
const PHYSICAL_DEVICE_IDX: usize = 0;
const DISPLAY_SCALING: f64 = 1.0;
const MAX_FRAMES_IN_FLIGHT: usize = 2;

#[repr(C)]
#[derive(Debug)]
struct ShaderData {
    proj: nalgebra_glm::Mat4,
    view: nalgebra_glm::Mat4,
    model: [nalgebra_glm::Mat4; 3],
    light_pos: nalgebra_glm::Vec4,
    selected: u32,
    shininess: f32,
}

struct ShaderDataBuffer {
    alloc: vk_mem::Allocation,
    alloc_info: vk_mem::AllocationInfo,
    buffer: vk::Buffer,
}

struct Texture {
    alloc: vk_mem::Allocation,
    image: vk::Image,
    view: vk::ImageView,
    sampler: vk::Sampler,
}

struct AppState<'a> {
    cam_pos: &'a mut nalgebra_glm::Vec3,
    obj_rotations: &'a mut [nalgebra_glm::Vec3; 3],
    selected: u32,
    frame_time: f32,
    update_swapchain: bool,
    shininess: f32,
    debug: bool,
}

#[allow(unused)]
fn main() {
    let entry = unsafe { Entry::load().expect("Wuh oh, no vulkan sdk and such") };

    let mut evl = EventLoop::new().unwrap();
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

    // TODO: Add case for non-wayland surfaces
    let mut win_size = {
        let ls = window.inner_size().to_logical::<u32>(DISPLAY_SCALING);
        (ls.width, ls.height)
    };

    let image_format = vk::Format::B8G8R8A8_SRGB;

    let mut swapchain_create_info = vk::SwapchainCreateInfoKHR {
        s_type: StructureType::SWAPCHAIN_CREATE_INFO_KHR,
        surface,
        min_image_count: sf_caps.min_image_count,
        image_format,
        image_color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
        image_extent: vk::Extent2D {
            width: win_size.0,
            height: win_size.1,
        },
        image_array_layers: 1_u32,
        image_usage: vk::ImageUsageFlags::COLOR_ATTACHMENT,
        pre_transform: vk::SurfaceTransformFlagsKHR::IDENTITY,
        composite_alpha: vk::CompositeAlphaFlagsKHR::OPAQUE,
        present_mode: vk::PresentModeKHR::FIFO,
        ..Default::default()
    };

    println!("Creating swapchain...");
    let swapchain_loader = khr::swapchain::Device::new(&instance, &logical_device);
    let (mut swapchain, mut swapchain_images, mut swapchain_image_views) = create_swapchain(
        &logical_device,
        &swapchain_loader,
        &swapchain_create_info,
        image_format,
    )
    .expect("Error creating the swapchain:");

    let mut image_count = swapchain_images.len();

    let (
        mut depth_image,
        mut depth_image_info,
        mut depth_image_alloc,
        mut depth_image_view,
        depth_format,
    ) = get_depth_image(
        &instance,
        &logical_device,
        physical_device,
        win_size,
        &vk_alloc,
    )
    .expect("Error creating an image");

    let (model_vertices, model_indices) = model::load();

    let index_count = model_indices.len() as u32;

    let v_buf_size = size_of::<Vertex>() * model_vertices.len();
    let i_buf_size = size_of::<u16>() * model_indices.len();

    let (buffer, mut buffer_alloc) = get_buffer(&vk_alloc, (v_buf_size + i_buf_size) as u64)
        .expect("Error creating the buffer:");

    let buffer_mapped_ptr = unsafe { vk_alloc.map_memory(&mut buffer_alloc).expect("fuck") };

    println!("Copying mesh into VRAM...");

    let vertices_ptr = model_vertices.as_ptr() as *const c_void;
    let indices_ptr = model_indices.as_ptr() as *const c_void;

    unsafe {
        buffer_mapped_ptr.copy_from(
            model_vertices.as_ptr() as *const u8,
            size_of_val(&model_vertices[0]) * model_vertices.len(),
        );

        buffer_mapped_ptr.add(v_buf_size).copy_from(
            model_indices.as_ptr() as *const u8,
            size_of_val(&model_indices[0]) * model_indices.len(),
        );
    };

    drop(model_vertices);
    drop(model_indices);

    unsafe { vk_alloc.unmap_memory(&mut buffer_alloc) }

    let mut shader_data_buffer = init_shader_buffer(&vk_alloc);

    let mut render_semaphores = Vec::<vk::Semaphore>::with_capacity(swapchain_images.len());

    println!("Creating synchronisation objects...");
    let (fences, present_semaphores, tmp, semaphore_create_info) =
        init_sync_objects(&logical_device, swapchain_images.len(), render_semaphores)
            .expect("Could not create synchronisation objects");

    render_semaphores = tmp;

    println!("Creating command buffers...");
    let (mut command_pool, mut command_buffers) = create_command_buffers(&logical_device, qf_idx)
        .expect("Could not create command pool or buffers");

    let mut cam_pos = nalgebra_glm::vec3(0.0, 0.0, -6.0);
    let mut obj_rotations = [nalgebra_glm::vec3(0.0, 0.0, 0.0); 3];

    let mut state = Rc::new(RefCell::new(AppState {
        cam_pos: &mut cam_pos,
        obj_rotations: &mut obj_rotations,
        selected: 1,
        frame_time: 16.0 / 1000.0,
        update_swapchain: false,
        shininess: 16.0,
        debug: false,
    }));

    let mut app = AppWindow::new(
        window.clone(),
        Arc::new(surface),
        Arc::new(surface_loader.clone()),
        Rc::clone(&state),
    );

    println!("Loading textures...");
    let (mut textures, texture_descriptors) =
        load_tex(&vk_alloc, &logical_device, command_pool, queue).expect("Could not load textures");

    println!("Initialising descriptors...");
    let (shader_data_set, shader_data_set_layout, texture_set, texture_set_layout, descriptor_pool) =
        setup_descriptors(
            &logical_device,
            &textures,
            &texture_descriptors,
            &shader_data_buffer,
        )
        .expect("Could not initialise descriptors");

    println!("Loading shaders...");

    let vert_shader_module =
        crate::include_shader_module!("shaders/shader.vert", vert, logical_device)
            .expect("Could not load shader");

    let frag_shader_module =
        crate::include_shader_module!("shaders/shader.frag", frag, logical_device)
            .expect("Could not load shader");

    let outline_vert_shader_module =
        crate::include_shader_module!("shaders/outline.vert", vert, logical_device)
            .expect("Could not load shader");

    let outline_frag_shader_module =
        crate::include_shader_module!("shaders/outline.frag", frag, logical_device)
            .expect("Could not load shader");

    println!("Creating graphics pipeline...");
    let (pipeline, pipeline_layout) = setup_main_pipeline(
        &logical_device,
        vert_shader_module,
        frag_shader_module,
        &[shader_data_set_layout, texture_set_layout],
        image_format,
        depth_format,
    )
    .expect("Could not set up the graphics pipeline");

    let (outline_pipeline, outline_pipeline_layout) = setup_outline_pipeline(
        &logical_device,
        outline_vert_shader_module,
        outline_frag_shader_module,
        &[shader_data_set_layout],
        image_format,
        depth_format,
    )
    .expect("Could not set up the graphics pipeline");

    let mut update_swapchain = false;

    let mut image_idx: usize = 0;
    let mut frame_idx: usize = 0;
    let mut last_time = std::time::Instant::now();

    println!("Starting render loop...");

    loop {
        unsafe {
            // Wait for GPU
            logical_device
                .wait_for_fences(&[fences[frame_idx]], true, u64::MAX)
                .expect("Could not wait for fence");
            logical_device
                .reset_fences(&[fences[frame_idx]])
                .expect("Could not reset fence");

            let res = swapchain_loader
                .acquire_next_image(
                    swapchain,
                    u64::MAX,
                    present_semaphores[frame_idx],
                    vk::Fence::null(),
                )
                .expect("Could not acquire next image from swapchain");
            (image_idx, update_swapchain) = (res.0 as usize, res.1);
        }

        let shader_data = calculate_shader_data(win_size, &state.borrow());

        update_shader_data_descriptor(
            &logical_device,
            &vk_alloc,
            shader_data_set,
            &shader_data,
            &mut shader_data_buffer,
        );

        let command_buffer = command_buffers[frame_idx];
        let command_buffer_begin_info = vk::CommandBufferBeginInfo {
            s_type: StructureType::COMMAND_BUFFER_BEGIN_INFO,
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            ..Default::default()
        };

        unsafe {
            logical_device
                .reset_command_buffer(command_buffer, vk::CommandBufferResetFlags::empty())
                .unwrap();
            logical_device
                .begin_command_buffer(command_buffer, &command_buffer_begin_info)
                .unwrap();
        }

        let output_barriers = [
            vk::ImageMemoryBarrier2 {
                s_type: StructureType::IMAGE_MEMORY_BARRIER_2,
                src_stage_mask: vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
                src_access_mask: vk::AccessFlags2::empty(),
                dst_stage_mask: vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT
                    | vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS,
                dst_access_mask: vk::AccessFlags2::COLOR_ATTACHMENT_READ
                    | vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
                old_layout: vk::ImageLayout::UNDEFINED,
                new_layout: vk::ImageLayout::ATTACHMENT_OPTIMAL,
                image: swapchain_images[image_idx],
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::COLOR,
                    level_count: 1,
                    layer_count: 1,
                    ..Default::default()
                },
                ..Default::default()
            },
            vk::ImageMemoryBarrier2 {
                s_type: StructureType::IMAGE_MEMORY_BARRIER_2,
                src_stage_mask: vk::PipelineStageFlags2::LATE_FRAGMENT_TESTS,
                src_access_mask: vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE,
                dst_stage_mask: vk::PipelineStageFlags2::EARLY_FRAGMENT_TESTS,
                dst_access_mask: vk::AccessFlags2::DEPTH_STENCIL_ATTACHMENT_WRITE,
                old_layout: vk::ImageLayout::UNDEFINED,
                new_layout: vk::ImageLayout::ATTACHMENT_OPTIMAL,
                image: depth_image,
                subresource_range: vk::ImageSubresourceRange {
                    aspect_mask: vk::ImageAspectFlags::DEPTH | vk::ImageAspectFlags::STENCIL,
                    level_count: 1,
                    layer_count: 1,
                    ..Default::default()
                },
                ..Default::default()
            },
        ];

        let barrier_dependency_info = vk::DependencyInfo {
            s_type: StructureType::DEPENDENCY_INFO,
            image_memory_barrier_count: 2,
            p_image_memory_barriers: output_barriers.as_ptr(),
            ..Default::default()
        };

        unsafe {
            logical_device.cmd_pipeline_barrier2(command_buffer, &barrier_dependency_info);
        }

        let colour_attachment_info = vk::RenderingAttachmentInfo {
            s_type: StructureType::RENDERING_ATTACHMENT_INFO,
            image_view: swapchain_image_views[image_idx],
            image_layout: vk::ImageLayout::ATTACHMENT_OPTIMAL,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            clear_value: vk::ClearValue {
                color: vk::ClearColorValue {
                    float32: [1.0, 1.0, 1.0, 1.0],
                },
            },
            ..Default::default()
        };

        let depth_attachment_info = vk::RenderingAttachmentInfo {
            s_type: StructureType::RENDERING_ATTACHMENT_INFO,
            image_view: depth_image_view,
            image_layout: vk::ImageLayout::ATTACHMENT_OPTIMAL,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::DONT_CARE,
            clear_value: vk::ClearValue {
                depth_stencil: vk::ClearDepthStencilValue {
                    depth: 1.0,
                    stencil: 0,
                },
            },
            ..Default::default()
        };

        let render_info = vk::RenderingInfo {
            s_type: StructureType::RENDERING_INFO,
            render_area: vk::Rect2D {
                extent: vk::Extent2D {
                    width: win_size.0,
                    height: win_size.1,
                },
                ..Default::default()
            },
            layer_count: 1,
            color_attachment_count: 1,
            p_color_attachments: &colour_attachment_info,
            p_depth_attachment: &depth_attachment_info,
            ..Default::default()
        };

        let viewport = vk::Viewport {
            width: win_size.0 as f32,
            height: win_size.1 as f32,
            min_depth: 0.0,
            max_depth: 1.0,
            ..Default::default()
        };

        let scissor = vk::Rect2D {
            extent: vk::Extent2D {
                width: win_size.0,
                height: win_size.1,
            },
            ..Default::default()
        };

        let v_offset: vk::DeviceSize = 0;

        unsafe {
            logical_device.cmd_begin_rendering(command_buffer, &render_info);

            logical_device.cmd_set_viewport(command_buffer, 0, &[viewport]);
            logical_device.cmd_set_scissor(command_buffer, 0, &[scissor]);

            logical_device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                pipeline,
            );

            logical_device.cmd_bind_descriptor_sets(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                pipeline_layout,
                0,
                &[shader_data_set, texture_set],
                &[],
            );

            logical_device.cmd_bind_vertex_buffers(command_buffer, 0, &[buffer], &[v_offset]);
            logical_device.cmd_bind_index_buffer(
                command_buffer,
                buffer,
                v_buf_size as vk::DeviceSize,
                vk::IndexType::UINT16,
            );

            logical_device.cmd_draw_indexed(command_buffer, index_count, 3, 0, 0, 0);

            logical_device.cmd_bind_pipeline(
                command_buffer,
                vk::PipelineBindPoint::GRAPHICS,
                outline_pipeline,
            );

            logical_device.cmd_draw_indexed(command_buffer, index_count, 3, 0, 0, 0);

            if state.borrow().debug {
                // TODO: debug view pipeline
            }

            logical_device.cmd_end_rendering(command_buffer);
        }

        let barrier_present = vk::ImageMemoryBarrier2 {
            s_type: StructureType::IMAGE_MEMORY_BARRIER_2,
            src_stage_mask: vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: vk::AccessFlags2::COLOR_ATTACHMENT_WRITE,
            dst_stage_mask: vk::PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
            dst_access_mask: vk::AccessFlags2::from_raw(0),
            old_layout: vk::ImageLayout::ATTACHMENT_OPTIMAL,
            new_layout: vk::ImageLayout::PRESENT_SRC_KHR,
            image: swapchain_images[image_idx],
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                level_count: 1,
                layer_count: 1,
                ..Default::default()
            },
            ..Default::default()
        };

        let barrier_present_dependencies = vk::DependencyInfo {
            s_type: StructureType::DEPENDENCY_INFO,
            image_memory_barrier_count: 1,
            p_image_memory_barriers: &barrier_present,
            ..Default::default()
        };

        unsafe {
            logical_device.cmd_pipeline_barrier2(command_buffer, &barrier_present_dependencies);
            logical_device
                .end_command_buffer(command_buffer)
                .expect("Could not end the current command buffer");
        }

        let wait_stages = vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
        let submit_info = vk::SubmitInfo {
            s_type: StructureType::SUBMIT_INFO,
            wait_semaphore_count: 1,
            p_wait_semaphores: &present_semaphores[frame_idx],
            p_wait_dst_stage_mask: &wait_stages,
            command_buffer_count: 1,
            p_command_buffers: &command_buffer,
            signal_semaphore_count: 1,
            p_signal_semaphores: &render_semaphores[image_idx],
            ..Default::default()
        };

        unsafe {
            logical_device
                .queue_submit(queue, &[submit_info], fences[frame_idx])
                .expect("Could not submit queue");
        }

        let present_info = vk::PresentInfoKHR {
            s_type: StructureType::PRESENT_INFO_KHR,
            wait_semaphore_count: 1,
            p_wait_semaphores: &render_semaphores[image_idx],
            swapchain_count: 1,
            p_swapchains: &swapchain,
            p_image_indices: &(image_idx as u32),
            ..Default::default()
        };

        unsafe {
            update_swapchain = swapchain_loader
                .queue_present(queue, &present_info)
                .expect("Could not acquire next image from swapchain");
        }

        frame_idx = (frame_idx + 1) % MAX_FRAMES_IN_FLIGHT;

        let now = std::time::Instant::now();
        state.borrow_mut().frame_time = (now - last_time).as_millis() as f32 / 1000.0;
        last_time = now;

        state.borrow_mut().update_swapchain = update_swapchain;

        if let pump_events::PumpStatus::Exit(..) =
            evl.pump_app_events(Some(Duration::from_millis(16)), &mut app)
        {
            println!("Exiting...");
            break;
        }

        if state.borrow().update_swapchain {
            state.borrow_mut().update_swapchain = false;

            let surface_caps = unsafe {
                logical_device.device_wait_idle().expect("Could not idle");
                surface_loader
                    .get_physical_device_surface_capabilities(physical_device, surface)
                    .expect("Could not get surface caps")
            };

            win_size = {
                let ls = window.inner_size().to_logical::<u32>(DISPLAY_SCALING);
                (ls.width, ls.height)
            };

            swapchain_create_info.old_swapchain = swapchain;
            swapchain_create_info.image_extent = vk::Extent2D {
                width: win_size.0,
                height: win_size.1,
            };

            unsafe {
                for image_view in &mut swapchain_image_views {
                    logical_device.destroy_image_view(*image_view, None);
                }

                (swapchain, swapchain_images, swapchain_image_views) = create_swapchain(
                    &logical_device,
                    &swapchain_loader,
                    &swapchain_create_info,
                    image_format,
                )
                .expect("Could not create swapchain");

                for semaphore in &mut render_semaphores {
                    logical_device.destroy_semaphore(*semaphore, None);
                }

                render_semaphores.resize(swapchain_images.len(), vk::Semaphore::null());

                let semaphore_create_info = vk::SemaphoreCreateInfo {
                    s_type: StructureType::SEMAPHORE_CREATE_INFO,
                    ..Default::default()
                };

                for mut semaphore in &mut render_semaphores {
                    *semaphore =
                        unsafe { logical_device.create_semaphore(&semaphore_create_info, None) }
                            .expect("Could not create semaphore");
                }

                logical_device.destroy_image(depth_image, None);
                vk_alloc.free_memory(&mut depth_image_alloc);
                logical_device.destroy_image_view(depth_image_view, None);
                swapchain_loader.destroy_swapchain(swapchain_create_info.old_swapchain, None);

                depth_image_info.extent = vk::Extent3D {
                    width: win_size.0,
                    height: win_size.1,
                    depth: 1,
                };

                let alloc_info = vk_mem::AllocationCreateInfo {
                    usage: vk_mem::MemoryUsage::Auto,
                    flags: vk_mem::AllocationCreateFlags::DEDICATED_MEMORY,
                    ..Default::default()
                };

                (depth_image, depth_image_alloc) = vk_alloc
                    .create_image(&depth_image_info, &alloc_info)
                    .expect("Could not create allocation for depth image");

                let view_info = vk::ImageViewCreateInfo {
                    s_type: StructureType::IMAGE_VIEW_CREATE_INFO,
                    image: depth_image,
                    view_type: vk::ImageViewType::TYPE_2D,
                    format: depth_format,
                    subresource_range: vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::DEPTH,
                        level_count: 1_u32,
                        layer_count: 1_u32,
                        ..Default::default()
                    },
                    ..Default::default()
                };

                depth_image_view = logical_device
                    .create_image_view(&view_info, None)
                    .expect("Could not create depth image");
            };
        }
    }

    // CLEANUP
    unsafe {
        logical_device.device_wait_idle().expect("Could not idle");

        for idx in 0..MAX_FRAMES_IN_FLIGHT {
            logical_device.destroy_fence(fences[idx], None);
            logical_device.destroy_semaphore(present_semaphores[idx], None);
        }

        logical_device.destroy_buffer(shader_data_buffer.buffer, None);
        vk_alloc.free_memory(&mut shader_data_buffer.alloc);

        for semaphore in render_semaphores {
            logical_device.destroy_semaphore(semaphore, None);
        }

        logical_device.destroy_image(depth_image, None);
        vk_alloc.free_memory(&mut depth_image_alloc);
        logical_device.destroy_image_view(depth_image_view, None);

        for view in swapchain_image_views {
            logical_device.destroy_image_view(view, None);
        }

        logical_device.destroy_buffer(buffer, None);
        vk_alloc.free_memory(&mut buffer_alloc);

        for mut texture in textures {
            logical_device.destroy_image_view(texture.view, None);
            logical_device.destroy_sampler(texture.sampler, None);
            logical_device.destroy_image(texture.image, None);
            vk_alloc.free_memory(&mut texture.alloc);
        }

        logical_device.destroy_descriptor_set_layout(shader_data_set_layout, None);
        logical_device.destroy_descriptor_set_layout(texture_set_layout, None);
        logical_device.destroy_descriptor_pool(descriptor_pool, None);

        logical_device.destroy_pipeline_layout(pipeline_layout, None);
        logical_device.destroy_pipeline(pipeline, None);
        logical_device.destroy_pipeline_layout(outline_pipeline_layout, None);
        logical_device.destroy_pipeline(outline_pipeline, None);
        swapchain_loader.destroy_swapchain(swapchain, None);

        surface_loader.destroy_surface(surface, None);
        logical_device.destroy_command_pool(command_pool, None);

        logical_device.destroy_shader_module(vert_shader_module, None);
        logical_device.destroy_shader_module(frag_shader_module, None);
        logical_device.destroy_shader_module(outline_vert_shader_module, None);
        logical_device.destroy_shader_module(outline_frag_shader_module, None);

        logical_device.destroy_device(None);
        instance.destroy_instance(None);
    }
}

fn create_instance(entry: &Entry, display_handle: RawDisplayHandle) -> VkResult<Instance> {
    let app_info;
    if let Ok(s) = CString::new("bingus") {
        app_info = vk::ApplicationInfo {
            s_type: StructureType::APPLICATION_INFO,
            p_application_name: s.as_ptr(),
            api_version: vk::API_VERSION_1_3,
            ..Default::default()
        }
    } else {
        panic!();
    };

    let extensions = ash_window::enumerate_required_extensions(display_handle)
        .expect("Failed to enumerate required extensions");

    let layers = [c"VK_LAYER_KHRONOS_validation"];
    let layers_raw: Vec<*const c_char> = layers.iter().map(|raw_name| raw_name.as_ptr()).collect();

    let enabled_validation_features = &[vk::ValidationFeatureEnableEXT::DEBUG_PRINTF];

    let validation_features = vk::ValidationFeaturesEXT {
        s_type: StructureType::VALIDATION_FEATURES_EXT,
        enabled_validation_feature_count: 1,
        p_enabled_validation_features: enabled_validation_features.as_ptr(),
        ..Default::default()
    };

    let mut instance_info = vk::InstanceCreateInfo {
        s_type: StructureType::INSTANCE_CREATE_INFO,
        p_application_info: &app_info,
        enabled_extension_count: extensions.len() as u32,
        pp_enabled_extension_names: extensions.as_ptr(),
        enabled_layer_count: layers_raw.len() as u32,
        pp_enabled_layer_names: layers_raw.as_ptr(),
        p_next: &validation_features as *const _ as *const c_void,
        ..Default::default()
    };

    #[cfg(feature = "no-layers")]
    {
        instance_info.enabled_layer_count = 0;
        instance_info.pp_enabled_layer_names = [].as_ptr();
        instance_info.p_next = core::ptr::null();
    }

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
        descriptor_indexing: vk::TRUE,
        descriptor_binding_variable_descriptor_count: vk::TRUE,
        descriptor_binding_uniform_buffer_update_after_bind: vk::TRUE,
        runtime_descriptor_array: vk::TRUE,
        buffer_device_address: vk::TRUE,
        shader_sampled_image_array_non_uniform_indexing: vk::TRUE,
        ..Default::default()
    };

    let mut enabled_vk13_features = vk::PhysicalDeviceVulkan13Features {
        s_type: StructureType::PHYSICAL_DEVICE_VULKAN_1_3_FEATURES,
        p_next: &mut enabled_vk12_features as *mut _ as *mut c_void,
        synchronization2: vk::TRUE,
        dynamic_rendering: vk::TRUE,
        ..Default::default()
    };

    let enabled_vk10_features = vk::PhysicalDeviceFeatures {
        sampler_anisotropy: vk::TRUE,
        fill_mode_non_solid: vk::TRUE,
        wide_lines: vk::TRUE,
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

fn create_swapchain<'a>(
    logical_device: &'a Device,
    swapchain_loader: &'a khr::swapchain::Device,
    swapchain_create_info: &'a vk::SwapchainCreateInfoKHR,
    image_format: vk::Format,
) -> VkResult<(vk::SwapchainKHR, Vec<vk::Image>, Vec<vk::ImageView>)> {
    unsafe {
        let swapchain = swapchain_loader.create_swapchain(swapchain_create_info, None)?;
        let swapchain_images = swapchain_loader.get_swapchain_images(swapchain).unwrap();
        let swapchain_image_views = swapchain_images
            .iter()
            .map(|image| {
                let view_create_info = vk::ImageViewCreateInfo {
                    s_type: StructureType::IMAGE_VIEW_CREATE_INFO,
                    image: *image,
                    view_type: vk::ImageViewType::TYPE_2D,
                    format: image_format,
                    subresource_range: vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        level_count: 1,
                        layer_count: 1,
                        ..Default::default()
                    },
                    ..Default::default()
                };

                logical_device.create_image_view(&view_create_info, None)
            })
            .collect::<Result<Vec<vk::ImageView>, vk::Result>>()?;
        Ok((swapchain, swapchain_images, swapchain_image_views))
    }
}

fn get_depth_image<'a>(
    instance: &'a Instance,
    logical_device: &'a Device,
    physical_device: vk::PhysicalDevice,
    (width, height): (u32, u32),
    vk_alloc: &'a vk_mem::Allocator,
) -> VkResult<(
    vk::Image,
    vk::ImageCreateInfo<'a>,
    vk_mem::Allocation,
    vk::ImageView,
    vk::Format,
)> {
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
        extent: vk::Extent3D {
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
        subresource_range: vk::ImageSubresourceRange {
            aspect_mask: vk::ImageAspectFlags::DEPTH,
            level_count: 1_u32,
            layer_count: 1_u32,
            ..Default::default()
        },
        ..Default::default()
    };

    let image_view = unsafe { logical_device.create_image_view(&view_info, None)? };

    Ok((
        depth_image.0,
        depth_image_info,
        depth_image.1,
        image_view,
        depth_format,
    ))
}

fn get_buffer(
    vk_alloc: &vk_mem::Allocator,
    size: u64,
) -> VkResult<(vk::Buffer, vk_mem::Allocation)> {
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

fn init_shader_buffer(vk_alloc: &vk_mem::Allocator) -> ShaderDataBuffer {
    let buffer_info = vk::BufferCreateInfo {
        s_type: StructureType::BUFFER_CREATE_INFO,
        size: size_of::<ShaderData>() as u64,
        usage: vk::BufferUsageFlags::UNIFORM_BUFFER,
        ..Default::default()
    };

    let alloc_info = vk_mem::AllocationCreateInfo {
        usage: vk_mem::MemoryUsage::CpuToGpu,
        ..Default::default()
    };

    let (buffer, alloc) = unsafe { vk_alloc.create_buffer(&buffer_info, &alloc_info).unwrap() };

    let alloc_info = vk_alloc.get_allocation_info(&alloc);

    ShaderDataBuffer {
        alloc,
        alloc_info,
        buffer,
    }
}

fn init_sync_objects<'a>(
    logical_device: &'a Device,
    n_swapchain_images: usize,
    mut render_semaphores: Vec<vk::Semaphore>,
) -> VkResult<(
    [vk::Fence; MAX_FRAMES_IN_FLIGHT],
    [vk::Semaphore; MAX_FRAMES_IN_FLIGHT],
    Vec<vk::Semaphore>,
    vk::SemaphoreCreateInfo<'a>,
)> {
    let semaphore_create_info = vk::SemaphoreCreateInfo {
        s_type: StructureType::SEMAPHORE_CREATE_INFO,
        ..Default::default()
    };

    let fence_create_info = vk::FenceCreateInfo {
        s_type: StructureType::FENCE_CREATE_INFO,
        flags: vk::FenceCreateFlags::SIGNALED,
        ..Default::default()
    };

    let mut fences = [vk::Fence::null(); MAX_FRAMES_IN_FLIGHT];
    let mut present_semaphores = [vk::Semaphore::null(); MAX_FRAMES_IN_FLIGHT];

    for idx in 0..MAX_FRAMES_IN_FLIGHT {
        unsafe {
            fences[idx] = logical_device.create_fence(&fence_create_info, None)?;
            present_semaphores[idx] =
                logical_device.create_semaphore(&semaphore_create_info, None)?;
        };
    }

    render_semaphores.resize(n_swapchain_images, vk::Semaphore::null());

    for semaphore in &mut render_semaphores {
        *semaphore = unsafe { logical_device.create_semaphore(&semaphore_create_info, None) }?;
    }

    Ok((
        fences,
        present_semaphores,
        render_semaphores,
        semaphore_create_info,
    ))
}

fn create_command_buffers(
    logical_device: &Device,
    qf_idx: u32,
) -> VkResult<(vk::CommandPool, Vec<vk::CommandBuffer>)> {
    let command_pool_create_info = vk::CommandPoolCreateInfo {
        s_type: StructureType::COMMAND_POOL_CREATE_INFO,
        flags: vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER,
        queue_family_index: qf_idx,
        ..Default::default()
    };

    let command_pool =
        unsafe { logical_device.create_command_pool(&command_pool_create_info, None)? };

    let command_buffer_alloc_info = vk::CommandBufferAllocateInfo {
        s_type: StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
        command_pool,
        command_buffer_count: MAX_FRAMES_IN_FLIGHT as u32,
        ..Default::default()
    };

    let command_buffers =
        unsafe { logical_device.allocate_command_buffers(&command_buffer_alloc_info)? };

    Ok((command_pool, command_buffers))
}

fn load_tex(
    vk_alloc: &vk_mem::Allocator,
    logical_device: &Device,
    command_pool: vk::CommandPool,
    queue: vk::Queue,
) -> VkResult<(Vec<Texture>, Vec<vk::DescriptorImageInfo>)> {
    let texture_files: Vec<Ktx<_>> = vec![
        include_ktx!("../assets/suzanne0.ktx"),
        include_ktx!("../assets/suzanne1.ktx"),
        include_ktx!("../assets/suzanne2.ktx"),
    ];

    let mut textures: Vec<Texture> = Vec::new();
    let mut texture_descriptors: Vec<vk::DescriptorImageInfo> = Vec::new();

    for tex in texture_files {
        // iunno why the ktx crate doesnt just expose the raw data
        let tex_data_layers: Vec<&[u8]> = tex.textures().collect();
        let mut tex_data: Vec<u8> = Vec::new();
        for layer in tex_data_layers {
            tex_data = [&tex_data, layer].concat();
        }
        let tex_data_size = tex_data.len();

        let texture_img_create_info = vk::ImageCreateInfo {
            s_type: StructureType::IMAGE_CREATE_INFO,
            image_type: vk::ImageType::TYPE_2D,
            format: vk_format::get_vk_format(tex).unwrap(),
            extent: vk::Extent3D {
                width: tex.pixel_width(),
                height: tex.pixel_height(),
                depth: 1,
            },
            mip_levels: tex.mipmap_levels(),
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
            tiling: vk::ImageTiling::OPTIMAL,
            usage: vk::ImageUsageFlags::TRANSFER_DST | vk::ImageUsageFlags::SAMPLED,
            initial_layout: vk::ImageLayout::UNDEFINED,
            ..Default::default()
        };

        let texture_img_alloc_info = vk_mem::AllocationCreateInfo {
            usage: vk_mem::MemoryUsage::Auto,
            ..Default::default()
        };

        let (image, image_alloc) =
            unsafe { vk_alloc.create_image(&texture_img_create_info, &texture_img_alloc_info)? };
        let view_create_info = vk::ImageViewCreateInfo {
            s_type: StructureType::IMAGE_VIEW_CREATE_INFO,
            image,
            view_type: vk::ImageViewType::TYPE_2D,
            format: texture_img_create_info.format,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                level_count: tex.mipmap_levels(),
                layer_count: 1,
                ..Default::default()
            },
            ..Default::default()
        };

        let view = unsafe { logical_device.create_image_view(&view_create_info, None)? };

        let img_src_buf_create_info = vk::BufferCreateInfo {
            s_type: StructureType::BUFFER_CREATE_INFO,
            size: tex_data_size as u64,
            usage: vk::BufferUsageFlags::TRANSFER_SRC,
            ..Default::default()
        };

        let img_src_alloc_create_info = vk_mem::AllocationCreateInfo {
            usage: vk_mem::MemoryUsage::Auto,
            flags: vk_mem::AllocationCreateFlags::HOST_ACCESS_SEQUENTIAL_WRITE
                | vk_mem::AllocationCreateFlags::MAPPED,
            ..Default::default()
        };

        let (img_src_buf, mut img_src_buf_alloc) = unsafe {
            vk_alloc.create_buffer(&img_src_buf_create_info, &img_src_alloc_create_info)?
        };

        let p_texture_data = unsafe { vk_alloc.map_memory(&mut img_src_buf_alloc)? };

        unsafe {
            p_texture_data.copy_from(tex_data.as_ptr(), tex_data_size);
        }

        drop(tex_data);

        unsafe { vk_alloc.unmap_memory(&mut img_src_buf_alloc) }

        let fence_create_info = vk::FenceCreateInfo {
            s_type: StructureType::FENCE_CREATE_INFO,
            ..Default::default()
        };

        let fence = unsafe { logical_device.create_fence(&fence_create_info, None)? };

        let command_buffer_alloc_info = vk::CommandBufferAllocateInfo {
            s_type: StructureType::COMMAND_BUFFER_ALLOCATE_INFO,
            command_pool,
            command_buffer_count: 1,
            ..Default::default()
        };

        let command_buffer =
            unsafe { logical_device.allocate_command_buffers(&command_buffer_alloc_info)? }[0];

        let command_buffer_begin_info = vk::CommandBufferBeginInfo {
            s_type: StructureType::COMMAND_BUFFER_BEGIN_INFO,
            flags: vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            ..Default::default()
        };

        unsafe { logical_device.begin_command_buffer(command_buffer, &command_buffer_begin_info)? };

        let barrier_tex_img = vk::ImageMemoryBarrier2 {
            s_type: StructureType::IMAGE_MEMORY_BARRIER_2,
            src_stage_mask: vk::PipelineStageFlags2::NONE,
            src_access_mask: vk::AccessFlags2::NONE,
            dst_stage_mask: vk::PipelineStageFlags2::TRANSFER,
            dst_access_mask: vk::AccessFlags2::TRANSFER_WRITE,
            old_layout: vk::ImageLayout::UNDEFINED,
            new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            image,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                level_count: tex.mipmap_levels(),
                layer_count: 1,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut barrier_tex_info = vk::DependencyInfo {
            s_type: StructureType::DEPENDENCY_INFO,
            image_memory_barrier_count: 1,
            p_image_memory_barriers: &barrier_tex_img,
            ..Default::default()
        };

        unsafe { logical_device.cmd_pipeline_barrier2(command_buffer, &barrier_tex_info) };

        let mut copy_regions: Vec<vk::BufferImageCopy> = Vec::new();

        for i in 0..tex.mipmap_levels() {
            if let Some(mip_offset) = ktx_texture_get_offset(tex, i, 0, 0) {
                copy_regions.push(vk::BufferImageCopy {
                    buffer_offset: mip_offset,
                    image_subresource: vk::ImageSubresourceLayers {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        mip_level: i,
                        layer_count: 1,
                        ..Default::default()
                    },
                    image_extent: vk::Extent3D {
                        width: tex.pixel_width() >> i,
                        height: tex.pixel_height() >> i,
                        depth: 1,
                    },
                    ..Default::default()
                })
            } else {
                return VkResult::Err(vk::Result::ERROR_UNKNOWN);
            }
        }

        unsafe {
            logical_device.cmd_copy_buffer_to_image(
                command_buffer,
                img_src_buf,
                image,
                vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                &copy_regions,
            )
        };

        let barrier_tex_read = vk::ImageMemoryBarrier2 {
            s_type: StructureType::IMAGE_MEMORY_BARRIER_2,
            src_stage_mask: vk::PipelineStageFlags2::TRANSFER,
            src_access_mask: vk::AccessFlags2::TRANSFER_WRITE,
            dst_stage_mask: vk::PipelineStageFlags2::FRAGMENT_SHADER,
            dst_access_mask: vk::AccessFlags2::SHADER_READ,
            old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            new_layout: vk::ImageLayout::READ_ONLY_OPTIMAL,
            image,
            subresource_range: vk::ImageSubresourceRange {
                aspect_mask: vk::ImageAspectFlags::COLOR,
                level_count: tex.mipmap_levels(),
                layer_count: 1,
                ..Default::default()
            },
            ..Default::default()
        };

        barrier_tex_info.p_image_memory_barriers = &barrier_tex_read;

        unsafe {
            logical_device.cmd_pipeline_barrier2(command_buffer, &barrier_tex_info);
            logical_device.end_command_buffer(command_buffer)?;
        };

        let submit_info = vk::SubmitInfo {
            s_type: StructureType::SUBMIT_INFO,
            command_buffer_count: 1,
            p_command_buffers: &command_buffer,
            ..Default::default()
        };

        unsafe {
            logical_device.queue_submit(queue, &[submit_info], fence)?;
            logical_device.wait_for_fences(&[fence], true, u64::MAX)?;
        }

        let sampler_create_info = vk::SamplerCreateInfo {
            s_type: StructureType::SAMPLER_CREATE_INFO,
            mag_filter: vk::Filter::LINEAR,
            min_filter: vk::Filter::LINEAR,
            mipmap_mode: vk::SamplerMipmapMode::LINEAR,
            anisotropy_enable: vk::TRUE,
            max_anisotropy: 8.0,
            max_lod: tex.mipmap_levels() as f32,
            ..Default::default()
        };

        let sampler = unsafe { logical_device.create_sampler(&sampler_create_info, None)? };

        textures.push(Texture {
            alloc: image_alloc,
            image,
            view,
            sampler,
        });

        texture_descriptors.push(vk::DescriptorImageInfo {
            sampler,
            image_view: view,
            image_layout: vk::ImageLayout::READ_ONLY_OPTIMAL,
        });

        unsafe {
            logical_device.destroy_fence(fence, None);
            vk_alloc.free_memory(&mut img_src_buf_alloc);
            logical_device.destroy_buffer(img_src_buf, None);
        };
    }

    Ok((textures, texture_descriptors))
}

fn setup_descriptors(
    logical_device: &Device,
    textures: &[Texture],
    texture_descriptors: &[vk::DescriptorImageInfo],
    shader_data_buffer: &ShaderDataBuffer,
) -> VkResult<(
    vk::DescriptorSet,
    vk::DescriptorSetLayout,
    vk::DescriptorSet,
    vk::DescriptorSetLayout,
    vk::DescriptorPool,
)> {
    let desc_variable_flag = vk::DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT;

    let texture_binding_flags = vk::DescriptorSetLayoutBindingFlagsCreateInfo {
        s_type: StructureType::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO,
        binding_count: 1,
        p_binding_flags: &desc_variable_flag,
        ..Default::default()
    };

    let desc_update_flag = vk::DescriptorBindingFlags::UPDATE_AFTER_BIND;

    let shader_binding_flags = vk::DescriptorSetLayoutBindingFlagsCreateInfo {
        s_type: StructureType::DESCRIPTOR_SET_LAYOUT_BINDING_FLAGS_CREATE_INFO,
        binding_count: 1,
        p_binding_flags: &desc_update_flag,
        ..Default::default()
    };

    // Bindings in separate sets since shader data
    // is updated per frame
    let shader_data_binding = vk::DescriptorSetLayoutBinding {
        descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
        descriptor_count: 1,
        binding: 0,
        stage_flags: vk::ShaderStageFlags::VERTEX,
        ..Default::default()
    };

    let texture_binding = vk::DescriptorSetLayoutBinding {
        descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
        descriptor_count: textures.len() as u32,
        binding: 0,
        stage_flags: vk::ShaderStageFlags::FRAGMENT,
        ..Default::default()
    };

    let shader_data_set_layout_create_info = vk::DescriptorSetLayoutCreateInfo {
        s_type: StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        flags: vk::DescriptorSetLayoutCreateFlags::UPDATE_AFTER_BIND_POOL,
        binding_count: 1,
        p_bindings: &shader_data_binding,
        p_next: &shader_binding_flags as *const _ as *const c_void,
        ..Default::default()
    };

    let texture_set_layout_create_info = vk::DescriptorSetLayoutCreateInfo {
        s_type: StructureType::DESCRIPTOR_SET_LAYOUT_CREATE_INFO,
        binding_count: 1,
        p_bindings: &texture_binding,
        p_next: &texture_binding_flags as *const _ as *const c_void,
        ..Default::default()
    };

    let (shader_data_set_layout, texture_set_layout) = unsafe {
        (
            logical_device
                .create_descriptor_set_layout(&shader_data_set_layout_create_info, None)?,
            logical_device.create_descriptor_set_layout(&texture_set_layout_create_info, None)?,
        )
    };

    let pool_sizes = &[
        // Shader data
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::UNIFORM_BUFFER,
            descriptor_count: 1,
        },
        vk::DescriptorPoolSize {
            ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            descriptor_count: textures.len() as u32,
        },
    ];

    let pool_create_info = vk::DescriptorPoolCreateInfo {
        s_type: StructureType::DESCRIPTOR_POOL_CREATE_INFO,
        flags: vk::DescriptorPoolCreateFlags::UPDATE_AFTER_BIND,
        max_sets: 2,
        pool_size_count: 2,
        p_pool_sizes: pool_sizes.as_ptr(),
        ..Default::default()
    };

    let descriptor_pool =
        unsafe { logical_device.create_descriptor_pool(&pool_create_info, None)? };

    let variable_desc_count = textures.len() as u32;
    let variable_desc_count_alloc_info = vk::DescriptorSetVariableDescriptorCountAllocateInfo {
        s_type: StructureType::DESCRIPTOR_SET_VARIABLE_DESCRIPTOR_COUNT_ALLOCATE_INFO,
        descriptor_set_count: 2,
        p_descriptor_counts: ([1, variable_desc_count]).as_ptr(),
        ..Default::default()
    };

    let descriptor_set_allocate_info = vk::DescriptorSetAllocateInfo {
        s_type: StructureType::DESCRIPTOR_SET_ALLOCATE_INFO,
        p_next: &variable_desc_count_alloc_info as *const _ as *const c_void,
        descriptor_pool,
        descriptor_set_count: 2,
        p_set_layouts: [shader_data_set_layout, texture_set_layout].as_ptr(),
        ..Default::default()
    };

    let descriptor_sets =
        unsafe { logical_device.allocate_descriptor_sets(&descriptor_set_allocate_info)? };

    let (shader_data_desc_set, texture_desc_set) = (descriptor_sets[0], descriptor_sets[1]);

    let shader_buffer_info = vk::DescriptorBufferInfo {
        buffer: shader_data_buffer.buffer,
        offset: 0,
        range: size_of::<ShaderData>() as u64,
    };

    let write_desc_sets = &[
        // Shader data
        vk::WriteDescriptorSet {
            s_type: StructureType::WRITE_DESCRIPTOR_SET,
            dst_set: shader_data_desc_set,
            dst_binding: 0,
            descriptor_count: 1,
            descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
            p_buffer_info: &shader_buffer_info,
            ..Default::default()
        },
        vk::WriteDescriptorSet {
            s_type: StructureType::WRITE_DESCRIPTOR_SET,
            dst_set: texture_desc_set,
            dst_binding: 0,
            descriptor_count: texture_descriptors.len() as u32,
            descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            p_image_info: texture_descriptors.as_ptr(),
            ..Default::default()
        },
    ];

    unsafe {
        logical_device.update_descriptor_sets(write_desc_sets, &[]);
    };

    Ok((
        shader_data_desc_set,
        shader_data_set_layout,
        texture_desc_set,
        texture_set_layout,
        descriptor_pool,
    ))
}

fn update_shader_data_descriptor(
    logical_device: &Device,
    vk_alloc: &vk_mem::Allocator,
    shader_data_set: vk::DescriptorSet,
    shader_data: &ShaderData,
    shader_data_buffer: &mut ShaderDataBuffer,
) -> VkResult<()> {
    let p_shader_buffer = unsafe { vk_alloc.map_memory(&mut shader_data_buffer.alloc)? };

    unsafe {
        p_shader_buffer.copy_from(
            shader_data as *const _ as *const u8,
            size_of::<ShaderData>(),
        );
    }

    let shader_buffer_info = vk::DescriptorBufferInfo {
        buffer: shader_data_buffer.buffer,
        offset: 0,
        range: size_of::<ShaderData>() as u64,
    };

    let write_desc_set = vk::WriteDescriptorSet {
        s_type: StructureType::WRITE_DESCRIPTOR_SET,
        dst_set: shader_data_set,
        dst_binding: 0,
        descriptor_count: 1,
        descriptor_type: vk::DescriptorType::UNIFORM_BUFFER,
        p_buffer_info: &shader_buffer_info,
        ..Default::default()
    };

    unsafe {
        logical_device.update_descriptor_sets(&[write_desc_set], &[]);
    };

    unsafe { vk_alloc.unmap_memory(&mut shader_data_buffer.alloc) }

    Ok(())
}

fn setup_main_pipeline(
    logical_device: &Device,
    vert_shader_module: vk::ShaderModule,
    frag_shader_module: vk::ShaderModule,
    descriptor_set_layouts: &[vk::DescriptorSetLayout],
    image_format: vk::Format,
    depth_format: vk::Format,
) -> VkResult<(vk::Pipeline, vk::PipelineLayout)> {
    let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo {
        s_type: StructureType::PIPELINE_LAYOUT_CREATE_INFO,
        set_layout_count: descriptor_set_layouts.len() as u32,
        p_set_layouts: descriptor_set_layouts.as_ptr(),
        ..Default::default()
    };

    let pipeline_layout =
        unsafe { logical_device.create_pipeline_layout(&pipeline_layout_create_info, None)? };

    let vertex_binding = vk::VertexInputBindingDescription {
        binding: 0,
        stride: size_of::<Vertex>() as u32,
        input_rate: vk::VertexInputRate::VERTEX,
    };

    let vertex_attributes = [
        vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32B32_SFLOAT,
            ..Default::default()
        },
        vk::VertexInputAttributeDescription {
            location: 1,
            binding: 0,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Vertex, normal) as u32,
        },
        vk::VertexInputAttributeDescription {
            location: 2,
            binding: 0,
            format: vk::Format::R32G32_SFLOAT,
            offset: offset_of!(Vertex, uv) as u32,
        },
    ];

    let vertex_input_state = vk::PipelineVertexInputStateCreateInfo {
        s_type: StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        vertex_binding_description_count: 1,
        p_vertex_binding_descriptions: &vertex_binding,
        vertex_attribute_description_count: vertex_attributes.len() as u32,
        p_vertex_attribute_descriptions: vertex_attributes.as_ptr(),
        ..Default::default()
    };

    let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo {
        s_type: StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
        topology: vk::PrimitiveTopology::TRIANGLE_LIST,
        ..Default::default()
    };

    let shader_stages = [
        vk::PipelineShaderStageCreateInfo {
            s_type: StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            stage: vk::ShaderStageFlags::VERTEX,
            module: vert_shader_module,
            p_name: c"main".as_ptr(),
            ..Default::default()
        },
        vk::PipelineShaderStageCreateInfo {
            s_type: StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            stage: vk::ShaderStageFlags::FRAGMENT,
            module: frag_shader_module,
            p_name: c"main".as_ptr(),
            ..Default::default()
        },
    ];

    let viewport_state = vk::PipelineViewportStateCreateInfo {
        s_type: StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
        viewport_count: 1,
        scissor_count: 1,
        ..Default::default()
    };

    let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];

    let dynamic_state = vk::PipelineDynamicStateCreateInfo {
        s_type: StructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO,
        dynamic_state_count: 2,
        p_dynamic_states: dynamic_states.as_ptr(),
        ..Default::default()
    };

    let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo {
        s_type: StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
        depth_test_enable: vk::TRUE,
        depth_write_enable: vk::TRUE,
        depth_compare_op: vk::CompareOp::LESS_OR_EQUAL,
        ..Default::default()
    };

    let rendering_create_info = vk::PipelineRenderingCreateInfo {
        s_type: StructureType::PIPELINE_RENDERING_CREATE_INFO,
        color_attachment_count: 1,
        p_color_attachment_formats: &image_format,
        depth_attachment_format: depth_format,
        ..Default::default()
    };

    let blend_attachment = vk::PipelineColorBlendAttachmentState {
        color_write_mask: vk::ColorComponentFlags::RGBA,
        ..Default::default()
    };

    let colour_blend_state = vk::PipelineColorBlendStateCreateInfo {
        s_type: StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
        attachment_count: 1,
        p_attachments: &blend_attachment,
        ..Default::default()
    };

    let raster_state = vk::PipelineRasterizationStateCreateInfo {
        s_type: StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
        line_width: 1.0,
        polygon_mode: vk::PolygonMode::FILL,
        cull_mode: vk::CullModeFlags::BACK,
        ..Default::default()
    };

    let multisample_state = vk::PipelineMultisampleStateCreateInfo {
        s_type: StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
        rasterization_samples: vk::SampleCountFlags::TYPE_1,
        ..Default::default()
    };

    let pipeline_create_info = vk::GraphicsPipelineCreateInfo {
        s_type: StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
        p_next: &rendering_create_info as *const _ as *const c_void,
        stage_count: 2,
        p_stages: shader_stages.as_ptr(),
        p_vertex_input_state: &vertex_input_state,
        p_input_assembly_state: &input_assembly_state,
        p_viewport_state: &viewport_state,
        p_rasterization_state: &raster_state,
        p_multisample_state: &multisample_state,
        p_depth_stencil_state: &depth_stencil_state,
        p_color_blend_state: &colour_blend_state,
        p_dynamic_state: &dynamic_state,
        layout: pipeline_layout,
        ..Default::default()
    };

    Ok((
        unsafe {
            logical_device
                .create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_create_info], None)
                .map_err(|e| e.1)?[0]
        },
        pipeline_layout,
    ))
}

fn setup_outline_pipeline(
    logical_device: &Device,
    vert_shader_module: vk::ShaderModule,
    frag_shader_module: vk::ShaderModule,
    descriptor_set_layouts: &[vk::DescriptorSetLayout],
    image_format: vk::Format,
    depth_format: vk::Format,
) -> VkResult<(vk::Pipeline, vk::PipelineLayout)> {
    let pipeline_layout_create_info = vk::PipelineLayoutCreateInfo {
        s_type: StructureType::PIPELINE_LAYOUT_CREATE_INFO,
        set_layout_count: descriptor_set_layouts.len() as u32,
        p_set_layouts: descriptor_set_layouts.as_ptr(),
        ..Default::default()
    };

    let pipeline_layout =
        unsafe { logical_device.create_pipeline_layout(&pipeline_layout_create_info, None)? };

    let vertex_binding = vk::VertexInputBindingDescription {
        binding: 0,
        stride: size_of::<Vertex>() as u32,
        input_rate: vk::VertexInputRate::VERTEX,
    };

    let vertex_attributes = [
        vk::VertexInputAttributeDescription {
            location: 0,
            binding: 0,
            format: vk::Format::R32G32B32_SFLOAT,
            ..Default::default()
        },
        vk::VertexInputAttributeDescription {
            location: 1,
            binding: 0,
            format: vk::Format::R32G32B32_SFLOAT,
            offset: offset_of!(Vertex, normal) as u32,
        },
        vk::VertexInputAttributeDescription {
            location: 2,
            binding: 0,
            format: vk::Format::R32G32_SFLOAT,
            offset: offset_of!(Vertex, uv) as u32,
        },
    ];

    let vertex_input_state = vk::PipelineVertexInputStateCreateInfo {
        s_type: StructureType::PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO,
        vertex_binding_description_count: 1,
        p_vertex_binding_descriptions: &vertex_binding,
        vertex_attribute_description_count: vertex_attributes.len() as u32,
        p_vertex_attribute_descriptions: vertex_attributes.as_ptr(),
        ..Default::default()
    };

    let input_assembly_state = vk::PipelineInputAssemblyStateCreateInfo {
        s_type: StructureType::PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO,
        topology: vk::PrimitiveTopology::TRIANGLE_LIST,
        ..Default::default()
    };

    let shader_stages = [
        vk::PipelineShaderStageCreateInfo {
            s_type: StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            stage: vk::ShaderStageFlags::VERTEX,
            module: vert_shader_module,
            p_name: c"main".as_ptr(),
            ..Default::default()
        },
        vk::PipelineShaderStageCreateInfo {
            s_type: StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
            stage: vk::ShaderStageFlags::FRAGMENT,
            module: frag_shader_module,
            p_name: c"main".as_ptr(),
            ..Default::default()
        },
    ];

    let viewport_state = vk::PipelineViewportStateCreateInfo {
        s_type: StructureType::PIPELINE_VIEWPORT_STATE_CREATE_INFO,
        viewport_count: 1,
        scissor_count: 1,
        ..Default::default()
    };

    let dynamic_states = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];

    let dynamic_state = vk::PipelineDynamicStateCreateInfo {
        s_type: StructureType::PIPELINE_DYNAMIC_STATE_CREATE_INFO,
        dynamic_state_count: 2,
        p_dynamic_states: dynamic_states.as_ptr(),
        ..Default::default()
    };

    let depth_stencil_state = vk::PipelineDepthStencilStateCreateInfo {
        s_type: StructureType::PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO,
        depth_test_enable: vk::TRUE,
        depth_write_enable: vk::TRUE,
        depth_compare_op: vk::CompareOp::LESS_OR_EQUAL,
        ..Default::default()
    };

    let rendering_create_info = vk::PipelineRenderingCreateInfo {
        s_type: StructureType::PIPELINE_RENDERING_CREATE_INFO,
        color_attachment_count: 1,
        p_color_attachment_formats: &image_format,
        depth_attachment_format: depth_format,
        ..Default::default()
    };

    let blend_attachment = vk::PipelineColorBlendAttachmentState {
        color_write_mask: vk::ColorComponentFlags::RGBA,
        ..Default::default()
    };

    let colour_blend_state = vk::PipelineColorBlendStateCreateInfo {
        s_type: StructureType::PIPELINE_COLOR_BLEND_STATE_CREATE_INFO,
        attachment_count: 1,
        p_attachments: &blend_attachment,
        ..Default::default()
    };

    let raster_state = vk::PipelineRasterizationStateCreateInfo {
        s_type: StructureType::PIPELINE_RASTERIZATION_STATE_CREATE_INFO,
        line_width: 5.0,
        polygon_mode: vk::PolygonMode::LINE,
        cull_mode: vk::CullModeFlags::FRONT,
        ..Default::default()
    };

    let multisample_state = vk::PipelineMultisampleStateCreateInfo {
        s_type: StructureType::PIPELINE_MULTISAMPLE_STATE_CREATE_INFO,
        rasterization_samples: vk::SampleCountFlags::TYPE_1,
        ..Default::default()
    };

    let pipeline_create_info = vk::GraphicsPipelineCreateInfo {
        s_type: StructureType::GRAPHICS_PIPELINE_CREATE_INFO,
        p_next: &rendering_create_info as *const _ as *const c_void,
        stage_count: 2,
        p_stages: shader_stages.as_ptr(),
        p_vertex_input_state: &vertex_input_state,
        p_input_assembly_state: &input_assembly_state,
        p_viewport_state: &viewport_state,
        p_rasterization_state: &raster_state,
        p_multisample_state: &multisample_state,
        p_depth_stencil_state: &depth_stencil_state,
        p_color_blend_state: &colour_blend_state,
        p_dynamic_state: &dynamic_state,
        layout: pipeline_layout,
        ..Default::default()
    };

    Ok((
        unsafe {
            logical_device
                .create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_create_info], None)
                .map_err(|e| e.1)?[0]
        },
        pipeline_layout,
    ))
}

fn calculate_shader_data(win_size: (u32, u32), state: &AppState) -> ShaderData {
    let proj = nalgebra_glm::perspective(
        win_size.0 as f32 / win_size.1 as f32,
        nalgebra_glm::radians(&nalgebra_glm::vec1(45.0)).x,
        0.1,
        32.0,
    );

    let view = nalgebra_glm::translate(&nalgebra_glm::Mat4::identity(), state.cam_pos);

    let model: [nalgebra_glm::Mat4; 3] = array::from_fn(|idx| {
        let instance_pos = nalgebra_glm::vec3((idx as f32 - 1.0) * 3.0, 0.0, 0.0);

        let before_rot = nalgebra_glm::translate(&nalgebra_glm::Mat4::identity(), &instance_pos);
        let rot_x = nalgebra_glm::rotate_x(&before_rot, state.obj_rotations[idx].x);
        let rot_y = nalgebra_glm::rotate_y(&rot_x, state.obj_rotations[idx].y);
        nalgebra_glm::rotate_z(&rot_y, state.obj_rotations[idx].z)
    });

    ShaderData {
        proj,
        view,
        model,
        light_pos: nalgebra_glm::vec4(0.0, -10.0, 10.0, 0.0),
        selected: state.selected,
        shininess: state.shininess,
    }
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
