#![allow(warnings)]

pub const GL_RED: u32 = 0x1903; // same as GL_RED_EXT;
pub const GL_GREEN: u32 = 0x1904; // deprecated;
pub const GL_BLUE: u32 = 0x1905; // deprecated;
pub const GL_ALPHA: u32 = 0x1906; // deprecated;
pub const GL_LUMINANCE: u32 = 0x1909; // deprecated;
pub const GL_SLUMINANCE: u32 = 0x8C46; // deprecated, same as GL_SLUMINANCE_EXT;
pub const GL_LUMINANCE_ALPHA: u32 = 0x190A; // deprecated;
pub const GL_SLUMINANCE_ALPHA: u32 = 0x8C44; // deprecated, same as GL_SLUMINANCE_ALPHA_EXT;
pub const GL_INTENSITY: u32 = 0x8049; // deprecated, same as GL_INTENSITY_EXT;
pub const GL_RG: u32 = 0x8227; // same as GL_RG_EXT;
pub const GL_RGB: u32 = 0x1907;
pub const GL_BGR: u32 = 0x80E0; // same as GL_BGR_EXT;
pub const GL_RGBA: u32 = 0x1908;
pub const GL_BGRA: u32 = 0x80E1; // same as GL_BGRA_EXT;
pub const GL_RED_INTEGER: u32 = 0x8D94; // same as GL_RED_INTEGER_EXT;
pub const GL_GREEN_INTEGER: u32 = 0x8D95; // deprecated, same as GL_GREEN_INTEGER_EXT;
pub const GL_BLUE_INTEGER: u32 = 0x8D96; // deprecated, same as GL_BLUE_INTEGER_EXT;
pub const GL_ALPHA_INTEGER: u32 = 0x8D97; // deprecated, same as GL_ALPHA_INTEGER_EXT;
pub const GL_LUMINANCE_INTEGER: u32 = 0x8D9C; // deprecated, same as GL_LUMINANCE_INTEGER_EXT;
pub const GL_LUMINANCE_ALPHA_INTEGER: u32 = 0x8D9D; // deprecated, same as GL_LUMINANCE_ALPHA_INTEGER_EXT;
pub const GL_RG_INTEGER: u32 = 0x8228; // same as GL_RG_INTEGER_EXT;
pub const GL_RGB_INTEGER: u32 = 0x8D98; // same as GL_RGB_INTEGER_EXT;
pub const GL_BGR_INTEGER: u32 = 0x8D9A; // same as GL_BGR_INTEGER_EXT;
pub const GL_RGBA_INTEGER: u32 = 0x8D99; // same as GL_RGBA_INTEGER_EXT;
pub const GL_BGRA_INTEGER: u32 = 0x8D9B; // same as GL_BGRA_INTEGER_EXT;
pub const GL_COLOR_INDEX: u32 = 0x1900; // deprecated;
pub const GL_STENCIL_INDEX: u32 = 0x1901;
pub const GL_DEPTH_COMPONENT: u32 = 0x1902;
pub const GL_DEPTH_STENCIL: u32 = 0x84F9; // same as GL_DEPTH_STENCIL_NV and GL_DEPTH_STENCIL_EXT and GL_DEPTH_STENCIL_OES;
/*
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = ;
Type to glTexImage2D, glTexImage3D and glVertexAttribPointer.
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = ;
*/
pub const GL_BYTE: u32 = 0x1400;
pub const GL_UNSIGNED_BYTE: u32 = 0x1401;
pub const GL_SHORT: u32 = 0x1402;
pub const GL_UNSIGNED_SHORT: u32 = 0x1403;
pub const GL_INT: u32 = 0x1404;
pub const GL_UNSIGNED_INT: u32 = 0x1405;
pub const GL_INT64: u32 = 0x140E; // same as GL_INT64_NV and GL_INT64_ARB;
pub const GL_UNSIGNED_INT64: u32 = 0x140F; // same as GL_UNSIGNED_INT64_NV and GL_UNSIGNED_INT64_ARB;
pub const GL_HALF_FLOAT: u32 = 0x140B; // same as GL_HALF_FLOAT_NV and GL_HALF_FLOAT_ARB;
pub const GL_HALF_FLOAT_OES: u32 = 0x8D61; // Note that this different from GL_HALF_FLOAT.;
pub const GL_FLOAT: u32 = 0x1406;
pub const GL_DOUBLE: u32 = 0x140A; // same as GL_DOUBLE_EXT;
pub const GL_UNSIGNED_BYTE_3_3_2: u32 = 0x8032; // same as GL_UNSIGNED_BYTE_3_3_2_EXT;
pub const GL_UNSIGNED_BYTE_2_3_3_REV: u32 = 0x8362; // same as GL_UNSIGNED_BYTE_2_3_3_REV_EXT;
pub const GL_UNSIGNED_SHORT_5_6_5: u32 = 0x8363; // same as GL_UNSIGNED_SHORT_5_6_5_EXT;
pub const GL_UNSIGNED_SHORT_5_6_5_REV: u32 = 0x8364; // same as GL_UNSIGNED_SHORT_5_6_5_REV_EXT;
pub const GL_UNSIGNED_SHORT_4_4_4_4: u32 = 0x8033; // same as GL_UNSIGNED_SHORT_4_4_4_4_EXT;
pub const GL_UNSIGNED_SHORT_4_4_4_4_REV: u32 = 0x8365; // same as GL_UNSIGNED_SHORT_4_4_4_4_REV_IMG and GL_UNSIGNED_SHORT_4_4_4_4_REV_EXT;
pub const GL_UNSIGNED_SHORT_5_5_5_1: u32 = 0x8034; // same as GL_UNSIGNED_SHORT_5_5_5_1_EXT;
pub const GL_UNSIGNED_SHORT_1_5_5_5_REV: u32 = 0x8366; // same as GL_UNSIGNED_SHORT_1_5_5_5_REV_EXT;
pub const GL_UNSIGNED_INT_8_8_8_8: u32 = 0x8035; // same as GL_UNSIGNED_INT_8_8_8_8_EXT;
pub const GL_UNSIGNED_INT_8_8_8_8_REV: u32 = 0x8367; // same as GL_UNSIGNED_INT_8_8_8_8_REV_EXT;
pub const GL_UNSIGNED_INT_10_10_10_2: u32 = 0x8036; // same as GL_UNSIGNED_INT_10_10_10_2_EXT;
pub const GL_UNSIGNED_INT_2_10_10_10_REV: u32 = 0x8368; // same as GL_UNSIGNED_INT_2_10_10_10_REV_EXT;
pub const GL_UNSIGNED_INT_10F_11F_11F_REV: u32 = 0x8C3B; // same as GL_UNSIGNED_INT_10F_11F_11F_REV_EXT;
pub const GL_UNSIGNED_INT_5_9_9_9_REV: u32 = 0x8C3E; // same as GL_UNSIGNED_INT_5_9_9_9_REV_EXT;
pub const GL_UNSIGNED_INT_24_8: u32 = 0x84FA; // same as GL_UNSIGNED_INT_24_8_NV and GL_UNSIGNED_INT_24_8_EXT and GL_UNSIGNED_INT_24_8_OES;
pub const GL_FLOAT_32_UNSIGNED_INT_24_8_REV: u32 = 0x8DAD; // same as GL_FLOAT_32_UNSIGNED_INT_24_8_REV_NV and GL_FLOAT_32_UNSIGNED_INT_24_8_REV_ARB;
/*
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = ;
Internal format to glTexImage2D, glTexImage3D, glCompressedTexImage2D, glCompressedTexImage3D, glTexStorage2D, glTexStorage3D
= = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = = ;
*/
//
// 8 bits per component
//
pub const GL_R8: u32 = 0x8229; // same as GL_R8_EXT;
pub const GL_RG8: u32 = 0x822B; // same as GL_RG8_EXT;
pub const GL_RGB8: u32 = 0x8051; // same as GL_RGB8_EXT and GL_RGB8_OES;
pub const GL_RGBA8: u32 = 0x8058; // same as GL_RGBA8_EXT and GL_RGBA8_OES;
pub const GL_R8_SNORM: u32 = 0x8F94;
pub const GL_RG8_SNORM: u32 = 0x8F95;
pub const GL_RGB8_SNORM: u32 = 0x8F96;
pub const GL_RGBA8_SNORM: u32 = 0x8F97;
pub const GL_R8UI: u32 = 0x8232;
pub const GL_RG8UI: u32 = 0x8238;
pub const GL_RGB8UI: u32 = 0x8D7D; // same as GL_RGB8UI_EXT;
pub const GL_RGBA8UI: u32 = 0x8D7C; // same as GL_RGBA8UI_EXT;
pub const GL_R8I: u32 = 0x8231;
pub const GL_RG8I: u32 = 0x8237;
pub const GL_RGB8I: u32 = 0x8D8F; // same as GL_RGB8I_EXT;
pub const GL_RGBA8I: u32 = 0x8D8E; // same as GL_RGBA8I_EXT;
pub const GL_SR8: u32 = 0x8FBD; // same as GL_SR8_EXT;
pub const GL_SRG8: u32 = 0x8FBE; // same as GL_SRG8_EXT;
pub const GL_SRGB8: u32 = 0x8C41; // same as GL_SRGB8_EXT;
pub const GL_SRGB8_ALPHA8: u32 = 0x8C43; // same as GL_SRGB8_ALPHA8_EXT;
//
// 16 bits per component
//
pub const GL_R16: u32 = 0x822A; // same as GL_R16_EXT;
pub const GL_RG16: u32 = 0x822C; // same as GL_RG16_EXT;
pub const GL_RGB16: u32 = 0x8054; // same as GL_RGB16_EXT;
pub const GL_RGBA16: u32 = 0x805B; // same as GL_RGBA16_EXT;
pub const GL_R16_SNORM: u32 = 0x8F98; // same as GL_R16_SNORM_EXT;
pub const GL_RG16_SNORM: u32 = 0x8F99; // same as GL_RG16_SNORM_EXT;
pub const GL_RGB16_SNORM: u32 = 0x8F9A; // same as GL_RGB16_SNORM_EXT;
pub const GL_RGBA16_SNORM: u32 = 0x8F9B; // same as GL_RGBA16_SNORM_EXT;
pub const GL_R16UI: u32 = 0x8234;
pub const GL_RG16UI: u32 = 0x823A;
pub const GL_RGB16UI: u32 = 0x8D77; // same as GL_RGB16UI_EXT;
pub const GL_RGBA16UI: u32 = 0x8D76; // same as GL_RGBA16UI_EXT;
pub const GL_R16I: u32 = 0x8233;
pub const GL_RG16I: u32 = 0x8239;
pub const GL_RGB16I: u32 = 0x8D89; // same as GL_RGB16I_EXT;
pub const GL_RGBA16I: u32 = 0x8D88; // same as GL_RGBA16I_EXT;
pub const GL_R16F: u32 = 0x822D; // same as GL_R16F_EXT;
pub const GL_RG16F: u32 = 0x822F; // same as GL_RG16F_EXT;
pub const GL_RGB16F: u32 = 0x881B; // same as GL_RGB16F_EXT and GL_RGB16F_ARB;
pub const GL_RGBA16F: u32 = 0x881A; // sama as GL_RGBA16F_EXT and GL_RGBA16F_ARB;
//
// 32 bits per component
//
pub const GL_R32UI: u32 = 0x8236;
pub const GL_RG32UI: u32 = 0x823C;
pub const GL_RGB32UI: u32 = 0x8D71; // same as GL_RGB32UI_EXT;
pub const GL_RGBA32UI: u32 = 0x8D70; // same as GL_RGBA32UI_EXT;
pub const GL_R32I: u32 = 0x8235;
pub const GL_RG32I: u32 = 0x823B;
pub const GL_RGB32I: u32 = 0x8D83; // same as GL_RGB32I_EXT;
pub const GL_RGBA32I: u32 = 0x8D82; // same as GL_RGBA32I_EXT;
pub const GL_R32F: u32 = 0x822E; // same as GL_R32F_EXT;
pub const GL_RG32F: u32 = 0x8230; // same as GL_RG32F_EXT;
pub const GL_RGB32F: u32 = 0x8815; // same as GL_RGB32F_EXT and GL_RGB32F_ARB;
pub const GL_RGBA32F: u32 = 0x8814; // same as GL_RGBA32F_EXT and GL_RGBA32F_ARB;
//
// Packed
//
pub const GL_R3_G3_B2: u32 = 0x2A10;
pub const GL_RGB4: u32 = 0x804F; // same as GL_RGB4_EXT;
pub const GL_RGB5: u32 = 0x8050; // same as GL_RGB5_EXT;
pub const GL_RGB565: u32 = 0x8D62; // same as GL_RGB565_EXT and GL_RGB565_OES;
pub const GL_RGB10: u32 = 0x8052; // same as GL_RGB10_EXT;
pub const GL_RGB12: u32 = 0x8053; // same as GL_RGB12_EXT;
pub const GL_RGBA2: u32 = 0x8055; // same as GL_RGBA2_EXT;
pub const GL_RGBA4: u32 = 0x8056; // same as GL_RGBA4_EXT and GL_RGBA4_OES;
pub const GL_RGBA12: u32 = 0x805A; // same as GL_RGBA12_EXT;
pub const GL_RGB5_A1: u32 = 0x8057; // same as GL_RGB5_A1_EXT and GL_RGB5_A1_OES;
pub const GL_RGB10_A2: u32 = 0x8059; // same as GL_RGB10_A2_EXT;
pub const GL_RGB10_A2UI: u32 = 0x906F;
pub const GL_R11F_G11F_B10F: u32 = 0x8C3A; // same as GL_R11F_G11F_B10F_APPLE and GL_R11F_G11F_B10F_EXT;
pub const GL_RGB9_E5: u32 = 0x8C3D; // same as GL_RGB9_E5_APPLE and GL_RGB9_E5_EXT;
//
// Alpha
//
pub const GL_ALPHA4: u32 = 0x803B; // deprecated, same as GL_ALPHA4_EXT;
pub const GL_ALPHA8: u32 = 0x803C; // deprecated, same as GL_ALPHA8_EXT;
pub const GL_ALPHA8_SNORM: u32 = 0x9014; // deprecated;
pub const GL_ALPHA8UI_EXT: u32 = 0x8D7E; // deprecated;
pub const GL_ALPHA8I_EXT: u32 = 0x8D90; // deprecated;
pub const GL_ALPHA12: u32 = 0x803D; // deprecated, same as GL_ALPHA12_EXT;
pub const GL_ALPHA16: u32 = 0x803E; // deprecated, same as GL_ALPHA16_EXT;
pub const GL_ALPHA16_SNORM: u32 = 0x9018; // deprecated;
pub const GL_ALPHA16UI_EXT: u32 = 0x8D78; // deprecated;
pub const GL_ALPHA16I_EXT: u32 = 0x8D8A; // deprecated;
pub const GL_ALPHA16F_ARB: u32 = 0x881C; // deprecated, same as GL_ALPHA_FLOAT16_APPLE and GL_ALPHA_FLOAT16_ATI;
pub const GL_ALPHA32UI_EXT: u32 = 0x8D72; // deprecated;
pub const GL_ALPHA32I_EXT: u32 = 0x8D84; // deprecated;
pub const GL_ALPHA32F_ARB: u32 = 0x8816; // deprecated, same as GL_ALPHA_FLOAT32_APPLE and GL_ALPHA_FLOAT32_ATI;
//
// Luminance
//
pub const GL_LUMINANCE4: u32 = 0x803F; // deprecated, same as GL_LUMINANCE4_EXT;
pub const GL_LUMINANCE8: u32 = 0x8040; // deprecated, same as GL_LUMINANCE8_EXT;
pub const GL_LUMINANCE8_SNORM: u32 = 0x9015; // deprecated;
pub const GL_SLUMINANCE8: u32 = 0x8C47; // deprecated, same as GL_SLUMINANCE8_EXT;
pub const GL_LUMINANCE8UI_EXT: u32 = 0x8D80; // deprecated;
pub const GL_LUMINANCE8I_EXT: u32 = 0x8D92; // deprecated;
pub const GL_LUMINANCE12: u32 = 0x8041; // deprecated, same as GL_LUMINANCE12_EXT;
pub const GL_LUMINANCE16: u32 = 0x8042; // deprecated, same as GL_LUMINANCE16_EXT;
pub const GL_LUMINANCE16_SNORM: u32 = 0x9019; // deprecated;
pub const GL_LUMINANCE16UI_EXT: u32 = 0x8D7A; // deprecated;
pub const GL_LUMINANCE16I_EXT: u32 = 0x8D8C; // deprecated;
pub const GL_LUMINANCE16F_ARB: u32 = 0x881E; // deprecated, same as GL_LUMINANCE_FLOAT16_APPLE and GL_LUMINANCE_FLOAT16_ATI;
pub const GL_LUMINANCE32UI_EXT: u32 = 0x8D74; // deprecated;
pub const GL_LUMINANCE32I_EXT: u32 = 0x8D86; // deprecated;
pub const GL_LUMINANCE32F_ARB: u32 = 0x8818; // deprecated, same as GL_LUMINANCE_FLOAT32_APPLE and GL_LUMINANCE_FLOAT32_ATI;
//
// Luminance/Alpha
//
pub const GL_LUMINANCE4_ALPHA4: u32 = 0x8043; // deprecated, same as GL_LUMINANCE4_ALPHA4_EXT;
pub const GL_LUMINANCE6_ALPHA2: u32 = 0x8044; // deprecated, same as GL_LUMINANCE6_ALPHA2_EXT;
pub const GL_LUMINANCE8_ALPHA8: u32 = 0x8045; // deprecated, same as GL_LUMINANCE8_ALPHA8_EXT;
pub const GL_LUMINANCE8_ALPHA8_SNORM: u32 = 0x9016; // deprecated;
pub const GL_SLUMINANCE8_ALPHA8: u32 = 0x8C45; // deprecated, same as GL_SLUMINANCE8_ALPHA8_EXT;
pub const GL_LUMINANCE_ALPHA8UI_EXT: u32 = 0x8D81; // deprecated;
pub const GL_LUMINANCE_ALPHA8I_EXT: u32 = 0x8D93; // deprecated;
pub const GL_LUMINANCE12_ALPHA4: u32 = 0x8046; // deprecated, same as GL_LUMINANCE12_ALPHA4_EXT;
pub const GL_LUMINANCE12_ALPHA12: u32 = 0x8047; // deprecated, same as GL_LUMINANCE12_ALPHA12_EXT;
pub const GL_LUMINANCE16_ALPHA16: u32 = 0x8048; // deprecated, same as GL_LUMINANCE16_ALPHA16_EXT;
pub const GL_LUMINANCE16_ALPHA16_SNORM: u32 = 0x901A; // deprecated;
pub const GL_LUMINANCE_ALPHA16UI_EXT: u32 = 0x8D7B; // deprecated;
pub const GL_LUMINANCE_ALPHA16I_EXT: u32 = 0x8D8D; // deprecated;
pub const GL_LUMINANCE_ALPHA16F_ARB: u32 = 0x881F; // deprecated, same as GL_LUMINANCE_ALPHA_FLOAT16_APPLE and GL_LUMINANCE_ALPHA_FLOAT16_ATI;
pub const GL_LUMINANCE_ALPHA32UI_EXT: u32 = 0x8D75; // deprecated;
pub const GL_LUMINANCE_ALPHA32I_EXT: u32 = 0x8D87; // deprecated;
pub const GL_LUMINANCE_ALPHA32F_ARB: u32 = 0x8819; // deprecated, same as GL_LUMINANCE_ALPHA_FLOAT32_APPLE and GL_LUMINANCE_ALPHA_FLOAT32_ATI;
//
// Intensity
//
pub const GL_INTENSITY4: u32 = 0x804A; // deprecated, same as GL_INTENSITY4_EXT;
pub const GL_INTENSITY8: u32 = 0x804B; // deprecated, same as GL_INTENSITY8_EXT;
pub const GL_INTENSITY8_SNORM: u32 = 0x9017; // deprecated;
pub const GL_INTENSITY8UI_EXT: u32 = 0x8D7F; // deprecated;
pub const GL_INTENSITY8I_EXT: u32 = 0x8D91; // deprecated;
pub const GL_INTENSITY12: u32 = 0x804C; // deprecated, same as GL_INTENSITY12_EXT;
pub const GL_INTENSITY16: u32 = 0x804D; // deprecated, same as GL_INTENSITY16_EXT;
pub const GL_INTENSITY16_SNORM: u32 = 0x901B; // deprecated;
pub const GL_INTENSITY16UI_EXT: u32 = 0x8D79; // deprecated;
pub const GL_INTENSITY16I_EXT: u32 = 0x8D8B; // deprecated;
pub const GL_INTENSITY16F_ARB: u32 = 0x881D; // deprecated, same as GL_INTENSITY_FLOAT16_APPLE and GL_INTENSITY_FLOAT16_ATI;
pub const GL_INTENSITY32UI_EXT: u32 = 0x8D73; // deprecated;
pub const GL_INTENSITY32I_EXT: u32 = 0x8D85; // deprecated;
pub const GL_INTENSITY32F_ARB: u32 = 0x8817; // deprecated, same as GL_INTENSITY_FLOAT32_APPLE and GL_INTENSITY_FLOAT32_ATI;
//
// Generic compression
//
pub const GL_COMPRESSED_RED: u32 = 0x8225;
pub const GL_COMPRESSED_ALPHA: u32 = 0x84E9; // deprecated, same as GL_COMPRESSED_ALPHA_ARB;
pub const GL_COMPRESSED_LUMINANCE: u32 = 0x84EA; // deprecated, same as GL_COMPRESSED_LUMINANCE_ARB;
pub const GL_COMPRESSED_SLUMINANCE: u32 = 0x8C4A; // deprecated, same as GL_COMPRESSED_SLUMINANCE_EXT;
pub const GL_COMPRESSED_LUMINANCE_ALPHA: u32 = 0x84EB; // deprecated, same as GL_COMPRESSED_LUMINANCE_ALPHA_ARB;
pub const GL_COMPRESSED_SLUMINANCE_ALPHA: u32 = 0x8C4B; // deprecated, same as GL_COMPRESSED_SLUMINANCE_ALPHA_EXT;
pub const GL_COMPRESSED_INTENSITY: u32 = 0x84EC; // deprecated, same as GL_COMPRESSED_INTENSITY_ARB;
pub const GL_COMPRESSED_RG: u32 = 0x8226;
pub const GL_COMPRESSED_RGB: u32 = 0x84ED; // same as GL_COMPRESSED_RGB_ARB;
pub const GL_COMPRESSED_RGBA: u32 = 0x84EE; // same as GL_COMPRESSED_RGBA_ARB;
pub const GL_COMPRESSED_SRGB: u32 = 0x8C48; // same as GL_COMPRESSED_SRGB_EXT;
pub const GL_COMPRESSED_SRGB_ALPHA: u32 = 0x8C49; // same as GL_COMPRESSED_SRGB_ALPHA_EXT;
//
// FXT1;
//
pub const GL_COMPRESSED_RGB_FXT1_3DFX: u32 = 0x86B0; // deprecated;
pub const GL_COMPRESSED_RGBA_FXT1_3DFX: u32 = 0x86B1; // deprecated;
//
// S3TC/DXT/BC
//
pub const GL_COMPRESSED_RGB_S3TC_DXT1_EXT: u32 = 0x83F0;
pub const GL_COMPRESSED_RGBA_S3TC_DXT1_EXT: u32 = 0x83F1;
pub const GL_COMPRESSED_RGBA_S3TC_DXT3_EXT: u32 = 0x83F2;
pub const GL_COMPRESSED_RGBA_S3TC_DXT5_EXT: u32 = 0x83F3;
pub const GL_COMPRESSED_SRGB_S3TC_DXT1_EXT: u32 = 0x8C4C;
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT1_EXT: u32 = 0x8C4D;
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT3_EXT: u32 = 0x8C4E;
pub const GL_COMPRESSED_SRGB_ALPHA_S3TC_DXT5_EXT: u32 = 0x8C4F;
pub const GL_COMPRESSED_LUMINANCE_LATC1_EXT: u32 = 0x8C70;
pub const GL_COMPRESSED_LUMINANCE_ALPHA_LATC2_EXT: u32 = 0x8C72;
pub const GL_COMPRESSED_SIGNED_LUMINANCE_LATC1_EXT: u32 = 0x8C71;
pub const GL_COMPRESSED_SIGNED_LUMINANCE_ALPHA_LATC2_EXT: u32 = 0x8C73;
pub const GL_COMPRESSED_RED_RGTC1: u32 = 0x8DBB; // same as GL_COMPRESSED_RED_RGTC1_EXT;
pub const GL_COMPRESSED_RG_RGTC2: u32 = 0x8DBD; // same as GL_COMPRESSED_RG_RGTC2_EXT;
pub const GL_COMPRESSED_SIGNED_RED_RGTC1: u32 = 0x8DBC; // same as GL_COMPRESSED_SIGNED_RED_RGTC1_EXT;
pub const GL_COMPRESSED_SIGNED_RG_RGTC2: u32 = 0x8DBE; // same as GL_COMPRESSED_SIGNED_RG_RGTC2_EXT;
pub const GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT: u32 = 0x8E8E; // same as GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT_ARB;
pub const GL_COMPRESSED_RGB_BPTC_UNSIGNED_FLOAT: u32 = 0x8E8F; // same as GL_COMPRESSED_RGB_BPTC_SIGNED_FLOAT_ARB;
pub const GL_COMPRESSED_RGBA_BPTC_UNORM: u32 = 0x8E8C; // same as GL_COMPRESSED_RGBA_BPTC_UNORM_ARB;
pub const GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM: u32 = 0x8E8D; // same as GL_COMPRESSED_SRGB_ALPHA_BPTC_UNORM_ARB;
//
// ETC
//
pub const GL_ETC1_RGB8_OES: u32 = 0x8D64;
pub const GL_COMPRESSED_RGB8_ETC2: u32 = 0x9274;
pub const GL_COMPRESSED_RGB8_PUNCHTHROUGH_ALPHA1_ETC2: u32 = 0x9276;
pub const GL_COMPRESSED_RGBA8_ETC2_EAC: u32 = 0x9278;
pub const GL_COMPRESSED_SRGB8_ETC2: u32 = 0x9275;
pub const GL_COMPRESSED_SRGB8_PUNCHTHROUGH_ALPHA1_ETC2: u32 = 0x9277;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ETC2_EAC: u32 = 0x9279;
pub const GL_COMPRESSED_R11_EAC: u32 = 0x9270;
pub const GL_COMPRESSED_RG11_EAC: u32 = 0x9272;
pub const GL_COMPRESSED_SIGNED_R11_EAC: u32 = 0x9271;
pub const GL_COMPRESSED_SIGNED_RG11_EAC: u32 = 0x9273;
//
// PVRTC
//
pub const GL_COMPRESSED_RGB_PVRTC_2BPPV1_IMG: u32 = 0x8C01;
pub const GL_COMPRESSED_RGB_PVRTC_4BPPV1_IMG: u32 = 0x8C00;
pub const GL_COMPRESSED_RGBA_PVRTC_2BPPV1_IMG: u32 = 0x8C03;
pub const GL_COMPRESSED_RGBA_PVRTC_4BPPV1_IMG: u32 = 0x8C02;
pub const GL_COMPRESSED_RGBA_PVRTC_2BPPV2_IMG: u32 = 0x9137;
pub const GL_COMPRESSED_RGBA_PVRTC_4BPPV2_IMG: u32 = 0x9138;
pub const GL_COMPRESSED_SRGB_PVRTC_2BPPV1_EXT: u32 = 0x8A54;
pub const GL_COMPRESSED_SRGB_PVRTC_4BPPV1_EXT: u32 = 0x8A55;
pub const GL_COMPRESSED_SRGB_ALPHA_PVRTC_2BPPV1_EXT: u32 = 0x8A56;
pub const GL_COMPRESSED_SRGB_ALPHA_PVRTC_4BPPV1_EXT: u32 = 0x8A57;
pub const GL_COMPRESSED_SRGB_ALPHA_PVRTC_2BPPV2_IMG: u32 = 0x93F0;
pub const GL_COMPRESSED_SRGB_ALPHA_PVRTC_4BPPV2_IMG: u32 = 0x93F1;
//
// ASTC
//
pub const GL_COMPRESSED_RGBA_ASTC_4x4_KHR: u32 = 0x93B0;
pub const GL_COMPRESSED_RGBA_ASTC_5x4_KHR: u32 = 0x93B1;
pub const GL_COMPRESSED_RGBA_ASTC_5x5_KHR: u32 = 0x93B2;
pub const GL_COMPRESSED_RGBA_ASTC_6x5_KHR: u32 = 0x93B3;
pub const GL_COMPRESSED_RGBA_ASTC_6x6_KHR: u32 = 0x93B4;
pub const GL_COMPRESSED_RGBA_ASTC_8x5_KHR: u32 = 0x93B5;
pub const GL_COMPRESSED_RGBA_ASTC_8x6_KHR: u32 = 0x93B6;
pub const GL_COMPRESSED_RGBA_ASTC_8x8_KHR: u32 = 0x93B7;
pub const GL_COMPRESSED_RGBA_ASTC_10x5_KHR: u32 = 0x93B8;
pub const GL_COMPRESSED_RGBA_ASTC_10x6_KHR: u32 = 0x93B9;
pub const GL_COMPRESSED_RGBA_ASTC_10x8_KHR: u32 = 0x93BA;
pub const GL_COMPRESSED_RGBA_ASTC_10x10_KHR: u32 = 0x93BB;
pub const GL_COMPRESSED_RGBA_ASTC_12x10_KHR: u32 = 0x93BC;
pub const GL_COMPRESSED_RGBA_ASTC_12x12_KHR: u32 = 0x93BD;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4_KHR: u32 = 0x93D0;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4_KHR: u32 = 0x93D1;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5_KHR: u32 = 0x93D2;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5_KHR: u32 = 0x93D3;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6_KHR: u32 = 0x93D4;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x5_KHR: u32 = 0x93D5;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x6_KHR: u32 = 0x93D6;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_8x8_KHR: u32 = 0x93D7;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x5_KHR: u32 = 0x93D8;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x6_KHR: u32 = 0x93D9;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x8_KHR: u32 = 0x93DA;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_10x10_KHR: u32 = 0x93DB;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x10_KHR: u32 = 0x93DC;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_12x12_KHR: u32 = 0x93DD;
pub const GL_COMPRESSED_RGBA_ASTC_3x3x3_OES: u32 = 0x93C0;
pub const GL_COMPRESSED_RGBA_ASTC_4x3x3_OES: u32 = 0x93C1;
pub const GL_COMPRESSED_RGBA_ASTC_4x4x3_OES: u32 = 0x93C2;
pub const GL_COMPRESSED_RGBA_ASTC_4x4x4_OES: u32 = 0x93C3;
pub const GL_COMPRESSED_RGBA_ASTC_5x4x4_OES: u32 = 0x93C4;
pub const GL_COMPRESSED_RGBA_ASTC_5x5x4_OES: u32 = 0x93C5;
pub const GL_COMPRESSED_RGBA_ASTC_5x5x5_OES: u32 = 0x93C6;
pub const GL_COMPRESSED_RGBA_ASTC_6x5x5_OES: u32 = 0x93C7;
pub const GL_COMPRESSED_RGBA_ASTC_6x6x5_OES: u32 = 0x93C8;
pub const GL_COMPRESSED_RGBA_ASTC_6x6x6_OES: u32 = 0x93C9;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_3x3x3_OES: u32 = 0x93E0;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x3x3_OES: u32 = 0x93E1;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4x3_OES: u32 = 0x93E2;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_4x4x4_OES: u32 = 0x93E3;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x4x4_OES: u32 = 0x93E4;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5x4_OES: u32 = 0x93E5;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_5x5x5_OES: u32 = 0x93E6;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x5x5_OES: u32 = 0x93E7;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6x5_OES: u32 = 0x93E8;
pub const GL_COMPRESSED_SRGB8_ALPHA8_ASTC_6x6x6_OES: u32 = 0x93E9;
//
// ATC
//
pub const GL_ATC_RGB_AMD: u32 = 0x8C92;
pub const GL_ATC_RGBA_EXPLICIT_ALPHA_AMD: u32 = 0x8C93;
pub const GL_ATC_RGBA_INTERPOLATED_ALPHA_AMD: u32 = 0x87EE;
//
// Palletized (combined palette)
//
pub const GL_PALETTE4_RGB8_OES: u32 = 0x8B90;
pub const GL_PALETTE4_RGBA8_OES: u32 = 0x8B91;
pub const GL_PALETTE4_R5_G6_B5_OES: u32 = 0x8B92;
pub const GL_PALETTE4_RGBA4_OES: u32 = 0x8B93;
pub const GL_PALETTE4_RGB5_A1_OES: u32 = 0x8B94;
pub const GL_PALETTE8_RGB8_OES: u32 = 0x8B95;
pub const GL_PALETTE8_RGBA8_OES: u32 = 0x8B96;
pub const GL_PALETTE8_R5_G6_B5_OES: u32 = 0x8B97;
pub const GL_PALETTE8_RGBA4_OES: u32 = 0x8B98;
pub const GL_PALETTE8_RGB5_A1_OES: u32 = 0x8B99;
//
// Palletized (separate palette)
//
pub const GL_COLOR_INDEX1_EXT: u32 = 0x80E2; // deprecated;
pub const GL_COLOR_INDEX2_EXT: u32 = 0x80E3; // deprecated;
pub const GL_COLOR_INDEX4_EXT: u32 = 0x80E4; // deprecated;
pub const GL_COLOR_INDEX8_EXT: u32 = 0x80E5; // deprecated;
pub const GL_COLOR_INDEX12_EXT: u32 = 0x80E6; // deprecated;
pub const GL_COLOR_INDEX16_EXT: u32 = 0x80E7; // deprecated;
//
// Depth/stencil
//
pub const GL_DEPTH_COMPONENT16: u32 = 0x81A5; // same as GL_DEPTH_COMPONENT16_SGIX and GL_DEPTH_COMPONENT16_ARB;
pub const GL_DEPTH_COMPONENT24: u32 = 0x81A6; // same as GL_DEPTH_COMPONENT24_SGIX and GL_DEPTH_COMPONENT24_ARB;
pub const GL_DEPTH_COMPONENT32: u32 = 0x81A7; // same as GL_DEPTH_COMPONENT32_SGIX and GL_DEPTH_COMPONENT32_ARB and GL_DEPTH_COMPONENT32_OES;
pub const GL_DEPTH_COMPONENT32F: u32 = 0x8CAC; // same as GL_DEPTH_COMPONENT32F_ARB;
pub const GL_DEPTH_COMPONENT32F_NV: u32 = 0x8DAB; // note that this is different from GL_DEPTH_COMPONENT32F;
pub const GL_STENCIL_INDEX1: u32 = 0x8D46; // same as GL_STENCIL_INDEX1_EXT;
pub const GL_STENCIL_INDEX4: u32 = 0x8D47; // same as GL_STENCIL_INDEX4_EXT;
pub const GL_STENCIL_INDEX8: u32 = 0x8D48; // same as GL_STENCIL_INDEX8_EXT;
pub const GL_STENCIL_INDEX16: u32 = 0x8D49; // same as GL_STENCIL_INDEX16_EXT;
pub const GL_DEPTH24_STENCIL8: u32 = 0x88F0; // same as GL_DEPTH24_STENCIL8_EXT and GL_DEPTH24_STENCIL8_OES;
pub const GL_DEPTH32F_STENCIL8: u32 = 0x8CAD; // same as GL_DEPTH32F_STENCIL8_ARB;
pub const GL_DEPTH32F_STENCIL8_NV: u32 = 0x8DAC; // note that this is different from GL_DEPTH32F_STENCIL8;
