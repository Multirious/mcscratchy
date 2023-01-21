use std::collections::HashMap;

use rs_sb3::{
    asset::{Asset, Costume, Sound},
    block::Block,
    broadcast::Broadcast,
    comment::Comment,
    list::List,
    monitor::Monitor,
    project::{Meta, Project},
    target::{RotationStyle, Sprite, SpriteOrStage, Stage, Target, VideoState},
    variable::Variable,
};

use crate::uid::Uid;
use target_builder::{SpriteBuilder, StageBuilder};

use self::file_manager::{File};

pub mod file_manager;
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
    pub fn new() -> ProjectBuilder {
        ProjectBuilder::default()
    }

    pub fn set_stage(mut self, stage_builder: StageBuilder) -> Self {
        self.stage_builder = stage_builder;
        self
    }

    pub fn add_sprite(mut self, sprite_builder: SpriteBuilder) -> Self {
        self.sprite_builders.push(sprite_builder);
        self
    }
}

impl ProjectBuilder {
    pub fn build(self, file_buff: &mut Vec<File>) -> Project {
        let ProjectBuilder {
            stage_builder,
            sprite_builders,
            monitors,
            meta,
        } = self;
        let mut targets = Vec::with_capacity(1 + sprite_builders.len());
        targets.push(SpriteOrStage::Stage(stage_builder.build(file_buff)));
        targets.extend(
            sprite_builders
                .into_iter()
                .map(|sprite_builder| SpriteOrStage::Sprite(sprite_builder.build(file_buff))),
        );
        Project {
            meta,
            extensions: serde_json::value::Value::Array(vec![]),
            monitors,
            targets,
        }
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
                vm:     "0.2.0-prerelease.20220222132735".to_owned(),
                agent:  "mcscratchy/0.1.0".to_owned(),
            },
        }
    }
}
