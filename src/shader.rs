use ash::{
    Device, Entry, Instance, khr,
    prelude::VkResult,
    vk::{self, StructureType},
};

use inline_spirv::include_spirv;

pub fn load_shader_module(
    logical_device: &Device,
) -> VkResult<(vk::ShaderModule, vk::ShaderModule)> {
    let vert_spv: &[u32] = include_spirv!("shaders/shader.vert", glsl, vert, vulkan1_2);
    let frag_spv: &[u32] = include_spirv!("shaders/shader.frag", glsl, frag, vulkan1_2);

    unsafe {
        Ok((
            logical_device.create_shader_module(&create_shader_info(vert_spv), None)?,
            logical_device.create_shader_module(&create_shader_info(frag_spv), None)?,
        ))
    }
}

fn create_shader_info(spv: &[u32]) -> vk::ShaderModuleCreateInfo {
    vk::ShaderModuleCreateInfo {
        s_type: StructureType::SHADER_MODULE_CREATE_INFO,
        code_size: spv.len() * size_of::<u32>(),
        p_code: spv.as_ptr(),
        ..Default::default()
    }
}
