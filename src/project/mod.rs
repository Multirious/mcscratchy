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
pub struct StageBuilder {
    target_builder:          TargetBuilder,
    tempo:                   f64,
    video_state:             VideoState,
    video_transparency:      f64,
    /// Not availiable yet.
    /// TODO:                do this.
    text_to_speech_language: (),
}

impl Default for StageBuilder {
    #[rustfmt::skip]
    fn default() -> Self {
        StageBuilder {
            target_builder:          TargetBuilder::default(),
            tempo:                   60.,
            video_state:             VideoState::On,
            video_transparency:      50.,
            text_to_speech_language: (),
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub struct SpriteBuilder {
    target_builder: TargetBuilder,
    visible:        bool,
    x:              f64,
    y:              f64,
    size:           f64,
    direction:      f64,
    draggable:      bool,
    rotation_style: RotationStyle,
}

impl Default for SpriteBuilder {
    #[rustfmt::skip]
    fn default() -> Self {
        SpriteBuilder {
            target_builder: TargetBuilder::default(),
            visible:        true,
            x:              0.,
            y:              0.,
            size:           100.,
            direction:      90.,
            draggable:      false,
            rotation_style: RotationStyle::AllAround,
        }
    }
}

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
