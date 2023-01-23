use super::*;

pub fn add<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> JustReporter<Number>
where
    Lhs: IntoArg<Number>,
    Rhs: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::operator_add)
            .add_input_into_arg("NUM1", lhs)
            .add_input_into_arg("NUM2", rhs),
    )
    .into()
}

pub fn sub<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> JustReporter<Number>
where
    Lhs: IntoArg<Number>,
    Rhs: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::operator_subtract)
            .add_input_into_arg("NUM1", lhs)
            .add_input_into_arg("NUM2", rhs),
    )
    .into()
}

pub fn mul<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> JustReporter<Number>
where
    Lhs: IntoArg<Number>,
    Rhs: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::operator_multiply)
            .add_input_into_arg("NUM1", lhs)
            .add_input_into_arg("NUM2", rhs),
    )
    .into()
}

pub fn div<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> JustReporter<Number>
where
    Lhs: IntoArg<Number>,
    Rhs: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::operator_divide)
            .add_input_into_arg("NUM1", lhs)
            .add_input_into_arg("NUM2", rhs),
    )
    .into()
}

pub fn random<From, To>(from: From, to: To) -> JustReporter<Bool>
where
    From: IntoArg<Number>,
    To: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::operator_random)
            .add_input_into_arg("FROM", from)
            .add_input_into_arg("TO", to),
    )
    .into()
}

pub fn less_than<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> JustReporter<Bool>
where
    Lhs: IntoArg<Value>,
    Rhs: IntoArg<Value>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::operator_lt)
            .add_input_into_arg("OPERAND1", lhs)
            .add_input_into_arg("OPERAND2", rhs),
    )
    .into()
}

pub fn greater_than<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> JustReporter<Bool>
where
    Lhs: IntoArg<Value>,
    Rhs: IntoArg<Value>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::operator_gt)
            .add_input_into_arg("OPERAND1", lhs)
            .add_input_into_arg("OPERAND2", rhs),
    )
    .into()
}

pub fn equals<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> JustReporter<Bool>
where
    Lhs: IntoArg<Value>,
    Rhs: IntoArg<Value>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::operator_equals)
            .add_input_into_arg("OPERAND1", lhs)
            .add_input_into_arg("OPERAND2", rhs),
    )
    .into()
}

pub fn join<TextA, TextB>(a: TextA, b: TextB) -> JustReporter<Text>
where
    TextA: IntoArg<Text>,
    TextB: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::operator_join)
            .add_input_into_arg("STRING1", a)
            .add_input_into_arg("STRING2", b),
    )
    .into()
}
