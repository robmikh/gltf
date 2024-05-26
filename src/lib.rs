use self::buffer::BufferWriter;
use std::ops::Range;

pub mod animation;
pub mod buffer;
pub mod export;
pub mod material;
pub mod node;
pub mod skin;
pub mod storage;
pub mod transform;

pub trait VertexAttributesSource {
    fn attribute_pairs(&self) -> Vec<(&'static str, usize)>;
}

pub trait Vertex: Sized {
    fn write_slices(
        writer: &mut BufferWriter,
        vertices: &[Self],
    ) -> Box<dyn VertexAttributesSource>;
}

#[derive(Clone)]
pub struct Mesh {
    pub texture_index: usize,
    pub indices_range: Range<usize>,
}

pub struct Model<V> {
    pub indices: Vec<u32>,
    pub vertices: Vec<V>,
    pub meshes: Vec<Mesh>,
}

pub fn add_and_get_index<T>(vec: &mut Vec<T>, value: T) -> usize {
    let index = vec.len();
    vec.push(value);
    index
}

#[macro_export]
macro_rules! enum_with_str {
    ($name:ident { $($var_name:ident : $str_value:literal),* $(,)* }) => {
        #[derive(Copy, Clone, Debug, serde::Serialize)]
        pub enum $name {
            $(
                #[serde(rename = $str_value)]
                $var_name,
            )*
        }
    };
}
