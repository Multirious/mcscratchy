use super::*;

pub fn when_flag_clicked() -> HatBlock {
    TypedStackBuilder::start(BlockBuilder::new(PrimaryOpCode::event_whenflagclicked))
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
pub fn when_key_pressed<K>(key: K) -> HatBlock
where
    K: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::event_whenkeypressed)
            .add_field("KEY_OPTION", key.into_field_arg_with_id(None)),
    )
}

/// Accepts:
///  - Backdrop name
pub fn when_backdrop_switches_to<BD>(backdrop: BD) -> HatBlock
where
    BD: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::event_whenbackdropswitchesto)
            .add_field("BACKDROP", backdrop.into_field_arg_with_id(None)),
    )
}

/// Accepts:
/// - "LOUDNESS"
/// - "TIMER"
pub fn when_greater_than<Var, Val>(variable: Var, value: Val) -> HatBlock
where
    Var: IntoFieldArg,
    Val: IntoArg<Number>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::event_whengreaterthan)
            .add_input_arg("VALUE", value.into_arg())
            .add_field("WHENGREATERTHANMENU", variable.into_field_arg_with_id(None)),
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
    Bcast: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::event_whenbroadcastreceived)
            .add_field("BROADCAST_OPTION", broadcast.into_field_arg_with_id(None)),
    )
}

pub fn broadcast<Bcast>(broadcast: Bcast) -> StackBlock
where
    Bcast: IntoArg<Broadcast>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::event_broadcast)
            .add_input_arg("BROADCAST_INPUT", broadcast.into_arg()),
    )
}

pub fn broadcast_and_wait<Bcast>(broadcast: Bcast) -> StackBlock
where
    Bcast: IntoArg<Broadcast>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::event_broadcastandwait)
            .add_input_arg("BROADCAST_INPUT", broadcast.into_arg()),
    )
}
