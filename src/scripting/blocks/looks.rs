use super::*;

pub fn think<Msg>(message: Msg) -> StackBlock
where
    Msg: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::looks_think).add_input_arg("MESSAGE", message.into_arg()),
    )
}

pub fn think_for_secs<Msg, Secs>(message: Msg, secs: Secs) -> StackBlock
where
    Msg: IntoArg<Text>,
    Secs: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::looks_thinkforsecs)
            .add_input_arg("MESSAGE", message.into_arg())
            .add_input_arg("SECS", secs.into_arg()),
    )
}

pub fn say<Msg>(message: Msg) -> StackBlock
where
    Msg: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::looks_say).add_input_arg("MESSAGE", message.into_arg()),
    )
}

pub fn say_for_secs<Msg, Secs>(message: Msg, secs: Secs) -> StackBlock
where
    Msg: IntoArg<Text>,
    Secs: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::looks_sayforsecs)
            .add_input_arg("MESSAGE", message.into_arg())
            .add_input_arg("SECS", secs.into_arg()),
    )
}

/// Accepts:
///  - Costume name
pub fn switch_costume_to<Costume>(costume: Costume) -> StackBlock
where
    Costume: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::looks_switchcostumeto)
            .add_input_arg("COSTUME", costume.into_arg()),
    )
}

/// Uses as argument to [`switch_costume_to`]
/// Accepts:
///  - Costume name
pub fn costume_menu<Costume>(costume: Costume) -> MenuReporter
where
    Costume: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::looks_costume)
            .add_field("COSTUME", costume.into_field_arg_with_id(None))
            .shadow(true),
    )
    .into()
}

pub fn next_costume() -> StackBlock {
    TypedStackBuilder::start(BlockBuilder::new(PrimaryOpCode::looks_nextcostume))
}

/// Accepts:
///  - Costume name
pub fn switch_backdrop_to<Backdrop>(backdrop: Backdrop) -> StackBlock
where
    Backdrop: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::looks_switchbackdropto)
            .add_input_arg("BACKDROP", backdrop.into_arg()),
    )
}

/// Uses as argument to [`switch_backdrop_to`]
/// Accepts:
///  - Backdrop name
pub fn backdrop_menu<Backdrop>(backdrop: Backdrop) -> MenuReporter
where
    Backdrop: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::looks_backdrops)
            .add_field("BACKDROP", backdrop.into_field_arg_with_id(None))
            .shadow(true),
    )
    .into()
}

pub fn next_backdrop() -> StackBlock {
    TypedStackBuilder::start(BlockBuilder::new(PrimaryOpCode::looks_nextbackdrop))
}

pub fn change_size_by<By>(by: By) -> StackBlock
where
    By: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::looks_changesizeby).add_input_arg("CHANGE", by.into_arg()),
    )
}

pub fn set_size_to<To>(to: To) -> StackBlock
where
    To: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::looks_setsizeto).add_input_arg("SIZE", to.into_arg()),
    )
}

/// Accepts
///  - "COLOR"
///  - "FISHEYE"
///  - "WHIRL"
///  - "PIXELATE"
///  - "MOSAIC"
///  - "BRIGHTNESS"
///  - "GHOST"
pub fn change_looks_effect_by<Fx, By>(effect: Fx, by: By) -> StackBlock
where
    Fx: IntoFieldArg,
    By: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::looks_changeeffectby)
            .add_input_arg("CHANGE", by.into_arg())
            .add_field("EFFECT", effect.into_field_arg_with_id(None)),
    )
}

/// Accepts
///  - "COLOR"
///  - "FISHEYE"
///  - "WHIRL"
///  - "PIXELATE"
///  - "MOSAIC"
///  - "BRIGHTNESS"
///  - "GHOST"
pub fn set_looks_effect_to<Fx, To>(effect: Fx, to: To) -> StackBlock
where
    Fx: IntoFieldArg,
    To: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::looks_seteffectto)
            .add_input_arg("TO", to.into_arg())
            .add_field("EFFECT", effect.into_field_arg_with_id(None)),
    )
}

pub fn clear_graphic_effects() -> StackBlock {
    TypedStackBuilder::start(BlockBuilder::new(PrimaryOpCode::looks_cleargraphiceffects))
}

pub fn show() -> StackBlock {
    TypedStackBuilder::start(BlockBuilder::new(PrimaryOpCode::looks_show))
}

pub fn hide() -> StackBlock {
    TypedStackBuilder::start(BlockBuilder::new(PrimaryOpCode::looks_hide))
}

/// Accepts:
///  - "front"
///  - "back"
pub fn go_to_layer<Layer>(layer: Layer) -> StackBlock
where
    Layer: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::looks_gotofrontback)
            .add_field("FRONT_BACK", layer.into_field_arg_with_id(None)),
    )
}

/// `layer` Accepts:
///  - "foward"
///  - "backward"
pub fn change_layer<Layer, By>(layer: Layer, by: By) -> StackBlock
where
    Layer: IntoFieldArg,
    By: IntoArg<Integer>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::looks_goforwardbackwardlayers)
            .add_input_arg("NUM", by.into_arg())
            .add_field("FORWARD_BACKWORD", layer.into_field_arg_with_id(None)),
    )
}

/// Accepts:
/// - "number"
/// - "name"
pub fn costume<Ty>(return_type: Ty) -> JustReporter<Value>
where
    Ty: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::looks_costumenumbername)
            .add_field("NUMBER_NAME", return_type.into_field_arg_with_id(None)),
    )
    .into()
}

/// Accepts:
/// - "number"
/// - "name"
pub fn backdrop<Ty>(return_type: Ty) -> JustReporter<Value>
where
    Ty: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::looks_backdropnumbername)
            .add_field("NUMBER_NAME", return_type.into_field_arg_with_id(None)),
    )
    .into()
}

pub fn size() -> JustReporter<Number> {
    TypedStackBuilder::start(BlockBuilder::new(PrimaryOpCode::looks_size)).into()
}
