use crate::{
    animation::Animations,
    document::{BufferSource, GltfDocument},
    mesh::Mesh,
};

use super::{
    Model, Vertex,
    buffer::BufferWriter,
    material::MaterialData,
    node::{NodeIndex, Nodes},
    skin::Skins,
};

pub fn write_gltf<T: Vertex>(
    buffer_source: BufferSource,
    buffer_writer: &mut BufferWriter,
    models: &[Model<T>],
    material_data: &MaterialData,
    scene_root: NodeIndex,
    nodes: &Nodes,
    skins: &Skins,
    animations: &Animations,
) -> String {
    let meshes: Vec<_> = models.iter().map(|x| Mesh::new(x, buffer_writer)).collect();

    let document = GltfDocument::new(
        buffer_source,
        &buffer_writer,
        meshes,
        material_data,
        scene_root,
        nodes,
        skins,
        animations,
    );

    serde_json::to_string_pretty(&document).unwrap()
}
