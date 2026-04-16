use ktx::{Ktx, KtxInfo};

pub fn ktxTexture_GetOffset(
    tex: Ktx<&[u8]>,
    level: u32,
    layer: u32,
    face_slice: u32,
) -> Option<u64> {
    if level >= tex.mipmap_levels() || layer >= tex.mipmap_levels() {
        return None;
    }

    if tex.faces() == 6 {
        if face_slice >= tex.faces() {
            return None;
        }
    } else {
        let max_slice = std::cmp::max(1, tex.pixel_depth() >> level);
        if face_slice >= max_slice {
            return None;
        }
    }

    let mut offset: u64 = 0;

    // ktxTexture_calcDataSizeLevels
    for (i, tex_layer) in tex.textures().enumerate() {
        if i >= level as usize {
            break;
        }
        offset += tex_layer.len() as u64;
    }

    if layer != 0 {
        let layer_size = tex.texture_level(level).len();
        offset += layer as u64 * layer_size as u64;
    }

    // should be done but i dont want to rn and i dont think i need to rn
    // if face_slice != 0 {
    // ktx_size_t imageSize;
    // imageSize = ktxTexture_GetImageSize(ktxTexture(This), level);
    // #if (KTX_GL_UNPACK_ALIGNMENT != 4)
    //     if (This->isCubemap)
    //     _KTX_PAD4(imageSize); // Account for cubePadding.
    // #endif
    // *pOffset += faceSlice * imageSize;
    // }

    Some(offset)
}
