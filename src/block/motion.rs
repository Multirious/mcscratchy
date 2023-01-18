use super::*;

/// Move sprite by steps
pub fn motion_move_steps<Steps: Into<Arg>>(
    steps: Steps,
) -> StackBuilder<LinkableSide, LinkableSide> {
    StackBuilder::start_with_capacity(
        BlockBuilder::new(PrimaryOpCode::motion_movesteps).input_arg("STEPS", steps.into()),
        1,
    )
}
