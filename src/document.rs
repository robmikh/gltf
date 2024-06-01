use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::{
    animation::Animations,
    buffer::BufferWriter,
    material::MaterialData,
    mesh::Mesh,
    node::{NodeIndex, Nodes},
    skin::Skins,
};

// TODO: Move
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct Buffer {
    uri: String,
    byte_length: usize,
}

// TODO: Move
#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct Scene {
    nodes: Vec<NodeIndex>,
}

#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct Asset {
    version: String,
}

#[skip_serializing_none]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GltfDocument<'a> {
    scene: usize,
    scenes: Vec<Scene>,
    #[serde(skip_serializing_if = "Nodes::is_empty")]
    nodes: &'a Nodes,
    #[serde(skip_serializing_if = "Vec::<_>::is_empty")]
    meshes: Vec<Mesh>,
    buffers: Vec<Buffer>,
    #[serde(flatten)]
    buffer_writer: &'a BufferWriter,
    #[serde(skip_serializing_if = "Skins::is_empty")]
    skins: &'a Skins,
    #[serde(skip_serializing_if = "Animations::is_empty")]
    animations: &'a Animations,
    #[serde(flatten)]
    material_data: &'a MaterialData,
    asset: Asset,
}

impl<'a> GltfDocument<'a> {
    pub fn new(
        buffer_uri: &str,
        buffer_writer: &'a BufferWriter,
        meshes: Vec<Mesh>,
        material_data: &'a MaterialData,
        scene_root: NodeIndex,
        nodes: &'a Nodes,
        skins: &'a Skins,
        animations: &'a Animations,
    ) -> Self {
        Self {
            scene: 0,
            scenes: vec![Scene {
                nodes: vec![scene_root],
            }],
            nodes,
            meshes,
            buffers: vec![Buffer {
                uri: buffer_uri.to_owned(),
                byte_length: buffer_writer.buffer_len(),
            }],
            buffer_writer,
            skins,
            animations,
            material_data,
            asset: Asset {
                version: "2.0".to_owned(),
            },
        }
    }
}
