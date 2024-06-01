use glam::{Mat4, Vec3, Vec4};
use serde::{ser::SerializeStruct, Serialize, Serializer};
use serde_repr::Serialize_repr;
use serde_with::skip_serializing_none;

use crate::{
    enum_with_str,
    storage::{Storage, StorageIndex},
};

pub trait BufferType: Sized {
    const COMPONENT_TY: AccessorComponentType;
    const TY: AccessorDataType;
    fn to_bytes(&self) -> Vec<u8>;
    fn stride() -> Option<usize>;
}

pub trait BufferTypeMinMax: BufferType {
    const MIN: Self;
    const MAX: Self;
    fn data_max(&self, other: &Self) -> Self;
    fn data_min(&self, other: &Self) -> Self;
    fn write_value(&self) -> String;
}

pub trait BufferTypeEx: Sized {
    fn find_min_max(data: &[Self]) -> (Self, Self);
}

#[derive(Debug)]
pub struct MinMax<T> {
    pub min: T,
    pub max: T,
}

impl Serialize for MinMax<String> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut s = serializer.serialize_struct("MinMax", 2)?;
        s.serialize_field(
            "min",
            &serde_json::value::RawValue::from_string(self.min.clone()).unwrap(),
        )?;
        s.serialize_field(
            "max",
            &serde_json::value::RawValue::from_string(self.max.clone()).unwrap(),
        )?;
        s.end()
    }
}

impl<T: BufferTypeMinMax> BufferTypeEx for T {
    fn find_min_max(data: &[Self]) -> (Self, Self) {
        let mut max = T::MIN;
        let mut min = T::MAX;
        for face in data {
            max = face.data_max(&max);
            min = face.data_min(&min);
        }
        (min, max)
    }
}

pub type BufferViewIndex = StorageIndex<BufferView>;
pub type AccessorIndex = StorageIndex<Accessor>;

#[derive(Copy, Clone, Debug)]
pub struct BufferViewAndAccessorPair {
    pub view: BufferViewIndex,
    pub accessor: AccessorIndex,
}

impl BufferViewAndAccessorPair {
    pub fn new(view: BufferViewIndex, accessor: AccessorIndex) -> Self {
        Self { view, accessor }
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BufferWriter {
    #[serde(skip)]
    buffer: Vec<u8>,
    #[serde(rename = "bufferViews")]
    views: Storage<BufferView>,
    accessors: Storage<Accessor>,
}

impl BufferWriter {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            views: Storage::new(),
            accessors: Storage::new(),
        }
    }

    pub fn create_view<T: BufferType + Copy>(
        &mut self,
        data: &[T],
        target: Option<BufferViewTarget>,
    ) -> BufferViewIndex {
        let offset = self.buffer.len();
        for item in data {
            let mut bytes = item.to_bytes();
            self.buffer.append(&mut bytes);
        }
        let byte_len = self.buffer.len() - offset;
        let stride = T::stride();
        let index = self.views.allocate_with(BufferView {
            buffer: 0,
            byte_offset: offset,
            byte_len,
            stride,
            target,
        });
        index
    }

    pub fn create_accessor<T: BufferType + Copy>(
        &mut self,
        view_index: BufferViewIndex,
        byte_offset: usize,
        len: usize,
    ) -> AccessorIndex {
        self.accessors.allocate_with(Accessor {
            buffer_view: view_index.0,
            byte_offset,
            count: len,
            component_ty: T::COMPONENT_TY,
            ty: T::TY,
            min_max: None,
        })
    }

    pub fn create_accessor_with_min_max<T: BufferTypeMinMax + Copy>(
        &mut self,
        view_index: BufferViewIndex,
        byte_offset: usize,
        len: usize,
        min_max: MinMax<T>,
    ) -> AccessorIndex {
        self.accessors.allocate_with(Accessor {
            buffer_view: view_index.0,
            byte_offset,
            count: len,
            component_ty: T::COMPONENT_TY,
            ty: T::TY,
            min_max: Some(MinMax {
                min: min_max.min.write_value(),
                max: min_max.max.write_value(),
            }),
        })
    }

    pub fn create_view_and_accessor<T: BufferType + Copy>(
        &mut self,
        data: &[T],
        target: Option<BufferViewTarget>,
    ) -> BufferViewAndAccessorPair {
        let view = self.create_view(data, target);
        let accessor = self.create_accessor::<T>(view, 0, data.len());
        BufferViewAndAccessorPair::new(view, accessor)
    }

    pub fn create_view_and_accessor_with_min_max<T: BufferTypeMinMax + Copy>(
        &mut self,
        data: &[T],
        target: Option<BufferViewTarget>,
    ) -> BufferViewAndAccessorPair {
        let view = self.create_view(data, target);
        let mut max = T::MIN;
        let mut min = T::MAX;
        for item in data {
            max = item.data_max(&max);
            min = item.data_min(&min);
        }
        let min_max = MinMax { min, max };
        let accessor = self.create_accessor_with_min_max(view, 0, data.len(), min_max);
        BufferViewAndAccessorPair::new(view, accessor)
    }

    pub fn write_buffer_views(&self) -> String {
        serde_json::to_string_pretty(&self.views).unwrap()
    }

    pub fn write_accessors(&self) -> String {
        serde_json::to_string_pretty(&self.accessors).unwrap()
    }

    pub fn buffer_len(&self) -> usize {
        self.buffer.len()
    }

    pub fn to_inner(self) -> Vec<u8> {
        self.buffer
    }

    pub fn data(&self) -> &[u8] {
        &self.buffer
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct BufferView {
    buffer: usize,
    #[serde(rename = "byteOffset")]
    byte_offset: usize,
    #[serde(rename = "byteLength")]
    byte_len: usize,
    #[serde(rename = "byteStride")]
    stride: Option<usize>,
    #[serde(rename = "target")]
    target: Option<BufferViewTarget>,
}

#[derive(Copy, Clone, Debug, Serialize_repr)]
#[repr(usize)]
pub enum BufferViewTarget {
    ArrayBuffer = 34962,
    ElementArrayBuffer = 34963,
}

// https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#accessor-data-types
#[derive(Copy, Clone, Debug, Default, Serialize_repr)]
#[repr(usize)]
pub enum AccessorComponentType {
    SignedByte = 5120,
    UnsignedByte = 5121,
    SignedShort = 5122,
    UnsignedShort = 5123,
    UnsignedInt = 5125,
    #[default]
    Float = 5126,
}

enum_with_str!(AccessorDataType {
    Scalar: "SCALAR",
    Vec2: "VEC2",
    Vec3: "VEC3",
    Vec4: "VEC4",
    Mat2: "MAT2",
    Mat3: "MAT3",
    Mat4: "MAT4",
});

impl Default for AccessorDataType {
    fn default() -> Self {
        Self::Scalar
    }
}

#[skip_serializing_none]
#[derive(Debug, Default, Serialize)]
pub struct Accessor {
    #[serde(rename = "bufferView")]
    buffer_view: usize,
    #[serde(rename = "byteOffset")]
    byte_offset: usize,
    count: usize,
    #[serde(rename = "componentType")]
    component_ty: AccessorComponentType,
    #[serde(rename = "type")]
    ty: AccessorDataType,
    #[serde(flatten)]
    min_max: Option<MinMax<String>>,
}

impl BufferType for u16 {
    const COMPONENT_TY: AccessorComponentType = AccessorComponentType::UnsignedShort;
    const TY: AccessorDataType = AccessorDataType::Scalar;

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn stride() -> Option<usize> {
        None
    }
}

impl BufferTypeMinMax for u16 {
    const MIN: Self = u16::MIN;
    const MAX: Self = u16::MAX;

    fn data_max(&self, other: &Self) -> Self {
        (*self).max(*other)
    }

    fn data_min(&self, other: &Self) -> Self {
        (*self).min(*other)
    }

    fn write_value(&self) -> String {
        format!(" [ {} ]", self)
    }
}

impl BufferType for u32 {
    const COMPONENT_TY: AccessorComponentType = AccessorComponentType::UnsignedInt;
    const TY: AccessorDataType = AccessorDataType::Scalar;

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn stride() -> Option<usize> {
        None
    }
}

impl BufferTypeMinMax for u32 {
    const MIN: Self = u32::MIN;
    const MAX: Self = u32::MAX;

    fn data_max(&self, other: &Self) -> Self {
        (*self).max(*other)
    }

    fn data_min(&self, other: &Self) -> Self {
        (*self).min(*other)
    }

    fn write_value(&self) -> String {
        format!(" [ {} ]", self)
    }
}

impl BufferType for f32 {
    const COMPONENT_TY: AccessorComponentType = AccessorComponentType::Float;
    const TY: AccessorDataType = AccessorDataType::Scalar;

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn stride() -> Option<usize> {
        None
    }
}

impl BufferTypeMinMax for f32 {
    const MIN: Self = f32::MIN;
    const MAX: Self = f32::MAX;

    fn data_max(&self, other: &Self) -> Self {
        (*self).max(*other)
    }

    fn data_min(&self, other: &Self) -> Self {
        (*self).min(*other)
    }

    fn write_value(&self) -> String {
        format!(" [ {} ]", self)
    }
}

impl BufferType for u8 {
    const COMPONENT_TY: AccessorComponentType = AccessorComponentType::UnsignedByte;
    const TY: AccessorDataType = AccessorDataType::Scalar;

    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    fn stride() -> Option<usize> {
        None
    }
}

impl BufferTypeMinMax for u8 {
    const MIN: Self = u8::MIN;
    const MAX: Self = u8::MAX;

    fn data_max(&self, other: &Self) -> Self {
        (*self).max(*other)
    }

    fn data_min(&self, other: &Self) -> Self {
        (*self).min(*other)
    }

    fn write_value(&self) -> String {
        format!(" [ {} ]", self)
    }
}

impl BufferType for [f32; 2] {
    const COMPONENT_TY: AccessorComponentType = AccessorComponentType::Float;
    const TY: AccessorDataType = AccessorDataType::Vec2;

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(std::mem::size_of_val(self));
        for value in self {
            let mut data = value.to_le_bytes().to_vec();
            bytes.append(&mut data);
        }
        bytes
    }

    fn stride() -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}

impl BufferTypeMinMax for [f32; 2] {
    const MIN: Self = [f32::MIN, f32::MIN];
    const MAX: Self = [f32::MAX, f32::MAX];

    fn data_max(&self, other: &Self) -> Self {
        [self[0].data_max(&other[0]), self[1].data_max(&other[1])]
    }

    fn data_min(&self, other: &Self) -> Self {
        [self[0].data_min(&other[0]), self[1].data_min(&other[1])]
    }

    fn write_value(&self) -> String {
        format!(" [ {}, {} ]", self[0], self[1])
    }
}

impl BufferType for [f32; 3] {
    const COMPONENT_TY: AccessorComponentType = AccessorComponentType::Float;
    const TY: AccessorDataType = AccessorDataType::Vec3;

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(std::mem::size_of_val(self));
        for value in self {
            let mut data = value.to_le_bytes().to_vec();
            bytes.append(&mut data);
        }
        bytes
    }

    fn stride() -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}

impl BufferTypeMinMax for [f32; 3] {
    const MIN: Self = [f32::MIN, f32::MIN, f32::MIN];
    const MAX: Self = [f32::MAX, f32::MAX, f32::MAX];

    fn data_max(&self, other: &Self) -> Self {
        [
            self[0].data_max(&other[0]),
            self[1].data_max(&other[1]),
            self[2].data_max(&other[2]),
        ]
    }

    fn data_min(&self, other: &Self) -> Self {
        [
            self[0].data_min(&other[0]),
            self[1].data_min(&other[1]),
            self[2].data_min(&other[2]),
        ]
    }

    fn write_value(&self) -> String {
        format!(" [ {}, {}, {} ]", self[0], self[1], self[2])
    }
}

impl BufferType for [f32; 4] {
    const COMPONENT_TY: AccessorComponentType = AccessorComponentType::Float;
    const TY: AccessorDataType = AccessorDataType::Vec4;

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(std::mem::size_of_val(self));
        for value in self {
            let mut data = value.to_le_bytes().to_vec();
            bytes.append(&mut data);
        }
        bytes
    }

    fn stride() -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}

impl BufferTypeMinMax for [f32; 4] {
    const MIN: Self = [f32::MIN, f32::MIN, f32::MIN, f32::MIN];
    const MAX: Self = [f32::MAX, f32::MAX, f32::MAX, f32::MAX];

    fn data_max(&self, other: &Self) -> Self {
        [
            self[0].data_max(&other[0]),
            self[1].data_max(&other[1]),
            self[2].data_max(&other[2]),
            self[3].data_max(&other[3]),
        ]
    }

    fn data_min(&self, other: &Self) -> Self {
        [
            self[0].data_min(&other[0]),
            self[1].data_min(&other[1]),
            self[2].data_min(&other[2]),
            self[3].data_min(&other[3]),
        ]
    }

    fn write_value(&self) -> String {
        format!(" [ {}, {}, {}, {} ]", self[0], self[1], self[2], self[3])
    }
}

impl BufferType for [u8; 4] {
    const COMPONENT_TY: AccessorComponentType = AccessorComponentType::UnsignedByte;
    const TY: AccessorDataType = AccessorDataType::Vec4;

    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(std::mem::size_of_val(self));
        for value in self {
            let mut data = value.to_le_bytes().to_vec();
            bytes.append(&mut data);
        }
        bytes
    }

    fn stride() -> Option<usize> {
        Some(std::mem::size_of::<Self>())
    }
}

impl BufferTypeMinMax for [u8; 4] {
    const MIN: Self = [u8::MIN, u8::MIN, u8::MIN, u8::MIN];
    const MAX: Self = [u8::MAX, u8::MAX, u8::MAX, u8::MAX];

    fn data_max(&self, other: &Self) -> Self {
        [
            self[0].data_max(&other[0]),
            self[1].data_max(&other[1]),
            self[2].data_max(&other[2]),
            self[3].data_max(&other[3]),
        ]
    }

    fn data_min(&self, other: &Self) -> Self {
        [
            self[0].data_min(&other[0]),
            self[1].data_min(&other[1]),
            self[2].data_min(&other[2]),
            self[3].data_min(&other[3]),
        ]
    }

    fn write_value(&self) -> String {
        format!(" [ {}, {}, {}, {} ]", self[0], self[1], self[2], self[3])
    }
}

impl BufferType for Mat4 {
    const COMPONENT_TY: AccessorComponentType = AccessorComponentType::Float;
    const TY: AccessorDataType = AccessorDataType::Mat4;

    fn to_bytes(&self) -> Vec<u8> {
        let array = self.to_cols_array();
        let mut bytes = Vec::with_capacity(std::mem::size_of_val(&array));
        for value in array {
            let mut data = value.to_le_bytes().to_vec();
            bytes.append(&mut data);
        }
        bytes
    }

    fn stride() -> Option<usize> {
        None
    }
}

impl BufferType for Vec3 {
    const COMPONENT_TY: AccessorComponentType = AccessorComponentType::Float;
    const TY: AccessorDataType = AccessorDataType::Vec3;

    fn to_bytes(&self) -> Vec<u8> {
        let array = self.to_array();
        let mut bytes = Vec::with_capacity(std::mem::size_of_val(&array));
        for value in array {
            let mut data = value.to_le_bytes().to_vec();
            bytes.append(&mut data);
        }
        bytes
    }

    fn stride() -> Option<usize> {
        None
    }
}

impl BufferType for Vec4 {
    const COMPONENT_TY: AccessorComponentType = AccessorComponentType::Float;
    const TY: AccessorDataType = AccessorDataType::Vec4;

    fn to_bytes(&self) -> Vec<u8> {
        let array = self.to_array();
        let mut bytes = Vec::with_capacity(std::mem::size_of_val(&array));
        for value in array {
            let mut data = value.to_le_bytes().to_vec();
            bytes.append(&mut data);
        }
        bytes
    }

    fn stride() -> Option<usize> {
        None
    }
}
