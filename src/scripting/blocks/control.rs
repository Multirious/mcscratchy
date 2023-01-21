use super::*;

pub fn if_else<Cond, IfT, IfF>(condition: Cond, if_true: IfT, if_false: IfF) -> StackBlock
where
    Cond: IntoArg<Bool>,
    IfT: IntoStackArg,
    IfF: IntoStackArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::control_if_else)
            .add_input_arg("CONDITION", condition.into_arg())
            .add_input_stack("SUBSTACK", if_true.into_stack_arg())
            .add_input_stack("SUBSTACK2", if_false.into_stack_arg()),
    )
}
