use crate::opcode::PrimaryOpCode;

use super::{arg::*, script_builder::BlockBuilder, typed_script_builder::*};

pub fn move_steps<Steps>(steps: Steps) -> StackBlock
where
    Steps: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::motion_movesteps).add_input_arg("STEPS", steps.into_arg()),
    )
}

pub fn turn_right<Deg>(degress: Deg) -> StackBlock
where
    Deg: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::motion_turnright)
            .add_input_arg("DEGREES", degress.into_arg()),
    )
}

pub fn turn_left<Deg>(degress: Deg) -> StackBlock
where
    Deg: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::motion_turnleft)
            .add_input_arg("DEGREES", degress.into_arg()),
    )
}
