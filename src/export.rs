use crate::{
    animation::Animations,
    document::{BufferSource, GltfDocument},
    mesh::Mesh,
};

use super::{
    buffer::BufferWriter,
    material::MaterialData,
    node::{NodeIndex, Nodes},
    skin::Skins,
    Model, Vertex,
};

pub fn write_gltf<T: Vertex>(
    buffer_source: BufferSource,
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
        buffer_source,
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
