use self::buffer::BufferWriter;
use std::ops::Range;

pub mod animation;
pub mod buffer;
pub mod document;
pub mod export;
pub mod material;
pub mod mesh;
pub mod node;
pub mod skin;
pub mod storage;
pub mod transform;

pub trait Vertex: Sized {
    fn write_slices(writer: &mut BufferWriter, vertices: &[Self]) -> Vec<(&'static str, usize)>;
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

#[macro_export]
macro_rules! vertex_def {
    ($name:ident { $(($attribute_name:literal) $field_name:ident : $field_ty:ty),* $(,)* }) => {
        #[repr(C)]
        #[derive(Copy, Clone, Debug, Default)]
        pub struct $name {
            $(
                pub $field_name : $field_ty,
            )*
        }

        impl gltf::Vertex for $name {
            fn write_slices(
                writer: &mut gltf::buffer::BufferWriter,
                vertices: &[Self]
            ) -> Vec<(&'static str, usize)> {
                // Split out the vertex data
                $(
                    let mut $field_name = Vec::with_capacity(vertices.len());
                )*
                for vertex in vertices {
                    $(
                        $field_name.push(vertex.$field_name);
                    )*
                }

                $(
                    let $field_name = writer.create_view_and_accessor_with_min_max(&$field_name, Some(gltf::buffer::BufferViewTarget::ArrayBuffer));
                )*

                let attributes = vec![
                    $(
                        ($attribute_name, $field_name.accessor.0),
                    )*
                ];

                attributes
            }
        }
    };
}
