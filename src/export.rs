use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::{animation::Animations, document::GltfDocument, mesh::Mesh};

use super::{
    buffer::BufferWriter,
    material::MaterialData,
    node::{NodeIndex, Nodes},
    skin::Skins,
    Model, Vertex,
};

#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
struct GltfParts {
    #[serde(skip_serializing_if = "Skins::is_empty")]
    skins: Skins,
    #[serde(skip_serializing_if = "Animations::is_empty")]
    animations: Animations,
    #[serde(flatten)]
    material_data: MaterialData,
}

pub fn write_gltf<T: Vertex>(
    buffer_name: &str,
    buffer_writer: &mut BufferWriter,
    model: &Model<T>,
    material_data: &MaterialData,
    scene_root: NodeIndex,
    nodes: &Nodes,
    skins: &Skins,
    animations: &Animations,
) -> String {
    let mesh = Mesh::new(model, buffer_writer);

    let document = GltfDocument::new(
        buffer_name,
        &buffer_writer,
        vec![mesh],
        material_data,
        scene_root,
        nodes,
        skins,
        animations,
    );

    serde_json::to_string_pretty(&document).unwrap()
}
