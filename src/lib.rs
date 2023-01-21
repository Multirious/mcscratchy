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
    use crate::project::file_manager::{File, ProjectFileBuilder};

    use super::{
        project::{target_builder::*, ProjectBuilder},
        scripting::{event::*, motion::*},
    };

    #[test]
    fn test_creating_project() {
        let project = ProjectBuilder::new()
            .set_stage(StageBuilder::new(
                TargetBuilder::new("Stage")
                    .add_costume(CostumeBuilder::new(AssetBuilder::new(
                        "backdrop1",
                        File::load_and_verify("blank.svg").unwrap().unwrap(),
                    )))
                    .add_comment(CommentBuilder::new("hi")),
            ))
            .add_sprite(SpriteBuilder::new(
                TargetBuilder::new("Cat")
                    .add_block_stacks(when_flag_clicked().next(goto(goto_menu("_random_"))))
                    .add_costume(CostumeBuilder::new(AssetBuilder::new(
                        "costume1",
                        File::load_and_verify("cat.svg").unwrap().unwrap(),
                    )))
                    .layer_order(1),
            ));
        ProjectFileBuilder::new(project)
            .name("McScratch Project")
            .path("C:\\Users\\USER\\OneDrive\\Desktop\\")
            .build()
            .unwrap();
    }
}
