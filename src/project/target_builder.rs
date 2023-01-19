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
    pub fn new(starting_values: Vec<Value>) -> ListBuilder {
        ListBuilder {
            values: starting_values,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct CommentBuilder {
    x: Option<f64>,
    y: Option<f64>,
    width: f64,
    height: f64,
    minimized: bool,
    content: String,
}

impl CommentBuilder {
    /// Requires:
    /// - block_uid?: To connect the block with comment
    ///
    /// Returns:
    /// - [`Uid`]: [`Uid`] of the built comment inside [`Target`]'s comment list
    pub fn build(self, block_uid: Option<Uid>) -> (Comment, Uid) {
        let CommentBuilder {
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
            width: width.map(|n| n.into()),
            height: height.map(|n| n.into()),
            minimized,
            text: content,
        };
        (comment, my_uid)
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
    comments:        Vec<Comment>,
    costumes:        Vec<Costume>,
    sounds:          Vec<Sound>,
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
    ) -> TargetBuilder {
        self.variables.insert(name.into(), variable_builder);
        self
    }

    pub fn add_list<S: Into<String>>(
        mut self,
        name: S,
        list_builder: ListBuilder,
    ) -> TargetBuilder {
        self.lists.insert(name.into(), list_builder);
        self
    }

    pub fn add_broadcast<S: Into<String>>(mut self, name: S) -> TargetBuilder {
        self.broadcasts.insert(name.into());
        self
    }

    pub fn add_block_stacks<SB: Into<StackBuilder>>(mut self, stack: SB) -> TargetBuilder {
        self.block_stackes.push(stack.into());
        self
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
            blocks:          HashMap::default(),
            comments:        HashMap::default(),
            costumes:        Vec::default(),
            sounds:          Vec::default(),
            current_costume: 0,
            layer_order:     0,
            volume:          100.,
        }
    }
}
