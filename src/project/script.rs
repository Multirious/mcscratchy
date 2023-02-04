use rs_sb3::{comment::Comment, list::List, value::Value, variable::Variable};

use crate::uid::Uid;

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
