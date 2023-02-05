use std::collections::HashMap;

use crate::{project::script::CommentBuilder, uid::Uid};
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
    pub fn new() -> BlockInputBuilder {
        BlockInputBuilder {
            shadow: ShadowInputType::NoShadow,
            values: vec![],
        }
    }

    pub fn shadow(mut self, shadow: ShadowInputType) -> Self {
        self.shadow = shadow;
        self
    }

    pub fn input(mut self, input: Option<StackOrValue>) -> Self {
        self.values.push(input);
        self
    }

    /// Shortcut for
    /// ```
    /// BlockInputBuilder::new()
    ///     .shadow(ShadowInputType::Shadow)
    ///     .input(Some(StackOrValue::Value(value)))
    /// ```
    pub fn value(value: BlockInputValue) -> Self {
        BlockInputBuilder::new()
            .shadow(ShadowInputType::Shadow)
            .input(Some(StackOrValue::Value(value)))
    }

    /// Shortcut for
    /// ```
    /// BlockInputBuilder::new()
    ///     .shadow(ShadowInputType::NoShadow)
    ///     .input(Some(StackOrValue::Stack(stack)))
    /// ```
    pub fn stack(stack: StackBuilder) -> Self {
        BlockInputBuilder::new()
            .shadow(ShadowInputType::NoShadow)
            .input(Some(StackOrValue::Stack(stack)))
    }

    /// Shortcut for
    /// ```
    /// BlockInputBuilder::new()
    ///     .shadow(ShadowInputType::ShadowObscured)
    ///     .input(Some(StackOrValue::Stack(stack)))
    ///     .input(Some(StackOrValue::Value(value)))
    /// ```
    pub fn stack_with_value_obscured(stack: StackBuilder, value: BlockInputValue) -> Self {
        BlockInputBuilder::new()
            .shadow(ShadowInputType::ShadowObscured)
            .input(Some(StackOrValue::Stack(stack)))
            .input(Some(StackOrValue::Value(value)))
    }

    pub fn build(
        self,
        this_block_uid: &Uid,
        comment_buff: &mut HashMap<Uid, Comment>,
        final_stack: &mut HashMap<Uid, Block>,
        target_context: &TargetContext,
    ) -> BlockInput {
        let BlockInputBuilder { shadow, values } = self;
        let mut values_b: Vec<Option<UidOrValue>> = vec![];
        for value in values {
            match value {
                Some(StackOrValue::Value(v)) => values_b.push(Some(UidOrValue::Value(v))),
                Some(StackOrValue::Stack(s)) => {
                    let first_block_uid = Uid::generate();
                    let mut s_builded = s.build(&first_block_uid, comment_buff, target_context);
                    let first_block = s_builded.get_mut(&first_block_uid).unwrap();
                    match first_block {
                        Block::Normal(n) => {
                            n.parent = Some(this_block_uid.clone().into_inner());
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

    pub fn add_field<S: Into<String>>(
        mut self,
        key: S,
        block_field_builder: BlockFieldBuilder,
    ) -> Self {
        self.fields.insert(key.into(), block_field_builder);
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

    fn build(
        self,
        my_uid: &Uid,
        comment_buff: &mut HashMap<Uid, Comment>,
        final_stack: &mut HashMap<Uid, Block>,
        target_context: &TargetContext,
    ) -> BlockNormal {
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
        // let mut inputs_b: HashMap<String, BlockInput> = HashMap::default();
        // for (key, input) in inputs {
        //     inputs_b.insert(key, input.build(comment_buff, final_stack, &my_uid));
        // }
        let inputs: HashMap<String, BlockInput> = inputs
            .into_iter()
            .map(|(key, input)| {
                (
                    key,
                    input.build(my_uid, comment_buff, final_stack, target_context),
                )
            })
            .collect();
        let fields: HashMap<String, BlockField> = fields
            .into_iter()
            .map(|(key, field)| (key, field.build(target_context)))
            .collect();
        let comment = match comment {
            Some(comment) => {
                let comment_uid = Uid::generate();
                let mut comment = comment.build();
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
        block_b
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
        .unwrap_or_else(|| Uid::new("__unknown__"));
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
    comment: Option<CommentBuilder>,
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
            comment: None,
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
            comment: None,
        }
    }

    pub fn comment(mut self, comment: CommentBuilder) -> BlockVarListBuilder {
        self.comment = Some(comment);
        self
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

    pub fn build(
        self,
        my_uid: &Uid,
        comment_buff: &mut HashMap<Uid, Comment>,
        target_context: &TargetContext,
    ) -> BlockVarListReporterTop {
        let BlockVarListBuilder {
            kind,
            from,
            name,
            x,
            y,
            comment,
        } = self;
        let varlist_id = match (&kind, from) {
            (ListOrVariable::Variable, VarListFrom::Global) => target_context.global_vars,
            (ListOrVariable::Variable, VarListFrom::Sprite) => target_context.this_sprite_vars,
            (ListOrVariable::List, VarListFrom::Global) => target_context.global_lists,
            (ListOrVariable::List, VarListFrom::Sprite) => target_context.this_sprite_lists,
        }
        .get(&name)
        .map(|uid| uid.clone())
        .unwrap_or(Uid::new("__unknown__"));
        if let Some(comment) = comment {
            let comment_uid = Uid::generate();
            let mut comment = comment.build();
            comment.block_id = Some(my_uid.clone().into_inner());
            comment_buff.insert(comment_uid.clone(), comment);
        }
        let block_varlist_b = BlockVarListReporterTop {
            kind,
            name,
            id: varlist_id.into_inner(),
            x: x.into(),
            y: y.into(),
        };
        block_varlist_b
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
        my_uid: &Uid,
        comment_buff: &mut HashMap<Uid, Comment>,
        final_stack: &mut HashMap<Uid, Block>,
        target_context: &TargetContext,
    ) -> Block {
        match self {
            BlockBuilder::Normal(n) => {
                let b = n.build(my_uid, comment_buff, final_stack, target_context);
                Block::Normal(b)
            }
            BlockBuilder::VarList(vl) => {
                let b = vl.build(my_uid, comment_buff, target_context);
                Block::VarList(b)
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

    pub fn with_capacity(capacity: usize) -> StackBuilder {
        let stack = Vec::with_capacity(capacity);
        StackBuilder { stack }
    }

    pub fn new() -> StackBuilder {
        StackBuilder { stack: Vec::new() }
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

    pub fn build(
        self,
        first_block_uid: &Uid,
        comment_buff: &mut HashMap<Uid, Comment>,
        target_context: &TargetContext,
    ) -> HashMap<Uid, Block> {
        let mut stack_b: HashMap<Uid, Block> = HashMap::default();
        let mut self_stack_iter = self.stack.into_iter();
        let first_block = self_stack_iter.next().unwrap().build(
            &first_block_uid,
            comment_buff,
            &mut stack_b,
            target_context,
        );

        match first_block {
            Block::Normal(mut first_block) => {
                first_block.top_level = true;
                first_block.x = Some(0.into());
                first_block.y = Some(0.into());
                let mut previous_block = (first_block, first_block_uid.clone());
                for block_builder2 in self_stack_iter {
                    let (mut block1, block1_uid) = previous_block;
                    let block2_uid = Uid::generate();
                    let Block::Normal(mut block2) =
                        block_builder2.build(&block2_uid, comment_buff, &mut stack_b, target_context) else {
                        unreachable!("BlockVarList shouldn't exist here")
                    };

                    block1.next = Some(block2_uid.clone().into_inner());
                    block2.parent = Some(block1_uid.clone().into_inner());

                    previous_block = (block2, block2_uid);

                    stack_b.insert(block1_uid, Block::Normal(block1));
                }
                stack_b.insert(previous_block.1, Block::Normal(previous_block.0));
                stack_b
            }
            Block::VarList(vl) => {
                stack_b.insert(first_block_uid.clone(), Block::VarList(vl));
                stack_b
            }
        }
    }
}
