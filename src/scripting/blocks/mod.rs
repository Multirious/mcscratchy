//! Blocks that ended with menu is a visual menu in scratch.
//! It's not required to be use in function argument in here
//! which might introduce some invalid argument to function that normally requires a menu in the editor.
//!
//! Some reserved input (you shouldn't try to name anything with thing in this list):
//!  - "_random_"
//!  - "_mouse_"
//!

use super::{arg::*, script_builder::BlockNormalBuilder, typed_script_builder::*};
use crate::opcode::PrimaryOpCode;

pub mod control;
pub mod event;
pub mod looks;
pub mod motion;
pub mod operator;
pub mod sensing;
pub mod sound;

pub mod data;

pub use {control::*, data::*, event::*, looks::*, motion::*, operator::*, sensing::*, sound::*};
