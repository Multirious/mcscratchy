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
    Var: IntoFieldArg<Variable>,
    To: IntoArg<Value>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_setvariableto)
            .add_input_into_arg("VALUE", to)
            .add_into_field("VARIABLE", var),
    )
}

pub fn change_var_by<Var, By>(var: Var, by: By) -> StackBlock
where
    Var: IntoFieldArg<Variable>,
    By: IntoArg<Value>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_changevariableby)
            .add_input_into_arg("VALUE", by)
            .add_into_field("VARIABLE", var),
    )
}

pub fn show_var<Var>(var: Var) -> StackBlock
where
    Var: IntoFieldArg<Variable>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_showvariable).add_into_field("VARIABLE", var),
    )
}

pub fn hide_var<Var>(var: Var) -> StackBlock
where
    Var: IntoFieldArg<Variable>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_hidevariable).add_into_field("VARIABLE", var),
    )
}

pub fn add_to_list<Item>(item: Item) -> StackBlock
where
    Item: IntoArg<Value>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_addtolist).add_input_into_arg("ITEM", item),
    )
}

pub fn delete_in_list<L, Idx>(list: L, idx: Idx) -> StackBlock
where
    Idx: IntoArg<Integer>,
    L: IntoFieldArg<List>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_deleteoflist)
            .add_input_into_arg("INDEX", idx)
            .add_into_field("LIST", list),
    )
}

pub fn delete_all_in_list<L>(list: L) -> StackBlock
where
    L: IntoFieldArg<List>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_deletealloflist).add_into_field("LIST", list),
    )
}

pub fn insert_in_list<L, Idx, Item>(list: L, idx: Idx, item: Item) -> StackBlock
where
    Idx: IntoArg<Integer>,
    L: IntoFieldArg<List>,
    Item: IntoArg<Value>,
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
    Idx: IntoArg<Integer>,
    L: IntoFieldArg<List>,
    Item: IntoArg<Value>,
{
    TypedStackBuilder::start(
        BlockNormalBuilder::new(PrimaryOpCode::data_replaceitemoflist)
            .add_input_into_arg("INDEX", idx)
            .add_input_into_arg("ITEM", item)
            .add_into_field("LIST", list),
    )
}
