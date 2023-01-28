use std::marker::PhantomData;

use super::{
    arg::{Arg, Bool, IntoArg, IntoStackArg},
    blocks::{if_ as if_block, if_else as if_else_block},
    script_builder::StackBuilder,
    typed_script_builder::{StackBlock, TypedStackBuilder},
};

pub struct Init;
pub struct Building;
pub struct End;

#[derive(Debug, Clone, PartialEq)]
pub struct IfElseBuilder<S> {
    if_: (Arg, Option<StackBuilder>),
    else_ifs: Vec<(Arg, Option<StackBuilder>)>,
    else_: Option<Option<StackBuilder>>,
    marker: PhantomData<S>,
}

impl IfElseBuilder<Init> {
    pub fn if_<Cond, Then>(cond: Cond, then: Option<Then>) -> IfElseBuilder<Building>
    where
        Cond: IntoArg<Bool>,
        Then: IntoStackArg,
    {
        IfElseBuilder {
            if_: (cond.into_arg(), then.map(|s| s.into_stack_arg())),
            else_ifs: vec![],
            else_: None,
            marker: PhantomData,
        }
    }
}

impl IfElseBuilder<Building> {
    pub fn else_if<Cond, Then>(mut self, cond: Cond, then: Option<Then>) -> IfElseBuilder<Building>
    where
        Cond: IntoArg<Bool>,
        Then: IntoStackArg,
    {
        self.else_ifs
            .push((cond.into_arg(), then.map(IntoStackArg::into_stack_arg)));
        self
    }

    pub fn else_<End>(mut self, else_: Option<End>) -> IfElseBuilder<End>
    where
        End: IntoStackArg,
    {
        self.else_ = Some(else_.map(IntoStackArg::into_stack_arg));
        let IfElseBuilder {
            if_,
            else_ifs,
            else_,
            marker: _,
        } = self;
        IfElseBuilder {
            if_,
            else_ifs,
            else_,
            marker: PhantomData,
        }
    }
}

impl<S> IfElseBuilder<S> {
    pub fn done(self) -> StackBlock {
        let IfElseBuilder {
            if_,
            else_ifs,
            else_,
            marker: _,
        } = self;
        match (&else_ifs[..], else_) {
            (&[], None) => if_block(if_.0, if_.1),
            (&[], Some(else_)) => if_else_block(if_.0, if_.1, else_),
            (else_ifs, None) => 
        }
    }
}
