pub mod opcode;
pub mod project;
pub mod scripting;
pub mod uid;

macro_rules! derive_everything {
    ($($item:item)*) => {
        $(
            #[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)] $item
        )*
    };
}

pub(crate) use derive_everything;

#[cfg(test)]
mod test {
    use crate::project::resource::{ProjectFileBuilder, Resource};
    use std::env::var;

    use super::{
        project::{target_builder::*, ProjectBuilder},
        scripting::blocks::*,
    };

    #[test]
    fn test_creating_project() {
        let export_dir = var("EXPORT_DIR").unwrap();
        let export_name = var("EXPORT_NAME").unwrap();
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
                    .add_block_stacks(when_flag_clicked().next(go_to(go_to_menu("_random_"))))
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
