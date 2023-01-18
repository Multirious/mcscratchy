use rs_sb3::block::BlockInputValue;

use crate::derive_everything;

use super::{
    script_builder::StackBuilder,
    typed_script_builder::{AdvReporter, StackableSide, TypedStackBuilder},
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
    pub struct Broadcast;
    pub struct Variable;
    pub struct List;
    pub struct Bool;

    pub struct Value; // Enum of text and number
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

impl<T, S, E> IntoArg<T> for AdvReporter<T, S, E> {
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
    Value => f64,
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
