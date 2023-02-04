use std::marker::PhantomData;

use super::{arg::*, script_builder::*};
use crate::scripting::{
    blocks::{if_ as if_block, if_else as if_else_block},
    script_builder::BlockInputBuilder,
};

pub struct Building;
pub struct End;

/// Shortcut to `IfElseChainBuilder::if_`
pub fn if_(
    cond: impl IntoInput<Bool>,
    then: Option<impl IntoInput<Stack>>,
) -> IfElseChainBuilder<Building> {
    IfElseChainBuilder::<Building>::if_(cond, then)
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfElseChainBuilder<S> {
    if_: (BlockInputBuilder, Option<BlockInputBuilder>),
    else_ifs: Vec<(BlockInputBuilder, Option<BlockInputBuilder>)>,
    else_: Option<Option<BlockInputBuilder>>,
    marker: PhantomData<S>,
}

impl IfElseChainBuilder<Building> {
    pub fn else_if(
        mut self,
        cond: impl IntoInput<Bool>,
        then: Option<impl IntoInput<Stack>>,
    ) -> IfElseChainBuilder<Building> {
        self.else_ifs
            .push((cond.into_input(), then.map(IntoInput::<Stack>::into_input)));
        self
    }

    pub fn else_(mut self, else_: Option<impl IntoInput<Stack>>) -> IfElseChainBuilder<End> {
        self.else_ = Some(else_.map(IntoInput::<Stack>::into_input));
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
    pub fn if_(
        cond: impl IntoInput<Bool>,
        then: Option<impl IntoInput<Stack>>,
    ) -> IfElseChainBuilder<Building> {
        IfElseChainBuilder {
            if_: (cond.into_input(), then.map(|s| s.into_input())),
            else_ifs: vec![],
            else_: None,
            marker: PhantomData,
        }
    }

    pub fn end(self) -> StackBlock {
        let IfElseChainBuilder {
            if_,
            else_ifs,
            else_,
            marker: _,
        } = self;
        // not very readable - fix later
        let b = match (else_ifs.len(), else_) {
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
                        let mut prev_parent = if_else_block(
                            parent_a,
                            parent_subtack,
                            Some(BlockInputBuilder::stack(last_else_if)),
                        );
                        for (parent_a, parent_substack) in else_ifs_rev_iter {
                            prev_parent = if_else_block(
                                parent_a,
                                parent_substack,
                                Some(BlockInputBuilder::stack(prev_parent)),
                            );
                        }
                        if_else_block(if_.0, if_.1, Some(BlockInputBuilder::stack(prev_parent)))
                    }
                    None => {
                        if_else_block(if_.0, if_.1, Some(BlockInputBuilder::stack(last_else_if)))
                    }
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
                            prev_parent = if_else_block(
                                parent_a,
                                parent_substack,
                                Some(BlockInputBuilder::stack(prev_parent)),
                            );
                        }
                        if_else_block(if_.0, if_.1, Some(BlockInputBuilder::stack(prev_parent)))
                    }
                    None => if_else_block(if_.0, if_.1, last_else),
                }
            }
        };
        TypedStackBuilder::assume_typed(b)
    }
}
