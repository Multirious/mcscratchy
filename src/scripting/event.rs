use crate::opcode::PrimaryOpCode;

use super::{arg::*, script_builder::BlockBuilder, typed_script_builder::*};

/// The script start when then green flag is clicked
pub fn when_flag_clicked() -> HatBlock {
    TypedStackBuilder::start(BlockBuilder::new(PrimaryOpCode::event_whenflagclicked))
}
