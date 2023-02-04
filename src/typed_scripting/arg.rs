use rs_sb3::block::BlockInputValue;

use crate::scripting::script_builder::{BlockFieldBuilder, BlockInputBuilder, FieldKind};

use super::script_builder::{Reporter, StackableSide, TypedStackBuilder};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Number;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PositiveNumber;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct PositiveInteger;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Integer;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Float;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Angle;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Color;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Text;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Bool;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Value; // Could be type text or number

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Stack;

// this is for IntoFieldArg when there's no id field
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NoRef;
// this is for IntoFieldArg when there's an id field but i've never seen it has id
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NoRefMaybe;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Broadcast;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Variable;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct List;

// Input =========================================================================
pub trait IntoInput<T> {
    fn into_input(self) -> BlockInputBuilder;
}

impl<T, S, E> IntoInput<T> for Reporter<T, S, E> {
    fn into_input(self) -> BlockInputBuilder {
        BlockInputBuilder::stack(self.0.into_untyped())
    }
}

macro_rules! into_arg_basic_impl {
    ($($arg:ty => $enum:ident => $from_ty:ty),*) => {
        $(
            impl IntoInput<$arg> for $from_ty {
                fn into_input(self) -> BlockInputBuilder {
                    BlockInputBuilder::value(BlockInputValue::$enum { value: self.into() })
                }
            }
        )*
    }
}

into_arg_basic_impl! {
    Number => Number => i64,
    Number => Number => f64,
    Text => String => String,
    Value => String => String,
    Value => Number => i64,
    Value => Number => f64
}

impl IntoInput<Bool> for bool {
    fn into_input(self) -> BlockInputBuilder {
        BlockInputBuilder::value(BlockInputValue::Number {
            value: if self { 1.into() } else { 0.into() },
        })
    }
}

impl IntoInput<Text> for &str {
    fn into_input(self) -> BlockInputBuilder {
        BlockInputBuilder::value(BlockInputValue::String {
            value: self.to_owned().into(),
        })
    }
}

impl IntoInput<Value> for &str {
    fn into_input(self) -> BlockInputBuilder {
        BlockInputBuilder::value(BlockInputValue::String {
            value: self.to_owned().into(),
        })
    }
}

impl<E> IntoInput<Stack> for TypedStackBuilder<StackableSide, E> {
    fn into_input(self) -> BlockInputBuilder {
        BlockInputBuilder::stack(self.into_untyped())
    }
}

// Field ====================================================================
pub trait IntoField<T = NoRefMaybe> {
    fn into_field(self) -> BlockFieldBuilder;
}

impl<S: Into<String>> IntoField for S {
    fn into_field(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new(self.into())
    }
}

impl<S: Into<String>> IntoField<Broadcast> for S {
    fn into_field(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new_with_kind(self.into(), FieldKind::Broadcast)
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GlobalVar<S: Into<String>>(pub S);
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SpriteVar<S: Into<String>>(pub S);
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct GlobalList<S: Into<String>>(pub S);
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SpriteList<S: Into<String>>(pub S);

impl<S: Into<String>> IntoField<Variable> for GlobalVar<S> {
    fn into_field(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new_with_kind(self.0.into(), FieldKind::GlobalVariable)
    }
}
impl<S: Into<String>> IntoField<Variable> for SpriteVar<S> {
    fn into_field(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new_with_kind(self.0.into(), FieldKind::SpriteVariable)
    }
}

impl<S: Into<String>> IntoField<List> for GlobalList<S> {
    fn into_field(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new_with_kind(self.0.into(), FieldKind::GlobalList)
    }
}
impl<S: Into<String>> IntoField<List> for SpriteList<S> {
    fn into_field(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new_with_kind(self.0.into(), FieldKind::SpriteList)
    }
}
