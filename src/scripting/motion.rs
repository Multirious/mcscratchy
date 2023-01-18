use crate::opcode::PrimaryOpCode;

use super::{arg::*, script_builder::BlockBuilder, typed_script_builder::*};

/// Move sprite by steps
pub fn move_steps<Steps: IntoArg<Number>>(steps: Steps) -> StackBlock {
    TypedStackBuilder::start_with_capacity(
        1,
        BlockBuilder::new(PrimaryOpCode::motion_movesteps).input_arg("STEPS", steps.into_arg()),
    )
}
