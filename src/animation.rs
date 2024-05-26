use serde::Serialize;

use crate::{
    enum_with_str,
    storage::{Storage, StorageIndex},
};

use super::{buffer::AccessorIndex, node::NodeIndex};

pub type ChannelIndex = StorageIndex<Channel>;
pub type SamplerIndex = StorageIndex<Sampler>;
pub type AnimationIndex = StorageIndex<Animation>;

#[derive(Clone, Default, Serialize)]
pub struct Animation {
    channels: Storage<Channel>,
    name: String,
    samplers: Storage<Sampler>,
}

#[derive(Clone, Default, Serialize)]
pub struct Channel {
    pub sampler: SamplerIndex,
    pub target: ChannelTarget,
}

#[derive(Clone, Default, Serialize)]
pub struct ChannelTarget {
    pub node: NodeIndex,
    pub path: AnimationTarget,
}

enum_with_str!(AnimationInterpolation { Linear: "LINEAR" });
enum_with_str!(AnimationTarget {
    Translation: "translation",
    Rotation: "rotation",
});

impl Default for AnimationInterpolation {
    fn default() -> Self {
        Self::Linear
    }
}
impl Default for AnimationTarget {
    fn default() -> Self {
        Self::Translation
    }
}

#[derive(Clone, Default, Serialize)]
pub struct Sampler {
    pub input: AccessorIndex,
    pub interpolation: AnimationInterpolation,
    pub output: AccessorIndex,
}

#[derive(Clone, Default, Serialize)]
#[serde(transparent)]
pub struct Animations {
    animations: Storage<Animation>,
}

impl Animation {
    pub fn new(name: String) -> Self {
        Self {
            channels: Storage::new(),
            name,
            samplers: Storage::new(),
        }
    }

    pub fn add_sampler(&mut self, sampler: Sampler) -> SamplerIndex {
        self.samplers.allocate_with(sampler)
    }

    pub fn add_channel(&mut self, channel: Channel) -> ChannelIndex {
        self.channels.allocate_with(channel)
    }

    pub fn write(&self) -> String {
        serde_json::to_string_pretty(&self).unwrap()
    }
}

impl Animations {
    pub fn new(capacity: usize) -> Self {
        Self {
            animations: Storage::with_capacity(capacity),
        }
    }

    pub fn add_animation(&mut self, animation: Animation) -> AnimationIndex {
        self.animations.allocate_with(animation)
    }

    pub fn is_empty(&self) -> bool {
        self.animations.is_empty()
    }

    pub fn write_animations(&self) -> String {
        serde_json::to_string_pretty(&self.animations).unwrap()
    }
}
