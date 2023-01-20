use std::collections::HashMap;

use rs_sb3::{
    asset::{Asset, Costume, Sound},
    block::Block,
    broadcast::Broadcast,
    comment::Comment,
    list::List,
    monitor::Monitor,
    project::{Meta, Project},
    target::{RotationStyle, Sprite, Stage, Target, VideoState},
    variable::Variable,
};

use crate::uid::Uid;

pub mod target_builder;

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub struct ProjectBuilder {
    pub stage_builder:   StageBuilder,
    pub sprite_builders: Vec<SpriteBuilder>,
    pub monitors:        Vec<Monitor>,
    pub meta:            Meta,
}

impl ProjectBuilder {
    pub fn set_stage(mut self, stage_builder: StageBuilder) -> Self {
        self.stage_builder = stage_builder;
        self
    }

    pub fn add_sprite(mut self, sprite_builder: SpriteBuilder) -> Self {
        self.sprite_builders.push(sprite_builder);
        self
    }
}

impl Default for ProjectBuilder {
    #[rustfmt::skip]
    fn default() -> Self {
        ProjectBuilder {
            stage_builder:   StageBuilder::default(),
            sprite_builders: Vec::default(),
            monitors:        Vec::default(),
            meta: Meta {
                semver: "3.0.0".to_owned(),
                vm:     "It ain't a vm, my guy.".to_owned(),
                agent:  "mcscratchy/0.1.0".to_owned(),
            },
        }
    }
}
