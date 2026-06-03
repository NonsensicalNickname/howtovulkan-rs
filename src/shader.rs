use ash::{
    Device, Entry, Instance, khr,
    prelude::VkResult,
    vk::{self, StructureType},
};

use inline_spirv::include_spirv;

pub fn load_shader_module() -> VkResult<()> {
    let frag_spv = include_spirv!("assets/shader.frag", frag, glsl);
    let vert_spv = include_spirv!("assets/shader.vert", vert, glsl);

    let shader_module_create_info = vk::ShaderModuleCreateInfo {
        s_type: StructureType::SHADER_MODULE_CREATE_INFO,
        //code_size: ,
        //p_code: ,
        ..Default::default()
    };

    Ok(())
}
