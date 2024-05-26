use glam::Vec4;
use serde::Serialize;
use serde_repr::Serialize_repr;
use serde_with::skip_serializing_none;

use crate::storage::{Storage, StorageIndex};

pub type MaterialIndex = StorageIndex<Material>;
pub type TextureIndex = StorageIndex<Texture>;
pub type ImageIndex = StorageIndex<Image>;
pub type SamplerIndex = StorageIndex<Sampler>;

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Material {
    pub pbr_metallic_roughness: PbrMetallicRoughness,
}

#[skip_serializing_none]
#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PbrMetallicRoughness {
    pub base_color_texture: Option<BaseColorTexture>,
    pub base_color_factor: Option<Vec4>,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
}

#[derive(Clone, Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseColorTexture {
    pub index: TextureIndex,
}

impl BaseColorTexture {
    pub fn new(index: TextureIndex) -> Self {
        Self { index }
    }
}

#[derive(Clone, Default, Serialize)]
pub struct Texture {
    pub sampler: SamplerIndex,
    pub source: ImageIndex,
}

#[derive(Clone, Default, Serialize)]
pub struct Image {
    pub uri: String,
}

#[derive(Copy, Clone, Debug, Default, Serialize_repr)]
#[repr(usize)]
pub enum MagFilter {
    #[default]
    Nearest = 9728,
    Linear = 9729,
}

#[derive(Copy, Clone, Debug, Default, Serialize_repr)]
#[repr(usize)]
pub enum MinFilter {
    #[default]
    Nearest = 9728,
    Linear = 9729,
    NearestMipMapNearest = 9984,
    LinearMipMapNearest = 9985,
    NearestMipMapLinear = 9986,
    LinearMipMapLinear = 9987,
}

#[derive(Copy, Clone, Debug, Default, Serialize_repr)]
#[repr(usize)]
pub enum Wrap {
    #[default]
    ClampToEdge = 33701,
    MirroredRepeat = 33648,
    Repeat = 10497,
}

#[derive(Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Sampler {
    pub mag_filter: MagFilter,
    pub min_filter: MinFilter,
    pub wrap_s: Wrap,
    pub wrap_t: Wrap,
}

#[derive(Clone, Default, Serialize)]
pub struct MaterialData {
    #[serde(skip_serializing_if = "Storage::is_empty")]
    materials: Storage<Material>,
    #[serde(skip_serializing_if = "Storage::is_empty")]
    textures: Storage<Texture>,
    #[serde(skip_serializing_if = "Storage::is_empty")]
    images: Storage<Image>,
    #[serde(skip_serializing_if = "Storage::is_empty")]
    samplers: Storage<Sampler>,
}

impl MaterialData {
    pub fn new() -> Self {
        Self {
            materials: Storage::new(),
            textures: Storage::new(),
            images: Storage::new(),
            samplers: Storage::new(),
        }
    }

    pub fn add_material(&mut self, material: Material) -> MaterialIndex {
        self.materials.allocate_with(material)
    }

    pub fn add_texture(&mut self, texture: Texture) -> TextureIndex {
        self.textures.allocate_with(texture)
    }

    pub fn add_images(&mut self, image: Image) -> ImageIndex {
        self.images.allocate_with(image)
    }

    pub fn add_sampler(&mut self, sampler: Sampler) -> SamplerIndex {
        self.samplers.allocate_with(sampler)
    }

    pub fn write_materials(&self) -> String {
        serde_json::to_string_pretty(&self.materials).unwrap()
    }

    pub fn write_textures(&self) -> String {
        serde_json::to_string_pretty(&self.textures).unwrap()
    }

    pub fn write_images(&self) -> String {
        serde_json::to_string_pretty(&self.images).unwrap()
    }

    pub fn write_samplers(&self) -> String {
        serde_json::to_string_pretty(&self.samplers).unwrap()
    }
}
