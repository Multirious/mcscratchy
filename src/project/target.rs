use std::collections::HashMap;

use rs_sb3::{
    asset::{Costume, Sound},
    block::Block,
    broadcast::Broadcast,
    comment::Comment,
    list::List,
    string_hashmap::StringHashMap,
    target::{RotationStyle, Sprite, Stage, Target, VideoState},
    variable::Variable,
};

use crate::{
    scripting::script_builder::{StackBuilder, TargetContext},
    uid::Uid,
};

use super::{
    asset::{CostumeBuilder, SoundBuilder},
    file::Resource,
    script::{CommentBuilder, ListBuilder, VariableBuilder},
};

pub struct GlobalVarListContext {
    vars: HashMap<String, Uid>,
    lists: HashMap<String, Uid>,
}

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub struct TargetBuilder {
    name:            String,
    variables:       HashMap<String, VariableBuilder>,
    lists:           HashMap<String, ListBuilder>,
    broadcasts:      HashMap<String, Uid>,
    block_stackes:   Vec<StackBuilder>,
    comments:        HashMap<Uid, Comment>,
    costumes:        Vec<CostumeBuilder>,
    sounds:          Vec<SoundBuilder>,
    current_costume: u64,
    layer_order:     u64,
    volume:          f64,
}

impl TargetBuilder {
    pub fn new<S: Into<String>>(name: S) -> TargetBuilder {
        TargetBuilder {
            name: name.into(),
            ..Default::default()
        }
    }

    pub fn add_variable<S: Into<String>>(
        mut self,
        name: S,
        variable_builder: VariableBuilder,
    ) -> Self {
        self.variables.insert(name.into(), variable_builder);
        self
    }

    pub fn add_list<S: Into<String>>(mut self, name: S, list_builder: ListBuilder) -> Self {
        self.lists.insert(name.into(), list_builder);
        self
    }

    pub fn add_broadcast<S: Into<String>>(mut self, name: S) -> Self {
        self.broadcasts.insert(name.into(), Uid::generate());
        self
    }

    pub fn add_block_stack(mut self, stack_builder: StackBuilder) -> Self {
        self.block_stackes.push(stack_builder.into());
        self
    }

    pub fn add_comment(mut self, comment_builder: CommentBuilder) -> Self {
        let (comment, comment_uid) = comment_builder.build();
        self.comments.insert(comment_uid, comment);
        self
    }

    pub fn add_costume(mut self, costume_builder: CostumeBuilder) -> Self {
        self.costumes.push(costume_builder);
        self
    }

    pub fn add_sound(mut self, sound_builder: SoundBuilder) -> Self {
        self.sounds.push(sound_builder);
        self
    }

    pub fn current_costume(mut self, index: u64) -> Self {
        self.current_costume = index;
        self
    }

    pub fn layer_order(mut self, layer: u64) -> Self {
        self.layer_order = layer;
        self
    }

    pub(crate) fn broadcasts(&self) -> &HashMap<String, Uid> {
        &self.broadcasts
    }

    /// When global_varlist_buf suppose to be none when the Stage itself is building.
    /// The .1 return value is going to return Some when stage itself is also building.
    pub fn build(
        self,
        file_buff: &mut Vec<Resource>,
        global_varlist_buf: Option<&GlobalVarListContext>,
        all_broadcasts: &HashMap<String, Uid>,
    ) -> (Target, Option<GlobalVarListContext>) {
        let TargetBuilder {
            name,
            variables,
            lists,
            broadcasts,
            block_stackes,
            comments,
            costumes,
            sounds,
            current_costume,
            layer_order,
            volume,
        } = self;
        let variables: HashMap<String, Variable> = variables
            .into_iter()
            .map(|(var_name, var_builder)| {
                let (var, uid) = var_builder.build(var_name);
                (uid.into_inner(), var)
            })
            .collect();
        let lists: HashMap<String, List> = lists
            .into_iter()
            .map(|(list_name, list_builder)| {
                let (list, uid) = list_builder.build(list_name);
                (uid.into_inner(), list)
            })
            .collect();
        let broadcasts: HashMap<String, Broadcast> = broadcasts
            .into_iter()
            .map(|(name, uid)| (uid.into_inner(), Broadcast { name }))
            .collect();

        let mut comments = comments;
        let variable_ctx: HashMap<String, Uid> = variables
            .iter()
            .map(|(uid, var)| (var.name.clone(), (&uid[..]).into()))
            .collect();
        let list_ctx: HashMap<String, Uid> = lists
            .iter()
            .map(|(uid, list)| (list.name.clone(), (&uid[..]).into()))
            .collect();
        let blocks: HashMap<String, Block> = block_stackes
            .into_iter()
            .flat_map(|stack_builder| {
                let (builded_stack, _first_block) = stack_builder.build(
                    &mut comments,
                    &match global_varlist_buf {
                        Some(global_varlist_buf) => TargetContext {
                            global_vars: &global_varlist_buf.vars,
                            global_lists: &global_varlist_buf.lists,
                            this_sprite_vars: &variable_ctx,
                            this_sprite_lists: &list_ctx,
                            all_broadcasts,
                        },
                        None => TargetContext {
                            global_vars: &variable_ctx,
                            global_lists: &list_ctx,
                            this_sprite_vars: &variable_ctx,
                            this_sprite_lists: &list_ctx,
                            all_broadcasts,
                        },
                    },
                );
                builded_stack
                    .into_iter()
                    .map(|(uid, block)| (uid.into_inner(), block))
            })
            .collect();
        let comments: HashMap<String, Comment> = comments
            .into_iter()
            .map(|(uid, comment)| (uid.into_inner(), comment))
            .collect();
        let costumes: Vec<Costume> = costumes
            .into_iter()
            .map(|costume_builder| costume_builder.build(file_buff))
            .collect();
        let sounds: Vec<Sound> = sounds
            .into_iter()
            .map(|sound_builder| sound_builder.build(file_buff))
            .collect();
        let target = Target {
            name,
            variables: StringHashMap(variables),
            lists: StringHashMap(lists),
            broadcasts: StringHashMap(broadcasts),
            blocks: StringHashMap(blocks),
            comments: StringHashMap(comments),
            current_costume: current_costume as i64,
            costumes,
            sounds,
            layer_order: layer_order as i64,
            volume: volume.into(),
        };
        (
            target,
            match global_varlist_buf {
                Some(_) => None,
                None => Some(GlobalVarListContext {
                    vars: variable_ctx,
                    lists: list_ctx,
                }),
            },
        )
    }
}

impl Default for TargetBuilder {
    #[rustfmt::skip]
    fn default() -> Self {
        TargetBuilder {
            name:            "".to_owned(),
            variables:       HashMap::default(),
            lists:           HashMap::default(),
            broadcasts:      HashMap::default(),
            block_stackes:   Vec::default(),
            comments:        HashMap::default(),
            costumes:        Vec::default(),
            sounds:          Vec::default(),
            current_costume: 0,
            layer_order:     0,
            volume:          100.,
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub struct StageBuilder {
    target:                  TargetBuilder,
    tempo:                   i64,
    video_state:             VideoState,
    video_transparency:      i64,
    /// Not availiable yet.
    /// TODO: do this.
    text_to_speech_language: (),
}

impl StageBuilder {
    pub fn new(target: TargetBuilder) -> StageBuilder {
        StageBuilder {
            target,
            ..Default::default()
        }
    }

    pub fn tempo(mut self, tempo: i64) -> Self {
        self.tempo = tempo;
        self
    }

    pub fn video_transparency(mut self, video_transparency: i64) -> Self {
        self.video_transparency = video_transparency;
        self
    }

    pub fn video_state(mut self, video_state: VideoState) -> Self {
        self.video_state = video_state;
        self
    }

    pub(crate) fn target(&self) -> &TargetBuilder {
        &self.target
    }

    pub fn build(
        self,
        file_buff: &mut Vec<Resource>,
        all_broadcasts: &HashMap<String, Uid>,
    ) -> (Stage, GlobalVarListContext) {
        let StageBuilder {
            target,
            tempo,
            video_state,
            video_transparency,
            text_to_speech_language: _,
        } = self;
        let (target, Some(global_var_list)) = target.build(file_buff, None, all_broadcasts) else {
            panic!("stage suppose to return what global var they had");
        };
        let stage = Stage {
            target,
            tempo: tempo.into(),
            video_state,
            video_transparency: video_transparency.into(),
            text_to_speech_language: None,
            is_stage: true,
        };
        (stage, global_var_list)
    }
}

impl Default for StageBuilder {
    #[rustfmt::skip]
    fn default() -> Self {
        StageBuilder {
            target:                  TargetBuilder::default(),
            tempo:                   60,
            video_state:             VideoState::On,
            video_transparency:      50,
            text_to_speech_language: (),
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub struct SpriteBuilder {
    target:         TargetBuilder,
    visible:        bool,
    x:              f64,
    y:              f64,
    size:           f64,
    direction:      f64,
    draggable:      bool,
    rotation_style: RotationStyle,
}

impl SpriteBuilder {
    pub fn new(target: TargetBuilder) -> SpriteBuilder {
        SpriteBuilder {
            target,
            ..Default::default()
        }
    }

    pub fn visible(mut self, visible: bool) -> Self {
        self.visible = visible;
        self
    }

    pub fn pos(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn size(mut self, size: f64) -> Self {
        self.size = size;
        self
    }

    pub fn direction(mut self, direction: f64) -> Self {
        self.direction = direction;
        self
    }

    pub fn draggable(mut self, draggable: bool) -> Self {
        self.draggable = draggable;
        self
    }

    pub fn rotation_style(mut self, rotation_style: RotationStyle) -> Self {
        self.rotation_style = rotation_style;
        self
    }

    pub(crate) fn target(&self) -> &TargetBuilder {
        &self.target
    }

    pub fn build(
        self,
        file_buff: &mut Vec<Resource>,
        global_varlist_buf: &GlobalVarListContext,
        all_broadcasts: &HashMap<String, Uid>,
    ) -> Sprite {
        let SpriteBuilder {
            target,
            visible,
            x,
            y,
            size,
            direction,
            draggable,
            rotation_style,
        } = self;
        Sprite {
            target: target
                .build(file_buff, Some(global_varlist_buf), all_broadcasts)
                .0,
            visible,
            x: x.into(),
            y: y.into(),
            size: size.into(),
            direction: direction.into(),
            draggable,
            rotation_style,
            is_stage: false,
        }
    }
}

impl Default for SpriteBuilder {
    #[rustfmt::skip]
    fn default() -> Self {
        SpriteBuilder {
            target:         TargetBuilder::default(),
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
