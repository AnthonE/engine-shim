//! A mesh's geometry, for drawing it: its vertex and index buffers (a VBIB
//! block, or the MBUF block a model's CTRL names for an embedded mesh),
//! each buffer's elements described by their inputs (POSITION, TEXCOORD,
//! NORMAL...) and, as s&box compiles them, meshopt-compressed. And which
//! of a model's meshes to draw, with the materials their draw calls name.
use super::kv3::Value;
use super::resource::Resource;

/// One input of a vertex: its semantic, which of them, its format (DXGI's)
/// and where in the vertex.
#[derive(Clone, Debug)]
pub struct Attribute {
    pub name: String,
    pub index: u32,
    pub format: u32,
    pub offset: usize,
}

pub struct VertexBuffer {
    pub count: usize,
    pub stride: usize,
    pub attributes: Vec<Attribute>,
    /// decompressed: `count * stride` bytes
    pub data: Vec<u8>,
}

pub struct IndexBuffer {
    pub count: usize,
    /// 2 or 4
    pub size: usize,
    pub data: Vec<u8>,
}

#[derive(Default)]
pub struct Buffers {
    pub vertices: Vec<VertexBuffer>,
    pub indices: Vec<IndexBuffer>,
}

// DXGI's formats, the ones vertices are made of
const R32G32B32A32_FLOAT: u32 = 2;
const R32G32B32_FLOAT: u32 = 6;
const R16G16B16A16_FLOAT: u32 = 10;
const R32G32_FLOAT: u32 = 16;
const R8G8B8A8_UNORM: u32 = 28;
const R8G8B8A8_UINT: u32 = 30;
const R16G16B16A16_UINT: u32 = 12;
const R16G16_UNORM: u32 = 35;
const R16G16B16A16_UNORM: u32 = 11;
const R16G16_FLOAT: u32 = 34;
const R32_UINT: u32 = 42;
const R10G10B10A2_UNORM: u32 = 24;

fn u32_at(b: &[u8], at: usize) -> Result<u32, String> {
    b.get(at..at + 4).map(|x| u32::from_le_bytes(x.try_into().unwrap())).ok_or_else(|| "vbib: too short".to_string())
}

fn f32_at(b: &[u8], at: usize) -> f32 {
    b.get(at..at + 4).map(|x| f32::from_le_bytes(x.try_into().unwrap())).unwrap_or(0.0)
}

fn half_at(b: &[u8], at: usize) -> f32 {
    b.get(at..at + 2).map(|x| half(u16::from_le_bytes(x.try_into().unwrap()))).unwrap_or(0.0)
}

fn half(h: u16) -> f32 {
    let sign = if h & 0x8000 != 0 { -1.0 } else { 1.0 };
    let exp = ((h >> 10) & 0x1f) as i32;
    let frac = (h & 0x3ff) as f32;
    sign * match exp {
        0 => frac * 2f32.powi(-24),
        31 => if frac == 0.0 { f32::INFINITY } else { f32::NAN },
        e => (1.0 + frac / 1024.0) * 2f32.powi(e - 15),
    }
}

/// A buffer's header in the binary layout: its elements, their size, its
/// inputs and its data (each offset from where it's written).
struct Header {
    count: usize,
    size: usize,
    attributes: Vec<Attribute>,
    data: Vec<u8>,
}

fn header(b: &[u8], at: usize) -> Result<Header, String> {
    let count = u32_at(b, at)? as usize;
    let size = (u32_at(b, at + 4)? & 0x3ff_ffff) as usize;
    let attributes_at = at + 8 + u32_at(b, at + 8)? as usize;
    let n = u32_at(b, at + 12)? as usize;
    let data_at = at + 16 + u32_at(b, at + 16)? as usize;
    let total = u32_at(b, at + 20)? as usize;
    let attributes = (0..n).map(|i| {
        let a = attributes_at + i * 56;
        let name = b.get(a..a + 32).ok_or("vbib: attribute past the end")?;
        let name = String::from_utf8_lossy(&name[..name.iter().position(|&c| c == 0).unwrap_or(32)]).into_owned();
        Ok(Attribute { name, index: u32_at(b, a + 32)?, format: u32_at(b, a + 36)?, offset: u32_at(b, a + 40)? as usize })
    }).collect::<Result<Vec<_>, String>>()?;
    let data = b.get(data_at..data_at + total).ok_or("vbib: data past the end")?.to_vec();
    Ok(Header { count, size, attributes, data })
}

/// Its data as `count * size` bytes: as it is, or meshopt's encoding
/// undone (smaller than that, it's compressed).
fn vertices(h: Header) -> Result<VertexBuffer, String> {
    let n = h.count * h.size;
    let data = if h.data.len() == n { h.data } else { decode_vertices(&h.data, h.count, h.size)? };
    Ok(VertexBuffer { count: h.count, stride: h.size, attributes: h.attributes, data })
}

fn indices(h: Header) -> Result<IndexBuffer, String> {
    let n = h.count * h.size;
    let data = if h.data.len() == n { h.data } else { decode_indices(&h.data, h.count, h.size)? };
    Ok(IndexBuffer { count: h.count, size: h.size, data })
}

fn decode_vertices(encoded: &[u8], count: usize, stride: usize) -> Result<Vec<u8>, String> {
    let mut out = vec![0u8; count * stride];
    let r = unsafe { meshopt::ffi::meshopt_decodeVertexBuffer(out.as_mut_ptr().cast(), count, stride, encoded.as_ptr(), encoded.len()) };
    if r != 0 { return Err(format!("vbib: meshopt vertices ({r})")); }
    Ok(out)
}

fn decode_indices(encoded: &[u8], count: usize, size: usize) -> Result<Vec<u8>, String> {
    let mut out = vec![0u8; count * size];
    let r = unsafe { meshopt::ffi::meshopt_decodeIndexBuffer(out.as_mut_ptr().cast(), count, size, encoded.as_ptr(), encoded.len()) };
    if r != 0 { return Err(format!("vbib: meshopt indices ({r})")); }
    Ok(out)
}

/// A VBIB block (or an MBUF): binary, or KV3 in older files.
pub fn read(b: &[u8]) -> Result<Buffers, String> {
    if let Ok(v) = super::kv3::read(b) { return read_kv3(&v); }
    let vertex_at = u32_at(b, 0)? as usize;
    let vertex_count = u32_at(b, 4)? as usize;
    let index_at = 8 + u32_at(b, 8)? as usize;
    let index_count = u32_at(b, 12)? as usize;
    Ok(Buffers {
        vertices: (0..vertex_count).map(|i| header(b, vertex_at + i * 24).and_then(vertices)).collect::<Result<_, _>>()?,
        indices: (0..index_count).map(|i| header(b, index_at + i * 24).and_then(indices)).collect::<Result<_, _>>()?,
    })
}

fn read_kv3(v: &Value) -> Result<Buffers, String> {
    let header = |b: &Value| Header {
        count: b.i64("m_nElementCount").max(0) as usize,
        size: b.i64("m_nElementSizeInBytes").max(0) as usize & 0x3ff_ffff,
        attributes: b.get("m_inputLayoutFields").map(Value::as_array).unwrap_or(&[]).iter().map(|a| Attribute {
            name: a.get("m_pSemanticName").and_then(Value::as_str).unwrap_or("").to_string(),
            index: a.i64("m_nSemanticIndex") as u32,
            format: a.i64("m_Format") as u32,
            offset: a.i64("m_nOffset").max(0) as usize,
        }).collect(),
        data: b.get("m_pData").map(Value::bytes).unwrap_or_default(),
    };
    Ok(Buffers {
        vertices: v.get("m_vertexBuffers").map(Value::as_array).unwrap_or(&[]).iter().map(|b| vertices(header(b))).collect::<Result<_, _>>()?,
        indices: v.get("m_indexBuffers").map(Value::as_array).unwrap_or(&[]).iter().map(|b| indices(header(b))).collect::<Result<_, _>>()?,
    })
}

impl VertexBuffer {
    pub fn attribute(&self, name: &str, index: u32) -> Option<&Attribute> {
        self.attributes.iter().find(|a| a.name.eq_ignore_ascii_case(name) && a.index == index)
    }

    /// An input of every vertex, as up to four floats (unpacked as its
    /// format says).
    pub fn floats(&self, name: &str, index: u32) -> Option<Vec<[f32; 4]>> {
        let a = self.attribute(name, index)?;
        Some((0..self.count).map(|v| {
            let at = v * self.stride + a.offset;
            let d = &self.data;
            match a.format {
                R32G32B32A32_FLOAT => [f32_at(d, at), f32_at(d, at + 4), f32_at(d, at + 8), f32_at(d, at + 12)],
                R32G32B32_FLOAT => [f32_at(d, at), f32_at(d, at + 4), f32_at(d, at + 8), 0.0],
                R32G32_FLOAT => [f32_at(d, at), f32_at(d, at + 4), 0.0, 0.0],
                R16G16B16A16_FLOAT => [half_at(d, at), half_at(d, at + 2), half_at(d, at + 4), half_at(d, at + 6)],
                R16G16_FLOAT => [half_at(d, at), half_at(d, at + 2), 0.0, 0.0],
                R8G8B8A8_UNORM => core::array::from_fn(|k| d.get(at + k).copied().unwrap_or(0) as f32 / 255.0),
                _ => [0.0; 4],
            }
        }).collect())
    }

    /// An input of every vertex as up to four whole numbers (a vertex's
    /// bones: BLENDINDICES).
    pub fn uints(&self, name: &str, index: u32) -> Option<Vec<[u32; 4]>> {
        let a = self.attribute(name, index)?;
        Some((0..self.count).map(|v| {
            let at = v * self.stride + a.offset;
            let d = &self.data;
            match a.format {
                R16G16B16A16_UINT => core::array::from_fn(|k| d.get(at + k * 2..at + k * 2 + 2).map(|x| u16::from_le_bytes(x.try_into().unwrap()) as u32).unwrap_or(0)),
                _ => core::array::from_fn(|k| d.get(at + k).copied().unwrap_or(0) as u32),
            }
        }).collect())
    }

    /// Each vertex's bones and how much each moves it: BLENDINDICES and
    /// BLENDWEIGHT (none: all on the first). None if it isn't skinned.
    pub fn skin(&self) -> Option<Vec<([u32; 4], [f32; 4])>> {
        let bones = self.uints("BLENDINDICES", 0)?;
        let weights = self.attribute("BLENDWEIGHT", 0).map(|a| {
            let unorm16 = |d: &[u8], at: usize| d.get(at..at + 2).map(|x| u16::from_le_bytes(x.try_into().unwrap()) as f32 / 65535.0).unwrap_or(0.0);
            (0..self.count).map(|v| {
                let at = v * self.stride + a.offset;
                let d = &self.data;
                match a.format {
                    R16G16_UNORM => [unorm16(d, at), unorm16(d, at + 2), 0.0, 0.0],
                    R16G16B16A16_UNORM => core::array::from_fn(|k| unorm16(d, at + k * 2)),
                    _ => core::array::from_fn(|k| d.get(at + k).copied().unwrap_or(0) as f32 / 255.0),
                }
            }).collect::<Vec<[f32; 4]>>()
        });
        Some(bones.into_iter().enumerate().map(|(i, b)| (b, weights.as_ref().map(|w| w[i]).unwrap_or([1.0, 0.0, 0.0, 0.0]))).collect())
    }

    /// Each vertex's normal, however it's kept: three floats, or packed
    /// with its tangent (four bytes as Source 2 first packed them, or a
    /// word, octahedral: in its top twenty bits, or as s&box packs them
    /// now, its first twenty, the tangent in the rest).
    pub fn normals(&self) -> Option<Vec<[f32; 3]>> {
        let a = self.attribute("NORMAL", 0)?;
        Some((0..self.count).map(|v| {
            let at = v * self.stride + a.offset;
            let d = &self.data;
            match a.format {
                R32G32B32_FLOAT | R32G32B32A32_FLOAT => normalize([f32_at(d, at), f32_at(d, at + 4), f32_at(d, at + 8)]),
                R8G8B8A8_UNORM => packed_normal(d.get(at).copied().unwrap_or(128) as f32, d.get(at + 1).copied().unwrap_or(128) as f32),
                R32_UINT => octahedral(u32_at(d, at).unwrap_or(0)),
                // as s&box packs them now: octahedral in the first twenty bits
                R10G10B10A2_UNORM => { let w = u32_at(d, at).unwrap_or(0); unfold(((w & 0x3ff) as f32 / 1023.0) * 2.0 - 1.0, (((w >> 10) & 0x3ff) as f32 / 1023.0) * 2.0 - 1.0) }
                _ => [0.0, 0.0, 1.0],
            }
        }).collect())
    }
}

fn normalize(v: [f32; 3]) -> [f32; 3] {
    let l = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if l > 1e-12 { [v[0] / l, v[1] / l, v[2] / l] } else { [0.0, 0.0, 1.0] }
}

/// A normal in two bytes, as Source 2 first packed them with their tangent.
fn packed_normal(x: f32, y: f32) -> [f32; 3] {
    let (mut x, mut y) = (x - 128.0, y - 128.0);
    let z_sign_bit = if x < 0.0 { 1.0 } else { 0.0 };
    let t_sign_bit = if y < 0.0 { 1.0 } else { 0.0 };
    let z_sign = -(2.0 * z_sign_bit - 1.0);
    let t_sign = -(2.0 * t_sign_bit - 1.0);
    x = x * z_sign - z_sign_bit - 64.0;
    y = y * t_sign - t_sign_bit - 64.0;
    let x_sign_bit = if x < 0.0 { 1.0 } else { 0.0 };
    let y_sign_bit = if y < 0.0 { 1.0 } else { 0.0 };
    let x_sign = -(2.0 * x_sign_bit - 1.0);
    let y_sign = -(2.0 * y_sign_bit - 1.0);
    x = (x * x_sign - x_sign_bit) / 63.0;
    y = (y * y_sign - y_sign_bit) / 63.0;
    let z = 1.0 - x - y;
    normalize([x * x_sign, y * y_sign, z * z_sign])
}

/// A normal and tangent frame in one word: the normal octahedral in its
/// top twenty bits (ten each), the tangent's angle and sign below.
fn octahedral(p: u32) -> [f32; 3] {
    unfold(((p >> 12) & 0x3ff) as f32 / 1023.0 * 2.0 - 1.0, ((p >> 22) & 0x3ff) as f32 / 1023.0 * 2.0 - 1.0)
}

/// A direction from its octahedral coordinates (each -1 to 1).
fn unfold(x: f32, y: f32) -> [f32; 3] {
    let z = 1.0 - x.abs() - y.abs();
    // past the octahedron's upper half: folded back
    let t = (-z).clamp(0.0, 1.0);
    let x = x + if x >= 0.0 { -t } else { t };
    let y = y + if y >= 0.0 { -t } else { t };
    normalize([x, y, z])
}

impl IndexBuffer {
    pub fn indices(&self) -> Vec<u32> {
        match self.size {
            2 => self.data.chunks_exact(2).map(|c| u16::from_le_bytes(c.try_into().unwrap()) as u32).collect(),
            _ => self.data.chunks_exact(4).map(|c| u32::from_le_bytes(c.try_into().unwrap())).collect(),
        }
    }
}

/// One draw: a material over a run of a buffer's triangles.
pub struct Draw {
    pub material: String,
    pub vertex_buffer: usize,
    pub index_buffer: usize,
    pub base_vertex: usize,
    pub start_index: usize,
    pub index_count: usize,
    pub tint: [f32; 3],
}

/// A mesh to draw: its draws and the buffers they read.
pub struct Mesh {
    pub draws: Vec<Draw>,
    pub buffers: Buffers,
    /// its bones (a vertex's BLENDINDICES) as the model's skeleton numbers them
    pub bones: Vec<u32>,
}

fn draws(mesh: &Value) -> Vec<Draw> {
    let mut out = Vec::new();
    for o in mesh.get("m_sceneObjects").map(Value::as_array).unwrap_or(&[]) {
        for d in o.get("m_drawCalls").map(Value::as_array).unwrap_or(&[]) {
            if d.get("m_nPrimitiveType").and_then(Value::as_str).is_some_and(|p| p != "RENDER_PRIM_TRIANGLES") { continue; }
            let tint = d.get("m_vTintColor").map(Value::floats).unwrap_or_default();
            out.push(Draw {
                material: d.get("m_material").and_then(Value::as_str).unwrap_or("").to_string(),
                vertex_buffer: d.get("m_vertexBuffers").map(Value::as_array).and_then(|v| v.first()).map(|b| b.i64("m_hBuffer").max(0) as usize).unwrap_or(0),
                index_buffer: d.get("m_indexBuffer").map(|b| b.i64("m_hBuffer").max(0) as usize).unwrap_or(0),
                base_vertex: d.i64("m_nBaseVertex").max(0) as usize,
                start_index: d.i64("m_nStartIndex").max(0) as usize,
                index_count: d.i64("m_nIndexCount").max(0) as usize,
                tint: [tint.first().copied().unwrap_or(1.0), tint.get(1).copied().unwrap_or(1.0), tint.get(2).copied().unwrap_or(1.0)],
            });
        }
    }
    out
}

/// What a model draws up close: its meshes of LOD 0 in its default mesh
/// groups (each embedded, or a `.vmesh_c` of its own), with their buffers.
pub fn model(r: &Resource, read_compiled: impl Fn(&str) -> Option<Vec<u8>>) -> Result<Vec<Mesh>, String> {
    let data = r.kv3("DATA").ok_or("no DATA")?.map_err(|e| e.to_string())?;
    let masks = |k: &str| data.get(k).map(Value::as_array).unwrap_or(&[]).iter().map(|m| m.as_i64().unwrap_or(-1) as u64).collect::<Vec<u64>>();
    let (lods, groups) = (masks("m_refLODGroupMasks"), masks("m_refMeshGroupMasks"));
    let default = data.get("m_nDefaultMeshGroupMask").and_then(Value::as_i64).map(|m| m as u64).unwrap_or(u64::MAX);
    // each mesh's bones in the skeleton: its run of the remapping table
    let table: Vec<u32> = data.get("m_remappingTable").map(Value::as_array).unwrap_or(&[]).iter().map(|b| b.as_i64().unwrap_or(0).max(0) as u32).collect();
    let starts: Vec<usize> = data.get("m_remappingTableStarts").map(Value::as_array).unwrap_or(&[]).iter().map(|b| b.as_i64().unwrap_or(0).max(0) as usize).collect();
    let bones = |i: usize| -> Vec<u32> {
        let Some(&start) = starts.get(i) else { return Vec::new() };
        let end = starts.get(i + 1).copied().unwrap_or(table.len()).max(start);
        table.get(start..end.min(table.len())).unwrap_or(&[]).to_vec()
    };
    let shown = |i: usize| lods.get(i).is_none_or(|m| m & 1 != 0) && groups.get(i).is_none_or(|m| m & default != 0);
    let mut out = Vec::new();
    if let Some(Ok(ctrl)) = r.kv3("CTRL") {
        for e in ctrl.get("embedded_meshes").map(Value::as_array).unwrap_or(&[]) {
            let index = e.i64("mesh_index").max(0) as usize;
            if !shown(index) { continue; }
            let block = |k: &str| r.blocks.get(e.i64(k).max(0) as usize).map(|&(_, at, n)| &r.bytes[at..at + n]);
            let (Some(mesh), Some(vbib)) = (block("data_block"), block("vbib_block")) else { continue };
            let mesh = super::kv3::read(mesh).map_err(|e| e.to_string())?;
            out.push(Mesh { draws: draws(&mesh), buffers: read(vbib)?, bones: bones(index) });
        }
    }
    for (i, name) in data.get("m_refMeshes").map(Value::as_array).unwrap_or(&[]).iter().enumerate() {
        let Some(name) = name.as_str().filter(|n| !n.is_empty()) else { continue };
        if !shown(i) { continue; }
        let Some(file) = read_compiled(name).and_then(|b| Resource::parse(b).ok()) else { continue };
        let (Some(Ok(mesh)), Some(vbib)) = (file.kv3("DATA"), file.block("VBIB").or(file.block("MBUF"))) else { continue };
        out.push(Mesh { draws: draws(&mesh), buffers: read(vbib)?, bones: bones(i) });
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A package's files fetched whole (tools/package.sh), if this machine
    /// has them: sbox.game's content, never copied here.
    fn content(path: &str) -> Option<Vec<u8>> {
        std::fs::read(std::path::Path::new(&std::env::var("ENGINE2_CONTENT").ok()?).join(path)).ok()
    }

    #[test]
    fn a_downloaded_models_meshes_and_textures_decode() {
        let Some(bytes) = content("models/sbox_props/tv/tv.vmdl_c") else { return };
        let r = Resource::parse(bytes).unwrap();
        let meshes = model(&r, content).unwrap();
        assert_eq!(meshes.len(), 1, "LOD 0 only");
        let m = &meshes[0];
        let vb = &m.buffers.vertices[0];
        let positions = vb.floats("POSITION", 0).unwrap();
        let normals = vb.normals().unwrap();
        let indices = m.buffers.indices[0].indices();
        eprintln!("{} vertices ({:?}), {} indices, draws {:?}", vb.count, vb.attributes.iter().map(|a| (&a.name, a.format)).collect::<Vec<_>>(), indices.len(), m.draws.iter().map(|d| (&d.material, d.index_count)).collect::<Vec<_>>());
        // within the mesh's bounds (the model file's): decoded right
        assert!(positions.iter().all(|p| p[0] > -12.0 && p[0] < 14.5 && p[1].abs() < 12.6 && p[2] > -0.01 && p[2] < 26.8));
        assert!(indices.iter().all(|&i| (i as usize) < vb.count));
        assert!(normals.iter().all(|n| ((n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt() - 1.0).abs() < 1e-3));
        // each vertex's normal agrees with the faces around it
        let mut around = vec![[0f32; 3]; vb.count];
        for t in indices.chunks_exact(3) {
            let p = |i: u32| positions[i as usize];
            let (a, b, c) = (p(t[0]), p(t[1]), p(t[2]));
            let (u, v) = ([b[0] - a[0], b[1] - a[1], b[2] - a[2]], [c[0] - a[0], c[1] - a[1], c[2] - a[2]]);
            let face = [u[1] * v[2] - u[2] * v[1], u[2] * v[0] - u[0] * v[2], u[0] * v[1] - u[1] * v[0]];
            for &i in t { for k in 0..3 { around[i as usize][k] += face[k]; } }
        }
        let agree = (0..vb.count).map(|i| { let (n, g) = (normals[i], normalize(around[i])); n[0] * g[0] + n[1] * g[1] + n[2] * g[2] }).sum::<f32>() / vb.count as f32;
        eprintln!("normals agree with their faces: {agree:.3} on average");
        assert!(agree > 0.95, "{agree}");

        let mat = crate::assets::material::read(&Resource::parse(content("models/sbox_props/tv/tv.vmat_c").unwrap()).unwrap()).unwrap();
        assert_eq!(mat.shader, "shaders/complex.shader");
        let color = &mat.textures["g_tColor"];
        let t = crate::assets::texture::read(Resource::parse(content(&format!("{color}_c")).unwrap()).unwrap()).unwrap();
        let (w, h, px) = t.rgba(0).unwrap();
        assert_eq!((w, h, px.len()), (2048, 2048, 2048 * 2048 * 4));
        let mean: Vec<f32> = (0..3).map(|c| px.chunks_exact(4).map(|p| p[c] as f32).sum::<f32>() / (w * h) as f32).collect();
        eprintln!("{color}: {w}x{h} format {} mean colour {mean:?}", t.format);
        assert!(mean.iter().all(|&m| m > 5.0 && m < 250.0), "a picture, not a blank");
    }
}
