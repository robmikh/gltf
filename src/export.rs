use std::collections::HashMap;

use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::{animation::Animations, material::MaterialIndex, mesh::Primitive};

use super::{
    buffer::{BufferTypeEx, BufferViewTarget, BufferWriter, MinMax},
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
    // Write our vertex and index data
    let indices_view =
        buffer_writer.create_view(&model.indices, Some(BufferViewTarget::ElementArrayBuffer));
    let vertex_attributes = T::write_slices(buffer_writer, &model.vertices);

    let mut mesh_primitives = Vec::new();
    for mesh in &model.meshes {
        let indices = &model.indices[mesh.indices_range.start..mesh.indices_range.end];
        let (min, max) = u32::find_min_max(indices);
        let indices_accessor = buffer_writer.create_accessor_with_min_max(
            indices_view,
            mesh.indices_range.start * std::mem::size_of::<u32>(),
            mesh.indices_range.end - mesh.indices_range.start,
            MinMax { min, max },
        );
        mesh_primitives.push((indices_accessor, mesh.texture_index));
    }

    // Create primitives
    let attributes = {
        let attribute_pairs = vertex_attributes.attribute_pairs();
        let mut attributes = HashMap::new();
        for (name, value) in attribute_pairs {
            attributes.insert(name, value);
        }
        attributes
    };
    let mut primitives = Vec::with_capacity(model.meshes.len());
    for (indices, material) in mesh_primitives {
        let mut material_index = MaterialIndex::default();
        material_index.0 = material;
        primitives.push(Primitive {
            attributes: attributes.clone(),
            indices,
            material: material_index,
        });
    }
    let primitives = serde_json::to_string_pretty(&primitives).unwrap();

    // Create buffer views and accessors
    let buffer_views = buffer_writer.write_buffer_views();
    let accessors = buffer_writer.write_accessors();

    // Write GLTF
    let gltf_parts = GltfParts {
        skins: skins.clone(),
        animations: animations.clone(),
        material_data: material_data.clone(),
    };
    let gltf_parts = serde_json::to_string_pretty(&gltf_parts).unwrap();
    let gltf_parts = {
        let first = gltf_parts.find('{').unwrap();
        let last = gltf_parts.rfind('}').unwrap();
        &gltf_parts[first + 1..last]
    };

    let gltf_text = format!(
        r#"{{
        "scene" : 0,
        "scenes" : [
            {{
                "nodes" : [ {} ]
            }}
        ],
        "nodes" : 
{}
        ,
        
        "meshes" : [
            {{
            "primitives" : 
{}
            }}
        ],

          "buffers" : [
            {{
                "uri" : "{}",
                "byteLength" : {}
            }}
          ],

            "bufferViews" : 
                {}
            ,

            "accessors" : 
                {}
            ,

{},

            "asset" : {{
                "version" : "2.0"
            }}
        }}
    "#,
        scene_root.0,
        nodes.write_nodes(),
        primitives,
        buffer_name,
        buffer_writer.buffer_len(),
        buffer_views,
        accessors,
        gltf_parts,
    );

    gltf_text
}
