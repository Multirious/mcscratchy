use std::collections::HashMap;

use rs_sb3::{
    block::{
        Block, BlockField, BlockInput, BlockInputValue, BlockMutation, IdOrValue, ShadowInputType,
    },
    string_hashmap::StringHashMap,
    value::{OpCode, Uid},
};

use super::arg::Arg;
use crate::uid::UidExt;

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

    pub fn input_some(mut self, input: StackOrValue) -> Self {
        self.values.push(Some(input));
        self
    }

    pub fn input_none(mut self) -> Self {
        self.values.push(None);
        self
    }
}

/// Raw block creation
#[derive(Debug, Default, Clone, PartialEq)]
pub struct BlockBuilder {
    opcode: OpCode,
    comment: Option<Uid>,
    inputs: HashMap<Uid, BlockInputBuilder>,
    fields: HashMap<Uid, BlockField>,
    mutation: Option<BlockMutation>,
    shadow: bool,
}

impl BlockBuilder {
    pub fn new<O: Into<OpCode>>(opcode: O) -> BlockBuilder {
        BlockBuilder {
            opcode: opcode.into(),
            ..Default::default()
        }
    }

    pub fn input<S: Into<String>>(
        mut self,
        key: S,
        block_input_builder: BlockInputBuilder,
    ) -> Self {
        self.inputs.insert(key.into(), block_input_builder);
        self
    }

    pub fn input_arg<K: Into<String>>(self, key: K, arg: Arg) -> Self {
        match arg {
            Arg::Value(v) => self.input(
                key.into(),
                BlockInputBuilder::new(ShadowInputType::Shadow).input_some(StackOrValue::Value(v)),
            ),
            Arg::Stack(b) => self.input(
                key.into(),
                BlockInputBuilder::new(ShadowInputType::NoShadow)
                    .input_some(StackOrValue::Stack(b)),
            ),
        }
    }

    pub fn input_stack<K: Into<String>>(mut self, key: K, stack_builder: StackBuilder) -> Self {
        self.inputs.insert(
            key.into(),
            BlockInputBuilder::new(ShadowInputType::NoShadow)
                .input_some(StackOrValue::Stack(stack_builder)),
        );
        self
    }

    pub fn field<S: Into<String>>(mut self, key: S, block_field: BlockField) -> Self {
        self.fields.insert(key.into(), block_field);
        self
    }

    pub fn shadow(mut self, is_shadow: bool) -> Self {
        self.shadow = is_shadow;
        self
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StackBuilder {
    stack: Vec<BlockBuilder>,
}

impl StackBuilder {
    pub fn start(block: BlockBuilder) -> StackBuilder {
        StackBuilder { stack: vec![block] }
    }

    pub fn start_with_capacity(block: BlockBuilder, capacity: usize) -> StackBuilder {
        let mut stack = Vec::with_capacity(capacity);
        stack.push(block);
        StackBuilder { stack }
    }

    pub fn next(mut self, mut next_stack: StackBuilder) -> StackBuilder {
        self.stack.append(&mut next_stack.stack);
        self
    }

    pub fn build(self) -> (HashMap<Uid, Block>, Uid) {
        fn block_builder_build(
            final_stack: &mut HashMap<Uid, Block>,
            uid_for_this_block: &Uid,
            block_builder: BlockBuilder,
        ) -> Block {
            let BlockBuilder {
                opcode,
                comment,
                inputs,
                fields,
                shadow,
                mutation,
            } = block_builder;
            let mut inputs_b: HashMap<Uid, BlockInput> = HashMap::default();
            for (key, input) in inputs {
                let BlockInputBuilder { shadow, values } = input;
                let mut values_b: Vec<Option<IdOrValue>> = vec![];
                for value in values {
                    match value {
                        Some(v) => match v {
                            StackOrValue::Value(v) => values_b.push(Some(IdOrValue::Value(v))),
                            StackOrValue::Stack(s) => {
                                let (mut s_builded, first_block_uid) = s.build();
                                let first_block = s_builded.get_mut(&first_block_uid).unwrap();
                                first_block.parent = Some(uid_for_this_block.clone());
                                first_block.top_level = false;
                                first_block.x = None;
                                first_block.y = None;
                                final_stack.extend(s_builded);
                                values_b.push(Some(IdOrValue::Uid(first_block_uid)))
                            }
                        },
                        None => values_b.push(None),
                    }
                }
                inputs_b.insert(
                    key,
                    BlockInput {
                        shadow,
                        inputs: values_b,
                    },
                );
            }
            Block {
                opcode,
                comment,
                next: None,
                parent: None,
                inputs: StringHashMap(inputs_b),
                fields: StringHashMap(fields),
                shadow,
                top_level: false,
                mutation,
                x: None,
                y: None,
            }
        }

        let mut stack_b: HashMap<Uid, Block> = HashMap::default();
        let mut self_stack_iter = self.stack.into_iter();
        let first_block_uid = Uid::generate();
        let mut first_block = block_builder_build(
            &mut stack_b,
            &first_block_uid,
            self_stack_iter.next().unwrap(),
        );
        first_block.top_level = true;
        first_block.x = Some(0.into());
        first_block.y = Some(0.into());
        let mut previous_block = Some((first_block_uid.clone(), first_block));
        for block_builder2 in self_stack_iter {
            let (block1_uid, mut block1) = previous_block.take().unwrap();

            let block2_uid = Uid::generate();
            let mut block2 = block_builder_build(&mut stack_b, &block2_uid, block_builder2);

            block1.next = Some(block2_uid.clone());
            block2.parent = Some(block1_uid.clone());

            previous_block = Some((block2_uid, block2));

            stack_b.insert(block1_uid, block1);
        }
        let previous_block = previous_block.unwrap();
        stack_b.insert(previous_block.0, previous_block.1);
        (stack_b, first_block_uid)
    }
}
