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

macro_rules! script {
    (@block {}) => {
        None
    };
    (@block { $($inner:tt)* }) => {
        Some(script!($($inner)*))
    };

    (
        @if_cond
        @cond ($($cond:tt)*)
        @rest ($($rest:tt)*)
    ) => {

    };
    (
        @if_cond
        @cond ($($cond:tt)*)
        @rest (($($then:tt)*) else ($($else:tt)*))
    ) => {

    }
}

#[cfg(test)]
mod test {
    use crate::project::resource::{ProjectFileBuilder, Resource};
    use std::env::var;

    use super::{
        project::{target_builder::*, ProjectBuilder},
        scripting::{blocks::*, typed_script_builder::HatBlock},
    };

    // #[test]
    // fn test_creating_project() {
    //     let change_dir = false;
    //     let start = script! (
    //         when_flag_clicked();
    //         forever {
    //             if key_pressed("w") {
    //                 change_y_by(10)
    //             }
    //             if key_pressed("s") {
    //                 change_y_by(-10);
    //             } else {
    //                 change_(-0.1)
    //             }
    //         }
    //     );

    //     let export_dir = var("EXPORT_DIR").expect("EXPORT_DIR env");
    //     let export_name = var("EXPORT_NAME").expect("EXPORT_NAME env");
    //     let project = ProjectBuilder::new()
    //         .set_stage(StageBuilder::new(
    //             TargetBuilder::new("Stage")
    //                 .add_costume(CostumeBuilder::new(AssetBuilder::new(
    //                     "backdrop1",
    //                     Resource::load_and_verify("blank.svg").unwrap().unwrap(),
    //                 )))
    //                 .add_comment(CommentBuilder::new("hi")),
    //         ))
    //         .add_sprite(SpriteBuilder::new(
    //             TargetBuilder::new("Cat")
    //                 .add_block_stacks(start)
    //                 .add_costume(CostumeBuilder::new(AssetBuilder::new(
    //                     "costume1",
    //                     Resource::load_and_verify("cat.svg").unwrap().unwrap(),
    //                 )))
    //                 .layer_order(1),
    //         ));
    //     ProjectFileBuilder::new(project)
    //         .name(export_name)
    //         .path(export_dir)
    //         .build()
    //         .unwrap();
    // }
}
