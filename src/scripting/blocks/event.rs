use super::*;

/// The script start when then green flag is clicked
pub fn when_flag_clicked() -> HatBlock {
    TypedStackBuilder::start(BlockBuilder::new(PrimaryOpCode::event_whenflagclicked))
}
