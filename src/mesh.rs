use std::collections::HashMap;

use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::{
    Model, Vertex,
    buffer::{AccessorIndex, BufferTypeEx, BufferViewTarget, BufferWriter, MinMax},
    material::MaterialIndex,
};

#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Primitive {
    pub attributes: HashMap<&'static str, usize>,
    pub indices: AccessorIndex,
    pub material: MaterialIndex,
}

#[skip_serializing_none]
#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mesh {
    pub primitives: Vec<Primitive>,
}

impl Mesh {
    pub fn new<T: Vertex>(model: &Model<T>, buffer_writer: &mut BufferWriter) -> Self {
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
            let mut attributes = HashMap::new();
            for (name, value) in vertex_attributes {
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

        Self { primitives }
    }
}
