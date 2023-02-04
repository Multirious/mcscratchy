//! Blocks that ended with menu is a visual menu in scratch.
//! It's not required to be use in function argument in here
//! which might introduce some invalid argument to function that normally requires a menu in the editor.
//!
//! Some reserved input (you shouldn't try to name anything with thing in this list):
//!  - "_random_"
//!  - "_mouse_"
//!

use super::{arg::*, script_builder::*};
use crate::scripting::blocks;

// Control
// Event
// Looks
// Motion
// Operator
// Sensing
// Sound
// Data

macro_rules! simple_typed_block_def {
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
                            simple_typed_block_def!(@arg_thing ($($arg_trait)+) $arg_name)
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
    (@arg_thing (Into $($o:tt)*)$arg:expr) => {
        $arg
    };
}

// Control =====================================================================
simple_typed_block_def! {
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

simple_typed_block_def! {
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

simple_typed_block_def! {
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
simple_typed_block_def! {
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
simple_typed_block_def! {
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
simple_typed_block_def! {
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
    point_in_direction(direction: (IntoInput<Angle>)) -> StackBlock
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
    set_y(y: (IntoInput<Number>)) -> StackBlock
    change_y_by(by: (IntoInput<Number>)) -> StackBlock
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
simple_typed_block_def! {
    add(lhs: (IntoInput<Number>), rhs: (IntoInput<Number>)) -> JustReporter<Number>
    sub(lhs: (IntoInput<Number>), rhs: (IntoInput<Number>)) -> JustReporter<Number>
    mul(lhs: (IntoInput<Number>), rhs: (IntoInput<Number>)) -> JustReporter<Number>
    div(lhs: (IntoInput<Number>), rhs: (IntoInput<Number>)) -> JustReporter<Number>
    random(from: (IntoInput<Number>), to: (IntoInput<Number>)) -> JustReporter<Bool>
    less_than(lhs: (IntoInput<Value>), rhs: (IntoInput<Value>)) -> JustReporter<Bool>
    greater_than(lhs: (IntoInput<Value>), rhs: (IntoInput<Value>)) -> JustReporter<Bool>
    equals(lhs: (IntoInput<Value>), rhs: (IntoInput<Value>)) -> JustReporter<Bool>
    and(a: (IntoInput<Bool>), b: (IntoInput<Bool>)) -> JustReporter<Bool>
    or(a: (IntoInput<Bool>), b: (IntoInput<Bool>)) -> JustReporter<Bool>
    not(val: (IntoInput<Bool>)) -> JustReporter<Bool>
    join(a: (IntoInput<Text>), b: (IntoInput<Text>)) -> JustReporter<Text>
    letter_of(idx: (IntoInput<PositiveInteger>), text: (IntoInput<Text>)) -> JustReporter<Text>
    length_of(text: (IntoInput<Text>)) -> JustReporter<PositiveInteger>
    contains(text: (IntoInput<Text>), contains: (IntoInput<Text>)) -> JustReporter<Bool>
    modulo(dividend: (IntoInput<Number>), divisor: (IntoInput<Number>)) -> JustReporter<Number>
    round(val: (IntoInput<Number>)) -> JustReporter<Number>
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
    math_op(op: (IntoField), val: (IntoInput<Number>)) -> JustReporter<Number>
}

// Sensing =====================================================================
simple_typed_block_def! {
    /// Accepts:
    ///  - Sprite name
    ///  - "_mouse_"
    ///  - "_edge_"
    touching(what: (IntoInput<Text>)) -> JustReporter<Bool>
    /// Uses as argument to [`touching`]
    /// Accepts:
    ///  - Sprite name
    ///  - "_mouse_"
    ///  - "_edge_"
    touching_menu(what: (IntoField)) -> MenuReporter
    touching_color(color: (IntoInput<Color>)) -> JustReporter<Bool>
    color_touching_color(color_a: (IntoInput<Color>), color_b: (IntoInput<Color>)) -> JustReporter<Bool>
    /// Accepts:
    ///  - Sprite name
    ///  - "_mouse_"
    distance_to(what: (IntoInput<Text>)) -> JustReporter<Number>
    /// Uses as argument to [`distance_to`]
    /// Accepts:
    ///  - Sprite name
    ///  - "_mouse_"
    distance_to_menu(what: (IntoField)) -> MenuReporter
    ask_and_wait(prompt_message: (IntoInput<Text>)) -> StackBlock
    answer() -> JustReporter<Text>
    /// Accepts:
    ///  - "any"
    ///  - "space"
    ///  - "left arrow"
    ///  - "right arrow"
    ///  - "up arrow"
    ///  - "down arrow"
    ///  - Number 0 - 9
    ///  - Letter a - z
    key_pressed(key: (IntoInput<Text>)) -> JustReporter<Bool>
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
    key_menu(key: (IntoInput<Text>)) -> MenuReporter
    mouse_down() -> JustReporter<Bool>
    mouse_x() -> JustReporter<Number>
    /// Accepts:
    ///  - "not draggable"
    ///  - "draggable"
    set_drag_mode(mode: (IntoField)) -> StackBlock
    loudness() -> JustReporter<Number>
    timer() -> JustReporter<Number>
    reset_timer() -> StackBlock
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
    var_of(var: (IntoField), what: (IntoInput<Text>)) -> JustReporter<Value>
    /// Uses as argument to [`var_of`]
    /// `what` Accepts:
    ///   - Sprite name
    ///   - "_stage_"
    var_of_object_menu(what: (IntoField)) -> MenuReporter
    /// Accepts:
    ///  - "SECOND"
    ///  - "MINUTE"
    ///  - "HOUR"
    ///  - "DAYOFWEEK"
    ///  - "DATE"
    ///  - "MONTH"
    ///  - "YEAR"
    current_datetime(format: (IntoField)) -> JustReporter<PositiveInteger>
    days_since_2000() -> JustReporter<Number>
    username() -> JustReporter<Text>
}

// Sound =======================================================================

simple_typed_block_def! {

    /// Accepts:
    ///  - Sound name
    play_sound_until_done(sound: (IntoInput<Text>)) -> StackBlock
    /// Accepts:
    ///  - Sound name
    play_sound(sound: (IntoInput<Text>)) -> StackBlock
    /// Uses as argument to [`play_sound_until_done`] and [`play_sound`]
    /// Accepts:
    ///  - Sound name
    sound_menu(sound: (IntoField)) -> MenuReporter
    stop_all_sound() -> StackBlock
    /// Accepts:
    ///  - "PITCH"
    ///  - "PAN"
    change_sound_effect_by(effect: (IntoField), by: (IntoInput<Number>)) -> StackBlock
    /// Accepts:
    ///  - "PITCH"
    ///  - "PAN"
    set_sound_effect_to(effect: (IntoField), to: (IntoInput<Number>)) -> StackBlock
    clear_sound_effects() -> StackBlock
    set_volume_to(volume: (IntoInput<Number>)) -> StackBlock
    change_volume_by(by: (IntoInput<Number>)) -> StackBlock
    volume() -> JustReporter<Number>
}

// Data ========================================================================

simple_typed_block_def! {
    sprite_var(name: (Into<String>)) -> JustReporter<Value>
    sprite_list(name: (Into<String>)) -> JustReporter<Value>
    global_var(name: (Into<String>)) -> JustReporter<Value>
    global_list(name: (Into<String>)) -> JustReporter<Value>
    set_var_to(var: (IntoField<Variable>), to: (IntoInput<Value>)) -> StackBlock
    change_var_by(var: (IntoField<Variable>), by: (IntoInput<Value>)) -> StackBlock
    show_var(var: (IntoField<Variable>)) -> StackBlock
    hide_var(var: (IntoField<Variable>)) -> StackBlock
    add_to_list(item: (IntoInput<Value>)) -> StackBlock
    delete_in_list(list: (IntoField<List>), idx: (IntoInput<Integer>)) -> StackBlock
    delete_all_in_list(list: (IntoField<List>)) -> StackBlock
    insert_in_list(list: (IntoField<List>), idx: (IntoInput<Integer>), item: (IntoInput<Value>)) -> StackBlock
    replace_in_list(list: (IntoField<List>), idx: (IntoInput<Integer>), item: (IntoInput<Value>)) -> StackBlock
    item_in_list(list: (IntoField<List>), idx: (IntoInput<Integer>)) -> JustReporter<Value>
    count_of_item_in_list(list: (IntoField<List>), item: (IntoInput<Value>)) -> JustReporter<Integer>
    length_of_list(list: (IntoField<List>)) -> JustReporter<Integer>
    list_contains(list: (IntoField<List>), item: (IntoInput<Value>)) -> JustReporter<Bool>
    show_list(list: (IntoField<List>)) -> StackBlock
    hide_list(list: (IntoField<List>)) -> StackBlock
}
