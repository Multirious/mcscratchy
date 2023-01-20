use std::collections::HashSet;

use rs_sb3::value::Value;

use crate::scripting::script_builder::StackBuilder;

use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct VariableBuilder {
    value: Value,
    /// Cloud variable can only store number. Becareful!
    is_cloud_variable: bool,
}

impl VariableBuilder {
    pub fn new(starting_value: Value) -> VariableBuilder {
        VariableBuilder {
            value: starting_value,
            is_cloud_variable: false,
        }
    }

    pub fn new_cloud_variable(starting_value: Value) -> VariableBuilder {
        debug_assert!(matches!(starting_value, Value::Number(_)));
        VariableBuilder {
            value: starting_value,
            is_cloud_variable: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ListBuilder {
    values: Vec<Value>,
}

impl ListBuilder {
    pub fn new(values: Vec<Value>) -> ListBuilder {
        ListBuilder { values }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub struct CommentBuilder {
    block_uid: Option<Uid>,
    x:         Option<f64>,
    y:         Option<f64>,
    width:     u64,
    height:    u64,
    minimized: bool,
    content:   String,
}

impl CommentBuilder {
    pub fn new<S: Into<String>>(content: S) -> CommentBuilder {
        CommentBuilder {
            content: content.into(),
            ..Default::default()
        }
    }

    pub fn pos(self, x: f64, y: f64) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }

    pub fn size(self, width: u64, height: u64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn minimized(self, minimized: bool) -> Self {
        self.minimized = minimized;
        self
    }

    /// Requires:
    /// - block_uid?: To connect the block with comment
    ///
    /// Returns:
    /// - [`Uid`]: [`Uid`] of the built comment inside [`Target`]'s comment list
    pub fn build(self) -> (Comment, Uid) {
        let CommentBuilder {
            block_uid,
            x,
            y,
            width,
            height,
            minimized,
            content,
        } = self;
        let my_uid = Uid::generate();
        let comment = Comment {
            block_id: block_uid.map(|u| u.into_inner()),
            x: x.map(|n| n.into()),
            y: y.map(|n| n.into()),
            width: (width as i64).into(),
            height: (height as i64).into(),
            minimized,
            text: content,
        };
        (comment, my_uid)
    }
}

impl Default for CommentBuilder {
    #[rustfmt::skip]
    fn default() -> Self {
        CommentBuilder {
            block_uid: None,
            x:         Some(0.),
            y:         Some(0.),
            width:     200,
            height:    200,
            minimized: false,
            content:   "".to_owned(),
        }
    }
}

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub struct TargetBuilder {
    name:            String,
    variables:       HashMap<String, VariableBuilder>,
    lists:           HashMap<String, ListBuilder>,
    broadcasts:      HashSet<String>,
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
        self.broadcasts.insert(name.into());
        self
    }

    pub fn add_block_stacks<SB: Into<StackBuilder>>(mut self, stack_builder: SB) -> Self {
        self.block_stackes.push(stack_builder.into());
        self
    }

    pub fn add_comment(mut self, comment_builder: CommentBuilder) -> Self {
        let (comment, comment_uid) = comment_builder.build();
        self.comments.insert(comment_uid, comment);
        self
    }

    pub(crate) fn add_comment_with_uid(mut self, uid: Uid, comment: Comment) -> Self {
        self.comments.insert(uid, comment);
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

    // pub fn build(self) -> Target {

    // }
}

impl Default for TargetBuilder {
    #[rustfmt::skip]
    fn default() -> Self {
        TargetBuilder {
            name:            "".to_owned(),
            variables:       HashMap::default(),
            lists:           HashMap::default(),
            broadcasts:      HashSet::default(),
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

/// Not really sure what to do here yet
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CostumeBuilder {
    rotation_center_x: i64,
    rotation_center_y: i64,
    asset: AssetBuilder,
}

/// Not really sure what to do here yet
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoundBuilder {
    rate: u64,
    sample_count: u64,
    format: Option<String>,
    asset: AssetBuilder,
}

/// Not really sure what to do here yet
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetBuilder {
    asset_id: Uid,
    name: String,
    md5ext: Option<String>,
    data_format: String,
}

#[rustfmt::skip]
#[derive(Debug, Clone, PartialEq)]
pub struct StageBuilder {
    target:                  TargetBuilder,
    tempo:                   f64,
    video_state:             VideoState,
    video_transparency:      f64,
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

    pub fn tempo(mut self, tempo: f64) -> Self {
        self.tempo = tempo;
        self
    }

    pub fn video_transparency(mut self, video_transparency: f64) -> Self {
        self.video_transparency = video_transparency;
        self
    }

    pub fn video_state(mut self, video_state: VideoState) -> Self {
        self.video_state = video_state;
        self
    }
}

impl Default for StageBuilder {
    #[rustfmt::skip]
    fn default() -> Self {
        StageBuilder {
            target:                  TargetBuilder::default(),
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
