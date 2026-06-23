use ash::vk::{self, StructureType};

#[macro_export]
macro_rules! include_shader_module {
    // path, stage, device
    ( $p:literal, $s:ident, $d:ident) => {
        unsafe {
            $d.create_shader_module(
                &crate::shader::create_shader_info(
                    include_spirv!($p, glsl, $s, vulkan1_2)), None)
        }
    };
}

pub fn create_shader_info(spv: &[u32]) -> vk::ShaderModuleCreateInfo {
    vk::ShaderModuleCreateInfo {
        s_type: StructureType::SHADER_MODULE_CREATE_INFO,
        code_size: spv.len() * size_of::<u32>(),
        p_code: spv.as_ptr(),
        ..Default::default()
    }
}
