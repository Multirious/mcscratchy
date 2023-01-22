use super::*;

/// Accepts:
///  - Sound name
pub fn play_sound_until_done<Sound>(sound: Sound) -> StackBlock
where
    Sound: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::sound_playuntildone)
            .add_input_arg("SOUND_MENU", sound.into_arg()),
    )
}

/// Accepts:
///  - Sound name
pub fn play_sound<Sound>(sound: Sound) -> StackBlock
where
    Sound: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::sound_play).add_input_arg("SOUND_MENU", sound.into_arg()),
    )
}

/// Uses as argument to [`play_sound_until_done`] and [`play_sound`]
/// Accepts:
///  - Sound name
pub fn sound_menu<Sound>(sound: Sound) -> MenuReporter
where
    Sound: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::sound_sounds_menu)
            .add_field("SOUND_MENU", sound.into_field_arg_with_id(None))
            .shadow(true),
    )
    .into()
}

pub fn stop_all_sound() -> StackBlock {
    TypedStackBuilder::start(BlockBuilder::new(PrimaryOpCode::sound_stopallsounds))
}

/// Accepts:
///  - "PITCH"
///  - "PAN"
pub fn change_sound_effect_by<By, Fx>(effect: Fx, by: By) -> StackBlock
where
    By: IntoArg<Number>,
    Fx: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::sound_changeeffectby)
            .add_input_arg("VALUE", by.into_arg())
            .add_field("EFFECT", effect.into_field_arg()),
    )
}

/// Accepts:
///  - "PITCH"
///  - "PAN"
pub fn set_sound_effect_to<To, Fx>(effect: Fx, to: To) -> StackBlock
where
    To: IntoArg<Number>,
    Fx: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::sound_seteffectto)
            .add_input_arg("VALUE", to.into_arg())
            .add_field("EFFECT", effect.into_field_arg()),
    )
}

pub fn clear_sound_effects() -> StackBlock {
    TypedStackBuilder::start(BlockBuilder::new(PrimaryOpCode::sound_cleareffects))
}

pub fn set_volume_to<Vol>(volume: Vol) -> StackBlock
where
    Vol: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::sound_setvolumeto)
            .add_input_arg("VOLUME", volume.into_arg()),
    )
}

pub fn change_volume_by<By>(by: By) -> StackBlock
where
    By: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::sound_changeeffectby)
            .add_input_arg("VOLUME", by.into_arg()),
    )
}

pub fn volume() -> JustReporter<Number> {
    TypedStackBuilder::start(BlockBuilder::new(PrimaryOpCode::sound_volume)).into()
}
