use rs_sb3::block::BlockInputValue;

use crate::{derive_everything, uid::Uid};

use super::{
    script_builder::BlockFieldBuilder,
    script_builder::StackBuilder,
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

    pub struct Broadcast;
    pub struct Variable;
    pub struct List;

    pub struct Value; // Could be type text or number

    // this is for IntoFieldArg when there's no id field
    pub struct NoRef;
    // this is for IntoFieldArg when there's an id field but i've never seen it has id
    pub struct NoRefMaybe;
}

pub trait IntoArgType {}
pub trait IntoFieldArgType {}

macro_rules! empty_trait_impl {
    ($($traitty:ty {$($tytoimpl:ty)*})*) => {
        $(
            $(
                impl $traitty for $tytoimpl {}
            )*
        )*
    }
}

empty_trait_impl! {
    IntoArgType {
        Number PositiveNumber PositiveInteger
        Integer Float Angle Color Text Bool Value
    }
    IntoFieldArgType {
        NoRef
        Broadcast
        Variable
        List
    }
}

pub enum Arg {
    Value(BlockInputValue),
    Stack(StackBuilder),
}

pub trait IntoArg<T> {
    fn into_arg(self) -> Arg;
}

pub trait IntoStackArg {
    fn into_stack_arg(self) -> StackBuilder;
}

pub trait IntoFieldArg<T = NoRefMaybe> {
    fn into_field_arg(self) -> BlockFieldBuilder;
}

impl<T> IntoArg<T> for Arg {
    fn into_arg(self) -> Arg {
        self
    }
}

impl<E> IntoStackArg for TypedStackBuilder<StackableSide, E> {
    fn into_stack_arg(self) -> StackBuilder {
        self.into_untyped()
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
