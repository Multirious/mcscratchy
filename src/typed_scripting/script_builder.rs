use std::marker::PhantomData;

use crate::scripting::script_builder::{
    BlockBuilder, BlockNormalBuilder, BlockVarListBuilder, StackBuilder,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct StackableSide;
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct UnstackableSide;

#[derive(Debug, Clone, PartialEq)]
pub struct Reporter<T, S, E>(pub TypedStackBuilder<S, E>, pub PhantomData<T>);

impl<T, S, E> Reporter<T, S, E> {
    pub fn new(typed_stack_builder: TypedStackBuilder<S, E>) -> Reporter<T, S, E> {
        Reporter(typed_stack_builder, PhantomData)
    }
}

impl<T, S, E> From<TypedStackBuilder<S, E>> for Reporter<T, S, E> {
    fn from(stb: TypedStackBuilder<S, E>) -> Self {
        Reporter::new(stb)
    }
}

pub type JustReporter<T> = Reporter<T, UnstackableSide, UnstackableSide>;
pub type HatBlock = TypedStackBuilder<UnstackableSide, StackableSide>;
pub type CapBlock = TypedStackBuilder<StackableSide, UnstackableSide>;
pub type StackBlock = TypedStackBuilder<StackableSide, StackableSide>;
pub type MenuReporter = JustReporter<super::arg::Text>;

#[derive(Debug, Clone, PartialEq)]
pub struct TypedStackBuilder<S, E> {
    stack_builder: StackBuilder,
    start: PhantomData<S>,
    end: PhantomData<E>,
}

impl<S, E> TypedStackBuilder<S, E> {
    pub fn start(block_builder: BlockNormalBuilder) -> TypedStackBuilder<S, E> {
        TypedStackBuilder {
            stack_builder: StackBuilder::start(block_builder),
            start: PhantomData,
            end: PhantomData,
        }
    }

    pub fn start_varlist(block_builder: BlockVarListBuilder) -> TypedStackBuilder<S, E> {
        TypedStackBuilder {
            stack_builder: StackBuilder::start_varlist(block_builder),
            start: PhantomData,
            end: PhantomData,
        }
    }

    pub fn into_untyped(self) -> StackBuilder {
        self.stack_builder
    }

    pub fn start_with_capacity(
        capacity: usize,
        block_builder: BlockBuilder,
    ) -> TypedStackBuilder<S, E> {
        TypedStackBuilder {
            stack_builder: StackBuilder::start_with_capacity(capacity, block_builder),
            start: PhantomData,
            end: PhantomData,
        }
    }

    pub fn move_head(mut self, x: f64, y: f64) -> Self {
        self.stack_builder.mut_move_head(x, y);
        self
    }

    pub fn assume_typed(stack_builder: StackBuilder) -> TypedStackBuilder<S, E> {
        TypedStackBuilder {
            stack_builder,
            start: PhantomData,
            end: PhantomData,
        }
    }
}

impl<S> TypedStackBuilder<S, StackableSide> {
    pub fn next<NE>(
        self,
        next_stack: TypedStackBuilder<StackableSide, NE>,
    ) -> TypedStackBuilder<S, NE> {
        let stack = self.into_untyped();
        let next_stack = next_stack.into_untyped();
        TypedStackBuilder {
            stack_builder: stack.next(next_stack),
            start: PhantomData,
            end: PhantomData,
        }
    }
}
