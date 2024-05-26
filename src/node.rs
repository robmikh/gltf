use glam::{Vec3, Vec4};
use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::storage::{Storage, StorageIndex};

use super::skin::SkinIndex;

#[derive(Copy, Clone, Debug, Serialize)]
pub struct MeshIndex(pub usize);
pub type NodeIndex = StorageIndex<Node>;

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub mesh: Option<MeshIndex>,
    pub skin: Option<SkinIndex>,
    pub name: Option<String>,
    pub translation: Option<Vec3>,
    pub rotation: Option<Vec4>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub children: Vec<NodeIndex>,
}

pub struct Nodes {
    nodes: Storage<Node>,
}

impl Nodes {
    pub fn new(capacity: usize) -> Self {
        Self {
            nodes: Storage::with_capacity(capacity),
        }
    }

    pub fn add_node(&mut self, node: Node) -> NodeIndex {
        self.nodes.allocate_with(node)
    }

    pub fn write_nodes(&self) -> Vec<String> {
        vec![serde_json::to_string_pretty(&self.nodes).unwrap()]
    }
}
