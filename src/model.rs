use tobj::{LoadError, LoadOptions, LoadResult};

#[repr(C)]
pub struct Vertex {
    pub pos: glm::Vec3,
    pub normal: glm::Vec3,
    pub uv: glm::Vec2,
}

pub fn load() -> (Vec<Vertex>, Vec<u16>) {
    let load_opts = LoadOptions::default();
    let load_res = tobj::load_obj("./assets/suzanne.obj", &load_opts).unwrap();
    let (model, mat) = (load_res.0, load_res.1.unwrap());

    let model = &model[0];

    let model_indices = &model.mesh.indices;
    let size = model_indices.len();
    let m_verts = &model.mesh.positions;

    let m_normals = &model.mesh.normals;
    let m_normal_indices = &model.mesh.normal_indices;

    let m_tex = &model.mesh.texcoords;
    let m_tex_indices = &model.mesh.texcoord_indices;

    let mut vertices: Vec<Vertex> = Vec::new();
    let mut indices: Vec<u16> = Vec::new();

    assert!(
        model_indices.len() == m_normal_indices.len() && model_indices.len() == m_tex_indices.len()
    );

    let mut idx: usize = 0;
    while idx < model_indices.len() {
        let v_idx = model_indices[idx] as usize;
        let n_idx = m_normal_indices[idx] as usize;
        let t_idx = m_tex_indices[idx] as usize;

        vertices.push(Vertex {
            pos: glm::vec3(
                m_verts[v_idx * 3],
                -m_verts[v_idx * 3 + 1],
                m_verts[v_idx * 3 + 2],
            ),
            normal: glm::vec3(
                m_normals[n_idx * 3],
                -m_normals[n_idx * 3 + 1],
                m_normals[n_idx * 3 + 2],
            ),
            uv: glm::vec2(m_tex[t_idx * 2], 1.0 - m_tex[t_idx * 2 + 1]),
        });

        indices.push(idx as u16);
        idx += 1;
    }

    (vertices, indices)
}
