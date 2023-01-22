use rs_sb3::block::BlockMutation;
use rs_sb3::block::BlockMutationEnum;

use super::*;

pub fn wait<Secs>(duration: Secs) -> StackBlock
where
    Secs: IntoArg<PositiveNumber>,
{
    asdfd
    // we're converting add_input and others to use the add input into don't forget
    // and after that we're add more blocks
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::control_wait).add_input_into_arg("DURATION", duration),
    )
}

pub fn repeat<Times, ToRepeat>(times: Times, to_repeat: Option<ToRepeat>) -> StackBlock
where
    Times: IntoArg<PositiveInteger>,
    ToRepeat: IntoStackArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::control_repeat)
            .add_input_into_arg("TIMES", times)
            .add_optional_into_input_stack("SUBSTACK", to_repeat),
    )
}

pub fn forever<ToRepeat>(to_repeat: Option<ToRepeat>) -> StackBlock
where
    ToRepeat: IntoStackArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::control_forever)
            .add_optional_into_input_stack("SUBSTACK", to_repeat),
    )
}

pub fn if_<Cond, IfT>(condition: Cond, if_true: Option<IfT>) -> StackBlock
where
    Cond: IntoArg<Bool>,
    IfT: IntoStackArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::control_if)
            .add_input_arg("CONDITION", condition.into_arg())
            .add_optional_input_stack("SUBSTACK", if_true.map(IntoStackArg::into_stack_arg)),
    )
}

pub fn if_else<Cond, IfT, IfF>(
    condition: Cond,
    if_true: Option<IfT>,
    if_false: Option<IfF>,
) -> StackBlock
where
    Cond: IntoArg<Bool>,
    IfT: IntoStackArg,
    IfF: IntoStackArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::control_if_else)
            .add_input_arg("CONDITION", condition.into_arg())
            .add_optional_input_stack("SUBSTACK", if_true.map(IntoStackArg::into_stack_arg))
            .add_optional_input_stack("SUBSTACK2", if_false.map(IntoStackArg::into_stack_arg)),
    )
}

pub fn wait_until<Cond>(condition: Cond) -> StackBlock
where
    Cond: IntoArg<Bool>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::control_wait_until)
            .add_input_arg("CONDITION", condition.into_arg()),
    )
}

pub fn repeat_until<Cond, ToRepeat>(condition: Cond, to_repeat: Option<ToRepeat>) -> StackBlock
where
    Cond: IntoArg<Bool>,
    ToRepeat: IntoStackArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::control_if_else)
            .add_input_arg("CONDITION", condition.into_arg())
            .add_optional_input_stack("SUBSTACK", to_repeat.map(IntoStackArg::into_stack_arg)),
    )
}

/// `stop_option` Accepts:
///  - "this script" and `has_next` should be `false`
///  - "other scripts in sprite" and `has_next` should be `true`
///  - "all" and `has_next` should be `false`
pub fn stop<Stop>(stop_option: Stop, has_next: bool) -> CapBlock
where
    Stop: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::control_stop)
            .add_field("STOP_OPTION", stop_option.into_field_arg_with_id(None))
            .mutation(BlockMutation {
                tag_name: "mutation".to_owned(),
                children: vec![],
                mutation_enum: BlockMutationEnum::ControlStop { hasnext: has_next },
            }),
    )
}

pub fn when_i_start_as_a_clone() -> HatBlock {
    TypedStackBuilder::start(BlockBuilder::new(PrimaryOpCode::control_start_as_clone))
}

/// Accepts:
///  - Sprite name
pub fn create_clone_of<Spr>(sprite: Spr) -> StackBlock
where
    Spr: IntoArg<Text>,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::control_create_clone_of)
            .add_input_arg("CLONE_OPTION", sprite.into_arg()),
    )
}

/// Uses as argument to [`create_clone_of`]
/// Accepts:
///  - Sprite name
pub fn create_clone_of_menu<Spr>(sprite: Spr) -> MenuReporter
where
    Spr: IntoFieldArg,
{
    TypedStackBuilder::start(
        BlockBuilder::new(PrimaryOpCode::control_create_clone_of)
            .add_field("CLONE_OPTION", sprite.into_field_arg_with_id(None)),
    )
    .into()
}

pub fn delete_this_clone() -> CapBlock {
    TypedStackBuilder::start(BlockBuilder::new(PrimaryOpCode::control_delete_this_clone))
}
