pub mod export;
pub mod opcode;
pub mod project;
pub mod resource;
pub mod scripting;
pub mod typed_scripting;
pub mod uid;

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::typed_scripting::{
        arg::{GlobalList, GlobalVar, SpriteList, SpriteVar},
        blocks::*,
        if_else_chain_builder::if_,
    };
    use crate::{
        export::export,
        project::{
            asset::{AssetBuilder, CostumeBuilder},
            script::{CommentBuilder, VariableBuilder},
            target::{SpriteBuilder, StageBuilder, TargetBuilder},
            ProjectBuilder,
        },
        resource::Resource,
    };
    use std::env::var as envvar;

    #[test]
    fn test_creating_project() {
        let export_path = envvar("EXPORT_NAME").expect("EXPORT_NAME env");

        let stage_inner = TargetBuilder::new("Stage")
            .add_costume(CostumeBuilder::new(AssetBuilder::new(
                "backdrop1",
                Resource::load("blank.svg").unwrap(),
            )))
            .add_comment(CommentBuilder::new("hi"));
        let stage = StageBuilder::new(stage_inner);

        #[rustfmt::skip]
        let start = when_flag_clicked().next(forever(Some(
            if_(
                equals(sprite_var("num"), 1),
                Some(change_looks_effect_by("COLOR", 1).next(set_var_to(SpriteVar("num"), 2))),
            )
            .else_if(
                equals(sprite_var("num"), 2),
                Some(change_looks_effect_by("GHOST", 1).next(set_var_to(SpriteVar("num"), 1))),
            )
            .end(),
        )));

        let sprite_inner = TargetBuilder::new("Cat")
            .add_block_stack(start.into_untyped())
            .add_variable("num", VariableBuilder::new(1.into()))
            .add_costume(CostumeBuilder::new(AssetBuilder::new(
                "costume1",
                Resource::load("cat.svg").unwrap(),
            )))
            .layer_order(1);
        let sprite = SpriteBuilder::new(sprite_inner);

        let project = ProjectBuilder::new().set_stage(stage).add_sprite(sprite);
        export(project, export_path).unwrap();
    }
}
