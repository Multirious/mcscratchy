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
        scripting::{control::*, event::*, motion::*, operator::*},
    };

    #[test]
    fn test_creating_project() {
        let project = ProjectBuilder::new()
            .set_stage(StageBuilder::new(
                TargetBuilder::new("Stage")
                    .add_costume(CostumeBuilder::new(AssetBuilder::new(
                        "backdrop1",
                        File::load("blank.svg").unwrap().verify().unwrap(),
                    )))
                    .add_comment(CommentBuilder::new("hi")),
            ))
            .add_sprite(SpriteBuilder::new(
                TargetBuilder::new("Cat")
                    .add_block_stacks(when_flag_clicked().next(move_steps(5)))
                    .add_costume(CostumeBuilder::new(AssetBuilder::new(
                        "costume1",
                        File::load("cat.svg").unwrap().verify().unwrap(),
                    ))),
            ));
        ProjectFileBuilder::new(project).build().unwrap();
    }
}
