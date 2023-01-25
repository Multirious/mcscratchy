use super::*;

pub fn move_steps<Steps>(steps: Steps) -> StackBlock
where
    Steps: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_movesteps).add_input_into_arg("STEPS", steps),
    )
}

pub fn turn_right<Deg>(degress: Deg) -> StackBlock
where
    Deg: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_turnright)
            .add_input_into_arg("DEGREES", degress),
    )
}

pub fn turn_left<Deg>(degress: Deg) -> StackBlock
where
    Deg: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_turnleft)
            .add_input_into_arg("DEGREES", degress),
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
        BlockNormalBuilder::new(PrimaryOpCode::motion_goto).add_input_into_arg("TO", to),
    )
}

/// Uses as argument to [`goto`]
/// Accepts:
///  - Sprite name
///  - "_mouse_" go to mouse position
///  - "_random_" go to random position
pub fn go_to_menu<To>(to: To) -> MenuReporter
where
    To: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_goto_menu)
            .add_into_field("TO", to)
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
        BlockNormalBuilder::new(PrimaryOpCode::motion_gotoxy)
            .add_input_into_arg("X", x)
            .add_input_into_arg("Y", y),
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
        BlockNormalBuilder::new(PrimaryOpCode::motion_gotoxy)
            .add_input_into_arg("SECS", duration_secs)
            .add_input_into_arg("TO", to),
    )
}

/// Uses as an argument for [`glide_to`] in `to`
/// Accepts:
///  - Sprite name
///  - "_mouse_" glide to mouse position
///  - "_random_" glide to random position
pub fn glide_to_menu<To>(to: To) -> MenuReporter
where
    To: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_glideto_menu)
            .add_into_field("TO", to)
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
        BlockNormalBuilder::new(PrimaryOpCode::motion_glidesecstoxy)
            .add_input_into_arg("SECS", dur)
            .add_input_into_arg("X", x)
            .add_input_into_arg("Y", y),
    )
}

pub fn point_in_direction<Dir>(direction: Dir) -> StackBlock
where
    Dir: IntoArg<Angle>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_pointindirection)
            .add_input_into_arg("DIRECTION", direction),
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
        BlockNormalBuilder::new(PrimaryOpCode::motion_pointtowards)
            .add_input_into_arg("TOWARDS", towards),
    )
}

/// Uses as an argument for [`point_towards`]
/// Accepts:
///  - Sprite name
///  - "_mouse_" glide to mouse position
pub fn point_towards_menu<Towards>(towards: Towards) -> MenuReporter
where
    Towards: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_pointtowards_menu)
            .add_into_field("TOWARDS", towards)
            .shadow(true),
    )
    .into()
}

pub fn set_x<X>(x: X) -> StackBlock
where
    X: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_setx).add_input_into_arg("X", x),
    )
}

pub fn set_y<Y>(y: Y) -> StackBlock
where
    Y: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_setx).add_input_into_arg("Y", y),
    )
}

pub fn change_x_by<By>(by: By) -> StackBlock
where
    By: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_changexby).add_input_into_arg("DX", by),
    )
}

pub fn change_y_by<By>(by: By) -> StackBlock
where
    By: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_changeyby).add_input_into_arg("DY", by),
    )
}

pub fn if_on_edge_bounce() -> StackBlock {
    TypedStackBuilder::start(BlockNormalBuilder::new(
        PrimaryOpCode::motion_ifonedgebounce,
    ))
}

/// Accepts:
///  - "left-right"
///  - "don't rotate"
///  - "all around"
pub fn set_rotation_style<Style>(style: Style) -> StackBlock
where
    Style: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_setrotationstyle)
            .add_into_field("STYLE", style),
    )
}

pub fn direction() -> JustReporter<Number> {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::motion_direction)).into()
}

pub fn y_position() -> JustReporter<Number> {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::motion_yposition)).into()
}

pub fn x_position() -> JustReporter<Number> {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::motion_xposition)).into()
}
