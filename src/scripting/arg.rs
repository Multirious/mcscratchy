use rs_sb3::block::{BlockInputValue, ShadowInputType};

use crate::derive_everything;

use super::{
    script_builder::BlockFieldBuilder,
    script_builder::{BlockInputBuilder, FieldKind, StackBuilder, StackOrValue},
    typed_script_builder::{Reporter, StackableSide, TypedStackBuilder},
};

derive_everything! {
    pub struct Number;
    pub struct PositiveNumber;
    pub struct PositiveInteger;
    pub struct Integer;
    pub struct Float;
    pub struct Angle;
    pub struct Color;
    pub struct Text;
    pub struct Bool;

    pub struct Value; // Could be type text or number

    pub struct Stack;

    // this is for IntoFieldArg when there's no id field
    pub struct NoRef;
    // this is for IntoFieldArg when there's an id field but i've never seen it has id
    pub struct NoRefMaybe;

    pub struct Broadcast;
    pub struct Variable;
    pub struct List;
}

// Arg =========================================================================
#[derive(Debug, Clone, PartialEq)]
pub enum Arg {
    Value(BlockInputValue),
    Stack(StackBuilder, Option<BlockInputValue>),
}

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

derive_everything! {
    pub struct GlobalVar<S: Into<String>>(pub S);
    pub struct SpriteVar<S: Into<String>>(pub S);
    pub struct GlobalList<S: Into<String>>(pub S);
    pub struct SpriteList<S: Into<String>>(pub S);
}

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
