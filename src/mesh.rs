use std::collections::HashMap;

use serde::Serialize;
use serde_with::skip_serializing_none;

use crate::{buffer::AccessorIndex, material::MaterialIndex};



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
