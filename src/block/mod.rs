use std::{collections::HashMap, marker::PhantomData};

use rs_sb3::{
    block::{
        Block, BlockField, BlockInput, BlockInputValue, BlockMutation, IdOrValue, ShadowInputType,
    },
    comment::Comment,
    string_hashmap::StringHashMap,
    value::{Float, Int, Number, OpCode, Text, Uid, Value, ValueWithBool},
};

use crate::opcode::PrimaryOpCode;
use crate::uid::UidExt;

// mod control;
mod event;
// mod looks;
mod motion;
mod operator;
// mod sensing;
// mod sound;
// mod variable;

// mod procedural;

#[derive(Debug, Clone)]
enum StackOrValue {
    Value(BlockInputValue),
    Stack(StackBuilder<Insertable, Insertable>),
}

#[derive(Debug, Clone)]
struct BlockInputBuilder {
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
}

/// Raw block creation
#[derive(Debug, Default, Clone)]
pub struct BlockBuilder {
    opcode: OpCode,
    comment: Option<Uid>,
    inputs: StringHashMap<BlockInputBuilder>,
    fields: StringHashMap<BlockField>,
    mutation: Option<BlockMutation>,
    shadow: bool,
}

impl BlockBuilder {
    // pub fn into_block(self) -> Block {
    //     Block {
    //         opcode: self.opcode,
    //         comment: None,
    //         next: None,
    //         parent: None,
    //         inputs: self.inputs,
    //         fields: self.fields,
    //         shadow: self.shawdow,
    //         top_level: true,
    //         mutation: self.mutation,
    //         x: Some(0.into()),
    //         y: Some(0.into()),
    //     }
    // }

    pub fn new<O: Into<OpCode>>(opcode: O) -> BlockBuilder {
        BlockBuilder {
            opcode: opcode.into(),
            ..Default::default()
        }
    }

    pub fn input<S: Into<String>>(
        mut self,
        key: S,
        block_input_buidler: BlockInputBuilder,
    ) -> Self {
        self.inputs.0.insert(key.into(), block_input_buidler);
        self
    }

    pub fn input_arg<S: Into<String>>(self, key: S, arg: Arg) -> Self {
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

    pub fn field<S: Into<String>>(mut self, key: S, block_field: BlockField) -> Self {
        self.fields.0.insert(key.into(), block_field);
        self
    }

    pub fn shadow(mut self, is_shadow: bool) -> Self {
        self.shadow = is_shadow;
        self
    }
}

/// Argument in the block input
/// You can insert a value but you could also insert a block in
pub enum Arg {
    /// Insert a value `T` as input
    Value(BlockInputValue),
    /// Insert a of block as input. This will take the first block in the stack of the StackBuilder.
    Stack(StackBuilder<Insertable, Insertable>),
}

/// A side that cannot connect to other block.
/// Like a HatBlock with no connector at the top or a CapBlock with not connector at the bottom.
#[derive(Debug, Clone)]
pub struct UnlinkableSide;
/// A side that can be connect to other block.
/// Like a stack block that can connect on top and bottom.
/// Like a HatBlock with connector at the bottom or a CapBlock with connector at the top.
#[derive(Debug, Clone)]
pub struct LinkableSide;

/// An Insertable is not a stackable block but more like a block that is used to be insert in a block input.
/// Like a reporter.
#[derive(Debug, Clone)]
pub struct Insertable;

/// An InsertableBool is is just like Insertable
/// Like a bool reporter.
#[derive(Debug, Clone)]
pub struct InsertableBool;

/// Build **1** stack of scratch block
/// The generic S is type of side of the starting block.
/// The generic E is type of side of the ending block.
/// They're here for figuring out of these 2 block can connect each other in compile time.
#[derive(Debug, Clone)]
pub struct StackBuilder<S, E> {
    stack: Vec<BlockBuilder>,
    start: PhantomData<S>,
    end: PhantomData<E>,
}

impl<S, E> StackBuilder<S, E> {
    pub fn start(block: BlockBuilder) -> StackBuilder<S, E> {
        StackBuilder {
            stack: vec![block],
            start: PhantomData,
            end: PhantomData,
        }
    }

    pub fn start_with_capacity(block: BlockBuilder, capacity: usize) -> StackBuilder<S, E> {
        let mut stack = Vec::with_capacity(capacity);
        stack.push(block);
        StackBuilder {
            stack,
            start: PhantomData,
            end: PhantomData,
        }
    }

    // pub fn into_stack(self) -> StringHashMap<Block> {
    //     self.stack.windows(2)
    // }
    pub fn build(self) -> (StringHashMap<Block>, Uid) {
        fn block_builder_build(
            final_stack: &mut StringHashMap<Block>,
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
            let mut inputs_b: StringHashMap<BlockInput> = StringHashMap::default();
            for (key, input) in inputs.0 {
                let BlockInputBuilder { shadow, values } = input;
                let mut values_b: Vec<Option<IdOrValue>> = vec![];
                for value in values {
                    match value {
                        Some(v) => match v {
                            StackOrValue::Value(v) => values_b.push(Some(IdOrValue::Value(v))),
                            StackOrValue::Stack(s) => {
                                let (mut s_builded, first_block_uid) = s.build();
                                let first_block = s_builded.0.get_mut(&first_block_uid).unwrap();
                                first_block.parent = Some(uid_for_this_block.clone());
                                first_block.top_level = false;
                                first_block.x = None;
                                first_block.y = None;
                                final_stack.0.extend(s_builded.0);
                                values_b.push(Some(IdOrValue::Uid(first_block_uid)))
                            }
                        },
                        None => values_b.push(None),
                    }
                }
                inputs_b.0.insert(
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
                inputs: inputs_b,
                fields,
                shadow,
                top_level: false,
                mutation,
                x: None,
                y: None,
            }
        }

        let mut stack_b: StringHashMap<Block> = StringHashMap::default();
        let first_block_uid = Uid::generate();
        let mut first_block =
            block_builder_build(&mut stack_b, &first_block_uid, self.stack[0].clone());
        first_block.top_level = true;
        first_block.x = Some(0.into());
        first_block.y = Some(0.into());
        let mut previous_block = Some((first_block_uid.clone(), first_block));
        for i in 1..(self.stack.len() - 1) {
            // TODO: Probably incredibly wasteful here. Optimize the .clone in these 2 lines
            let block_builder2 = self.stack[i + 1].clone();

            let (block1_uid, mut block1) = previous_block.take().unwrap();

            let block2_uid = Uid::generate();
            let mut block2 = block_builder_build(&mut stack_b, &block2_uid, block_builder2);

            block1.next = Some(block2_uid.clone());
            block2.parent = Some(block1_uid.clone());

            previous_block = Some((block2_uid, block2));

            stack_b.0.insert(block1_uid, block1);
        }
        let previous_block = previous_block.unwrap();
        stack_b.0.insert(previous_block.0, previous_block.1);
        (stack_b, first_block_uid)
    }
}

impl<S> StackBuilder<S, LinkableSide> {
    pub fn push<NE>(
        mut self,
        mut next_stack: StackBuilder<LinkableSide, NE>,
    ) -> StackBuilder<S, NE> {
        self.stack.append(&mut next_stack.stack);
        StackBuilder {
            stack: self.stack,
            start: self.start,
            end: next_stack.end,
        }
    }
}

#[test]
fn test() {
    let stack = event::event_when_flag_clicked();
    // .push(motion::motion_move_steps(Arg::Value(
    //     BlockInputValue::Number { value: 6.into() },
    // )))
    // .push(motion::motion_move_steps(Arg::Stack(
    //     operator::operator_join(
    //         Arg::Value(BlockInputValue::String {
    //             value: "a".to_owned().into(),
    //         }),
    //         Arg::Stack(operator::operator_join(
    //             Arg::Value(BlockInputValue::String {
    //                 value: "b".to_owned().into(),
    //             }),
    //             Arg::Value(BlockInputValue::String {
    //                 value: "c".to_owned().into(),
    //             }),
    //         )),
    //     ),
    // )));
    let (stack, _) = stack.build();
    println!("{}", serde_json::to_string_pretty(&stack).unwrap());
}
