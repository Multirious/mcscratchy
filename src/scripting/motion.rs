use crate::opcode::PrimaryOpCode;

use super::{
    arg::*,
    script_builder::{BlockBuilder, BlockFieldBuilder},
    typed_script_builder::*,
};

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

pub fn goto<To>(to: To) -> StackBlock
where
    To: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::motion_goto).add_input_arg("TO", to.into_arg()),
    )
}

pub fn goto_menu<To>(to: To) -> JustReporter<Text>
where
    To: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::motion_goto_menu)
            .add_field(
                "TO",
                BlockFieldBuilder::new_with_id(to.into_field_arg(), None),
            )
            .shadow(true),
    )
    .into()
}

// pub fn random_menu<To>(to: To) -> JustReporter<Text>
// where
//     To: IntoFieldArg,
// {
//     TypedStackBuilder::start(
//         BlockBuilder::new(PrimaryOpCode::motion_goto_menu)
//             .add_field(
//                 "LMAO",
//                 BlockFieldBuilder::new_with_id(to.into_field_arg(), None),
//             )
//             .shadow(true),
//     )
//     .into()
// }
