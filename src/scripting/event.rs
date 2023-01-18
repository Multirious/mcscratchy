use crate::opcode::PrimaryOpCode;

use super::{arg::*, script_builder::BlockBuilder, typed_script_builder::*};

/// This script start when then green flag is clicked
pub fn when_flag_clicked() -> HatBlock {
    TypedStackBuilder::start_with_capacity(
        1,
        BlockBuilder::new(PrimaryOpCode::event_whenflagclicked),
    )
}
