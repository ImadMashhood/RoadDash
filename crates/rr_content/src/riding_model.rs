//! Bounded renderer-only mesh intake. Physics is never derived from these assets.
use serde::Deserialize;
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Material {
    pub region: String,
    #[serde(default)]
    pub color: Option<[f32; 4]>,
    #[serde(default)]
    pub metallic: f32,
    #[serde(default)]
    pub roughness: f32,
    #[serde(default)]
    pub alpha: f32,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Record {
    name: String,
    group: String,
    vertices: usize,
    indices: usize,
    material: Material,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Header {
    version: u32,
    scale: f32,
    visual_offset_y: f32,
    vertex_stride_bytes: usize,
    physics_data: bool,
    records: Vec<Record>,
}
#[derive(Debug)]
pub struct MeshData {
    pub name: String,
    pub group: String,
    pub material: Material,
    pub positions: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub uvs: Vec<[f32; 2]>,
    pub weights: Vec<[f32; 4]>,
    pub joints: Vec<[u16; 4]>,
    pub indices: Vec<u32>,
}
/// The small proprietary-free binary is authored by tools/export_riding_models.py.
pub fn decode(bytes: &[u8]) -> Result<Vec<MeshData>, String> {
    if bytes.len() < 12 || &bytes[..8] != b"RDMESH01" {
        return Err("invalid visual mesh magic".into());
    }
    let size = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let end = 12_usize.checked_add(size).ok_or("header overflow")?;
    let h: Header = serde_json::from_slice(bytes.get(12..end).ok_or("truncated header")?)
        .map_err(|e| e.to_string())?;
    if h.version != 1
        || h.vertex_stride_bytes != 56
        || h.physics_data
        || h.scale != 1.2
        || h.visual_offset_y != 0.03
        || h.records.is_empty()
        || h.records.len() > 64
    {
        return Err("unsupported visual-only mesh contract".into());
    }
    let mut cursor = end;
    let mut out = Vec::new();
    for r in h.records {
        if r.vertices == 0
            || r.vertices > 100_000
            || r.indices == 0
            || r.indices > 600_000
            || !r.indices.is_multiple_of(3)
            || ![
                "standing",
                "seated",
                "motorcycle",
                "front-wheel",
                "rear-wheel",
            ]
            .contains(&r.group.as_str())
            || !["body", "shirt", "helmet", "visor", "fixed", "paint"]
                .contains(&r.material.region.as_str())
            || [r.material.metallic, r.material.roughness, r.material.alpha]
                .iter()
                .any(|v| !v.is_finite() || !(0.0..=1.0).contains(v))
            || r.material
                .color
                .is_some_and(|c| c.iter().any(|v| !v.is_finite() || !(0.0..=1.0).contains(v)))
        {
            return Err("invalid mesh counts, group or material".into());
        }
        let len = r.vertices * 56 + r.indices * 4;
        let data = bytes
            .get(cursor..cursor.checked_add(len).ok_or("buffer overflow")?)
            .ok_or("truncated vertex/index buffer")?;
        cursor += len;
        let mut m = MeshData {
            name: r.name,
            group: r.group,
            material: r.material,
            positions: Vec::new(),
            normals: Vec::new(),
            uvs: Vec::new(),
            weights: Vec::new(),
            joints: Vec::new(),
            indices: Vec::new(),
        };
        for v in data[..r.vertices * 56].chunks_exact(56) {
            let f: Vec<f32> = v[..48]
                .chunks_exact(4)
                .map(|b| f32::from_le_bytes(b.try_into().unwrap()))
                .collect();
            let j: Vec<u16> = v[48..]
                .chunks_exact(2)
                .map(|b| u16::from_le_bytes(b.try_into().unwrap()))
                .collect();
            if f.iter().any(|v| !v.is_finite())
                || f[..3].iter().any(|v| v.abs() > 5.0)
                || f[8..12].iter().any(|w| *w < 0.0 || *w > 1.0)
                || (f[8..12].iter().sum::<f32>() - 1.0).abs() > 0.0001
                || j.iter().any(|j| *j >= 11)
            {
                return Err("non-finite or invalid visual vertex/skin weights".into());
            }
            m.positions.push(f[0..3].try_into().unwrap());
            m.normals.push(f[3..6].try_into().unwrap());
            m.uvs.push(f[6..8].try_into().unwrap());
            m.weights.push(f[8..12].try_into().unwrap());
            m.joints.push(j.try_into().unwrap());
        }
        for i in data[r.vertices * 56..].chunks_exact(4) {
            let i = u32::from_le_bytes(i.try_into().unwrap());
            if i as usize >= r.vertices {
                return Err("visual index out of range".into());
            }
            m.indices.push(i);
        }
        out.push(m);
    }
    if cursor != bytes.len() {
        return Err("unexpected visual bundle trailer".into());
    }
    Ok(out)
}
