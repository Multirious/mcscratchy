use crate::opcode::PrimaryOpCode;

use super::{arg::*, script_builder::BlockBuilder, typed_script_builder::*};

/// Add 2 number together
pub fn add<NumA, NumB>(a: NumA, b: NumB) -> JustReporter<Number>
where
    NumA: IntoArg<Number>,
    NumB: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::operator_add)
            .add_input_arg("NUM1", a.into_arg())
            .add_input_arg("NUM2", b.into_arg()),
    )
    .into()
}

/// Check if 2 value is equal
pub fn equals<ValA, ValB>(a: ValA, b: ValB) -> JustReporter<Bool>
where
    ValA: IntoArg<Value>,
    ValB: IntoArg<Value>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::operator_equals)
            .add_input_arg("OPERAND1", a.into_arg())
            .add_input_arg("OPERAND2", b.into_arg()),
    )
    .into()
}

/// Join 2 text together
pub fn join<TextA, TextB>(a: TextA, b: TextB) -> JustReporter<Text>
where
    TextA: IntoArg<Text>,
    TextB: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::operator_join)
            .add_input_arg("STRING1", a.into_arg())
            .add_input_arg("STRING2", b.into_arg()),
    )
    .into()
}
