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
    use std::fs::File;
    use std::path::Path;
    use std::{env::var, sync::mpsc::channel};

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
        use notify::Watcher;
        use std::io::Write;

        fn get_blocks<'a>(
            project: &'a rs_sb3::project::Project,
            sprite_name: &str,
        ) -> &'a rs_sb3::string_hashmap::StringHashMap<rs_sb3::block::Block> {
            let sprite = project
                .targets
                .iter()
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
                .expect("finding sprite");
            &sprite.target.blocks
        }

        let script_result_path = var("SCRIPT_RESULT_PATH").expect("SCRIPT_RESULT_PATH environment");
        let import_path = var("IMPORT_PATH").expect("IMPORT_PATH environment");
        let sprite_name = var("SPRITE_NAME").expect("SPRITE_NAME environment");
        let (tx, rx) = channel();

        let mut watcher = notify::RecommendedWatcher::new(tx, notify::Config::default()).unwrap();
        watcher
            .watch(Path::new(&import_path), notify::RecursiveMode::NonRecursive)
            .unwrap();

        println!("start watching");
        for res in rx {
            match res {
                Ok(event) => {
                    println!("{event:?}");
                    if let notify::EventKind::Create(_) = event.kind {
                        let file = File::options().read(true).open(&import_path).unwrap();
                        let mut zip_read = zip::read::ZipArchive::new(file).unwrap();
                        let json_zip = zip_read.by_name("project.json").unwrap();

                        let scratch_project: rs_sb3::project::Project =
                            serde_json::from_reader(json_zip).unwrap();

                        let to_print = get_blocks(&scratch_project, &sprite_name);
                        let to_print = serde_json::to_string_pretty(&to_print).unwrap();

                        let mut file = std::fs::File::options()
                            .write(true)
                            .truncate(true)
                            .create(true)
                            .open(&script_result_path)
                            .unwrap();
                        file.write(to_print.as_bytes()).unwrap();
                    }
                }
                Err(e) => eprintln!("{e}"),
            }
        }
    }
}
