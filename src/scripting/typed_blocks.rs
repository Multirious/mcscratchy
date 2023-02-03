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
        $fn_name:ident( $(
            $arg_name:ident: ($($arg_trait:tt)+)
        ),* ) -> $return_ty:ty
    )* ) => {
        $(
            pub fn $fn_name(
                $($arg_name: impl $($arg_trait)+)*
            ) -> $return_ty
            {
                TypedStackBuilder::assume_typed(
                    blocks::$fn_name(
                        $(
                            typed_block!(@arg_thing ($($arg_trait)+) $arg_name)
                        ),*
                    )
                )
            }
        )*
    };

    (@arg_thing (IntoInput $($o:tt)*) $arg:expr) => {
        $arg.into_input()
    };
    (@arg_thing (IntoFieldArg $($o:tt)*)$arg:expr) => {
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

pub fn if_<Cond, IfT>(condition: Cond, if_true: Option<IfT>) -> StackBlock
where
    Cond: IntoInput<Bool>,
    IfT: IntoInput<Stack>,
{
    TypedStackBuilder::assume_typed(blocks::if_(
        condition.into_input(),
        if_true.map(IntoInput::into_input),
    ))
}

pub fn if_else<Cond, IfT, IfF>(
    condition: Cond,
    if_true: Option<IfT>,
    if_false: Option<IfF>,
) -> StackBlock
where
    Cond: IntoInput<Bool>,
    IfT: IntoInput<Stack>,
    IfF: IntoInput<Stack>,
{
    TypedStackBuilder::assume_typed(blocks::if_else(
        condition.into_input(),
        if_true.map(IntoInput::into_input),
        if_false.map(IntoInput::into_input),
    ))
}

typed_block! {
    wait_until(condition: (IntoInput<Bool>)) -> StackBlock
}

pub fn repeat_until<Cond, ToRepeat>(condition: Cond, to_repeat: Option<ToRepeat>) -> StackBlock
where
    Cond: IntoInput<Bool>,
    ToRepeat: IntoInput<Stack>,
{
    TypedStackBuilder::assume_typed(blocks::repeat_until(
        condition.into_input(),
        to_repeat.map(IntoInput::into_input),
    ))
}

/// `stop_option` Accepts:
///  - "this script" and `has_next` should be `false`
///  - "other scripts in sprite" and `has_next` should be `true`
///  - "all" and `has_next` should be `false`
pub fn stop<Stop>(stop_option: Stop, has_next: bool) -> CapBlock
where
    Stop: IntoField,
{
    TypedStackBuilder::assume_typed(blocks::stop(stop_option.into_field(), has_next))
}

typed_block! {
    fn when_i_start_as_a_clone() -> HatBlock;
}

/// Accepts:
///  - Sprite name
pub fn create_clone_of<Spr>(sprite: Spr) -> StackBlock
where
    Spr: IntoInput<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::control_create_clone_of)
            .add_input_into_arg("CLONE_OPTION", sprite),
    )
}

/// Uses as argument to [`create_clone_of`]
/// Accepts:
///  - Sprite name
pub fn create_clone_of_menu<Spr>(sprite: Spr) -> MenuReporter
where
    Spr: IntoField,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::control_create_clone_of)
            .add_into_field("CLONE_OPTION", sprite)
            .shadow(true),
    )
    .into()
}

pub fn delete_this_clone() -> CapBlock {
    TypedStackBuilder::start(BlockNormalBuilder::new(
        PrimaryOpCode::control_delete_this_clone,
    ))
}

// Event =======================================================================
pub fn when_flag_clicked() -> HatBlock {
    TypedStackBuilder::start(BlockNormalBuilder::new(
        PrimaryOpCode::event_whenflagclicked,
    ))
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
pub fn when_key_pressed<Key>(key: Key) -> HatBlock
where
    Key: IntoField,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::event_whenkeypressed)
            .add_into_field("KEY_OPTION", key),
    )
}

/// Accepts:
///  - Backdrop name
pub fn when_backdrop_switches_to<BD>(backdrop: BD) -> HatBlock
where
    BD: IntoField,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::event_whenbackdropswitchesto)
            .add_into_field("BACKDROP", backdrop),
    )
}

/// Accepts:
/// - "LOUDNESS"
/// - "TIMER"
pub fn when_greater_than<Var, Val>(variable: Var, value: Val) -> HatBlock
where
    Var: IntoField,
    Val: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::event_whengreaterthan)
            .add_input_into_arg("VALUE", value)
            .add_into_field("WHENGREATERTHANMENU", variable),
    )
}

// "/WX1|^uaG.w=HaU!@=-s": {
//   "opcode": "event_whenbroadcastreceived",
//   "next": null,
//   "parent": null,
//   "inputs": {},
//   "fields": {
//     "BROADCAST_OPTION": [
//       "message1",
//       "F_xb?#z]3ABkO[##9_zM" << Broadcast ID
//     ]
//   },
//   "shadow": false,
//   "topLevel": true,
//   "x": -92,
//   "y": 522
// }
/// TODO: Don't forgot to create build for broadcast id
pub fn when_broadcast_received<Bcast>(broadcast: Bcast) -> HatBlock
where
    Bcast: IntoField<Broadcast>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::event_whenbroadcastreceived)
            .add_into_field("BROADCAST_OPTION", broadcast),
    )
}

pub fn broadcast<Bcast>(broadcast: Bcast) -> StackBlock
where
    Bcast: IntoInput<Broadcast>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::event_broadcast)
            .add_input_into_arg("BROADCAST_INPUT", broadcast),
    )
}

pub fn broadcast_and_wait<Bcast>(broadcast: Bcast) -> StackBlock
where
    Bcast: IntoInput<Broadcast>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::event_broadcastandwait)
            .add_input_into_arg("BROADCAST_INPUT", broadcast),
    )
}

// Looks =======================================================================
pub fn think<Msg>(message: Msg) -> StackBlock
where
    Msg: IntoInput<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_think)
            .add_input_arg("MESSAGE", message.into_arg()),
    )
}

pub fn think_for_secs<Msg, Secs>(message: Msg, secs: Secs) -> StackBlock
where
    Msg: IntoInput<Text>,
    Secs: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_thinkforsecs)
            .add_input_into_arg("MESSAGE", message)
            .add_input_into_arg("SECS", secs),
    )
}

pub fn say<Msg>(message: Msg) -> StackBlock
where
    Msg: IntoInput<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_say).add_input_into_arg("MESSAGE", message),
    )
}

pub fn say_for_secs<Msg, Secs>(message: Msg, secs: Secs) -> StackBlock
where
    Msg: IntoInput<Text>,
    Secs: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_sayforsecs)
            .add_input_into_arg("MESSAGE", message)
            .add_input_into_arg("SECS", secs),
    )
}

/// Accepts:
///  - Costume name
pub fn switch_costume_to<Costume>(costume: Costume) -> StackBlock
where
    Costume: IntoInput<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_switchcostumeto)
            .add_input_into_arg("COSTUME", costume),
    )
}

/// Uses as argument to [`switch_costume_to`]
/// Accepts:
///  - Costume name
pub fn costume_menu<Costume>(costume: Costume) -> MenuReporter
where
    Costume: IntoField,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_costume)
            .add_into_field("COSTUME", costume)
            .shadow(true),
    )
    .into()
}

pub fn next_costume() -> StackBlock {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::looks_nextcostume))
}

/// Accepts:
///  - Costume name
pub fn switch_backdrop_to<Backdrop>(backdrop: Backdrop) -> StackBlock
where
    Backdrop: IntoInput<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_switchbackdropto)
            .add_input_into_arg("BACKDROP", backdrop),
    )
}

/// Uses as argument to [`switch_backdrop_to`]
/// Accepts:
///  - Backdrop name
pub fn backdrop_menu<Backdrop>(backdrop: Backdrop) -> MenuReporter
where
    Backdrop: IntoField,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_backdrops)
            .add_into_field("BACKDROP", backdrop)
            .shadow(true),
    )
    .into()
}

pub fn next_backdrop() -> StackBlock {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::looks_nextbackdrop))
}

pub fn change_size_by<By>(by: By) -> StackBlock
where
    By: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_changesizeby).add_input_into_arg("CHANGE", by),
    )
}

pub fn set_size_to<To>(to: To) -> StackBlock
where
    To: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_setsizeto).add_input_into_arg("SIZE", to),
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
    Fx: IntoField,
    By: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_changeeffectby)
            .add_input_into_arg("CHANGE", by)
            .add_into_field("EFFECT", effect),
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
    Fx: IntoField,
    To: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_seteffectto)
            .add_input_into_arg("TO", to)
            .add_into_field("EFFECT", effect),
    )
}

pub fn clear_graphic_effects() -> StackBlock {
    TypedStackBuilder::start(BlockNormalBuilder::new(
        PrimaryOpCode::looks_cleargraphiceffects,
    ))
}

pub fn show() -> StackBlock {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::looks_show))
}

pub fn hide() -> StackBlock {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::looks_hide))
}

/// Accepts:
///  - "front"
///  - "back"
pub fn go_to_layer<Layer>(layer: Layer) -> StackBlock
where
    Layer: IntoField,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_gotofrontback)
            .add_into_field("FRONT_BACK", layer),
    )
}

/// `layer` Accepts:
///  - "foward"
///  - "backward"
pub fn change_layer<Layer, By>(layer: Layer, by: By) -> StackBlock
where
    Layer: IntoField,
    By: IntoInput<Integer>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_goforwardbackwardlayers)
            .add_input_into_arg("NUM", by)
            .add_into_field("FORWARD_BACKWORD", layer),
    )
}

/// Accepts:
/// - "number"
/// - "name"
pub fn costume<Ty>(return_type: Ty) -> JustReporter<Value>
where
    Ty: IntoField,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_costumenumbername)
            .add_into_field("NUMBER_NAME", return_type),
    )
    .into()
}

/// Accepts:
/// - "number"
/// - "name"
pub fn backdrop<Ty>(return_type: Ty) -> JustReporter<Value>
where
    Ty: IntoField,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_backdropnumbername)
            .add_into_field("NUMBER_NAME", return_type),
    )
    .into()
}

pub fn size() -> JustReporter<Number> {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::looks_size)).into()
}

// Motion ======================================================================
pub fn move_steps<Steps>(steps: Steps) -> StackBlock
where
    Steps: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_movesteps).add_input_into_arg("STEPS", steps),
    )
}

pub fn turn_right<Deg>(degress: Deg) -> StackBlock
where
    Deg: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_turnright)
            .add_input_into_arg("DEGREES", degress),
    )
}

pub fn turn_left<Deg>(degress: Deg) -> StackBlock
where
    Deg: IntoInput<Number>,
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
    To: IntoInput<Text>,
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
    To: IntoField,
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
    X: IntoInput<Number>,
    Y: IntoInput<Number>,
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
    Dur: IntoInput<Number>,
    To: IntoInput<Text>,
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
    To: IntoField,
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
    Dur: IntoInput<Number>,
    X: IntoInput<Number>,
    Y: IntoInput<Number>,
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
    Dir: IntoInput<Angle>,
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
    Towards: IntoInput<Text>,
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
    Towards: IntoField,
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
    X: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_setx).add_input_into_arg("X", x),
    )
}

pub fn set_y<Y>(y: Y) -> StackBlock
where
    Y: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_setx).add_input_into_arg("Y", y),
    )
}

pub fn change_x_by<By>(by: By) -> StackBlock
where
    By: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_changexby).add_input_into_arg("DX", by),
    )
}

pub fn change_y_by<By>(by: By) -> StackBlock
where
    By: IntoInput<Number>,
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
    Style: IntoField,
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

// Operators ===================================================================
pub fn add<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> JustReporter<Number>
where
    Lhs: IntoInput<Number>,
    Rhs: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_add)
            .add_input_into_arg("NUM1", lhs)
            .add_input_into_arg("NUM2", rhs),
    )
    .into()
}

pub fn sub<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> JustReporter<Number>
where
    Lhs: IntoInput<Number>,
    Rhs: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_subtract)
            .add_input_into_arg("NUM1", lhs)
            .add_input_into_arg("NUM2", rhs),
    )
    .into()
}

pub fn mul<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> JustReporter<Number>
where
    Lhs: IntoInput<Number>,
    Rhs: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_multiply)
            .add_input_into_arg("NUM1", lhs)
            .add_input_into_arg("NUM2", rhs),
    )
    .into()
}

pub fn div<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> JustReporter<Number>
where
    Lhs: IntoInput<Number>,
    Rhs: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_divide)
            .add_input_into_arg("NUM1", lhs)
            .add_input_into_arg("NUM2", rhs),
    )
    .into()
}

pub fn random<From, To>(from: From, to: To) -> JustReporter<Bool>
where
    From: IntoInput<Number>,
    To: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_random)
            .add_input_into_arg("FROM", from)
            .add_input_into_arg("TO", to),
    )
    .into()
}

pub fn less_than<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> JustReporter<Bool>
where
    Lhs: IntoInput<Value>,
    Rhs: IntoInput<Value>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_lt)
            .add_input_into_arg("OPERAND1", lhs)
            .add_input_into_arg("OPERAND2", rhs),
    )
    .into()
}

pub fn greater_than<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> JustReporter<Bool>
where
    Lhs: IntoInput<Value>,
    Rhs: IntoInput<Value>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_gt)
            .add_input_into_arg("OPERAND1", lhs)
            .add_input_into_arg("OPERAND2", rhs),
    )
    .into()
}

pub fn equals<Lhs, Rhs>(lhs: Lhs, rhs: Rhs) -> JustReporter<Bool>
where
    Lhs: IntoInput<Value>,
    Rhs: IntoInput<Value>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_equals)
            .add_input_into_arg("OPERAND1", lhs)
            .add_input_into_arg("OPERAND2", rhs),
    )
    .into()
}

pub fn and<A, B>(a: A, b: B) -> JustReporter<Bool>
where
    A: IntoInput<Bool>,
    B: IntoInput<Bool>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_and)
            .add_input_into_arg("OPERAND1", a)
            .add_input_into_arg("OPERAND2", b),
    )
    .into()
}

pub fn or<A, B>(a: A, b: B) -> JustReporter<Bool>
where
    A: IntoInput<Bool>,
    B: IntoInput<Bool>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_or)
            .add_input_into_arg("OPERAND1", a)
            .add_input_into_arg("OPERAND2", b),
    )
    .into()
}

pub fn not<Val>(val: Val) -> JustReporter<Bool>
where
    Val: IntoInput<Bool>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_or).add_input_into_arg("OPERAND", val),
    )
    .into()
}

pub fn join<TextA, TextB>(a: TextA, b: TextB) -> JustReporter<Text>
where
    TextA: IntoInput<Text>,
    TextB: IntoInput<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_join)
            .add_input_into_arg("STRING1", a)
            .add_input_into_arg("STRING2", b),
    )
    .into()
}

pub fn letter_of<Idx, TextA>(idx: Idx, text: TextA) -> JustReporter<Text>
where
    Idx: IntoInput<PositiveInteger>,
    TextA: IntoInput<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_letter_of)
            .add_input_into_arg("LETTER", idx)
            .add_input_into_arg("STRING", text),
    )
    .into()
}

pub fn length_of<TextA>(text: TextA) -> JustReporter<PositiveInteger>
where
    TextA: IntoInput<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_length).add_input_into_arg("STRING", text),
    )
    .into()
}

pub fn contains<TextA, Contains>(text: TextA, contains: Contains) -> JustReporter<Bool>
where
    TextA: IntoInput<Text>,
    Contains: IntoInput<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_contains)
            .add_input_into_arg("STRING1", text)
            .add_input_into_arg("STRING2", contains),
    )
    .into()
}

pub fn modulo<Dividend, Divisor>(dividend: Dividend, divisor: Divisor) -> JustReporter<Number>
where
    Dividend: IntoInput<Number>,
    Divisor: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_mod)
            .add_input_into_arg("NUM1", dividend)
            .add_input_into_arg("NUM2", divisor),
    )
    .into()
}

pub fn round<Val>(val: Val) -> JustReporter<Number>
where
    Val: IntoInput<Number>,
{
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
pub fn math_op<Op, Val>(op: Op, val: Val) -> JustReporter<Number>
where
    Op: IntoField,
    Val: IntoInput<Number>,
{
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
pub fn touching<What>(what: What) -> JustReporter<Bool>
where
    What: IntoInput<Text>,
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
    What: IntoField,
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
    Col: IntoInput<Color>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_touchingcolor)
            .add_input_into_arg("COLOR", color),
    )
    .into()
}

pub fn color_touching_color<ColA, ColB>(color_a: ColA, color_b: ColB) -> JustReporter<Bool>
where
    ColA: IntoInput<Color>,
    ColB: IntoInput<Color>,
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
    What: IntoInput<Text>,
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
    What: IntoField,
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
    Msg: IntoInput<Text>,
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
    Key: IntoInput<Text>,
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
    Key: IntoInput<Text>,
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
    Mode: IntoField,
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
    Var: IntoField,
    What: IntoInput<Text>,
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
    What: IntoField,
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
    Fmt: IntoField,
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

// Sound =======================================================================

/// Accepts:
///  - Sound name
pub fn play_sound_until_done<Sound>(sound: Sound) -> StackBlock
where
    Sound: IntoInput<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_playuntildone)
            .add_input_into_arg("SOUND_MENU", sound),
    )
}

/// Accepts:
///  - Sound name
pub fn play_sound<Sound>(sound: Sound) -> StackBlock
where
    Sound: IntoInput<Text>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_play).add_input_into_arg("SOUND_MENU", sound),
    )
}

/// Uses as argument to [`play_sound_until_done`] and [`play_sound`]
/// Accepts:
///  - Sound name
pub fn sound_menu<Sound>(sound: Sound) -> MenuReporter
where
    Sound: IntoField,
{
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
pub fn change_sound_effect_by<By, Fx>(effect: Fx, by: By) -> StackBlock
where
    By: IntoInput<Number>,
    Fx: IntoField,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_changeeffectby)
            .add_input_into_arg("VALUE", by)
            .add_field("EFFECT", effect.into_field()),
    )
}

/// Accepts:
///  - "PITCH"
///  - "PAN"
pub fn set_sound_effect_to<To, Fx>(effect: Fx, to: To) -> StackBlock
where
    To: IntoInput<Number>,
    Fx: IntoField,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_seteffectto)
            .add_input_into_arg("VALUE", to)
            .add_field("EFFECT", effect.into_field()),
    )
}

pub fn clear_sound_effects() -> StackBlock {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sound_cleareffects))
}

pub fn set_volume_to<Vol>(volume: Vol) -> StackBlock
where
    Vol: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_setvolumeto)
            .add_input_into_arg("VOLUME", volume),
    )
}

pub fn change_volume_by<By>(by: By) -> StackBlock
where
    By: IntoInput<Number>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_changeeffectby)
            .add_input_into_arg("VOLUME", by),
    )
}

pub fn volume() -> JustReporter<Number> {
    TypedStackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sound_volume)).into()
}

// Data ========================================================================
pub fn sprite_var<Name>(name: Name) -> JustReporter<Value>
where
    Name: Into<String>,
{
    TypedStackBuilder::start_varlist(BlockVarListBuilder::sprite(ListOrVariable::Variable, name))
        .into()
}

pub fn sprite_list<Name>(name: Name) -> JustReporter<Value>
where
    Name: Into<String>,
{
    TypedStackBuilder::start_varlist(BlockVarListBuilder::sprite(ListOrVariable::List, name)).into()
}

pub fn global_var<Name>(name: Name) -> JustReporter<Value>
where
    Name: Into<String>,
{
    TypedStackBuilder::start_varlist(BlockVarListBuilder::global(ListOrVariable::Variable, name))
        .into()
}

pub fn global_list<Name>(name: Name) -> JustReporter<Value>
where
    Name: Into<String>,
{
    TypedStackBuilder::start_varlist(BlockVarListBuilder::global(ListOrVariable::List, name)).into()
}

pub fn set_var_to<Var, To>(var: Var, to: To) -> StackBlock
where
    Var: IntoField<Variable>,
    To: IntoInput<Value>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_setvariableto)
            .add_input_into_arg("VALUE", to)
            .add_into_field("VARIABLE", var),
    )
}

pub fn change_var_by<Var, By>(var: Var, by: By) -> StackBlock
where
    Var: IntoField<Variable>,
    By: IntoInput<Value>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_changevariableby)
            .add_input_into_arg("VALUE", by)
            .add_into_field("VARIABLE", var),
    )
}

pub fn show_var<Var>(var: Var) -> StackBlock
where
    Var: IntoField<Variable>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_showvariable).add_into_field("VARIABLE", var),
    )
}

pub fn hide_var<Var>(var: Var) -> StackBlock
where
    Var: IntoField<Variable>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_hidevariable).add_into_field("VARIABLE", var),
    )
}

pub fn add_to_list<Item>(item: Item) -> StackBlock
where
    Item: IntoInput<Value>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_addtolist).add_input_into_arg("ITEM", item),
    )
}

pub fn delete_in_list<L, Idx>(list: L, idx: Idx) -> StackBlock
where
    Idx: IntoInput<Integer>,
    L: IntoField<List>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_deleteoflist)
            .add_input_into_arg("INDEX", idx)
            .add_into_field("LIST", list),
    )
}

pub fn delete_all_in_list<L>(list: L) -> StackBlock
where
    L: IntoField<List>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_deletealloflist).add_into_field("LIST", list),
    )
}

pub fn insert_in_list<L, Idx, Item>(list: L, idx: Idx, item: Item) -> StackBlock
where
    Idx: IntoInput<Integer>,
    L: IntoField<List>,
    Item: IntoInput<Value>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_insertatlist)
            .add_input_into_arg("INDEX", idx)
            .add_input_into_arg("ITEM", item)
            .add_into_field("LIST", list),
    )
}

pub fn replace_in_list<L, Idx, Item>(list: L, idx: Idx, item: Item) -> StackBlock
where
    Idx: IntoInput<Integer>,
    L: IntoField<List>,
    Item: IntoInput<Value>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_replaceitemoflist)
            .add_input_into_arg("INDEX", idx)
            .add_input_into_arg("ITEM", item)
            .add_into_field("LIST", list),
    )
}

pub fn item_in_list<L, Idx>(list: L, idx: Idx) -> JustReporter<Value>
where
    L: IntoField<List>,
    Idx: IntoInput<Integer>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_itemoflist)
            .add_input_into_arg("INDEX", idx)
            .add_into_field("LIST", list),
    )
    .into()
}

pub fn count_of_item_in_list<L, Item>(list: L, item: Item) -> JustReporter<Integer>
where
    L: IntoField<List>,
    Item: IntoInput<Value>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_itemoflist)
            .add_input_into_arg("ITEM", item)
            .add_into_field("LIST", list),
    )
    .into()
}

pub fn length_of_list<L>(list: L) -> JustReporter<Integer>
where
    L: IntoField<List>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_lengthoflist).add_into_field("LIST", list),
    )
    .into()
}

pub fn list_contains<L, Item>(list: L, item: Item) -> JustReporter<Bool>
where
    L: IntoField<List>,
    Item: IntoInput<Value>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_listcontainsitem)
            .add_input_into_arg("ITEM", item)
            .add_into_field("LIST", list),
    )
    .into()
}

pub fn show_list<L>(list: L) -> StackBlock
where
    L: IntoField<List>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_showlist).add_into_field("LIST", list),
    )
}

pub fn hide_list<L>(list: L) -> StackBlock
where
    L: IntoField<List>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_hidelist).add_into_field("LIST", list),
    )
}
