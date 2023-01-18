use std::marker::PhantomData;

use crate::opcode::PrimaryOpCode;

use super::{arg::*, script_builder::BlockBuilder, typed_script_builder::*};

/// Add 2 number together
pub fn add<NumA: IntoArg<Number>, NumB: IntoArg<Number>>(a: NumA, b: NumB) -> Reporter<Number> {
    TypedStackBuilder::start_with_capacity(
        1,
        BlockBuilder::new(PrimaryOpCode::operator_add)
            .input_arg("NUM1", a.into_arg())
            .input_arg("NUM2", b.into_arg()),
    )
    .into()
}

/// Check if 2 value is equal
pub fn equals<ValA: IntoArg<Value>, ValB: IntoArg<Value>>(a: ValA, b: ValB) -> Reporter<Number> {
    TypedStackBuilder::start_with_capacity(
        1,
        BlockBuilder::new(PrimaryOpCode::operator_equals)
            .input_arg("OPERAND1", a.into_arg())
            .input_arg("OPERAND2", b.into_arg()),
    )
    .into()
}

/// Join 2 text together
pub fn join<TextA: IntoArg<Text>, TextB: IntoArg<Text>>(a: TextA, b: TextB) -> Reporter<Text> {
    TypedStackBuilder::start_with_capacity(
        1,
        BlockBuilder::new(PrimaryOpCode::operator_join)
            .input_arg("STRING1", a.into_arg())
            .input_arg("STRING2", b.into_arg()),
    )
    .into()
}
