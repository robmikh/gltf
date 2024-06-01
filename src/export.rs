use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::{animation::Animations, mesh::Mesh};

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
{}
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
        serde_json::to_string_pretty(&mesh).unwrap(),
        buffer_name,
        buffer_writer.buffer_len(),
        buffer_views,
        accessors,
        gltf_parts,
    );

    gltf_text
}
