use rs_sb3::block::BlockInputValue;

use crate::derive_everything;

use super::{
    script_builder::BlockFieldBuilder,
    script_builder::{BlockInputBuilder, FieldKind, StackBuilder},
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
    Stack(StackBuilder),
}

{
    // am here
}

pub trait IntoArg<T> {
    fn into_arg(self) -> Arg;
    fn into_block_input_builder(self) -> BlockInputBuilder {
        match self.into_arg() {
            Arg::Value(v) => self.add_input(
                key.into(),
                BlockInputBuilder::new(ShadowInputType::Shadow).input_some(StackOrValue::Value(v)),
            ),
            Arg::Stack(b) => self.add_input(
                key.into(),
                BlockInputBuilder::new(ShadowInputType::NoShadow)
                    .input_some(StackOrValue::Stack(b)),
            ),
        }
    }
}

impl<T> IntoArg<T> for Arg {
    fn into_arg(self) -> Arg {
        self
    }
}

impl<T, S, E> IntoArg<T> for Reporter<T, S, E> {
    fn into_arg(self) -> Arg {
        Arg::Stack(self.0.into_untyped())
    }
}

macro_rules! into_arg_basic_impl {
    ($($arg_ty:ty => $from_ty:ty),*) => {
        $(
            impl IntoArg<$arg_ty> for $from_ty {
                fn into_arg(self) -> Arg {
                    Arg::Value(BlockInputValue::Number { value: self.into() })
                }
            }
        )*
    }
}

into_arg_basic_impl! {
    Number => i64,
    Number => f64,
    Text => String,
    Value => String,
    Value => i64,
    Value => f64
}

impl IntoArg<Bool> for bool {
    fn into_arg(self) -> Arg {
        Arg::Value(BlockInputValue::Number { value: 1.into() })
    }
}

impl IntoArg<Text> for &str {
    fn into_arg(self) -> Arg {
        Arg::Value(BlockInputValue::Number {
            value: self.to_owned().into(),
        })
    }
}

impl IntoArg<Value> for &str {
    fn into_arg(self) -> Arg {
        Arg::Value(BlockInputValue::Number {
            value: self.to_owned().into(),
        })
    }
}

// StackArg ====================================================================
pub trait IntoStackArg {
    fn into_stack_arg(self) -> StackBuilder;
}

impl<E> IntoStackArg for TypedStackBuilder<StackableSide, E> {
    fn into_stack_arg(self) -> StackBuilder {
        self.into_untyped()
    }
}

impl IntoStackArg for StackBuilder {
    fn into_stack_arg(self) -> StackBuilder {
        self
    }
}

// FieldArg ====================================================================
pub trait IntoFieldArg<T = NoRefMaybe> {
    fn into_field_arg(self) -> BlockFieldBuilder;
}

impl IntoFieldArg for &str {
    fn into_field_arg(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new(self.into())
    }
}

impl IntoFieldArg for String {
    fn into_field_arg(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new(self)
    }
}

impl IntoFieldArg<Broadcast> for String {
    fn into_field_arg(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new_with_kind(self, FieldKind::Broadcast)
    }
}

impl IntoFieldArg<Broadcast> for &str {
    fn into_field_arg(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new_with_kind(self.into(), FieldKind::Broadcast)
    }
}

pub struct GlobalVar<S: Into<String>>(pub S);
pub struct SpriteVar<S: Into<String>>(pub S);
pub struct GlobalList<S: Into<String>>(pub S);
pub struct SpriteList<S: Into<String>>(pub S);

impl<S: Into<String>> IntoFieldArg<Variable> for GlobalVar<S> {
    fn into_field_arg(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new_with_kind(self.0.into(), FieldKind::GlobalVariable)
    }
}
impl<S: Into<String>> IntoFieldArg<Variable> for SpriteVar<S> {
    fn into_field_arg(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new_with_kind(self.0.into(), FieldKind::SpriteVariable)
    }
}

impl<S: Into<String>> IntoFieldArg<List> for GlobalList<S> {
    fn into_field_arg(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new_with_kind(self.0.into(), FieldKind::GlobalList)
    }
}
impl<S: Into<String>> IntoFieldArg<List> for SpriteList<S> {
    fn into_field_arg(self) -> BlockFieldBuilder {
        BlockFieldBuilder::new_with_kind(self.0.into(), FieldKind::SpriteList)
    }
}
