use crate::opcode::PrimaryOpCode;

use super::{arg::*, script_builder::BlockBuilder, typed_script_builder::*};

pub fn if_else<Condition: IntoArg<Bool>, IfT: IntoStackArg, IfF: IntoStackArg>(
    condition: Condition,
    if_true: IfT,
    if_false: IfF,
) -> StackBlock {
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::control_if_else)
            .input_arg("CONDITION", condition.into_arg())
            .input_stack("SUBSTACK", if_true.into_stack_arg())
            .input_stack("SUBSTACK2", if_false.into_stack_arg()),
    )
}
