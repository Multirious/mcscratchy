use std::collections::HashSet;

use rs_sb3::{string_hashmap::StringHashMap, value::Value};

use crate::scripting::script_builder::StackBuilder;

use super::*;
use resource::{Resource, ValidResource};

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

    pub fn build(self, name_for_this_var: String) -> (Variable, Uid) {
        let VariableBuilder {
            value,
            is_cloud_variable,
        } = self;
        let my_uid = Uid::generate();
        let var = Variable {
            name: name_for_this_var,
            value,
            is_cloud_variable,
        };
        (var, my_uid)
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

    pub fn build(self, name_for_this_list: String) -> (List, Uid) {
        let ListBuilder { values } = self;
        let my_uid = Uid::generate();
        let list = List {
            name: name_for_this_list,
            values,
        };
        (list, my_uid)
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

    pub fn pos(mut self, x: f64, y: f64) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }

    pub fn size(mut self, width: u64, height: u64) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn minimized(mut self, minimized: bool) -> Self {
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

    pub fn build(self, file_buff: &mut Vec<Resource>) -> Target {
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
            .map(|broadcast_name| {
                (
                    Uid::generate().into_inner(),
                    Broadcast {
                        name: broadcast_name,
                    },
                )
            })
            .collect();
        let mut comments = comments;
        let blocks: HashMap<String, Block> = block_stackes
            .into_iter()
            .flat_map(|stack_builder| {
                let (builded_stack, _first_block) = stack_builder.build(&mut comments);
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
        Target {
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
        }
    }
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CostumeBuilder {
    rotation_center_x: i64,
    rotation_center_y: i64,
    asset: AssetBuilder,
}

impl CostumeBuilder {
    pub fn new(asset_builder: AssetBuilder) -> CostumeBuilder {
        CostumeBuilder {
            asset: asset_builder,
            rotation_center_x: 0,
            rotation_center_y: 0,
        }
    }

    pub fn rotation_center(mut self, x: i64, y: i64) -> Self {
        self.rotation_center_x = x;
        self.rotation_center_y = y;
        self
    }

    pub fn build(self, file_buff: &mut Vec<Resource>) -> Costume {
        let CostumeBuilder {
            rotation_center_x,
            rotation_center_y,
            asset,
        } = self;
        Costume {
            rotation_center_x: rotation_center_x.into(),
            rotation_center_y: rotation_center_y.into(),
            bitmap_resolution: Some(1),
            asset: asset.build(file_buff),
        }
    }
}

/// Not really sure what to do here yet
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoundBuilder {
    rate: u64,
    sample_count: u64,
    format: Option<String>,
    asset: AssetBuilder,
}

impl SoundBuilder {
    pub fn build(self, file_buff: &mut Vec<Resource>) -> Sound {
        let SoundBuilder {
            rate,
            sample_count,
            format,
            asset,
        } = self;
        Sound {
            rate,
            sample_count,
            format,
            asset: asset.build(file_buff),
        }
    }
}

/// Not really sure what to do here yet
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetBuilder {
    name: String,
    file: ValidResource,
}

impl AssetBuilder {
    pub fn new<S: Into<String>>(name: S, file: ValidResource) -> AssetBuilder {
        AssetBuilder {
            name: name.into(),
            file,
        }
    }

    pub fn build(self, file_buff: &mut Vec<Resource>) -> Asset {
        let AssetBuilder { name, file } = self;
        let md5_hash = resource::hex(&file.md5_hash());
        let extension = file.extension;
        let asset = Asset {
            asset_id: md5_hash.clone(),
            name,
            md5ext: Some(md5_hash.clone() + "." + &extension),
            data_format: extension.clone(),
        };
        let md5_path: std::path::PathBuf = md5_hash.into();
        file_buff.push(Resource {
            path: md5_path.with_extension(extension),
            content: file.file.content,
        });
        asset
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

    pub fn build(self, file_buff: &mut Vec<Resource>) -> Stage {
        let StageBuilder {
            target,
            tempo,
            video_state,
            video_transparency,
            text_to_speech_language: _,
        } = self;
        Stage {
            target: target.build(file_buff),
            tempo: tempo.into(),
            video_state,
            video_transparency: video_transparency.into(),
            text_to_speech_language: None,
            is_stage: true,
        }
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

    pub fn build(self, file_buff: &mut Vec<Resource>) -> Sprite {
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
            target: target.build(file_buff),
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
