use super::*;

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

/// Accepts:
///  - Sprite name
///  - "_mouse_" go to mouse position
///  - "_random_" go to random position
pub fn go_to<To>(to: To) -> StackBlock
where
    To: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::motion_goto).add_input_arg("TO", to.into_arg()),
    )
}

/// Uses as argument to [`goto`]
/// Accepts:
///  - Sprite name
///  - "_mouse_" go to mouse position
///  - "_random_" go to random position
pub fn go_to_menu<To>(to: To) -> JustReporter<Text>
where
    To: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::motion_goto_menu)
            .add_field("TO", to.into_field_arg_with_id(None))
            .shadow(true),
    )
    .into()
}

pub fn goto_xy<X, Y>(x: X, y: Y) -> StackBlock
where
    X: IntoArg<Number>,
    Y: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::motion_gotoxy)
            .add_input_arg("X", x.into_arg())
            .add_input_arg("Y", y.into_arg()),
    )
}

/// Accepts:
///  - Sprite name
///  - "_mouse_" glide to mouse position
///  - "_random_" glide to random position
pub fn glide_to<Dur, To>(duration_secs: Dur, to: To) -> StackBlock
where
    Dur: IntoArg<Number>,
    To: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::motion_gotoxy)
            .add_input_arg("SECS", duration_secs.into_arg())
            .add_input_arg("TO", to.into_arg()),
    )
}

/// Uses as an argument for [`glide_to`] in `to`
/// Accepts:
///  - Sprite name
///  - "_mouse_" glide to mouse position
///  - "_random_" glide to random position
pub fn glide_to_menu<To>(to: To) -> JustReporter<Text>
where
    To: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::motion_glideto_menu)
            .add_field("TO", to.into_field_arg_with_id(None))
            .shadow(true),
    )
    .into()
}

pub fn glide_to_xy<Dur, X, Y>(dur: Dur, x: X, y: Y) -> StackBlock
where
    Dur: IntoArg<Number>,
    X: IntoArg<Number>,
    Y: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::motion_glidesecstoxy)
            .add_input_arg("SECS", dur.into_arg())
            .add_input_arg("X", x.into_arg())
            .add_input_arg("Y", y.into_arg()),
    )
}

pub fn point_in_direction<Dir>(direction: Dir) -> StackBlock
where
    Dir: IntoArg<Angle>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::motion_pointindirection)
            .add_input_arg("DIRECTION", direction.into_arg()),
    )
}

/// Accepts:
///  - Sprite name
///  - "_mouse_" glide to mouse position
pub fn point_towards<Towards>(towards: Towards) -> StackBlock
where
    Towards: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::motion_pointtowards)
            .add_input_arg("TOWARDS", towards.into_arg()),
    )
}

/// Uses as an argument for [`point_towards`]
/// Accepts:
///  - Sprite name
///  - "_mouse_" glide to mouse position
pub fn point_towards_menu<Towards>(towards: Towards) -> StackBlock
where
    Towards: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::motion_pointtowards_menu)
            .add_field("TOWARDS", towards.into_field_arg_with_id(None))
            .shadow(true),
    )
}
