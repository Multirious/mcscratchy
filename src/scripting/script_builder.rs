use std::collections::HashMap;

use super::{
    arg::{Arg, IntoArg, IntoFieldArg, IntoStackArg},
    typed_script_builder::TypedStackBuilder,
};
use crate::{project::target_builder::CommentBuilder, uid::Uid};
use rs_sb3::{
    block::{
        Block, BlockField, BlockInput, BlockInputValue, BlockMutation, BlockNormal,
        BlockVarListReporterTop, ListOrVariable, ShadowInputType, UidOrValue,
    },
    comment::Comment,
    string_hashmap::StringHashMap,
    value::OpCode,
};

#[derive(Debug, Clone, PartialEq)]
pub enum StackOrValue {
    Value(BlockInputValue),
    Stack(StackBuilder),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockInputBuilder {
    shadow: ShadowInputType,
    values: Vec<Option<StackOrValue>>,
}

impl BlockInputBuilder {
    pub fn new(shadow: ShadowInputType) -> BlockInputBuilder {
        BlockInputBuilder {
            shadow,
            values: vec![],
        }
    }

    pub fn input(mut self, input: Option<StackOrValue>) -> Self {
        self.values.push(input);
        self
    }

    pub fn input_some(self, input: StackOrValue) -> Self {
        self.input(Some(input))
    }

    pub fn input_none(self) -> Self {
        self.input(None)
    }

    /// Requires:
    /// - comment_buff: To continue building into [`StackBuilder`]
    ///
    /// Returns:
    /// - [`BlockInput`]: The built [`BlockInput`]
    pub fn build(
        self,
        comment_buff: &mut HashMap<Uid, Comment>,
        final_stack: &mut HashMap<Uid, Block>,
        uid_of_this_block: &Uid,
        target_context: &TargetContext,
    ) -> BlockInput {
        let BlockInputBuilder { shadow, values } = self;
        let mut values_b: Vec<Option<UidOrValue>> = vec![];
        for value in values {
            match value {
                Some(StackOrValue::Value(v)) => values_b.push(Some(UidOrValue::Value(v))),
                Some(StackOrValue::Stack(s)) => {
                    let (mut s_builded, first_block_uid) = s.build(comment_buff, target_context);
                    let first_block = s_builded.get_mut(&first_block_uid).unwrap();
                    match first_block {
                        Block::Normal(n) => {
                            n.parent = Some(uid_of_this_block.clone().into_inner());
                            n.top_level = false;
                            n.x = None;
                            n.y = None;
                        }
                        Block::VarList(_) => {
                            let Block::VarList(vl) = s_builded.remove(&first_block_uid).unwrap() else {
                                unreachable!()
                            };
                            let BlockVarListReporterTop { kind, name, id, .. } = vl;
                            values_b.push(Some(UidOrValue::Value(match kind {
                                ListOrVariable::Variable => BlockInputValue::Variable { name, id },
                                ListOrVariable::List => BlockInputValue::List { name, id },
                            })))
                        }
                    }
                    final_stack.extend(s_builded);
                    values_b.push(Some(UidOrValue::Uid(first_block_uid.into_inner())))
                }
                None => values_b.push(None),
            }
        }
        BlockInput {
            shadow,
            inputs: values_b,
        }
    }
}

/// Raw block creation
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockNormalBuilder {
    opcode: OpCode,
    comment: Option<CommentBuilder>,
    inputs: HashMap<String, BlockInputBuilder>,
    fields: HashMap<String, BlockFieldBuilder>,
    mutation: Option<BlockMutation>,
    shadow: bool,
    x: Option<f64>,
    y: Option<f64>,
}

impl BlockNormalBuilder {
    pub fn new<O: Into<OpCode>>(opcode: O) -> BlockNormalBuilder {
        BlockNormalBuilder {
            opcode: opcode.into(),
            ..Default::default()
        }
    }

    pub fn add_input<K: Into<String>>(
        mut self,
        key: K,
        block_input_builder: BlockInputBuilder,
    ) -> Self {
        self.inputs.insert(key.into(), block_input_builder);
        self
    }

    pub fn add_input_arg<K: Into<String>>(self, key: K, arg: Arg) -> Self {
        match arg {
            Arg::Value(v) => self.add_input(
                key.into(),
                BlockInputBuilder::new(ShadowInputType::Shadow).input_some(StackOrValue::Value(v)),
            ),
            Arg::Stack(b) => self.add_input(
                key.into(),
                BlockInputBuilder::new(ShadowInputType::NoShadow)
                    .input_some(StackOrValue::Stack(b)),
            ),
        }
    }

    pub fn add_input_into_arg<K: Into<String>, A: IntoArg<AT>, AT>(self, key: K, arg: A) -> Self {
        self.add_input_arg(key, arg.into_arg())
    }

    pub fn add_input_stack<K: Into<String>>(mut self, key: K, stack_builder: StackBuilder) -> Self {
        self.inputs.insert(
            key.into(),
            BlockInputBuilder::new(ShadowInputType::NoShadow)
                .input_some(StackOrValue::Stack(stack_builder)),
        );
        self
    }

    pub fn add_input_into_stack<K: Into<String>, S: IntoStackArg>(self, key: K, stack: S) -> Self {
        self.add_input_stack(key, stack.into_stack_arg())
    }

    pub fn add_optional_input_stack<K: Into<String>>(
        self,
        key: K,
        stack_builder: Option<StackBuilder>,
    ) -> Self {
        match stack_builder {
            Some(stack_builder) => self.add_input_stack(key, stack_builder),
            None => self,
        }
    }

    pub fn add_optional_into_input_stack<K: Into<String>, IS: IntoStackArg>(
        self,
        key: K,
        stack: Option<IS>,
    ) -> Self {
        self.add_optional_input_stack(key, stack.map(IntoStackArg::into_stack_arg))
    }

    pub fn add_field<S: Into<String>>(
        mut self,
        key: S,
        block_field_builder: BlockFieldBuilder,
    ) -> Self {
        self.fields.insert(key.into(), block_field_builder);
        self
    }

    pub fn add_into_field<S: Into<String>, F: IntoFieldArg<FT>, FT>(
        mut self,
        key: S,
        field: F,
    ) -> Self {
        self.fields.insert(key.into(), field.into_field_arg());
        self
    }

    pub fn shadow(mut self, is_shadow: bool) -> Self {
        self.shadow = is_shadow;
        self
    }

    pub fn comment(mut self, comment_builder: CommentBuilder) -> Self {
        self.comment = Some(comment_builder);
        self
    }

    pub fn pos(mut self, x: f64, y: f64) -> Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }

    pub fn mut_pos(&mut self, x: f64, y: f64) -> &mut Self {
        self.x = Some(x);
        self.y = Some(y);
        self
    }

    pub fn mutation(mut self, mutation: BlockMutation) -> Self {
        self.mutation = Some(mutation);
        self
    }

    /// Requires:
    /// - comment_buff: To continue building into [`BlockInputBuilder`] and add comment to it
    /// - final_stack: To continue building into [`BlockInputBuilder`]
    ///
    /// Returns:
    /// - [`Block`]: The built [`Block`]
    /// - [`Uid`]: [`Uid`] of the built block
    fn build(
        self,
        comment_buff: &mut HashMap<Uid, Comment>,
        final_stack: &mut HashMap<Uid, Block>,
        target_context: &TargetContext,
    ) -> (BlockNormal, Uid) {
        let BlockNormalBuilder {
            opcode,
            comment,
            inputs,
            fields,
            shadow,
            mutation,
            x,
            y,
        } = self;
        let my_uid = Uid::generate();
        // let mut inputs_b: HashMap<String, BlockInput> = HashMap::default();
        // for (key, input) in inputs {
        //     inputs_b.insert(key, input.build(comment_buff, final_stack, &my_uid));
        // }
        let inputs: HashMap<String, BlockInput> = inputs
            .into_iter()
            .map(|(key, input)| {
                (
                    key,
                    input.build(comment_buff, final_stack, &my_uid, target_context),
                )
            })
            .collect();
        let fields: HashMap<String, BlockField> = fields
            .into_iter()
            .map(|(key, field)| (key, field.build(target_context)))
            .collect();
        let comment = match comment {
            Some(comment) => {
                let (mut comment, comment_uid) = comment.build();
                comment.block_id = Some(my_uid.clone().into_inner());
                comment_buff.insert(comment_uid.clone(), comment);
                Some(comment_uid.into_inner())
            }
            None => None,
        };
        let block_b = BlockNormal {
            opcode,
            comment,
            next: None,
            parent: None,
            inputs: StringHashMap(inputs),
            fields: StringHashMap(fields),
            shadow,
            top_level: false,
            mutation,
            x: x.map(|x| x.into()),
            y: y.map(|y| y.into()),
        };
        (block_b, my_uid)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FieldKind {
    NoRef,
    #[default]
    NoRefMaybe,
    Broadcast,
    SpriteVariable,
    GlobalVariable,
    SpriteList,
    GlobalList,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockFieldBuilder {
    value: String,
    kind: FieldKind,
}

impl BlockFieldBuilder {
    pub fn new_with_kind(value: String, kind: FieldKind) -> BlockFieldBuilder {
        BlockFieldBuilder { value, kind }
    }

    pub fn new(value: String) -> BlockFieldBuilder {
        BlockFieldBuilder {
            value,
            kind: FieldKind::NoRefMaybe,
        }
    }

    pub fn build(self, target_context: &TargetContext) -> BlockField {
        let BlockFieldBuilder { value, kind } = self;
        let value = value.into();
        let rs_sb3::value::Value::Text(ref value_str) = value else {
            unreachable!("why the hell the `not text` would be here")
        };
        let id = match kind {
            FieldKind::NoRef => return BlockField::NoId { value },
            FieldKind::NoRefMaybe => return BlockField::WithId { value, id: None },

            FieldKind::Broadcast => target_context.all_broadcasts,
            FieldKind::SpriteVariable => target_context.this_sprite_vars,
            FieldKind::GlobalVariable => target_context.global_vars,
            FieldKind::SpriteList => target_context.this_sprite_lists,
            FieldKind::GlobalList => target_context.global_lists,
        }
        .get(value_str)
        .map(|uid| uid.clone())
        .unwrap_or("__unknown__".into());
        BlockField::WithId {
            value,
            id: Some(id.into_inner()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VarListFrom {
    Global,
    Sprite,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlockVarListBuilder {
    kind: ListOrVariable,
    from: VarListFrom,
    name: String,
    x: f64,
    y: f64,
}

impl BlockVarListBuilder {
    pub fn global<Name: Into<String>>(
        var_or_list: ListOrVariable,
        name: Name,
    ) -> BlockVarListBuilder {
        BlockVarListBuilder {
            kind: var_or_list,
            name: name.into(),
            from: VarListFrom::Global,
            x: 0.,
            y: 0.,
        }
    }

    pub fn sprite<Name: Into<String>>(
        var_or_list: ListOrVariable,
        name: Name,
    ) -> BlockVarListBuilder {
        BlockVarListBuilder {
            kind: var_or_list,
            from: VarListFrom::Sprite,
            name: name.into(),
            x: 0.,
            y: 0.,
        }
    }

    pub fn pos(mut self, x: f64, y: f64) -> Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn mut_pos(&mut self, x: f64, y: f64) -> &mut Self {
        self.x = x;
        self.y = y;
        self
    }

    pub fn build(self, target_context: &TargetContext) -> (BlockVarListReporterTop, Uid) {
        let BlockVarListBuilder {
            kind,
            from,
            name,
            x,
            y,
        } = self;
        let my_uid = Uid::generate();
        let varlist_id = match (&kind, from) {
            (ListOrVariable::Variable, VarListFrom::Global) => target_context.global_vars,
            (ListOrVariable::Variable, VarListFrom::Sprite) => target_context.this_sprite_vars,
            (ListOrVariable::List, VarListFrom::Global) => target_context.global_lists,
            (ListOrVariable::List, VarListFrom::Sprite) => target_context.this_sprite_lists,
        }
        .get(&name)
        .map(|uid| uid.clone())
        .unwrap_or("__unknown__".into());
        let block_varlist_b = BlockVarListReporterTop {
            kind,
            name,
            id: varlist_id.into_inner(),
            x: x.into(),
            y: y.into(),
        };
        (block_varlist_b, my_uid)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BlockBuilder {
    Normal(BlockNormalBuilder),
    VarList(BlockVarListBuilder),
}

pub struct TargetContext<'a> {
    pub global_vars: &'a HashMap<String, Uid>,
    pub global_lists: &'a HashMap<String, Uid>,
    pub this_sprite_vars: &'a HashMap<String, Uid>,
    pub this_sprite_lists: &'a HashMap<String, Uid>,
    pub all_broadcasts: &'a HashMap<String, Uid>,
}

impl BlockBuilder {
    pub fn build(
        self,
        comment_buff: &mut HashMap<Uid, Comment>,
        final_stack: &mut HashMap<Uid, Block>,
        target_context: &TargetContext,
    ) -> (Block, Uid) {
        match self {
            BlockBuilder::Normal(n) => {
                let (b, uid) = n.build(comment_buff, final_stack, target_context);
                (Block::Normal(b), uid)
            }
            BlockBuilder::VarList(vl) => {
                let (b, uid) = vl.build(target_context);
                (Block::VarList(b), uid)
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StackBuilder {
    stack: Vec<BlockBuilder>,
}

impl StackBuilder {
    pub fn start(block: BlockNormalBuilder) -> StackBuilder {
        StackBuilder::start_with_capacity(1, BlockBuilder::Normal(block))
    }

    pub fn start_varlist(block: BlockVarListBuilder) -> StackBuilder {
        StackBuilder::start_with_capacity(1, BlockBuilder::VarList(block))
    }

    pub fn start_with_capacity(capacity: usize, block: BlockBuilder) -> StackBuilder {
        let mut stack = Vec::with_capacity(capacity);
        stack.push(block);
        StackBuilder { stack }
    }

    pub fn next(mut self, mut next_stack: StackBuilder) -> StackBuilder {
        self.stack.append(&mut next_stack.stack);
        self
    }

    pub fn move_head(mut self, x: f64, y: f64) -> Self {
        match &mut self.stack[0] {
            BlockBuilder::Normal(n) => {
                n.mut_pos(x, y);
            }
            BlockBuilder::VarList(vl) => {
                vl.mut_pos(x, y);
            }
        }
        self
    }

    pub fn mut_move_head(&mut self, x: f64, y: f64) -> &mut Self {
        match &mut self.stack[0] {
            BlockBuilder::Normal(n) => {
                n.mut_pos(x, y);
            }
            BlockBuilder::VarList(vl) => {
                vl.mut_pos(x, y);
            }
        }
        self
    }

    /// Requires:
    /// - target: to continue building into [`BlockBuilder`]
    ///
    /// Returns:
    /// - [`HashMap`]<Uid, Block>: The built stack
    /// - [`Uid`]: Uid of the first block on the stack
    pub fn build(
        self,
        comment_buff: &mut HashMap<Uid, Comment>,
        target_context: &TargetContext,
    ) -> (HashMap<Uid, Block>, Uid) {
        let mut stack_b: HashMap<Uid, Block> = HashMap::default();
        let mut self_stack_iter = self.stack.into_iter();
        let (first_block, first_block_uid) =
            self_stack_iter
                .next()
                .unwrap()
                .build(comment_buff, &mut stack_b, target_context);

        match first_block {
            Block::Normal(mut first_block) => {
                first_block.top_level = true;
                first_block.x = Some(0.into());
                first_block.y = Some(0.into());
                let mut previous_block = Some((first_block, first_block_uid.clone()));
                for block_builder2 in self_stack_iter {
                    let (mut block1, block1_uid) = previous_block.take().unwrap();
                    let (Block::Normal(mut block2), block2_uid) =
                        block_builder2.build(comment_buff, &mut stack_b, target_context) else {
                        panic!("BlockVarList shouldn't exist here")
                    };

                    block1.next = Some(block2_uid.clone().into_inner());
                    block2.parent = Some(block1_uid.clone().into_inner());

                    previous_block = Some((block2, block2_uid));

                    stack_b.insert(block1_uid, Block::Normal(block1));
                }
                let previous_block = previous_block.unwrap();
                stack_b.insert(previous_block.1, Block::Normal(previous_block.0));
                (stack_b, first_block_uid)
            }
            Block::VarList(vl) => {
                stack_b.insert(first_block_uid.clone(), Block::VarList(vl));
                (stack_b, first_block_uid)
            }
        }
    }
}

impl<S, E> From<TypedStackBuilder<S, E>> for StackBuilder {
    fn from(value: TypedStackBuilder<S, E>) -> Self {
        value.into_untyped()
    }
}
