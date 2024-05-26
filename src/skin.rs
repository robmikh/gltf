use serde::Serialize;

use crate::storage::{Storage, StorageIndex};

use super::{buffer::AccessorIndex, node::NodeIndex};

pub type SkinIndex = StorageIndex<Skin>;

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Skin {
    pub inverse_bind_matrices: AccessorIndex,
    pub joints: Vec<NodeIndex>,
}

#[derive(Clone, Default, Serialize)]
#[serde(transparent)]
pub struct Skins {
    skins: Storage<Skin>,
}

impl Skins {
    pub fn new() -> Self {
        Self {
            skins: Storage::new(),
        }
    }

    pub fn add_skin(&mut self, skin: Skin) -> SkinIndex {
        self.skins.allocate_with(skin)
    }

    pub fn is_empty(&self) -> bool {
        self.skins.is_empty()
    }

    pub fn write_skins(&self) -> String {
        serde_json::to_string_pretty(&self.skins).unwrap()
    }
}
