use super::*;

pub fn add<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> JustReporter<Number>
where
    Lhs: IntoArg<Number>,
    Rhs: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_add)
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
        BlockNormalBuilder::new(PrimaryOpCode::operator_subtract)
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
        BlockNormalBuilder::new(PrimaryOpCode::operator_multiply)
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
        BlockNormalBuilder::new(PrimaryOpCode::operator_divide)
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
        BlockNormalBuilder::new(PrimaryOpCode::operator_random)
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
        BlockNormalBuilder::new(PrimaryOpCode::operator_lt)
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
        BlockNormalBuilder::new(PrimaryOpCode::operator_gt)
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
        BlockNormalBuilder::new(PrimaryOpCode::operator_equals)
            .add_input_into_arg("OPERAND1", lhs)
            .add_input_into_arg("OPERAND2", rhs),
    )
    .into()
}

pub fn and<A, B>(a: A, b: B) -> JustReporter<Bool>
where
    A: IntoArg<Bool>,
    B: IntoArg<Bool>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_and)
            .add_input_into_arg("OPERAND1", a)
            .add_input_into_arg("OPERAND2", b),
    )
    .into()
}

pub fn or<A, B>(a: A, b: B) -> JustReporter<Bool>
where
    A: IntoArg<Bool>,
    B: IntoArg<Bool>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_or)
            .add_input_into_arg("OPERAND1", a)
            .add_input_into_arg("OPERAND2", b),
    )
    .into()
}

pub fn not<Val>(val: Val) -> JustReporter<Bool>
where
    Val: IntoArg<Bool>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_or).add_input_into_arg("OPERAND", val),
    )
    .into()
}

pub fn join<TextA, TextB>(a: TextA, b: TextB) -> JustReporter<Text>
where
    TextA: IntoArg<Text>,
    TextB: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_join)
            .add_input_into_arg("STRING1", a)
            .add_input_into_arg("STRING2", b),
    )
    .into()
}

pub fn letter_of<Idx, TextA>(idx: Idx, text: TextA) -> JustReporter<Text>
where
    Idx: IntoArg<PositiveInteger>,
    TextA: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_letter_of)
            .add_input_into_arg("LETTER", idx)
            .add_input_into_arg("STRING", text),
    )
    .into()
}

pub fn length_of<TextA>(text: TextA) -> JustReporter<PositiveInteger>
where
    TextA: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_length).add_input_into_arg("STRING", text),
    )
    .into()
}

pub fn contains<TextA, Contains>(text: TextA, contains: Contains) -> JustReporter<Bool>
where
    TextA: IntoArg<Text>,
    Contains: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_contains)
            .add_input_into_arg("STRING1", text)
            .add_input_into_arg("STRING2", contains),
    )
    .into()
}

pub fn modulo<Dividend, Divisor>(dividend: Dividend, divisor: Divisor) -> JustReporter<Number>
where
    Dividend: IntoArg<Number>,
    Divisor: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_mod)
            .add_input_into_arg("NUM1", dividend)
            .add_input_into_arg("NUM2", divisor),
    )
    .into()
}

pub fn round<Val>(val: Val) -> JustReporter<Number>
where
    Val: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_round).add_input_into_arg("NUM", val),
    )
    .into()
}

/// `op` Accepts:
///  - "abs"
///  - "floor"
///  - "ceiling"
///  - "sqrt"
///  - "sin"
///  - "cos"
///  - "tan"
///  - "asin"
///  - "acos"
///  - "atan"
///  - "ln"
///  - "log"
///  - "e ^"
///  - "10 ^"
pub fn math_op<Op, Val>(op: Op, val: Val) -> JustReporter<Number>
where
    Op: IntoFieldArg,
    Val: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_mathop)
            .add_input_into_arg("NUM", val)
            .add_into_field("OPERATOR", op),
    )
    .into()
}
