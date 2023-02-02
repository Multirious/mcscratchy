use std::marker::PhantomData;

use super::{
    arg::{Arg, Bool, IntoArg, IntoStackArg},
    script_builder::StackBuilder,
    typed_blocks::{if_ as if_block, if_else as if_else_block},
    typed_script_builder::{StackBlock, TypedStackBuilder},
};

pub struct Init;
pub struct Building;
pub struct End;

pub fn if_<Cond, Then>(cond: Cond, then: Option<Then>) -> IfElseChainBuilder<Building>
where
    Cond: IntoArg<Bool>,
    Then: IntoStackArg,
{
    IfElseChainBuilder::if_(cond, then)
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfElseChainBuilder<S> {
    if_: (Arg, Option<StackBuilder>),
    else_ifs: Vec<(Arg, Option<StackBuilder>)>,
    else_: Option<Option<StackBuilder>>,
    marker: PhantomData<S>,
}

impl IfElseChainBuilder<Init> {
    pub fn if_<Cond, Then>(cond: Cond, then: Option<Then>) -> IfElseChainBuilder<Building>
    where
        Cond: IntoArg<Bool>,
        Then: IntoStackArg,
    {
        IfElseChainBuilder {
            if_: (cond.into_arg(), then.map(|s| s.into_stack_arg())),
            else_ifs: vec![],
            else_: None,
            marker: PhantomData,
        }
    }
}

impl IfElseChainBuilder<Building> {
    pub fn else_if<Cond, Then>(
        mut self,
        cond: Cond,
        then: Option<Then>,
    ) -> IfElseChainBuilder<Building>
    where
        Cond: IntoArg<Bool>,
        Then: IntoStackArg,
    {
        self.else_ifs
            .push((cond.into_arg(), then.map(IntoStackArg::into_stack_arg)));
        self
    }

    pub fn else_<End>(mut self, else_: Option<End>) -> IfElseChainBuilder<End>
    where
        End: IntoStackArg,
    {
        self.else_ = Some(else_.map(IntoStackArg::into_stack_arg));
        let IfElseChainBuilder {
            if_,
            else_ifs,
            else_,
            marker: _,
        } = self;
        IfElseChainBuilder {
            if_,
            else_ifs,
            else_,
            marker: PhantomData,
        }
    }
}

impl<S> IfElseChainBuilder<S> {
    pub fn end(self) -> StackBlock {
        let IfElseChainBuilder {
            if_,
            else_ifs,
            else_,
            marker: _,
        } = self;
        // not very readable - fix later
        match (else_ifs.len(), else_) {
            (0, None) => if_block(if_.0, if_.1),
            (0, Some(else_)) => if_else_block(if_.0, if_.1, else_),
            (_, None) => {
                // building from inside out
                let mut else_ifs_rev_iter = else_ifs.into_iter().rev();
                let last_else_if = {
                    let (a, substack) = else_ifs_rev_iter.next().unwrap();
                    if_block(a, substack)
                };
                match else_ifs_rev_iter.next() {
                    Some((parent_a, parent_subtack)) => {
                        let mut prev_parent =
                            if_else_block(parent_a, parent_subtack, Some(last_else_if));
                        for (parent_a, parent_substack) in else_ifs_rev_iter {
                            prev_parent =
                                if_else_block(parent_a, parent_substack, Some(prev_parent));
                        }
                        if_else_block(if_.0, if_.1, Some(prev_parent))
                    }
                    None => if_else_block(if_.0, if_.1, Some(last_else_if)),
                }
            }
            (_, Some(else_)) => {
                // building from inside out
                let mut else_ifs_rev_iter = else_ifs.into_iter().rev();
                let last_else = else_;
                match else_ifs_rev_iter.next() {
                    Some((parent_a, parent_subtack)) => {
                        let mut prev_parent = if_else_block(parent_a, parent_subtack, last_else);
                        for (parent_a, parent_substack) in else_ifs_rev_iter {
                            prev_parent =
                                if_else_block(parent_a, parent_substack, Some(prev_parent));
                        }
                        if_else_block(if_.0, if_.1, Some(prev_parent))
                    }
                    None => if_else_block(if_.0, if_.1, last_else),
                }
            }
        }
    }
}
