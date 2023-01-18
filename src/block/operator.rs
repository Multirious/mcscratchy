use super::*;

/// Join 2 text together
pub fn operator_join<TextA: Into<Arg>, TextB: Into<Arg>>(
    a: TextA,
    b: TextB,
) -> StackBuilder<Insertable, Insertable> {
    StackBuilder::start_with_capacity(
        BlockBuilder::new(PrimaryOpCode::operator_join)
            .input_arg("STRING1", a.into())
            .input_arg("STRING2", b.into()),
        1,
    )
}
