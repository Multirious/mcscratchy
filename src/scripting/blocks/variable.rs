use rs_sb3::block::ListOrVariable;

use crate::scripting::script_builder::BlockVarListBuilder;

use super::*;

pub fn sprite_var<Name>(name: Name) -> StackBlock
where
    Name: Into<String>,
{
    TypedStackBuilder::start_varlist(BlockVarListBuilder::sprite(ListOrVariable::Variable, name))
}

pub fn sprite_list<Name>(name: Name) -> StackBlock
where
    Name: Into<String>,
{
    TypedStackBuilder::start_varlist(BlockVarListBuilder::sprite(ListOrVariable::List, name))
}

pub fn global_var<Name>(name: Name) -> StackBlock
where
    Name: Into<String>,
{
    TypedStackBuilder::start_varlist(BlockVarListBuilder::global(ListOrVariable::Variable, name))
}

pub fn global_list<Name>(name: Name) -> StackBlock
where
    Name: Into<String>,
{
    TypedStackBuilder::start_varlist(BlockVarListBuilder::global(ListOrVariable::List, name))
}

pub fn set_var_to<Var, To>(var: Var, to: To) -> StackBlock
where
    Var: IntoFieldArg,
    To: IntoArg<Value>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_setvariableto)
            .add_input("VALUE", to)
            .add_field("VARIABLE", var.into_field_arg_with_id()),
    )
}
