//! Blocks that ended with menu is a visual menu in scratch.
//! It's not required to be use in function argument in here
//! which might introduce some invalid argument to function that normally requires a menu in the editor.
//!
//! Some reserved input (you shouldn't try to name anything with thing in this list):
//!  - "_random_"
//!  - "_mouse_"
//!

use super::script_builder::{
    BlockFieldBuilder, BlockInputBuilder, BlockNormalBuilder, StackBuilder,
};
use crate::opcode::PrimaryOpCode;
use crate::scripting::script_builder::BlockVarListBuilder;
use rs_sb3::block::{BlockMutation, BlockMutationEnum, ListOrVariable};

// Control
// Event
// Looks
// Motion
// Operator
// Sensing
// Sound
// Data

type BFB = BlockFieldBuilder;
type BIB = BlockInputBuilder;

// Control =====================================================================
pub fn wait(duration: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::control_wait).add_input("DURATION", duration),
    )
}

pub fn repeat(times: BIB, to_repeat: Option<BIB>) -> StackBuilder {
    StackBuilder::start({
        let mut b =
            BlockNormalBuilder::new(PrimaryOpCode::control_repeat).add_input("TIMES", times);
        if let Some(to_repeat) = to_repeat {
            b = b.add_input("SUBSTACK", to_repeat);
        }
        b
    })
}

pub fn forever(to_repeat: Option<BIB>) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(PrimaryOpCode::control_forever);
        if let Some(to_repeat) = to_repeat {
            b = b.add_input("SUBSTACK", to_repeat);
        }
        b
    })
}

pub fn if_(condition: BIB, if_true: Option<BIB>) -> StackBuilder {
    StackBuilder::start({
        let mut b =
            BlockNormalBuilder::new(PrimaryOpCode::control_if).add_input("CONDITION", condition);
        if let Some(if_true) = if_true {
            b = b.add_input("SUBSTACK", if_true);
        }
        b
    })
}

pub fn if_else(condition: BIB, if_true: Option<BIB>, if_false: Option<BIB>) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(PrimaryOpCode::control_if_else)
            .add_input("CONDITION", condition);
        if let Some(if_true) = if_true {
            b = b.add_input("SUBSTACK", if_true);
        }
        if let Some(if_false) = if_false {
            b = b.add_input("SUBSTACK2", if_false);
        }
        b
    })
}

pub fn wait_until(condition: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::control_wait_until)
            .add_input("CONDITION", condition),
    )
}

pub fn repeat_until(condition: BIB, to_repeat: Option<BIB>) -> StackBuilder {
    StackBuilder::start({
        let mut b = BlockNormalBuilder::new(PrimaryOpCode::control_if_else)
            .add_input("CONDITION", condition);
        if let Some(to_repeat) = to_repeat {
            b = b.add_input("SUBSTACK", to_repeat);
        }
        b
    })
}

/// `stop_option` Accepts:
///  - "this script" and `has_next` should be `false`
///  - "other scripts in sprite" and `has_next` should be `true`
///  - "all" and `has_next` should be `false`
pub fn stop(stop_option: BFB, has_next: bool) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::control_stop)
            .add_field("STOP_OPTION", stop_option)
            .mutation(BlockMutation {
                tag_name: "mutation".to_owned(),
                children: vec![],
                mutation_enum: BlockMutationEnum::ControlStop { hasnext: has_next },
            }),
    )
}

pub fn when_i_start_as_a_clone() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(
        PrimaryOpCode::control_start_as_clone,
    ))
}

/// Accepts:
///  - Sprite name
pub fn create_clone_of(sprite: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::control_create_clone_of)
            .add_input("CLONE_OPTION", sprite),
    )
}

/// Uses as argument to [`create_clone_of`]
/// Accepts:
///  - Sprite name
pub fn create_clone_of_menu(sprite: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::control_create_clone_of)
            .add_field("CLONE_OPTION", sprite)
            .shadow(true),
    )
}

pub fn delete_this_clone() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(
        PrimaryOpCode::control_delete_this_clone,
    ))
}

// Event =======================================================================
pub fn when_flag_clicked() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(
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
pub fn when_key_pressed(key: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::event_whenkeypressed).add_field("KEY_OPTION", key),
    )
}

pub fn when_this_sprite_clicked() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(
        PrimaryOpCode::event_whenthisspriteclicked,
    ))
}

/// Accepts:
///  - Backdrop name
pub fn when_backdrop_switches_to(backdrop: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::event_whenbackdropswitchesto)
            .add_field("BACKDROP", backdrop),
    )
}

/// Accepts:
/// - "LOUDNESS"
/// - "TIMER"
pub fn when_greater_than(variable: BFB, value: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::event_whengreaterthan)
            .add_input("VALUE", value)
            .add_field("WHENGREATERTHANMENU", variable),
    )
}

pub fn when_broadcast_received(broadcast: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::event_whenbroadcastreceived)
            .add_field("BROADCAST_OPTION", broadcast),
    )
}

pub fn broadcast(broadcast: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::event_broadcast)
            .add_input("BROADCAST_INPUT", broadcast),
    )
}

pub fn broadcast_and_wait(broadcast: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::event_broadcastandwait)
            .add_input("BROADCAST_INPUT", broadcast),
    )
}

// Looks =======================================================================
pub fn think(message: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_think).add_input("MESSAGE", message),
    )
}

pub fn think_for_secs(message: BIB, secs: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_thinkforsecs)
            .add_input("MESSAGE", message)
            .add_input("SECS", secs),
    )
}

pub fn say(message: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_say).add_input("MESSAGE", message),
    )
}

pub fn say_for_secs(message: BIB, secs: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_sayforsecs)
            .add_input("MESSAGE", message)
            .add_input("SECS", secs),
    )
}

/// Accepts:
///  - Costume name
pub fn switch_costume_to(costume: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_switchcostumeto).add_input("COSTUME", costume),
    )
}

/// Uses as argument to [`switch_costume_to`]
/// Accepts:
///  - Costume name
pub fn costume_menu(costume: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_costume)
            .add_field("COSTUME", costume)
            .shadow(true),
    )
}

pub fn next_costume() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::looks_nextcostume))
}

/// Accepts:
///  - Costume name
pub fn switch_backdrop_to(backdrop: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_switchbackdropto)
            .add_input("BACKDROP", backdrop),
    )
}

/// Uses as argument to [`switch_backdrop_to`]
/// Accepts:
///  - Backdrop name
pub fn backdrop_menu(backdrop: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_backdrops)
            .add_field("BACKDROP", backdrop)
            .shadow(true),
    )
}

pub fn next_backdrop() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::looks_nextbackdrop))
}

pub fn change_size_by(by: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_changesizeby).add_input("CHANGE", by),
    )
}

pub fn set_size_to(to: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_setsizeto).add_input("SIZE", to),
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
pub fn change_looks_effect_by(effect: BFB, by: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_changeeffectby)
            .add_input("CHANGE", by)
            .add_field("EFFECT", effect),
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
pub fn set_looks_effect_to(effect: BFB, to: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_seteffectto)
            .add_input("TO", to)
            .add_field("EFFECT", effect),
    )
}

pub fn clear_graphic_effects() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(
        PrimaryOpCode::looks_cleargraphiceffects,
    ))
}

pub fn show() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::looks_show))
}

pub fn hide() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::looks_hide))
}

/// Accepts:
///  - "front"
///  - "back"
pub fn go_to_layer(layer: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_gotofrontback).add_field("FRONT_BACK", layer),
    )
}

/// `layer` Accepts:
///  - "foward"
///  - "backward"
pub fn change_layer(layer: BFB, by: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_goforwardbackwardlayers)
            .add_input("NUM", by)
            .add_field("FORWARD_BACKWORD", layer),
    )
}

/// Accepts:
/// - "number"
/// - "name"
pub fn costume(return_type: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_costumenumbername)
            .add_field("NUMBER_NAME", return_type),
    )
}

/// Accepts:
/// - "number"
/// - "name"
pub fn backdrop(return_type: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::looks_backdropnumbername)
            .add_field("NUMBER_NAME", return_type),
    )
}

pub fn size() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::looks_size))
}

// Motion ======================================================================
pub fn move_steps(steps: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_movesteps).add_input("STEPS", steps),
    )
}

pub fn turn_right(degress: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_turnright).add_input("DEGREES", degress),
    )
}

pub fn turn_left(degress: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_turnleft).add_input("DEGREES", degress),
    )
}

/// Accepts:
///  - Sprite name
///  - "_mouse_" go to mouse position
///  - "_random_" go to random position
pub fn go_to(to: BIB) -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::motion_goto).add_input("TO", to))
}

/// Uses as argument to [`goto`]
/// Accepts:
///  - Sprite name
///  - "_mouse_" go to mouse position
///  - "_random_" go to random position
pub fn go_to_menu(to: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_goto_menu)
            .add_field("TO", to)
            .shadow(true),
    )
}

pub fn goto_xy(x: BIB, y: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_gotoxy)
            .add_input("X", x)
            .add_input("Y", y),
    )
}

/// Accepts:
///  - Sprite name
///  - "_mouse_" glide to mouse position
///  - "_random_" glide to random position
pub fn glide_to(duration_secs: BIB, to: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_gotoxy)
            .add_input("SECS", duration_secs)
            .add_input("TO", to),
    )
}

/// Uses as an argument for [`glide_to`] in `to`
/// Accepts:
///  - Sprite name
///  - "_mouse_" glide to mouse position
///  - "_random_" glide to random position
pub fn glide_to_menu(to: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_glideto_menu)
            .add_field("TO", to)
            .shadow(true),
    )
}

pub fn glide_to_xy(dur: BIB, x: BIB, y: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_glidesecstoxy)
            .add_input("SECS", dur)
            .add_input("X", x)
            .add_input("Y", y),
    )
}

pub fn point_in_direction(direction: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_pointindirection)
            .add_input("DIRECTION", direction),
    )
}

/// Accepts:
///  - Sprite name
///  - "_mouse_" glide to mouse position
pub fn point_towards(towards: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_pointtowards).add_input("TOWARDS", towards),
    )
}

/// Uses as an argument for [`point_towards`]
/// Accepts:
///  - Sprite name
///  - "_mouse_" glide to mouse position
pub fn point_towards_menu(towards: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_pointtowards_menu)
            .add_field("TOWARDS", towards)
            .shadow(true),
    )
}

pub fn set_x(x: BIB) -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::motion_setx).add_input("X", x))
}

pub fn set_y(y: BIB) -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::motion_setx).add_input("Y", y))
}

pub fn change_x_by(by: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_changexby).add_input("DX", by),
    )
}

pub fn change_y_by(by: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_changeyby).add_input("DY", by),
    )
}

pub fn if_on_edge_bounce() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(
        PrimaryOpCode::motion_ifonedgebounce,
    ))
}

/// Accepts:
///  - "left-right"
///  - "don't rotate"
///  - "all around"
pub fn set_rotation_style(style: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::motion_setrotationstyle).add_field("STYLE", style),
    )
}

pub fn direction() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::motion_direction))
}

pub fn y_position() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::motion_yposition))
}

pub fn x_position() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::motion_xposition))
}

// Operators ===================================================================
pub fn add(lhs: BIB, rhs: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_add)
            .add_input("NUM1", lhs)
            .add_input("NUM2", rhs),
    )
}

pub fn sub(lhs: BIB, rhs: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_subtract)
            .add_input("NUM1", lhs)
            .add_input("NUM2", rhs),
    )
}

pub fn mul(lhs: BIB, rhs: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_multiply)
            .add_input("NUM1", lhs)
            .add_input("NUM2", rhs),
    )
}

pub fn div(lhs: BIB, rhs: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_divide)
            .add_input("NUM1", lhs)
            .add_input("NUM2", rhs),
    )
}

pub fn random(from: BIB, to: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_random)
            .add_input("FROM", from)
            .add_input("TO", to),
    )
}

pub fn less_than(lhs: BIB, rhs: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_lt)
            .add_input("OPERAND1", lhs)
            .add_input("OPERAND2", rhs),
    )
}

pub fn greater_than(lhs: BIB, rhs: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_gt)
            .add_input("OPERAND1", lhs)
            .add_input("OPERAND2", rhs),
    )
}

pub fn equals(lhs: BIB, rhs: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_equals)
            .add_input("OPERAND1", lhs)
            .add_input("OPERAND2", rhs),
    )
}

pub fn and(a: BIB, b: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_and)
            .add_input("OPERAND1", a)
            .add_input("OPERAND2", b),
    )
}

pub fn or(a: BIB, b: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_or)
            .add_input("OPERAND1", a)
            .add_input("OPERAND2", b),
    )
}

pub fn not(val: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_or).add_input("OPERAND", val),
    )
}

pub fn join(a: BIB, b: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_join)
            .add_input("STRING1", a)
            .add_input("STRING2", b),
    )
}

pub fn letter_of(idx: BIB, text: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_letter_of)
            .add_input("LETTER", idx)
            .add_input("STRING", text),
    )
}

pub fn length_of(text: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_length).add_input("STRING", text),
    )
}

pub fn contains(text: BIB, contains: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_contains)
            .add_input("STRING1", text)
            .add_input("STRING2", contains),
    )
}

pub fn modulo(dividend: BIB, divisor: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_mod)
            .add_input("NUM1", dividend)
            .add_input("NUM2", divisor),
    )
}

pub fn round(val: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_round).add_input("NUM", val),
    )
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
pub fn math_op(op: BFB, val: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::operator_mathop)
            .add_input("NUM", val)
            .add_field("OPERATOR", op),
    )
}

// Sensing =====================================================================

/// Accepts:
///  - Sprite name
///  - "_mouse_"
///  - "_edge_"
pub fn touching(what: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_touchingobject)
            .add_input("TOUCHINGOBJECTMENU", what),
    )
}

/// Uses as argument to [`touching`]
/// Accepts:
///  - Sprite name
///  - "_mouse_"
///  - "_edge_"
pub fn touching_menu(what: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_touchingobjectmenu)
            .add_field("TOUCHINGOBJECTMENU", what)
            .shadow(true),
    )
}

pub fn touching_color(color: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_touchingcolor).add_input("COLOR", color),
    )
}

pub fn color_touching_color(color_a: BIB, color_b: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_coloristouchingcolor)
            .add_input("COLOR", color_a)
            .add_input("COLOR2", color_b),
    )
}

/// Accepts:
///  - Sprite name
///  - "_mouse_"
pub fn distance_to(what: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_coloristouchingcolor)
            .add_input("DISTANCETOMENU", what),
    )
}

/// Uses as argument to [`distance_to`]
/// Accepts:
///  - Sprite name
///  - "_mouse_"
pub fn distance_to_menu(what: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_coloristouchingcolor)
            .add_field("DISTANCETOMENU", what)
            .shadow(true),
    )
}

pub fn ask_and_wait(prompt_message: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_askandwait)
            .add_input("QUESTION", prompt_message),
    )
}

pub fn answer() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sensing_answer))
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
pub fn key_pressed(key: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_keypressed).add_input("KEY_OPTION", key),
    )
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
pub fn key_menu(key: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_keyoptions)
            .add_input("KEY_OPTION", key)
            .shadow(true),
    )
}

pub fn mouse_down() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sensing_mousedown))
}

pub fn mouse_x() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sensing_mousex))
}

/// Accepts:
///  - "not draggable"
///  - "draggable"
pub fn set_drag_mode(mode: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_setdragmode).add_field("DRAG_MODE", mode),
    )
}

pub fn loudness() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sensing_loudness))
}

pub fn timer() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sensing_timer))
}

pub fn reset_timer() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sensing_resettimer))
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
pub fn var_of(var: BFB, what: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_of)
            .add_input("OBJECT", what)
            .add_field("PROPERTY", var),
    )
}

/// Uses as argument to [`var_of`]
/// `what` Accepts:
///   - Sprite name
///   - "_stage_"
pub fn var_of_object_menu(what: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_of_object_menu)
            .add_field("OBJECT", what)
            .shadow(true),
    )
}

/// Accepts:
///  - "SECOND"
///  - "MINUTE"
///  - "HOUR"
///  - "DAYOFWEEK"
///  - "DATE"
///  - "MONTH"
///  - "YEAR"
pub fn current_datetime(format: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sensing_current).add_field("CURRENTMENU", format),
    )
}

pub fn days_since_2000() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(
        PrimaryOpCode::sensing_dayssince2000,
    ))
}

pub fn username() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sensing_username))
}

// Sound =======================================================================

/// Accepts:
///  - Sound name
pub fn play_sound_until_done(sound: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_playuntildone).add_input("SOUND_MENU", sound),
    )
}

/// Accepts:
///  - Sound name
pub fn play_sound(sound: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_play).add_input("SOUND_MENU", sound),
    )
}

/// Uses as argument to [`play_sound_until_done`] and [`play_sound`]
/// Accepts:
///  - Sound name
pub fn sound_menu(sound: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_sounds_menu)
            .add_field("SOUND_MENU", sound)
            .shadow(true),
    )
}

pub fn stop_all_sound() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sound_stopallsounds))
}

/// Accepts:
///  - "PITCH"
///  - "PAN"
pub fn change_sound_effect_by(effect: BFB, by: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_changeeffectby)
            .add_input("VALUE", by)
            .add_field("EFFECT", effect),
    )
}

/// Accepts:
///  - "PITCH"
///  - "PAN"
pub fn set_sound_effect_to(effect: BFB, to: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_seteffectto)
            .add_input("VALUE", to)
            .add_field("EFFECT", effect),
    )
}

pub fn clear_sound_effects() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sound_cleareffects))
}

pub fn set_volume_to(volume: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_setvolumeto).add_input("VOLUME", volume),
    )
}

pub fn change_volume_by(by: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::sound_changeeffectby).add_input("VOLUME", by),
    )
}

pub fn volume() -> StackBuilder {
    StackBuilder::start(BlockNormalBuilder::new(PrimaryOpCode::sound_volume))
}

// Data ========================================================================
pub fn sprite_var<S: Into<String>>(name: S) -> StackBuilder {
    StackBuilder::start_varlist(BlockVarListBuilder::sprite(ListOrVariable::Variable, name))
}

pub fn sprite_list<S: Into<String>>(name: S) -> StackBuilder {
    StackBuilder::start_varlist(BlockVarListBuilder::sprite(ListOrVariable::List, name))
}

pub fn global_var<S: Into<String>>(name: S) -> StackBuilder {
    StackBuilder::start_varlist(BlockVarListBuilder::global(ListOrVariable::Variable, name))
}

pub fn global_list<S: Into<String>>(name: S) -> StackBuilder {
    StackBuilder::start_varlist(BlockVarListBuilder::global(ListOrVariable::List, name))
}

pub fn set_var_to(var: BFB, to: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_setvariableto)
            .add_input("VALUE", to)
            .add_field("VARIABLE", var),
    )
}

pub fn change_var_by(var: BFB, by: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_changevariableby)
            .add_input("VALUE", by)
            .add_field("VARIABLE", var),
    )
}

pub fn show_var(var: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_showvariable).add_field("VARIABLE", var),
    )
}

pub fn hide_var(var: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_hidevariable).add_field("VARIABLE", var),
    )
}

pub fn add_to_list(item: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_addtolist).add_input("ITEM", item),
    )
}

pub fn delete_in_list(list: BFB, idx: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_deleteoflist)
            .add_input("INDEX", idx)
            .add_field("LIST", list),
    )
}

pub fn delete_all_in_list(list: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_deletealloflist).add_field("LIST", list),
    )
}

pub fn insert_in_list(list: BFB, idx: BIB, item: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_insertatlist)
            .add_input("INDEX", idx)
            .add_input("ITEM", item)
            .add_field("LIST", list),
    )
}

pub fn replace_in_list(list: BFB, idx: BIB, item: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_replaceitemoflist)
            .add_input("INDEX", idx)
            .add_input("ITEM", item)
            .add_field("LIST", list),
    )
}

pub fn item_in_list(list: BFB, idx: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_itemoflist)
            .add_input("INDEX", idx)
            .add_field("LIST", list),
    )
}

pub fn count_of_item_in_list(list: BFB, item: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_itemoflist)
            .add_input("ITEM", item)
            .add_field("LIST", list),
    )
}

pub fn length_of_list(list: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_lengthoflist).add_field("LIST", list),
    )
}

pub fn list_contains(list: BFB, item: BIB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_listcontainsitem)
            .add_input("ITEM", item)
            .add_field("LIST", list),
    )
}

pub fn show_list(list: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_showlist).add_field("LIST", list),
    )
}

pub fn hide_list(list: BFB) -> StackBuilder {
    StackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_hidelist).add_field("LIST", list),
    )
}
