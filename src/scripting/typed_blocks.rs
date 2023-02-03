//! Blocks that ended with menu is a visual menu in scratch.
//! It's not required to be use in function argument in here
//! which might introduce some invalid argument to function that normally requires a menu in the editor.
//!
//! Some reserved input (you shouldn't try to name anything with thing in this list):
//!  - "_random_"
//!  - "_mouse_"
//!

use super::{arg::*, blocks, script_builder::BlockNormalBuilder, typed_script_builder::*};
use crate::opcode::PrimaryOpCode;
use crate::scripting::script_builder::BlockVarListBuilder;
use rs_sb3::block::{BlockMutation, BlockMutationEnum, ListOrVariable};

// Why don't I put this in different file?
// It's kinda easier to refactor because this is all just implementation

// Use search on your editor to go through sections
// Control
// Event
// Looks
// Motion
// Operator
// Sensing
// Sound
// Data

macro_rules! typed_block {
    ( $(
        $(#[$attributes:meta])*
        $fn_name:ident( $(
            $arg_name:ident: ($($arg_trait:tt)+)
        ),* ) -> $return_ty:ty
    )* ) => {
        $(
            $(#[$attributes])*
            pub fn $fn_name(
                $($arg_name: impl $($arg_trait)+),*
            ) -> $return_ty
            {
                TypedStackBuilder::assume_typed(
                    blocks::$fn_name(
                        $(
                            typed_block!(@arg_thing ($($arg_trait)+) $arg_name)
                        ),*
                    )
                ).into()
            }
        )*
    };

    (@arg_thing (IntoInput $($o:tt)*) $arg:expr) => {
        $arg.into_input()
    };
    (@arg_thing (IntoField $($o:tt)*)$arg:expr) => {
        $arg.into_field()
    };
}

// Control =====================================================================
typed_block! {
    wait(duration: (IntoInput<PositiveNumber>)) -> StackBlock
}

pub fn repeat(
    times: impl IntoInput<PositiveInteger>,
    to_repeat: Option<impl IntoInput<Stack>>,
) -> StackBlock {
    TypedStackBuilder::assume_typed(blocks::repeat(
        times.into_input(),
        to_repeat.map(IntoInput::into_input),
    ))
}

pub fn forever(to_repeat: Option<impl IntoInput<Stack>>) -> StackBlock {
    TypedStackBuilder::assume_typed(blocks::forever(to_repeat.map(IntoInput::into_input)))
}

pub fn if_(condition: impl IntoInput<Bool>, if_true: Option<impl IntoInput<Stack>>) -> StackBlock {
    TypedStackBuilder::assume_typed(blocks::if_(
        condition.into_input(),
        if_true.map(IntoInput::into_input),
    ))
}

pub fn if_else(
    condition: impl IntoInput<Bool>,
    if_true: Option<impl IntoInput<Stack>>,
    if_false: Option<impl IntoInput<Stack>>,
) -> StackBlock {
    TypedStackBuilder::assume_typed(blocks::if_else(
        condition.into_input(),
        if_true.map(IntoInput::into_input),
        if_false.map(IntoInput::into_input),
    ))
}

typed_block! {
    wait_until(condition: (IntoInput<Bool>)) -> StackBlock
}

pub fn repeat_until(
    condition: impl IntoInput<Bool>,
    to_repeat: Option<impl IntoInput<Stack>>,
) -> StackBlock {
    TypedStackBuilder::assume_typed(blocks::repeat_until(
        condition.into_input(),
        to_repeat.map(IntoInput::into_input),
    ))
}

/// `stop_option` Accepts:
///  - "this script" and `has_next` should be `false`
///  - "other scripts in sprite" and `has_next` should be `true`
///  - "all" and `has_next` should be `false`
pub fn stop(stop_option: impl IntoField, has_next: bool) -> CapBlock {
    TypedStackBuilder::assume_typed(blocks::stop(stop_option.into_field(), has_next))
}

typed_block! {
    when_i_start_as_a_clone() -> HatBlock

    /// Accepts:
    ///  - Sprite name
    create_clone_of(sprite: (IntoInput<Text>)) -> StackBlock

    /// Uses as argument to [`create_clone_of`]
    /// Accepts:
    ///  - Sprite name
    create_clone_of_menu(sprite: (IntoField)) -> MenuReporter

    delete_this_clone() -> CapBlock
}

// Event =======================================================================
typed_block! {
    when_flag_clicked() -> HatBlock
    /// Accepts:
    ///  - "any"
    ///  - "space"
    ///  - "left arrow"
    ///  - "right arrow"
    ///  - "up arrow"
    ///  - "down arrow"
    ///  - Number 0 - 9
    ///  - Letter a - z
    when_key_pressed(key: (IntoField)) -> HatBlock
    when_this_sprite_clicked() -> HatBlock
    /// Accepts:
    ///  - Backdrop name
    when_backdrop_switches_to(backdrop: (IntoField)) -> HatBlock
    /// Accepts:
    /// - "LOUDNESS"
    /// - "TIMER"
    when_greater_than(variable: (IntoField), value: (IntoInput<Number>)) -> HatBlock
    when_broadcast_received(broadcast: (IntoField<Broadcast>)) -> HatBlock
    broadcast(broadcast: (IntoInput<Broadcast>)) -> StackBlock
    broadcast_and_wait(broadcast: (IntoInput<Broadcast>)) -> StackBlock
}

// Looks =======================================================================
typed_block! {
    say(message: (IntoInput<Text>)) -> StackBlock
    say_for_secs(message: (IntoInput<Text>), secs: (IntoInput<Number>)) -> StackBlock
    think(message: (IntoInput<Text>)) -> StackBlock
    think_for_secs(message: (IntoInput<Text>), secs: (IntoInput<Number>)) -> StackBlock
    /// Accepts:
    ///  - Costume name
    switch_costume_to(costume: (IntoInput<Text>)) -> StackBlock
    /// Uses as argument to [`switch_costume_to`]
    /// Accepts:
    ///  - Costume name
    costume_menu(costume: (IntoField)) -> MenuReporter
    next_costume() -> StackBlock
    /// Accepts:
    ///  - Costume name
    switch_backdrop_to(backdrop: (IntoInput<Text>)) -> StackBlock
    /// Uses as argument to [`switch_backdrop_to`]
    /// Accepts:
    ///  - Backdrop name
    backdrop_menu(backdrop: (IntoField)) -> MenuReporter
    next_backdrop() -> StackBlock
    change_size_by(by: (IntoInput<Number>)) -> StackBlock
    set_size_to(to: (IntoInput<Number>)) -> StackBlock
    /// Accepts
    ///  - "COLOR"
    ///  - "FISHEYE"
    ///  - "WHIRL"
    ///  - "PIXELATE"
    ///  - "MOSAIC"
    ///  - "BRIGHTNESS"
    ///  - "GHOST"
    change_looks_effect_by(effect: (IntoField), by: (IntoInput<Number>)) -> StackBlock
    /// Accepts
    ///  - "COLOR"
    ///  - "FISHEYE"
    ///  - "WHIRL"
    ///  - "PIXELATE"
    ///  - "MOSAIC"
    ///  - "BRIGHTNESS"
    ///  - "GHOST"
    set_looks_effect_to(effect: (IntoField), to: (IntoInput<Number>)) -> StackBlock
    clear_graphic_effects() -> StackBlock
    show() -> StackBlock
    hide() -> StackBlock
    /// Accepts:
    ///  - "front"
    ///  - "back"
    go_to_layer(layer: (IntoField)) -> StackBlock
    /// `layer` Accepts:
    ///  - "foward"
    ///  - "backward"
    change_layer(layer: (IntoField), by: (IntoInput<Integer>)) -> StackBlock
    /// Accepts:
    /// - "number"
    /// - "name"
    costume(return_type: (IntoField)) -> JustReporter<Value>
    /// Accepts:
    /// - "number"
    /// - "name"
    backdrop(return_type: (IntoField)) -> JustReporter<Value>
    size() -> JustReporter<Number>
}

// Motion ======================================================================
typed_block! {
    move_steps(steps: (IntoInput<Number>)) -> StackBlock
    turn_right(degress: (IntoInput<Number>)) -> StackBlock
    turn_left(degress: (IntoInput<Number>)) -> StackBlock
    /// Accepts:
    ///  - Sprite name
    ///  - "_mouse_" go to mouse position
    ///  - "_random_" go to random position
    go_to(to: (IntoInput<Text>)) -> StackBlock
    /// Uses as argument to [`goto`]
    /// Accepts:
    ///  - Sprite name
    ///  - "_mouse_" go to mouse position
    ///  - "_random_" go to random position
    go_to_menu(to: (IntoField)) -> MenuReporter
    goto_xy(x: (IntoInput<Number>), y: (IntoInput<Number>)) -> StackBlock
    /// Accepts:
    ///  - Sprite name
    ///  - "_mouse_" glide to mouse position
    ///  - "_random_" glide to random position
    glide_to(duration_secs: (IntoInput<Number>), to: (IntoInput<Text>)) -> StackBlock
    /// Uses as an argument for [`glide_to`] in `to`
    /// Accepts:
    ///  - Sprite name
    ///  - "_mouse_" glide to mouse position
    ///  - "_random_" glide to random position
    glide_to_menu(to: (IntoField)) -> MenuReporter
    glide_to_xy(dur: (IntoInput<Number>), x: (IntoInput<Number>), y: (IntoInput<Number>)) -> StackBlock
    /// Accepts:
    ///  - Sprite name
    ///  - "_mouse_" glide to mouse position
    point_towards(towards: (IntoInput<Text>)) -> StackBlock
    /// Uses as an argument for [`point_towards`]
    /// Accepts:
    ///  - Sprite name
    ///  - "_mouse_" glide to mouse position
    point_towards_menu(towards: (IntoField)) -> MenuReporter
    set_x(x: (IntoInput<Number>)) -> StackBlock
    change_x_by(by: (IntoInput<Number>)) -> StackBlock
    if_on_edge_bounce() -> StackBlock
    /// Accepts:
    ///  - "left-right"
    ///  - "don't rotate"
    ///  - "all around"
    set_rotation_style(style: (IntoField)) -> StackBlock
    direction() -> JustReporter<Number>
    y_position() -> JustReporter<Number>
    x_position() -> JustReporter<Number>
}

// Operators ===================================================================
typed_block! {
    add(lhs: (IntoInput<Number>), rhs: (IntoInput<Number>)) -> JustReporter<Number>
    sub(lhs: (IntoInput<Number>), rhs: (IntoInput<Number>)) -> JustReporter<Number>
    mul(lhs: (IntoInput<Number>), rhs: (IntoInput<Number>)) -> JustReporter<Number>
    div(lhs: (IntoInput<Number>), rhs: (IntoInput<Number>)) -> JustReporter<Number>
}

pub fn random(from: impl IntoInput<Number>, to: impl IntoInput<Number>) -> JustReporter<Bool> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_random)
            .add_input_into_arg("FROM", from)
            .add_input_into_arg("TO", to),
    )
    .into()
}

pub fn less_than(lhs: impl IntoInput<Value>, rhs: impl IntoInput<Value>) -> JustReporter<Bool> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_lt)
            .add_input_into_arg("OPERAND1", lhs)
            .add_input_into_arg("OPERAND2", rhs),
    )
    .into()
}

pub fn greater_than(lhs: impl IntoInput<Value>, rhs: impl IntoInput<Value>) -> JustReporter<Bool> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_gt)
            .add_input_into_arg("OPERAND1", lhs)
            .add_input_into_arg("OPERAND2", rhs),
    )
    .into()
}

pub fn equals(lhs: impl IntoInput<Value>, rhs: impl IntoInput<Value>) -> JustReporter<Bool>
where
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_equals)
            .add_input_into_arg("OPERAND1", lhs)
            .add_input_into_arg("OPERAND2", rhs),
    )
    .into()
}

pub fn and(a: impl IntoInput<Bool>, b: impl IntoInput<Bool>) -> JustReporter<Bool> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_and)
            .add_input_into_arg("OPERAND1", a)
            .add_input_into_arg("OPERAND2", b),
    )
    .into()
}

pub fn or(a: impl IntoInput<Bool>, b: impl IntoInput<Bool>) -> JustReporter<Bool> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_or)
            .add_input_into_arg("OPERAND1", a)
            .add_input_into_arg("OPERAND2", b),
    )
    .into()
}

pub fn not(val: impl IntoInput<Bool>) -> JustReporter<Bool> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_or).add_input_into_arg("OPERAND", val),
    )
    .into()
}

pub fn join(a: impl IntoInput<Text>, b: impl IntoInput<Text>) -> JustReporter<Text> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_join)
            .add_input_into_arg("STRING1", a)
            .add_input_into_arg("STRING2", b),
    )
    .into()
}

pub fn letter_of(
    idx: impl IntoInput<PositiveInteger>,
    text: impl IntoInput<Text>,
) -> JustReporter<Text> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_letter_of)
            .add_input_into_arg("LETTER", idx)
            .add_input_into_arg("STRING", text),
    )
    .into()
}

pub fn length_of(text: impl IntoInput<Text>) -> JustReporter<PositiveInteger> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_length).add_input_into_arg("STRING", text),
    )
    .into()
}

pub fn contains(text: impl IntoInput<Text>, contains: impl IntoInput<Text>) -> JustReporter<Bool> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_contains)
            .add_input_into_arg("STRING1", text)
            .add_input_into_arg("STRING2", contains),
    )
    .into()
}

pub fn modulo(
    dividend: impl IntoInput<Number>,
    divisor: impl IntoInput<Number>,
) -> JustReporter<Number> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_mod)
            .add_input_into_arg("NUM1", dividend)
            .add_input_into_arg("NUM2", divisor),
    )
    .into()
}

pub fn round(val: impl IntoInput<Number>) -> JustReporter<Number> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_round).add_input_into_arg("NUM", val),
    )
    .into()
}

/// `op` Accepts:
///  - "abs"
///  - "floor"
///  - "ceiling"
///  - "sqrt"
///  - "sin"
///  - "cos"
///  - "tan"
///  - "asin"
///  - "acos"
///  - "atan"
///  - "ln"
///  - "log"
///  - "e ^"
///  - "10 ^"
pub fn math_op(op: impl IntoField, val: impl IntoInput<Number>) -> JustReporter<Number> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_mathop)
            .add_input_into_arg("NUM", val)
            .add_into_field("OPERATOR", op),
    )
    .into()
}

// Sensing =====================================================================

/// Accepts:
///  - Sprite name
///  - "_mouse_"
///  - "_edge_"
pub fn touching(what: impl IntoInput<Text>) -> JustReporter<Bool> {
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
pub fn touching_menu(what: impl IntoField) -> MenuReporter {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_touchingobjectmenu)
            .add_into_field("TOUCHINGOBJECTMENU", what)
            .shadow(true),
    )
    .into()
}

pub fn touching_color(color: impl IntoInput<Color>) -> JustReporter<Bool> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_touchingcolor)
            .add_input_into_arg("COLOR", color),
    )
    .into()
}

pub fn color_touching_color(
    color_a: impl IntoInput<Color>,
    color_b: impl IntoInput<Color>,
) -> JustReporter<Bool> {
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
pub fn distance_to(what: impl IntoInput<Text>) -> JustReporter<Number> {
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
pub fn distance_to_menu(what: impl IntoField) -> MenuReporter
where
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_coloristouchingcolor)
            .add_into_field("DISTANCETOMENU", what)
            .shadow(true),
    )
    .into()
}

pub fn ask_and_wait(prompt_message: impl IntoInput<Text>) -> StackBlock {
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
pub fn key_pressed(key: impl IntoInput<Text>) -> JustReporter<Bool> {
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
pub fn key_menu(key: impl IntoInput<Text>) -> MenuReporter {
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
pub fn set_drag_mode(mode: impl IntoField) -> StackBlock {
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
pub fn var_of(var: impl IntoField, what: impl IntoInput<Text>) -> JustReporter<Value> {
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
pub fn var_of_object_menu(what: impl IntoField) -> MenuReporter {
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
pub fn current_datetime(format: impl IntoField) -> JustReporter<PositiveInteger> {
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

// Sound =======================================================================

/// Accepts:
///  - Sound name
pub fn play_sound_until_done(sound: impl IntoInput<Text>) -> StackBlock {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_playuntildone)
            .add_input_into_arg("SOUND_MENU", sound),
    )
}

/// Accepts:
///  - Sound name
pub fn play_sound(sound: impl IntoInput<Text>) -> StackBlock {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_play).add_input_into_arg("SOUND_MENU", sound),
    )
}

/// Uses as argument to [`play_sound_until_done`] and [`play_sound`]
/// Accepts:
///  - Sound name
pub fn sound_menu(sound: impl IntoField) -> MenuReporter {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_sounds_menu)
            .add_into_field("SOUND_MENU", sound)
            .shadow(true),
    )
    .into()
}

pub fn stop_all_sound() -> StackBlock {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sound_stopallsounds))
}

/// Accepts:
///  - "PITCH"
///  - "PAN"
pub fn change_sound_effect_by(effect: impl IntoField, by: impl IntoInput<Number>) -> StackBlock {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_changeeffectby)
            .add_input_into_arg("VALUE", by)
            .add_field("EFFECT", effect.into_field()),
    )
}

/// Accepts:
///  - "PITCH"
///  - "PAN"
pub fn set_sound_effect_to(effect: impl IntoField, to: impl IntoInput<Number>) -> StackBlock {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_seteffectto)
            .add_input_into_arg("VALUE", to)
            .add_field("EFFECT", effect.into_field()),
    )
}

pub fn clear_sound_effects() -> StackBlock {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sound_cleareffects))
}

pub fn set_volume_to(volume: impl IntoInput<Number>) -> StackBlock {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_setvolumeto)
            .add_input_into_arg("VOLUME", volume),
    )
}

pub fn change_volume_by(by: impl IntoInput<Number>) -> StackBlock {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_changeeffectby)
            .add_input_into_arg("VOLUME", by),
    )
}

pub fn volume() -> JustReporter<Number> {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sound_volume)).into()
}

// Data ========================================================================
pub fn sprite_var(name: impl Into<String>) -> JustReporter<Value> {
    TypedStackBuilder::start_varlist(BlockVarListBuilder::sprite(ListOrVariable::Variable, name))
        .into()
}

pub fn sprite_list(name: impl Into<String>) -> JustReporter<Value> {
    TypedStackBuilder::start_varlist(BlockVarListBuilder::sprite(ListOrVariable::List, name)).into()
}

pub fn global_var(name: impl Into<String>) -> JustReporter<Value> {
    TypedStackBuilder::start_varlist(BlockVarListBuilder::global(ListOrVariable::Variable, name))
        .into()
}

pub fn global_list(name: impl Into<String>) -> JustReporter<Value> {
    TypedStackBuilder::start_varlist(BlockVarListBuilder::global(ListOrVariable::List, name)).into()
}

pub fn set_var_to(var: impl IntoField<Variable>, to: impl IntoInput<Value>) -> StackBlock {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_setvariableto)
            .add_input_into_arg("VALUE", to)
            .add_into_field("VARIABLE", var),
    )
}

pub fn change_var_by(var: impl IntoField<Variable>, by: impl IntoInput<Value>) -> StackBlock {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_changevariableby)
            .add_input_into_arg("VALUE", by)
            .add_into_field("VARIABLE", var),
    )
}

pub fn show_var(var: impl IntoField<Variable>) -> StackBlock {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_showvariable).add_into_field("VARIABLE", var),
    )
}

pub fn hide_var(var: impl IntoField<Variable>) -> StackBlock {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_hidevariable).add_into_field("VARIABLE", var),
    )
}

pub fn add_to_list(item: impl IntoInput<Value>) -> StackBlock {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_addtolist).add_input_into_arg("ITEM", item),
    )
}

pub fn delete_in_list(list: impl IntoField<List>, idx: impl IntoInput<Integer>) -> StackBlock {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_deleteoflist)
            .add_input_into_arg("INDEX", idx)
            .add_into_field("LIST", list),
    )
}

pub fn delete_all_in_list(list: impl IntoField<List>) -> StackBlock {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_deletealloflist).add_into_field("LIST", list),
    )
}

pub fn insert_in_list(
    list: impl IntoField<List>,
    idx: impl IntoInput<Integer>,
    item: impl IntoInput<Value>,
) -> StackBlock {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_insertatlist)
            .add_input_into_arg("INDEX", idx)
            .add_input_into_arg("ITEM", item)
            .add_into_field("LIST", list),
    )
}

pub fn replace_in_list(
    list: impl IntoField<List>,
    idx: impl IntoInput<Integer>,
    item: impl IntoInput<Value>,
) -> StackBlock {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_replaceitemoflist)
            .add_input_into_arg("INDEX", idx)
            .add_input_into_arg("ITEM", item)
            .add_into_field("LIST", list),
    )
}

pub fn item_in_list(
    list: impl IntoField<List>,
    idx: impl IntoInput<Integer>,
) -> JustReporter<Value> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_itemoflist)
            .add_input_into_arg("INDEX", idx)
            .add_into_field("LIST", list),
    )
    .into()
}

pub fn count_of_item_in_list(
    list: impl IntoField<List>,
    item: impl IntoInput<Value>,
) -> JustReporter<Integer> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_itemoflist)
            .add_input_into_arg("ITEM", item)
            .add_into_field("LIST", list),
    )
    .into()
}

pub fn length_of_list(list: impl IntoField<List>) -> JustReporter<Integer> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_lengthoflist).add_into_field("LIST", list),
    )
    .into()
}

pub fn list_contains(
    list: impl IntoField<List>,
    item: impl IntoInput<Value>,
) -> JustReporter<Bool> {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_listcontainsitem)
            .add_input_into_arg("ITEM", item)
            .add_into_field("LIST", list),
    )
    .into()
}

pub fn show_list(list: impl IntoField<List>) -> StackBlock {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_showlist).add_into_field("LIST", list),
    )
}

pub fn hide_list(list: impl IntoField<List>) -> StackBlock {
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_hidelist).add_into_field("LIST", list),
    )
}
