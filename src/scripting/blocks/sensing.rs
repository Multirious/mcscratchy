use super::*;

/// Accepts:
///  - Sprite name
///  - "_mouse_"
///  - "_edge_"
pub fn touching<What>(what: What) -> JustReporter<Bool>
where
    What: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_touchingobject)
            .add_input_into_arg("TOUCHINGOBJECTMENU", what),
    )
    .into()
}

/// Uses as argument to [`touching`]
/// Accepts:
///  - Sprite name
///  - "_mouse_"
///  - "_edge_"
pub fn touching_menu<What>(what: What) -> MenuReporter
where
    What: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_touchingobjectmenu)
            .add_into_field("TOUCHINGOBJECTMENU", what)
            .shadow(true),
    )
    .into()
}

pub fn touching_color<Col>(color: Col) -> JustReporter<Bool>
where
    Col: IntoArg<Color>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_touchingcolor)
            .add_input_into_arg("COLOR", color),
    )
    .into()
}

pub fn color_touching_color<ColA, ColB>(color_a: ColA, color_b: ColB) -> JustReporter<Bool>
where
    ColA: IntoArg<Color>,
    ColB: IntoArg<Color>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_coloristouchingcolor)
            .add_input_into_arg("COLOR", color_a)
            .add_input_into_arg("COLOR2", color_b),
    )
    .into()
}

/// Accepts:
///  - Sprite name
///  - "_mouse_"
pub fn distance_to<What>(what: What) -> JustReporter<Number>
where
    What: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_coloristouchingcolor)
            .add_input_into_arg("DISTANCETOMENU", what),
    )
    .into()
}

/// Uses as argument to [`distance_to`]
/// Accepts:
///  - Sprite name
///  - "_mouse_"
pub fn distance_to_menu<What>(what: What) -> MenuReporter
where
    What: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_coloristouchingcolor)
            .add_into_field("DISTANCETOMENU", what)
            .shadow(true),
    )
    .into()
}

pub fn ask_and_wait<Msg>(prompt_message: Msg) -> StackBlock
where
    Msg: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_askandwait)
            .add_input_into_arg("QUESTION", prompt_message),
    )
}

pub fn answer() -> JustReporter<Text> {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sensing_answer)).into()
}

/// Accepts:
///  - "any"
///  - "space"
///  - "left arrow"
///  - "right arrow"
///  - "up arrow"
///  - "down arrow"
///  - Number 0 - 9
///  - Letter a - z
pub fn key_pressed<Key>(key: Key) -> JustReporter<Bool>
where
    Key: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_keypressed)
            .add_input_into_arg("KEY_OPTION", key),
    )
    .into()
}

/// Uses as argument to [`key_pressed`]
/// Accepts:
///  - "any"
///  - "space"
///  - "left arrow"
///  - "right arrow"
///  - "up arrow"
///  - "down arrow"
///  - Number 0 - 9
///  - Letter a - z
pub fn key_menu<Key>(key: Key) -> MenuReporter
where
    Key: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_keyoptions)
            .add_input_into_arg("KEY_OPTION", key)
            .shadow(true),
    )
    .into()
}

pub fn mouse_down() -> JustReporter<Bool> {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sensing_mousedown)).into()
}

pub fn mouse_x() -> JustReporter<Number> {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sensing_mousex)).into()
}

/// Accepts:
///  - "not draggable"
///  - "draggable"
pub fn set_drag_mode<Mode>(mode: Mode) -> StackBlock
where
    Mode: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_setdragmode)
            .add_into_field("DRAG_MODE", mode),
    )
}

pub fn loudness() -> JustReporter<Number> {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sensing_loudness)).into()
}

pub fn timer() -> JustReporter<Number> {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sensing_timer)).into()
}

pub fn reset_timer() -> StackBlock {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sensing_resettimer))
}

/// `what` Accepts:
///   - Sprite name
///   - "_stage_"
///
/// If `what` is "_stage_"
///    `var` Accepts:
///      - Stage's custom variable name
///      - "backdrop #"
///      - "backdrop name"
///      - "volume"
///
/// Else `what` is a Sprite name
///    `var` Accepts:
///      - That sprite's custome variable name
///      - "x position"
///      - "y position"
///      - "direction"
///      - "costume #"
///      - "costume name"
///      - "size"
///      - "volume"
pub fn var_of<Var, What>(var: Var, what: What) -> JustReporter<Value>
where
    Var: IntoFieldArg,
    What: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_of)
            .add_input_into_arg("OBJECT", what)
            .add_into_field("PROPERTY", var),
    )
    .into()
}

/// Uses as argument to [`var_of`]
/// `what` Accepts:
///   - Sprite name
///   - "_stage_"
pub fn var_of_object_menu<What>(what: What) -> MenuReporter
where
    What: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_of_object_menu)
            .add_into_field("OBJECT", what)
            .shadow(true),
    )
    .into()
}

/// Accepts:
///  - "SECOND"
///  - "MINUTE"
///  - "HOUR"
///  - "DAYOFWEEK"
///  - "DATE"
///  - "MONTH"
///  - "YEAR"
pub fn current_datetime<Fmt>(format: Fmt) -> JustReporter<PositiveInteger>
where
    Fmt: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_current)
            .add_into_field("CURRENTMENU", format),
    )
    .into()
}

pub fn days_since_2000() -> JustReporter<Number> {
    TypedStackBuilder::start(BlockNormalBuilder::new(
        PrimaryOpCode::sensing_dayssince2000,
    ))
    .into()
}

pub fn username() -> JustReporter<Text> {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sensing_username)).into()
}
