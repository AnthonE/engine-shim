//! Materials (`.vmat_c`): the shader a surface is drawn with
//! (`shaders/complex.shader` for nearly all of them) and what it's set to:
//! its textures by parameter (`g_tColor`, `g_tNormal`...), and its ints,
//! floats and vectors (`F_ALPHA_TEST`, `g_flMetalness`, `g_vColorTint`...).
use std::collections::HashMap;

use super::kv3::Value;
use super::resource::Resource;

#[derive(Default, Debug)]
pub struct Material {
    pub name: String,
    pub shader: String,
    pub textures: HashMap<String, String>,
    pub ints: HashMap<String, i64>,
    pub floats: HashMap<String, f32>,
    pub vectors: HashMap<String, [f32; 4]>,
}

pub fn read(r: &Resource) -> Result<Material, String> {
    let d = r.kv3("DATA").ok_or("vmat: no DATA")?.map_err(|e| e.to_string())?;
    let list = |k: &str| d.get(k).map(Value::as_array).unwrap_or(&[]);
    let name = |p: &Value| p.get("m_name").and_then(Value::as_str).unwrap_or("").to_string();
    Ok(Material {
        name: d.get("m_materialName").and_then(Value::as_str).unwrap_or("").to_string(),
        shader: d.get("m_shaderName").and_then(Value::as_str).unwrap_or("").to_string(),
        textures: list("m_textureParams").iter().filter_map(|p| Some((name(p), p.get("m_pValue")?.as_str()?.to_string()))).collect(),
        ints: list("m_intParams").iter().map(|p| (name(p), p.i64("m_nValue"))).collect(),
        floats: list("m_floatParams").iter().map(|p| (name(p), p.f32("m_flValue"))).collect(),
        vectors: list("m_vectorParams").iter().map(|p| {
            let v = p.get("m_value").map(Value::floats).unwrap_or_default();
            (name(p), core::array::from_fn(|k| v.get(k).copied().unwrap_or(0.0)))
        }).collect(),
    })
}

impl Material {
    pub fn int(&self, name: &str) -> i64 { self.ints.get(name).copied().unwrap_or(0) }
    pub fn float(&self, name: &str, default: f32) -> f32 { self.floats.get(name).copied().unwrap_or(default) }
    pub fn vector(&self, name: &str, default: [f32; 4]) -> [f32; 4] { self.vectors.get(name).copied().unwrap_or(default) }
}
