use std::marker::PhantomData;

use super::{
    arg::{Bool, IntoInput, Stack},
    blocks::{if_ as if_block, if_else as if_else_block},
    script_builder::BlockInputBuilder,
    typed_script_builder::{StackBlock, TypedStackBuilder},
};

pub struct Init;
pub struct Building;
pub struct End;

pub fn if_<Cond, Then>(cond: Cond, then: Option<Then>) -> IfElseChainBuilder<Building>
where
    Cond: IntoInput<Bool>,
    Then: IntoInput<Stack>,
{
    IfElseChainBuilder::if_(cond, then)
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfElseChainBuilder<S> {
    if_: (BlockInputBuilder, Option<BlockInputBuilder>),
    else_ifs: Vec<(BlockInputBuilder, Option<BlockInputBuilder>)>,
    else_: Option<Option<BlockInputBuilder>>,
    marker: PhantomData<S>,
}

impl IfElseChainBuilder<Init> {
    pub fn if_<Cond, Then>(cond: Cond, then: Option<Then>) -> IfElseChainBuilder<Building>
    where
        Cond: IntoInput<Bool>,
        Then: IntoInput<Stack>,
    {
        IfElseChainBuilder {
            if_: (cond.into_input(), then.map(|s| s.into_input())),
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
        Cond: IntoInput<Bool>,
        Then: IntoInput<Stack>,
    {
        self.else_ifs
            .push((cond.into_input(), then.map(IntoInput::<Stack>::into_input)));
        self
    }

    pub fn else_<End>(mut self, else_: Option<End>) -> IfElseChainBuilder<End>
    where
        End: IntoInput<Stack>,
    {
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
