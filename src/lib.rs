pub mod opcode;
pub mod project;
pub mod scripting;
pub mod uid;

macro_rules! derive_everything {
    ($($item:item)*) => {
        $(
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)] $item
        )*
    };
}

pub(crate) use derive_everything;
use scripting::script_builder::StackBuilder;

#[cfg(test)]
mod test {
    use super::scripting::if_else_builder::if_;
    use super::{
        project::{target_builder::*, ProjectBuilder},
        scripting::{
            arg::{GlobalList, GlobalVar, SpriteList, SpriteVar},
            typed_blocks::*,
            typed_script_builder::HatBlock,
        },
    };
    use crate::project::resource::{ProjectFileBuilder, Resource};
    use std::env::var as envvar;

    #[test]
    fn test_creating_project() {
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

        let export_dir = envvar("EXPORT_DIR").expect("EXPORT_DIR env");
        let export_name = envvar("EXPORT_NAME").expect("EXPORT_NAME env");
        let project = ProjectBuilder::new()
            .set_stage(StageBuilder::new(
                TargetBuilder::new("Stage")
                    .add_costume(CostumeBuilder::new(AssetBuilder::new(
                        "backdrop1",
                        Resource::load_and_verify("blank.svg").unwrap().unwrap(),
                    )))
                    .add_comment(CommentBuilder::new("hi")),
            ))
            .add_sprite(SpriteBuilder::new(
                TargetBuilder::new("Cat")
                    .add_block_stacks(start)
                    .add_variable("num", VariableBuilder::new(1.into()))
                    .add_costume(CostumeBuilder::new(AssetBuilder::new(
                        "costume1",
                        Resource::load_and_verify("cat.svg").unwrap().unwrap(),
                    )))
                    .layer_order(1),
            ));
        ProjectFileBuilder::new(project)
            .name(export_name)
            .path(export_dir)
            .build()
            .unwrap();
    }
}
