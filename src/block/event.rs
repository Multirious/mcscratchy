use super::*;

/// This script start when then green flag is clicked
pub fn event_when_flag_clicked() -> StackBuilder<UnlinkableSide, LinkableSide> {
    StackBuilder::start_with_capacity(BlockBuilder::new(PrimaryOpCode::event_whenflagclicked), 1)
}
