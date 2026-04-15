#![allow(warnings)]

use crate::gl_format::*;
use ash::vk::Format;
use ktx::{Ktx, KtxInfo};

pub fn get_vk_format(texture: Ktx<&[u8]>) -> Option<Format> {
    Some(
        get_vk_format_from_gl_internal(texture.gl_internal_format()).unwrap_or(
            get_vk_format_from_gl(texture.gl_format(), texture.gl_type())?,
        ),
    )
}

pub fn get_vk_format_from_gl_internal(format: u32) -> Option<Format> {
    let vk_format = match format {
        //
        // 8 bits per component
        //
        GL_R8 => Format::R8_UNORM, // 1-component, 8-bit unsigned normalized
        GL_RG8 => Format::R8G8_UNORM, // 2-component, 8-bit unsigned normalized
        GL_RGB8 => Format::R8G8B8_UNORM, // 3-component, 8-bit unsigned normalized
        GL_RGBA8 => Format::R8G8B8A8_UNORM, // 4-component, 8-bit unsigned normalized

        GL_R8_SNORM => Format::R8_SNORM, // 1-component, 8-bit signed normalized
        GL_RG8_SNORM => Format::R8G8_SNORM, // 2-component, 8-bit signed normalized
        GL_RGB8_SNORM => Format::R8G8B8_SNORM, // 3-component, 8-bit signed normalized
        GL_RGBA8_SNORM => Format::R8G8B8A8_SNORM, // 4-component, 8-bit signed normalized

        GL_R8UI => Format::R8_UINT, // 1-component, 8-bit unsigned integer
        GL_RG8UI => Format::R8G8_UINT, // 2-component, 8-bit unsigned integer
        GL_RGB8UI => Format::R8G8B8_UINT, // 3-component, 8-bit unsigned integer
        GL_RGBA8UI => Format::R8G8B8A8_UINT, // 4-component, 8-bit unsigned integer

        GL_R8I => Format::R8_SINT,    // 1-component, 8-bit signed integer
        GL_RG8I => Format::R8G8_SINT, // 2-component, 8-bit signed integer
        GL_RGB8I => Format::R8G8B8_SINT, // 3-component, 8-bit signed integer
        GL_RGBA8I => Format::R8G8B8A8_SINT, // 4-component, 8-bit signed integer

        GL_SR8 => Format::R8_SRGB,       // 1-component, 8-bit sRGB
        GL_SRG8 => Format::R8G8_SRGB,    // 2-component, 8-bit sRGB
        GL_SRGB8 => Format::R8G8B8_SRGB, // 3-component, 8-bit sRGB
        GL_SRGB8_ALPHA8 => Format::R8G8B8A8_SRGB, // 4-component, 8-bit sRGB

        //
        // 16 bits per component
        //
        GL_R16 => Format::R16_UNORM, // 1-component, 16-bit unsigned normalized
        GL_RG16 => Format::R16G16_UNORM, // 2-component, 16-bit unsigned normalized
        GL_RGB16 => Format::R16G16B16_UNORM, // 3-component, 16-bit unsigned normalized
        GL_RGBA16 => Format::R16G16B16A16_UNORM, // 4-component, 16-bit unsigned normalized

        GL_R16_SNORM => Format::R16_SNORM, // 1-component, 16-bit signed normalized
        GL_RG16_SNORM => Format::R16G16_SNORM, // 2-component, 16-bit signed normalized
        GL_RGB16_SNORM => Format::R16G16B16_SNORM, // 3-component, 16-bit signed normalized
        GL_RGBA16_SNORM => Format::R16G16B16A16_SNORM, // 4-component, 16-bit signed normalized

        GL_R16UI => Format::R16_UINT, // 1-component, 16-bit unsigned integer
        GL_RG16UI => Format::R16G16_UINT, // 2-component, 16-bit unsigned integer
        GL_RGB16UI => Format::R16G16B16_UINT, // 3-component, 16-bit unsigned integer
        GL_RGBA16UI => Format::R16G16B16A16_UINT, // 4-component, 16-bit unsigned integer

        GL_R16I => Format::R16_SINT, // 1-component, 16-bit signed integer
        GL_RG16I => Format::R16G16_SINT, // 2-component, 16-bit signed integer
        GL_RGB16I => Format::R16G16B16_SINT, // 3-component, 16-bit signed integer
        GL_RGBA16I => Format::R16G16B16A16_SINT, // 4-component, 16-bit signed integer

        GL_R16F => Format::R16_SFLOAT, // 1-component, 16-bit floating-point
        GL_RG16F => Format::R16G16_SFLOAT, // 2-component, 16-bit floating-point
        GL_RGB16F => Format::R16G16B16_SFLOAT, // 3-component, 16-bit floating-point
        GL_RGBA16F => Format::R16G16B16A16_SFLOAT, // 4-component, 16-bit floating-point

        //
        // 32 bits per component
        //
        GL_R32UI => Format::R32_UINT, // 1-component, 32-bit unsigned integer
        GL_RG32UI => Format::R32G32_UINT, // 2-component, 32-bit unsigned integer
        GL_RGB32UI => Format::R32G32B32_UINT, // 3-component, 32-bit unsigned integer
        GL_RGBA32UI => Format::R32G32B32A32_UINT, // 4-component, 32-bit unsigned integer

        GL_R32I => Format::R32_SINT, // 1-component, 32-bit signed integer
        GL_RG32I => Format::R32G32_SINT, // 2-component, 32-bit signed integer
        GL_RGB32I => Format::R32G32B32_SINT, // 3-component, 32-bit signed integer
        GL_RGBA32I => Format::R32G32B32A32_SINT, // 4-component, 32-bit signed integer

        GL_R32F => Format::R32_SFLOAT, // 1-component, 32-bit floating-point
        GL_RG32F => Format::R32G32_SFLOAT, // 2-component, 32-bit floating-point
        GL_RGB32F => Format::R32G32B32_SFLOAT, // 3-component, 32-bit floating-point
        GL_RGBA32F => Format::R32G32B32A32_SFLOAT, // 4-component, 32-bit floating-point

        //
        // Packed
        //
        GL_R3_G3_B2 => Format::UNDEFINED, // 3-component 3:3:2,       unsigned normalized
        GL_RGB4 => Format::UNDEFINED,     // 3-component 4:4:4,       unsigned normalized
        GL_RGB5 => Format::R5G5B5A1_UNORM_PACK16, // 3-component 5:5:5,       unsigned normalized
        GL_RGB565 => Format::R5G6B5_UNORM_PACK16, // 3-component 5:6:5,       unsigned normalized
        GL_RGB10 => Format::A2R10G10B10_UNORM_PACK32, // 3-component 10:10:10,    unsigned normalized
        GL_RGB12 => Format::UNDEFINED, // 3-component 12:12:12,    unsigned normalized
        GL_RGBA2 => Format::UNDEFINED, // 4-component 2:2:2:2,     unsigned normalized
        GL_RGBA4 => Format::R4G4B4A4_UNORM_PACK16, // 4-component 4:4:4:4,     unsigned normalized
        GL_RGBA12 => Format::UNDEFINED, // 4-component 12:12:12:12, unsigned normalized
        GL_RGB5_A1 => Format::A1R5G5B5_UNORM_PACK16, // 4-component 5:5:5:1,     unsigned normalized
        GL_RGB10_A2 => Format::A2R10G10B10_UNORM_PACK32, // 4-component 10:10:10:2,  unsigned normalized
        GL_RGB10_A2UI => Format::A2R10G10B10_UINT_PACK32, // 4-component 10:10:10:2,  unsigned integer
        GL_R11F_G11F_B10F => Format::B10G11R11_UFLOAT_PACK32, // 3-component 11:11:10,    floating-point
        GL_RGB9_E5 => Format::E5B9G9R9_UFLOAT_PACK32, // 3-component/exp 9:9:9/5, floating-point

        //
        // S3TC/DXT/BC
        //
        GL_COMPRESSED_RGB_S3TC_DXT1_EXT => Format::BC1_RGB_UNORM_BLOCK, // line through 3D space, 4x4 blocks, unsigned normalized
        GL_COMPRESSED_RGBA_S3TC_DXT1_EXT => Format::BC1_RGBA_UNORM_BLOCK, // line through 3D space plus 1-bit alpha, 4x4 blocks, unsigned normalized
        GL_COMPRESSED_RGBA_S3TC_DXT3_EXT => Format::BC2_UNORM_BLOCK, // line through 3D space plus line through 1D space, 4x4 blocks, unsigned normalized
        GL_COMPRESSED_RGBA_S3TC_DXT5_EXT => Format::BC3_UNORM_BLOCK, // line through 3D space plus 4-bit alpha, 4x4 blocks, unsigned normalized

        GL_COMPRESSED_SRGB_S3TC_DXT1_EXT => Format::BC1_RGB_SRGB_BLOCK, // line through 3D space, 4x4 blocks, sRGB
        GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT => Format::BC1_RGBA_SRGB_BLOCK, // line through 3D space plus 1-bit alpha, 4x4 blocks, sRGB
        GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT => Format::BC2_SRGB_BLOCK, // line through 3D space plus line through 1D space, 4x4 blocks, sRGB
        GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT => Format::BC3_SRGB_BLOCK, // line through 3D space plus 4-bit alpha, 4x4 blocks, sRGB

        GL_COMPRESSED_LUMINANCE_LATC1_EXT => Format::BC4_UNORM_BLOCK, // line through 1D space, 4x4 blocks, unsigned normalized
        GL_COMPRESSED_LUMINANCE_ALPHA_LATC2_EXT => Format::BC5_UNORM_BLOCK, // two lines through 1D space, 4x4 blocks, unsigned normalized
        GL_COMPRESSED_SIGNED_LUMINANCE_LATC1_EXT => Format::BC4_SNORM_BLOCK, // line through 1D space, 4x4 blocks, signed normalized
        GL_COMPRESSED_SIGNED_LUMINANCE_ALPHA_LATC2_EXT => Format::BC5_SNORM_BLOCK, // two lines through 1D space, 4x4 blocks, signed normalized

        GL_COMPRESSED_RED_RGTC1 => Format::BC4_UNORM_BLOCK, // line through 1D space, 4x4 blocks, unsigned normalized
        GL_COMPRESSED_RG_RGTC2 => Format::BC5_UNORM_BLOCK, // two lines through 1D space, 4x4 blocks, unsigned normalized
        GL_COMPRESSED_SIGNED_RED_RGTC1 => Format::BC4_SNORM_BLOCK, // line through 1D space, 4x4 blocks, signed normalized
        GL_COMPRESSED_SIGNED_RG_RGTC2 => Format::BC5_SNORM_BLOCK, // two lines through 1D space, 4x4 blocks, signed normalized

        GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT => Format::BC6H_UFLOAT_BLOCK, // 3-component, 4x4 blocks, unsigned floating-point
        GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT => Format::BC6H_SFLOAT_BLOCK, // 3-component, 4x4 blocks, signed floating-point
        GL_COMPRESSED_RGBA_BPTC_UNORM => Format::BC7_UNORM_BLOCK, // 4-component, 4x4 blocks, unsigned normalized
        GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM => Format::BC7_SRGB_BLOCK, // 4-component, 4x4 blocks, sRGB

        //
        // ETC
        //
        GL_ETC1_RGB8_OES => Format::ETC2_R8G8B8_UNORM_BLOCK, // 3-component ETC1, 4x4 blocks, unsigned normalized

        GL_COMPRESSED_RGB8_ETC2 => Format::ETC2_R8G8B8_UNORM_BLOCK, // 3-component ETC2, 4x4 blocks, unsigned normalized
        GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2 => Format::ETC2_R8G8B8A1_UNORM_BLOCK, // 4-component ETC2 with 1-bit alpha, 4x4 blocks, unsigned normalized
        GL_COMPRESSED_RGBA8_ETC2_EAC => Format::ETC2_R8G8B8A8_UNORM_BLOCK, // 4-component ETC2, 4x4 blocks, unsigned normalized

        GL_COMPRESSED_SRGB8_ETC2 => Format::ETC2_R8G8B8_SRGB_BLOCK, // 3-component ETC2, 4x4 blocks, sRGB
        GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2 => Format::ETC2_R8G8B8A1_SRGB_BLOCK, // 4-component ETC2 with 1-bit alpha, 4x4 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC => Format::ETC2_R8G8B8A8_SRGB_BLOCK, // 4-component ETC2, 4x4 blocks, sRGB

        GL_COMPRESSED_R11_EAC => Format::EAC_R11_UNORM_BLOCK, // 1-component ETC, 4x4 blocks, unsigned normalized
        GL_COMPRESSED_RG11_EAC => Format::EAC_R11G11_UNORM_BLOCK, // 2-component ETC, 4x4 blocks, unsigned normalized
        GL_COMPRESSED_SIGNED_R11_EAC => Format::EAC_R11_SNORM_BLOCK, // 1-component ETC, 4x4 blocks, signed normalized
        GL_COMPRESSED_SIGNED_RG11_EAC => Format::EAC_R11G11_SNORM_BLOCK, // 2-component ETC, 4x4 blocks, signed normalized

        //
        // PVRTC
        //
        GL_COMPRESSED_RGB_PVRTC_2BPPV1_IMG => Format::PVRTC1_2BPP_UNORM_BLOCK_IMG, // 3-component PVRTC, 16x8 blocks, unsigned normalized
        GL_COMPRESSED_RGB_PVRTC_4BPPV1_IMG => Format::PVRTC1_4BPP_UNORM_BLOCK_IMG, // 3-component PVRTC,  8x8 blocks, unsigned normalized
        GL_COMPRESSED_RGBA_PVRTC_2BPPV1_IMG => Format::PVRTC1_2BPP_UNORM_BLOCK_IMG, // 4-component PVRTC, 16x8 blocks, unsigned normalized
        GL_COMPRESSED_RGBA_PVRTC_4BPPV1_IMG => Format::PVRTC1_4BPP_UNORM_BLOCK_IMG, // 4-component PVRTC,  8x8 blocks, unsigned normalized
        GL_COMPRESSED_RGBA_PVRTC_2BPPV2_IMG => Format::PVRTC2_2BPP_UNORM_BLOCK_IMG, // 4-component PVRTC,  8x4 blocks, unsigned normalized
        GL_COMPRESSED_RGBA_PVRTC_4BPPV2_IMG => Format::PVRTC2_4BPP_UNORM_BLOCK_IMG, // 4-component PVRTC,  4x4 blocks, unsigned normalized

        GL_COMPRESSED_SRGB_PVRTC_2BPPV1_EXT => Format::PVRTC1_2BPP_SRGB_BLOCK_IMG, // 3-component PVRTC, 16x8 blocks, sRGB
        GL_COMPRESSED_SRGB_PVRTC_4BPPV1_EXT => Format::PVRTC1_4BPP_SRGB_BLOCK_IMG, // 3-component PVRTC,  8x8 blocks, sRGB
        GL_COMPRESSED_SRGB_ALPHA_PVRTC_2BPPV1_EXT => Format::PVRTC1_2BPP_SRGB_BLOCK_IMG, // 4-component PVRTC, 16x8 blocks, sRGB
        GL_COMPRESSED_SRGB_ALPHA_PVRTC_4BPPV1_EXT => Format::PVRTC1_4BPP_SRGB_BLOCK_IMG, // 4-component PVRTC,  8x8 blocks, sRGB
        GL_COMPRESSED_SRGB_ALPHA_PVRTC_2BPPV2_IMG => Format::PVRTC2_2BPP_SRGB_BLOCK_IMG, // 4-component PVRTC,  8x4 blocks, sRGB
        GL_COMPRESSED_SRGB_ALPHA_PVRTC_4BPPV2_IMG => Format::PVRTC2_4BPP_SRGB_BLOCK_IMG, // 4-component PVRTC,  4x4 blocks, sRGB

        /*
                //
                // ASTC
                //
          #define SUPPORT_ASTC_HDR 1
          #if !SUPPORT_ASTC_HDR
                GL_COMPRESSED_RGBA_ASTC_4x4_KHR => Format::ASTC_4x4_UNORM_BLOCK,	// 4-component ASTC, 4x4 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_5x4_KHR => Format::ASTC_5x4_UNORM_BLOCK,	// 4-component ASTC, 5x4 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_5x5_KHR => Format::ASTC_5x5_UNORM_BLOCK,	// 4-component ASTC, 5x5 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_6x5_KHR => Format::ASTC_6x5_UNORM_BLOCK,	// 4-component ASTC, 6x5 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_6x6_KHR => Format::ASTC_6x6_UNORM_BLOCK,	// 4-component ASTC, 6x6 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_8x5_KHR => Format::ASTC_8x5_UNORM_BLOCK,	// 4-component ASTC, 8x5 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_8x6_KHR => Format::ASTC_8x6_UNORM_BLOCK,	// 4-component ASTC, 8x6 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_8x8_KHR => Format::ASTC_8x8_UNORM_BLOCK,	// 4-component ASTC, 8x8 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_10x5_KHR => Format::ASTC_10x5_UNORM_BLOCK,	// 4-component ASTC, 10x5 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_10x6_KHR => Format::ASTC_10x6_UNORM_BLOCK,	// 4-component ASTC, 10x6 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_10x8_KHR => Format::ASTC_10x8_UNORM_BLOCK,	// 4-component ASTC, 10x8 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_10x10_KHR => Format::ASTC_10x10_UNORM_BLOCK,// 4-component ASTC, 10x10 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_12x10_KHR => Format::ASTC_12x10_UNORM_BLOCK,// 4-component ASTC, 12x10 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_12x12_KHR => Format::ASTC_12x12_UNORM_BLOCK,// 4-component ASTC, 12x12 blocks, unsigned normalized
        #else
                GL_COMPRESSED_RGBA_ASTC_4x4_KHR => Format::ASTC_4x4_SFLOAT_BLOCK,	// 4-component ASTC, 4x4 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_5x4_KHR => Format::ASTC_5x4_SFLOAT_BLOCK,	// 4-component ASTC, 5x4 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_5x5_KHR => Format::ASTC_5x5_SFLOAT_BLOCK,	// 4-component ASTC, 5x5 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_6x5_KHR => Format::ASTC_6x5_SFLOAT_BLOCK,	// 4-component ASTC, 6x5 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_6x6_KHR => Format::ASTC_6x6_SFLOAT_BLOCK,	// 4-component ASTC, 6x6 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_8x5_KHR => Format::ASTC_8x5_SFLOAT_BLOCK,	// 4-component ASTC, 8x5 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_8x6_KHR => Format::ASTC_8x6_SFLOAT_BLOCK,	// 4-component ASTC, 8x6 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_8x8_KHR => Format::ASTC_8x8_SFLOAT_BLOCK,	// 4-component ASTC, 8x8 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_10x5_KHR => Format::ASTC_10x5_SFLOAT_BLOCK,	// 4-component ASTC, 10x5 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_10x6_KHR => Format::ASTC_10x6_SFLOAT_BLOCK,	// 4-component ASTC, 10x6 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_10x8_KHR => Format::ASTC_10x8_SFLOAT_BLOCK,	// 4-component ASTC, 10x8 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_10x10_KHR => Format::ASTC_10x10_SFLOAT_BLOCK,// 4-component ASTC, 10x10 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_12x10_KHR => Format::ASTC_12x10_SFLOAT_BLOCK,// 4-component ASTC, 12x10 blocks, unsigned normalized
                GL_COMPRESSED_RGBA_ASTC_12x12_KHR => Format::ASTC_12x12_SFLOAT_BLOCK,// 4-component ASTC, 12x12 blocks, unsigned normalized
        #endif
        */
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR => Format::ASTC_4X4_SRGB_BLOCK, // 4-component ASTC, 4x4 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR => Format::ASTC_5X4_SRGB_BLOCK, // 4-component ASTC, 5x4 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR => Format::ASTC_5X5_SRGB_BLOCK, // 4-component ASTC, 5x5 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR => Format::ASTC_6X5_SRGB_BLOCK, // 4-component ASTC, 6x5 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR => Format::ASTC_6X6_SRGB_BLOCK, // 4-component ASTC, 6x6 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR => Format::ASTC_8X5_SRGB_BLOCK, // 4-component ASTC, 8x5 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR => Format::ASTC_8X6_SRGB_BLOCK, // 4-component ASTC, 8x6 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR => Format::ASTC_8X8_SRGB_BLOCK, // 4-component ASTC, 8x8 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR => Format::ASTC_10X5_SRGB_BLOCK, // 4-component ASTC, 10x5 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR => Format::ASTC_10X6_SRGB_BLOCK, // 4-component ASTC, 10x6 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR => Format::ASTC_10X8_SRGB_BLOCK, // 4-component ASTC, 10x8 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR => Format::ASTC_10X10_SRGB_BLOCK, // 4-component ASTC, 10x10 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR => Format::ASTC_12X10_SRGB_BLOCK, // 4-component ASTC, 12x10 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR => Format::ASTC_12X12_SRGB_BLOCK, // 4-component ASTC, 12x12 blocks, sRGB

        GL_COMPRESSED_RGBA_ASTC_3x3x3_OES => Format::UNDEFINED, // 4-component ASTC, 3x3x3 blocks, unsigned normalized
        GL_COMPRESSED_RGBA_ASTC_4x3x3_OES => Format::UNDEFINED, // 4-component ASTC, 4x3x3 blocks, unsigned normalized
        GL_COMPRESSED_RGBA_ASTC_4x4x3_OES => Format::UNDEFINED, // 4-component ASTC, 4x4x3 blocks, unsigned normalized
        GL_COMPRESSED_RGBA_ASTC_4x4x4_OES => Format::UNDEFINED, // 4-component ASTC, 4x4x4 blocks, unsigned normalized
        GL_COMPRESSED_RGBA_ASTC_5x4x4_OES => Format::UNDEFINED, // 4-component ASTC, 5x4x4 blocks, unsigned normalized
        GL_COMPRESSED_RGBA_ASTC_5x5x4_OES => Format::UNDEFINED, // 4-component ASTC, 5x5x4 blocks, unsigned normalized
        GL_COMPRESSED_RGBA_ASTC_5x5x5_OES => Format::UNDEFINED, // 4-component ASTC, 5x5x5 blocks, unsigned normalized
        GL_COMPRESSED_RGBA_ASTC_6x5x5_OES => Format::UNDEFINED, // 4-component ASTC, 6x5x5 blocks, unsigned normalized
        GL_COMPRESSED_RGBA_ASTC_6x6x5_OES => Format::UNDEFINED, // 4-component ASTC, 6x6x5 blocks, unsigned normalized
        GL_COMPRESSED_RGBA_ASTC_6x6x6_OES => Format::UNDEFINED, // 4-component ASTC, 6x6x6 blocks, unsigned normalized

        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_3x3x3_OES => Format::UNDEFINED, // 4-component ASTC, 3x3x3 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x3x3_OES => Format::UNDEFINED, // 4-component ASTC, 4x3x3 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4x3_OES => Format::UNDEFINED, // 4-component ASTC, 4x4x3 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4x4_OES => Format::UNDEFINED, // 4-component ASTC, 4x4x4 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4x4_OES => Format::UNDEFINED, // 4-component ASTC, 5x4x4 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5x4_OES => Format::UNDEFINED, // 4-component ASTC, 5x5x4 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5x5_OES => Format::UNDEFINED, // 4-component ASTC, 5x5x5 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5x5_OES => Format::UNDEFINED, // 4-component ASTC, 6x5x5 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6x5_OES => Format::UNDEFINED, // 4-component ASTC, 6x6x5 blocks, sRGB
        GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6x6_OES => Format::UNDEFINED, // 4-component ASTC, 6x6x6 blocks, sRGB

        //
        // ATC
        //
        GL_ATC_RGB_AMD => Format::UNDEFINED, // 3-component, 4x4 blocks, unsigned normalized
        GL_ATC_RGBA_EXPLICIT_ALPHA_AMD => Format::UNDEFINED, // 4-component, 4x4 blocks, unsigned normalized
        GL_ATC_RGBA_INTERPOLATED_ALPHA_AMD => Format::UNDEFINED, // 4-component, 4x4 blocks, unsigned normalized

        //
        // Palletized
        //
        GL_PALETTE4_RGB8_OES => Format::UNDEFINED, // 3-component 8:8:8,   4-bit palette, unsigned normalized
        GL_PALETTE4_RGBA8_OES => Format::UNDEFINED, // 4-component 8:8:8:8, 4-bit palette, unsigned normalized
        GL_PALETTE4_R5_G6_B5_OES => Format::UNDEFINED, // 3-component 5:6:5,   4-bit palette, unsigned normalized
        GL_PALETTE4_RGBA4_OES => Format::UNDEFINED, // 4-component 4:4:4:4, 4-bit palette, unsigned normalized
        GL_PALETTE4_RGB5_A1_OES => Format::UNDEFINED, // 4-component 5:5:5:1, 4-bit palette, unsigned normalized
        GL_PALETTE8_RGB8_OES => Format::UNDEFINED, // 3-component 8:8:8,   8-bit palette, unsigned normalized
        GL_PALETTE8_RGBA8_OES => Format::UNDEFINED, // 4-component 8:8:8:8, 8-bit palette, unsigned normalized
        GL_PALETTE8_R5_G6_B5_OES => Format::UNDEFINED, // 3-component 5:6:5,   8-bit palette, unsigned normalized
        GL_PALETTE8_RGBA4_OES => Format::UNDEFINED, // 4-component 4:4:4:4, 8-bit palette, unsigned normalized
        GL_PALETTE8_RGB5_A1_OES => Format::UNDEFINED, // 4-component 5:5:5:1, 8-bit palette, unsigned normalized

        //
        // Depth/stencil
        //
        GL_DEPTH_COMPONENT16 => Format::D16_UNORM,
        GL_DEPTH_COMPONENT24 => Format::X8_D24_UNORM_PACK32,
        GL_DEPTH_COMPONENT32 => Format::UNDEFINED,
        GL_DEPTH_COMPONENT32F => Format::D32_SFLOAT,
        GL_DEPTH_COMPONENT32F_NV => Format::D32_SFLOAT,
        GL_STENCIL_INDEX1 => Format::UNDEFINED,
        GL_STENCIL_INDEX4 => Format::UNDEFINED,
        GL_STENCIL_INDEX8 => Format::S8_UINT,
        GL_STENCIL_INDEX16 => Format::UNDEFINED,
        GL_DEPTH24_STENCIL8 => Format::D24_UNORM_S8_UINT,
        GL_DEPTH32F_STENCIL8 => Format::D32_SFLOAT_S8_UINT,
        GL_DEPTH32F_STENCIL8_NV => Format::D32_SFLOAT_S8_UINT,

        _ => Format::UNDEFINED,
    };

    if vk_format == Format::UNDEFINED {
        None
    } else {
        Some(vk_format)
    }
}

pub fn get_vk_format_from_gl(format: u32, gl_type: u32) -> Option<Format> {
    let vk_format = match gl_type {
        //
        // 8 bits per component
        //
        GL_UNSIGNED_BYTE => match format {
            GL_RED => Format::R8_UNORM,
            GL_RG => Format::R8G8_UNORM,
            GL_RGB => Format::R8G8B8_UNORM,
            GL_BGR => Format::B8G8R8_UNORM,
            GL_RGBA => Format::R8G8B8A8_UNORM,
            GL_BGRA => Format::B8G8R8A8_UNORM,
            GL_RED_INTEGER => Format::R8_UINT,
            GL_RG_INTEGER => Format::R8G8_UINT,
            GL_RGB_INTEGER => Format::R8G8B8_UINT,
            GL_BGR_INTEGER => Format::B8G8R8_UINT,
            GL_RGBA_INTEGER => Format::R8G8B8A8_UINT,
            GL_BGRA_INTEGER => Format::B8G8R8A8_UINT,
            GL_STENCIL_INDEX => Format::S8_UINT,
            GL_DEPTH_COMPONENT => Format::UNDEFINED,
            GL_DEPTH_STENCIL => Format::UNDEFINED,
            _ => Format::UNDEFINED,
        },
        GL_BYTE => match format {
            GL_RED => Format::R8_SNORM,
            GL_RG => Format::R8G8_SNORM,
            GL_RGB => Format::R8G8B8_SNORM,
            GL_BGR => Format::B8G8R8_SNORM,
            GL_RGBA => Format::R8G8B8A8_SNORM,
            GL_BGRA => Format::B8G8R8A8_SNORM,
            GL_RED_INTEGER => Format::R8_SINT,
            GL_RG_INTEGER => Format::R8G8_SINT,
            GL_RGB_INTEGER => Format::R8G8B8_SINT,
            GL_BGR_INTEGER => Format::B8G8R8_SINT,
            GL_RGBA_INTEGER => Format::R8G8B8A8_SINT,
            GL_BGRA_INTEGER => Format::B8G8R8A8_SINT,
            GL_STENCIL_INDEX => Format::UNDEFINED,
            GL_DEPTH_COMPONENT => Format::UNDEFINED,
            GL_DEPTH_STENCIL => Format::UNDEFINED,
            _ => Format::UNDEFINED,
        },

        //
        // 16 bits per component
        //
        GL_UNSIGNED_SHORT => match format {
            GL_RED => Format::R16_UNORM,
            GL_RG => Format::R16G16_UNORM,
            GL_RGB => Format::R16G16B16_UNORM,
            GL_BGR => Format::UNDEFINED,
            GL_RGBA => Format::R16G16B16A16_UNORM,
            GL_BGRA => Format::UNDEFINED,
            GL_RED_INTEGER => Format::R16_UINT,
            GL_RG_INTEGER => Format::R16G16_UINT,
            GL_RGB_INTEGER => Format::R16G16B16_UINT,
            GL_BGR_INTEGER => Format::UNDEFINED,
            GL_RGBA_INTEGER => Format::R16G16B16A16_UINT,
            GL_BGRA_INTEGER => Format::UNDEFINED,
            GL_STENCIL_INDEX => Format::UNDEFINED,
            GL_DEPTH_COMPONENT => Format::D16_UNORM,
            GL_DEPTH_STENCIL => Format::D16_UNORM_S8_UINT,
            _ => Format::UNDEFINED,
        },
        GL_SHORT => match format {
            GL_RED => Format::R16_SNORM,
            GL_RG => Format::R16G16_SNORM,
            GL_RGB => Format::R16G16B16_SNORM,
            GL_BGR => Format::UNDEFINED,
            GL_RGBA => Format::R16G16B16A16_SNORM,
            GL_BGRA => Format::UNDEFINED,
            GL_RED_INTEGER => Format::R16_SINT,
            GL_RG_INTEGER => Format::R16G16_SINT,
            GL_RGB_INTEGER => Format::R16G16B16_SINT,
            GL_BGR_INTEGER => Format::UNDEFINED,
            GL_RGBA_INTEGER => Format::R16G16B16A16_SINT,
            GL_BGRA_INTEGER => Format::UNDEFINED,
            GL_STENCIL_INDEX => Format::UNDEFINED,
            GL_DEPTH_COMPONENT => Format::UNDEFINED,
            GL_DEPTH_STENCIL => Format::UNDEFINED,
            _ => Format::UNDEFINED,
        },
        GL_HALF_FLOAT => match format {
            GL_RED => Format::R16_SFLOAT,
            GL_RG => Format::R16G16_SFLOAT,
            GL_RGB => Format::R16G16B16_SFLOAT,
            GL_BGR => Format::UNDEFINED,
            GL_RGBA => Format::R16G16B16A16_SFLOAT,
            GL_BGRA => Format::UNDEFINED,
            GL_RED_INTEGER => Format::UNDEFINED,
            GL_RG_INTEGER => Format::UNDEFINED,
            GL_RGB_INTEGER => Format::UNDEFINED,
            GL_BGR_INTEGER => Format::UNDEFINED,
            GL_RGBA_INTEGER => Format::UNDEFINED,
            GL_BGRA_INTEGER => Format::UNDEFINED,
            GL_STENCIL_INDEX => Format::UNDEFINED,
            GL_DEPTH_COMPONENT => Format::UNDEFINED,
            GL_DEPTH_STENCIL => Format::UNDEFINED,
            _ => Format::UNDEFINED,
        },
        GL_HALF_FLOAT_OES => match format {
            GL_RED => Format::R16_SFLOAT,
            GL_RG => Format::R16G16_SFLOAT,
            GL_RGB => Format::R16G16B16_SFLOAT,
            GL_BGR => Format::UNDEFINED,
            GL_RGBA => Format::R16G16B16A16_SFLOAT,
            GL_BGRA => Format::UNDEFINED,
            GL_RED_INTEGER => Format::UNDEFINED,
            GL_RG_INTEGER => Format::UNDEFINED,
            GL_RGB_INTEGER => Format::UNDEFINED,
            GL_BGR_INTEGER => Format::UNDEFINED,
            GL_RGBA_INTEGER => Format::UNDEFINED,
            GL_BGRA_INTEGER => Format::UNDEFINED,
            GL_STENCIL_INDEX => Format::UNDEFINED,
            GL_DEPTH_COMPONENT => Format::UNDEFINED,
            GL_DEPTH_STENCIL => Format::UNDEFINED,
            _ => Format::UNDEFINED,
        },

        //
        // 32 bits per component
        //
        GL_UNSIGNED_INT => match format {
            GL_RED => Format::R32_UINT,
            GL_RG => Format::R32G32_UINT,
            GL_RGB => Format::R32G32B32_UINT,
            GL_BGR => Format::UNDEFINED,
            GL_RGBA => Format::R32G32B32A32_UINT,
            GL_BGRA => Format::UNDEFINED,
            GL_RED_INTEGER => Format::R32_UINT,
            GL_RG_INTEGER => Format::R32G32_UINT,
            GL_RGB_INTEGER => Format::R32G32B32_UINT,
            GL_BGR_INTEGER => Format::UNDEFINED,
            GL_RGBA_INTEGER => Format::R32G32B32A32_UINT,
            GL_BGRA_INTEGER => Format::UNDEFINED,
            GL_STENCIL_INDEX => Format::UNDEFINED,
            GL_DEPTH_COMPONENT => Format::X8_D24_UNORM_PACK32,
            GL_DEPTH_STENCIL => Format::D24_UNORM_S8_UINT,
            _ => Format::UNDEFINED,
        },
        GL_INT => match format {
            GL_RED => Format::R32_SINT,
            GL_RG => Format::R32G32_SINT,
            GL_RGB => Format::R32G32B32_SINT,
            GL_BGR => Format::UNDEFINED,
            GL_RGBA => Format::R32G32B32A32_SINT,
            GL_BGRA => Format::UNDEFINED,
            GL_RED_INTEGER => Format::R32_SINT,
            GL_RG_INTEGER => Format::R32G32_SINT,
            GL_RGB_INTEGER => Format::R32G32B32_SINT,
            GL_BGR_INTEGER => Format::UNDEFINED,
            GL_RGBA_INTEGER => Format::R32G32B32A32_SINT,
            GL_BGRA_INTEGER => Format::UNDEFINED,
            GL_STENCIL_INDEX => Format::UNDEFINED,
            GL_DEPTH_COMPONENT => Format::UNDEFINED,
            GL_DEPTH_STENCIL => Format::UNDEFINED,
            _ => Format::UNDEFINED,
        },
        GL_FLOAT => match format {
            GL_RED => Format::R32_SFLOAT,
            GL_RG => Format::R32G32_SFLOAT,
            GL_RGB => Format::R32G32B32_SFLOAT,
            GL_BGR => Format::UNDEFINED,
            GL_RGBA => Format::R32G32B32A32_SFLOAT,
            GL_BGRA => Format::UNDEFINED,
            GL_RED_INTEGER => Format::UNDEFINED,
            GL_RG_INTEGER => Format::UNDEFINED,
            GL_RGB_INTEGER => Format::UNDEFINED,
            GL_BGR_INTEGER => Format::UNDEFINED,
            GL_RGBA_INTEGER => Format::UNDEFINED,
            GL_BGRA_INTEGER => Format::UNDEFINED,
            GL_STENCIL_INDEX => Format::UNDEFINED,
            GL_DEPTH_COMPONENT => Format::D32_SFLOAT,
            GL_DEPTH_STENCIL => Format::D32_SFLOAT_S8_UINT,
            _ => Format::UNDEFINED,
        },

        //
        // 64 bits per component
        //
        GL_UNSIGNED_INT64 => match format {
            GL_RED => Format::R64_UINT,
            GL_RG => Format::R64G64_UINT,
            GL_RGB => Format::R64G64B64_UINT,
            GL_BGR => Format::UNDEFINED,
            GL_RGBA => Format::R64G64B64A64_UINT,
            GL_BGRA => Format::UNDEFINED,
            GL_RED_INTEGER => Format::UNDEFINED,
            GL_RG_INTEGER => Format::UNDEFINED,
            GL_RGB_INTEGER => Format::UNDEFINED,
            GL_BGR_INTEGER => Format::UNDEFINED,
            GL_RGBA_INTEGER => Format::UNDEFINED,
            GL_BGRA_INTEGER => Format::UNDEFINED,
            GL_STENCIL_INDEX => Format::UNDEFINED,
            GL_DEPTH_COMPONENT => Format::UNDEFINED,
            GL_DEPTH_STENCIL => Format::UNDEFINED,
            _ => Format::UNDEFINED,
        },
        GL_INT64 => match format {
            GL_RED => Format::R64_SINT,
            GL_RG => Format::R64G64_SINT,
            GL_RGB => Format::R64G64B64_SINT,
            GL_BGR => Format::UNDEFINED,
            GL_RGBA => Format::R64G64B64A64_SINT,
            GL_BGRA => Format::UNDEFINED,
            GL_RED_INTEGER => Format::R64_SINT,
            GL_RG_INTEGER => Format::R64G64_SINT,
            GL_RGB_INTEGER => Format::R64G64B64_SINT,
            GL_BGR_INTEGER => Format::UNDEFINED,
            GL_RGBA_INTEGER => Format::R64G64B64A64_SINT,
            GL_BGRA_INTEGER => Format::UNDEFINED,
            GL_STENCIL_INDEX => Format::UNDEFINED,
            GL_DEPTH_COMPONENT => Format::UNDEFINED,
            GL_DEPTH_STENCIL => Format::UNDEFINED,
            _ => Format::UNDEFINED,
        },
        GL_DOUBLE => match format {
            GL_RED => Format::R64_SFLOAT,
            GL_RG => Format::R64G64_SFLOAT,
            GL_RGB => Format::R64G64B64_SFLOAT,
            GL_BGR => Format::UNDEFINED,
            GL_RGBA => Format::R64G64B64A64_SFLOAT,
            GL_BGRA => Format::UNDEFINED,
            GL_RED_INTEGER => Format::R64_SFLOAT,
            GL_RG_INTEGER => Format::R64G64_SFLOAT,
            GL_RGB_INTEGER => Format::R64G64B64_SFLOAT,
            GL_BGR_INTEGER => Format::UNDEFINED,
            GL_RGBA_INTEGER => Format::R64G64B64A64_SFLOAT,
            GL_BGRA_INTEGER => Format::UNDEFINED,
            GL_STENCIL_INDEX => Format::UNDEFINED,
            GL_DEPTH_COMPONENT => Format::UNDEFINED,
            GL_DEPTH_STENCIL => Format::UNDEFINED,
            _ => Format::UNDEFINED,
        },

        //
        // Packed
        //
        GL_UNSIGNED_BYTE_3_3_2 => {
            assert!(format == GL_RGB || format == GL_RGB_INTEGER);
            Format::UNDEFINED
        }
        GL_UNSIGNED_BYTE_2_3_3_REV => {
            assert!(format == GL_BGR || format == GL_BGR_INTEGER);
            Format::UNDEFINED
        }
        GL_UNSIGNED_SHORT_5_6_5 => {
            assert!(format == GL_RGB || format == GL_RGB_INTEGER);
            Format::R5G6B5_UNORM_PACK16
        }
        GL_UNSIGNED_SHORT_5_6_5_REV => {
            assert!(format == GL_BGR || format == GL_BGR_INTEGER);
            Format::B5G6R5_UNORM_PACK16
        }
        GL_UNSIGNED_SHORT_4_4_4_4 => {
            assert!(
                format == GL_RGB
                    || format == GL_BGRA
                    || format == GL_RGB_INTEGER
                    || format == GL_BGRA_INTEGER
            );
            Format::R4G4B4A4_UNORM_PACK16
        }
        GL_UNSIGNED_SHORT_4_4_4_4_REV => {
            assert!(
                format == GL_RGB
                    || format == GL_BGRA
                    || format == GL_RGB_INTEGER
                    || format == GL_BGRA_INTEGER
            );
            Format::B4G4R4A4_UNORM_PACK16
        }
        GL_UNSIGNED_SHORT_5_5_5_1 => {
            assert!(
                format == GL_RGB
                    || format == GL_BGRA
                    || format == GL_RGB_INTEGER
                    || format == GL_BGRA_INTEGER
            );
            Format::R5G5B5A1_UNORM_PACK16
        }
        GL_UNSIGNED_SHORT_1_5_5_5_REV => {
            assert!(
                format == GL_RGB
                    || format == GL_BGRA
                    || format == GL_RGB_INTEGER
                    || format == GL_BGRA_INTEGER
            );
            Format::A1R5G5B5_UNORM_PACK16
        }
        GL_UNSIGNED_INT_8_8_8_8 => {
            assert!(
                format == GL_RGB
                    || format == GL_BGRA
                    || format == GL_RGB_INTEGER
                    || format == GL_BGRA_INTEGER
            );
            if format == GL_RGB_INTEGER || format == GL_BGRA_INTEGER {
                Format::R8G8B8A8_UINT
            } else {
                Format::R8G8B8A8_UNORM
            }
        }
        GL_UNSIGNED_INT_8_8_8_8_REV => {
            assert!(
                format == GL_RGB
                    || format == GL_BGRA
                    || format == GL_RGB_INTEGER
                    || format == GL_BGRA_INTEGER
            );
            if format == GL_RGB_INTEGER || format == GL_BGRA_INTEGER {
                Format::A8B8G8R8_UINT_PACK32
            } else {
                Format::A8B8G8R8_UNORM_PACK32
            }
        }
        GL_UNSIGNED_INT_10_10_10_2 => {
            assert!(
                format == GL_RGB
                    || format == GL_BGRA
                    || format == GL_RGB_INTEGER
                    || format == GL_BGRA_INTEGER
            );
            if format == GL_RGB_INTEGER || format == GL_BGRA_INTEGER {
                Format::A2R10G10B10_UINT_PACK32
            } else {
                Format::A2R10G10B10_UNORM_PACK32
            }
        }
        GL_UNSIGNED_INT_2_10_10_10_REV => {
            assert!(
                format == GL_RGB
                    || format == GL_BGRA
                    || format == GL_RGB_INTEGER
                    || format == GL_BGRA_INTEGER
            );
            if format == GL_RGB_INTEGER || format == GL_BGRA_INTEGER {
                Format::A2B10G10R10_UINT_PACK32
            } else {
                Format::A2B10G10R10_UNORM_PACK32
            }
        }
        GL_UNSIGNED_INT_10F_11F_11F_REV => {
            assert!(format == GL_RGB || format == GL_BGR);
            Format::B10G11R11_UFLOAT_PACK32
        }
        GL_UNSIGNED_INT_5_9_9_9_REV => {
            assert!(format == GL_RGB || format == GL_BGR);
            Format::E5B9G9R9_UFLOAT_PACK32
        }
        GL_UNSIGNED_INT_24_8 => {
            assert!(format == GL_DEPTH_STENCIL);
            Format::D24_UNORM_S8_UINT
        }
        GL_FLOAT_32_UNSIGNED_INT_24_8_REV => {
            assert!(format == GL_DEPTH_STENCIL);
            Format::D32_SFLOAT_S8_UINT
        }
        _ => Format::UNDEFINED,
    };

    if vk_format == Format::UNDEFINED {
        None
    } else {
        Some(vk_format)
    }
}
