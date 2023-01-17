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

mod control;
mod event;
mod looks;
mod motion;
mod operator;
mod sensing;
mod sound;
mod variable;

mod procedural;

/// Hat blocks are the blocks that start every script.
pub struct HatBlock;

/// Stack blocks are the blocks that perform specific commands.
/// They can be place on top or below each other
pub struct StackBlock;

/// Reporter blocks are the values.
pub struct ReporterBlock;

/// Boolean block is a block that reports boolean values.
pub struct BooleanBlock;

/// C Block is a block that is a wrapper of other stack blocks.
pub struct CBlock;

/// Cap blocks are the blocks that end scripts.
/// You cannot place any blocks below them.
pub struct CapBlock;

pub trait BlockKind {}
pub trait CanContinue {}

macro_rules! impl_block_kind {
    ($($ty:ident)*) => {
        $(
            impl BlockKind for $ty {}
        )*
    }
}

impl_block_kind! {HatBlock StackBlock ReporterBlock BooleanBlock CBlock CapBlock}

/// Raw block creation
#[derive(Debug, Default)]
struct BlockBuilder {
    opcode: OpCode,
    inputs: StringHashMap<BlockInput>,
    fields: StringHashMap<BlockField>,
    mutation: Option<BlockMutation>,
    shawdow: bool,
}

impl BlockBuilder {
    pub fn end(self) -> Block {
        Block {
            opcode: self.opcode,
            comment: None,
            next: None,
            parent: None,
            inputs: self.inputs,
            fields: self.fields,
            shadow: self.shawdow,
            top_level: true,
            mutation: self.mutation,
            x: Some(0.into()),
            y: Some(0.into()),
        }
    }

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
pub enum Arg<T> {
    /// Insert a value `T` as input
    Value(T),
    /// Insert a block as input
    BlockUid(Uid),
}

macro_rules! arg_inner {
    ($arg:expr, $input_type:ident) => {
        match $arg {
            Arg::Value(v) => BlockInput {
                shadow: ShadowInputType::Shadow,
                inputs: vec![Some(IdOrValue::Value(BlockInputValue::$input_type {
                    value: v.into(),
                }))],
            },
            Arg::BlockUid(b) => BlockInput {
                shadow: ShadowInputType::NoShadow,
                inputs: vec![Some(IdOrValue::Uid(b))],
            },
        }
    };
}

impl<T: Into<Value>> Arg<T> {
    fn arg_number(self) -> BlockInput {
        arg_inner!(self, Number)
    }

    fn arg_positive_number(self) -> BlockInput {
        arg_inner!(self, PositiveNumber)
    }
    fn arg_positive_integer(self) -> BlockInput {
        arg_inner!(self, PositiveInteger)
    }
    fn arg_integer(self) -> BlockInput {
        arg_inner!(self, Integer)
    }
    fn arg_angle(self) -> BlockInput {
        arg_inner!(self, Angle)
    }
    fn arg_color(self) -> BlockInput {
        arg_inner!(self, Color)
    }
    fn arg_string(self) -> BlockInput {
        arg_inner!(self, String)
    }
    // fn arg_broadcast(self) -> BlockInput {}
    // fn arg_variable(self) -> BlockInput {}
    // fn arg_list(self) -> BlockInput {}
}

macro_rules! into_arg {
    ($($ty:ident)*) => {
        $(
            impl From<$ty> for Arg<$ty> {
                fn from(v: $ty) -> Self {
                    Arg::Value(v)
                }
            }
        )*
    };
}

into_arg! {Int Float Text bool Number Value ValueWithBool}

/// This script start when then green flag is clicked
pub fn event_when_flag_clicked() -> StackBuilder<HatBlock, HatBlock> {
    StackBuilder::start(BlockBuilder::new(PrimaryOpCode::event_whenflagclicked).end())
}

/// Move sprite by steps
pub fn motion_move_steps<Steps: Into<Arg<Number>>>(
    steps: Steps,
) -> StackBuilder<StackBlock, StackBlock> {
    StackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::motion_movesteps)
            .input("STEPS", steps.into().arg_number())
            .end(),
    )
}

/// Join 2 text together
pub fn operator_join<TextA: Into<Arg<Text>>, TextB: Into<Arg<Text>>>(
    a: TextA,
    b: TextB,
) -> StackBuilder<ReporterBlock, ReporterBlock> {
    StackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::operator_join)
            .input("STRING1", a.into().arg_string())
            .input("STRING2", b.into().arg_string())
            .end()
            .into(),
    )
}

// /// Move sprite by steps
// pub fn looks_switch_costume_to<Costume: Into<Arg<Number>>>(steps: Steps) -> StackBlock {
//     BlockBuilder::new(PrimaryOpCode::motion_movesteps)
//         .input("STEPS", steps.into().arg_number())
//         .end()
//         .into()
// }

/// Build **1** stack of scratch block
pub struct StackBuilder<S, E> {
    stack: HashMap<Uid, Block>,
    end: (Uid, PhantomData<E>),
    start: (Uid, PhantomData<S>),
}

impl<S, E> StackBuilder<S, E> {
    pub fn start(block: Block) -> StackBuilder<S, E> {
        let uid = Uid::new();
        StackBuilder {
            stack: HashMap::from_iter([(uid.clone(), block)]),
            start: (uid.clone(), PhantomData),
            end: (uid, PhantomData),
        }
    }
}

trait CanBeStacked {}
trait CanStackOnTop {}

impl CanBeStacked for StackBlock {}
impl CanBeStacked for CapBlock {}

impl CanStackOnTop for StackBlock {}
impl CanStackOnTop for HatBlock {}

impl<S, E: CanStackOnTop> StackBuilder<S, E> {
    pub fn next<NS, NE>(mut self, next_stack: StackBuilder<NS, NE>) -> StackBuilder<S, NE>
    where
        NS: CanBeStacked,
    {
        let StackBuilder { stack, end, start } = self;
        let StackBuilder {
            stack: next_stack,
            end: next_end,
            start: next_start,
        } = next_stack;

        let end_block = stack.get_mut(&end.0).unwrap();
        end_block.next = Some(next_start.0);

        StackBuilder {
            stack: self.stack,
            start,
            end: next_end,
        }
    }
}
