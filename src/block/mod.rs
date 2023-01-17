use std::{collections::HashMap, marker::PhantomData};

use rs_sb3::{
    block::{
        Block, BlockField, BlockInput, BlockInputValue, BlockMutation, IdOrValue, ShadowInputType,
    },
    string_hashmap::StringHashMap,
    value::{Float, Int, Number, OpCode, Text, Uid, Value, ValueWithBool},
};

use crate::opcode::PrimaryOpCode;
use crate::uid::UidExt;

// mod control;
// mod event;
// mod looks;
// mod motion;
// mod operator;
// mod sensing;
// mod sound;
// mod variable;

// mod procedural;

struct BlockInputBuilder {}

/// Raw block creation
#[derive(Debug, Default)]
struct BlockBuilder {
    opcode: OpCode,
    inputs: StringHashMap<BlockInput>,
    fields: StringHashMap<BlockField>,
    // mutation: Option<BlockMutation>,
    shawdow: bool,
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

    pub fn input<S: Into<String>>(mut self, key: S, block_input: BlockInput) -> Self {
        self.inputs.0.insert(key.into(), block_input);
        self
    }

    pub fn field<S: Into<String>>(mut self, key: S, block_field: BlockField) -> Self {
        self.fields.0.insert(key.into(), block_field);
        self
    }

    pub fn shadow(mut self, is_shadow: bool) -> Self {
        self.shawdow = is_shadow;
        self
    }
}

/// Argument in the block input
/// You can insert a value but you could also insert a block in
pub enum Arg {
    /// Insert a value `T` as input
    Value(Value),
    /// Insert a of block as input. This will take the first block in the stack of the StackBuilder.
    Block(StackBuilder<Insertable, Insertable>),
}

// macro_rules! arg_inner {
//     ($arg:expr, $input_type:ident) => {
//         match $arg {
//             Arg::Value(v) => BlockInput {
//                 shadow: ShadowInputType::Shadow,
//                 inputs: vec![Some(IdOrValue::Value(BlockInputValue::$input_type {
//                     value: v,
//                 }))],
//             },
//             Arg::BlockUid(b) => BlockInput {
//                 shadow: ShadowInputType::NoShadow,
//                 inputs: vec![Some(IdOrValue::Uid(b))],
//             },
//         }
//     };
// }

// impl Arg {
//     fn arg_number(self) -> BlockInput {
//         arg_inner!(self, Number)
//     }

//     fn arg_positive_number(self) -> BlockInput {
//         arg_inner!(self, PositiveNumber)
//     }
//     fn arg_positive_integer(self) -> BlockInput {
//         arg_inner!(self, PositiveInteger)
//     }
//     fn arg_integer(self) -> BlockInput {
//         arg_inner!(self, Integer)
//     }
//     fn arg_angle(self) -> BlockInput {
//         arg_inner!(self, Angle)
//     }
//     fn arg_color(self) -> BlockInput {
//         arg_inner!(self, Color)
//     }
//     fn arg_string(self) -> BlockInput {
//         arg_inner!(self, String)
//     }
//     // fn arg_broadcast(self) -> BlockInput {}
//     // fn arg_variable(self) -> BlockInput {}
//     // fn arg_list(self) -> BlockInput {}
// }

// macro_rules! into_arg {
//     ($($ty:ident)*) => {
//         $(
//             impl From<$ty> for Arg {
//                 fn from(v: $ty) -> Self {
//                     Arg::Value(v.into())
//                 }
//             }
//         )*
//     };
// }

// into_arg! {Int Float Text bool Number Value ValueWithBool}

impl From<Int> for Arg {
    fn from(v: i64) -> Self {
        Arg::Value(Value::Number(Number::Int(v)))
    }
}

impl From<Float> for Arg {
    fn from(v: f64) -> Self {
        Arg::Value(Value::Number(Number::Float(v)))
    }
}

impl From<Text> for Arg {
    fn from(v: Text) -> Self {
        Arg::Value(Value::Text(v))
    }
}

impl<'a> From<&'a str> for Arg {
    fn from(v: &'a str) -> Self {
        Arg::Value(Value::Text(v.to_string()))
    }
}

impl<'a> From<StackBuilder<Insertable, Insertable>> for Arg {
    fn from(v: ) -> Self {
        Arg::Value(Value::Text(v.to_string()))
    }
}

/// This script start when then green flag is clicked
pub fn event_when_flag_clicked() -> StackBuilder<UnlinkableSide, LinkableSide> {
    StackBuilder::start_with_capacity(BlockBuilder::new(PrimaryOpCode::event_whenflagclicked), 1)
}

/// Move sprite by steps
pub fn motion_move_steps<Steps: Into<Arg>>(
    steps: Steps,
) -> StackBuilder<LinkableSide, LinkableSide> {
    StackBuilder::start_with_capacity(
        BlockBuilder::new(PrimaryOpCode::motion_movesteps)
            .input("STEPS", steps.into().arg_number()),
        1,
    )
}

/// Join 2 text together
pub fn operator_join<TextA: Into<Arg>, TextB: Into<Arg>>(
    a: TextA,
    b: TextB,
) -> StackBuilder<Insertable, Insertable> {
    StackBuilder::start_with_capacity(
        BlockBuilder::new(PrimaryOpCode::operator_join)
            .input("STRING1", a.into().arg_string())
            .input("STRING2", b.into().arg_string()),
        1,
    )
}

/// A side that cannot connect to other block.
/// Like a HatBlock with no connector at the top or a CapBlock with not connector at the bottom.
struct UnlinkableSide;
/// A side that can be connect to other block.
/// Like a stack block that can connect on top and bottom.
/// Like a HatBlock with connector at the bottom or a CapBlock with connector at the top.
struct LinkableSide;

/// An Insertable is not a stackable block but more like a block that is used to be insert in a block input.
/// Like a reporter.
struct Insertable;

/// An InsertableBool is is just like Insertable
/// Like a bool reporter.
struct InsertableBool;

/// Build **1** stack of scratch block
/// The generic S is type of side of the starting block.
/// The generic E is type of side of the ending block.
/// They're here for figuring out of these 2 block can connect each other in compile time.
pub struct StackBuilder<S, E> {
    stack: Vec<BlockBuilder>,
    end: PhantomData<E>,
    start: PhantomData<S>,
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
        let stack = Vec::with_capacity(capacity);
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
}

impl<S> StackBuilder<S, LinkableSide> {
    pub fn push<NE>(mut self, next_stack: StackBuilder<LinkableSide, NE>) -> StackBuilder<S, NE> {
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
    event_when_flag_clicked()
        .push(motion_move_steps(5))
        .push(motion_move_steps(5));
    // .into_stack();
}
