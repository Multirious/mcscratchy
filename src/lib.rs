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
    use rs_sb3::target::SpriteOrStage;

    use crate::project::resource::{ProjectFileBuilder, Resource};
    use std::env::var;
    use std::fs::File;

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

    #[test]
    fn import_script() {
        let sprite_name = var("SPRITE_NAME").unwrap();
        let import = var("IMPORT_PATH").unwrap();
        let file = File::options().read(true).open(import).unwrap();
        let mut zip_read = zip::read::ZipArchive::new(file).unwrap();
        let json_zip = zip_read.by_name("project.json").unwrap();
        let scratch_project: rs_sb3::project::Project = serde_json::from_reader(json_zip).unwrap();
        let sprite = scratch_project
            .targets
            .into_iter()
            .find_map(|target| {
                if let SpriteOrStage::Sprite(sprite) = target {
                    if sprite.target.name == sprite_name {
                        Some(sprite)
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .unwrap();
        let to_print = sprite.target.blocks;
        serde_json::to_writer_pretty(std::io::stdout(), &to_print).unwrap()
    }
}
