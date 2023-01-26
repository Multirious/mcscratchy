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

macro_rules! script {
    () => {};

    (forever {}) => {
        forever(None)
    };
    (forever {$($inside:tt)*}) => {
        forever(Some(script!($($inside)*)))
    };

    (if ($($cond:tt)*) {}) => {
        if_(script!($($cond)*), None)
    };
    (if ($($cond:tt)*) { $($then:tt)* }) => {
        if_(script!($($cond)*), Some(script!($($then)*)))
    };
    (if ($($cond:tt)*) { $($then:tt)* } $($next:tt)*) => {
        if_(script!($($cond)*), Some(script!($($then)*))).next(script!($($next)*))
    };

    (if ($($cond:tt)*) {} else {}) => {
        if_else(script!($($cond)*), None, None)
    };
    (if ($($cond:tt)*) {$($ift:tt)*} else {}) => {
        if_else(script!($($cond)*), Some(script!($($ift)*)), None)
    };
    (if ($($cond:tt)*) {} else {$($iff:tt)*}) => {
        if_else(script!($($cond)*), None, Some(script!($($iff)*)))
    };
    (if ($($cond:tt)*) {$($ift:tt)*} else {$($iff:tt)*}) => {
        if_else(script!($($cond)*), Some(script!($($ift)*)), Some(script!($($iff)*)))
    };
    (if ($($cond:tt)*) {} else {} $($next:tt)*) => {
        if_else(script!($($cond)*), None, None).next(script!($($next)*))
    };
    (if ($($cond:tt)*) {$($ift:tt)*} else {} $($next:tt)*) => {
        if_else(script!($($cond)*), Some(script!($($ift)*)), None).next(script!($($next)*))
    };
    (if ($($cond:tt)*) {} else {$($iff:tt)*} $($next:tt)*) => {
        if_else(script!($($cond)*), None, Some(script!($($iff)*))).next(script!($($next)*))
    };
    (if ($($cond:tt)*) {$($ift:tt)*} else {$($iff:tt)*} $($next:tt)*) => {
        if_else(script!($($cond)*), Some(script!($($ift)*)), Some(script!($($iff)*))).next(script!($($next)*))
    };

    ($block:expr) => {
        $block
    };
    ($block:expr;) => {
        $block
    };
    ($block:expr; $($next:tt)*) => {
        $block.next(script!($($next)*))
    };
}

#[cfg(test)]
mod test {
    use crate::project::resource::{ProjectFileBuilder, Resource};
    use std::env::var;

    use super::{
        project::{target_builder::*, ProjectBuilder},
        scripting::{blocks::*, typed_script_builder::HatBlock},
    };

    #[test]
    fn test_creating_project() {
        // let flag_clicked = when_flag_clicked().next();
        // let start = when_flag_clicked().next(forever(Some(
        //     if_(key_pressed("w"), Some(change_y_by(10)))
        //         .next(if_(key_pressed("s"), Some(change_y_by(-10))))
        //         .next(if_(key_pressed("a"), Some(change_x_by(-10))))
        //         .next(if_(key_pressed("d"), Some(change_x_by(10)))),
        // )));
        let start = script! (
            when_flag_clicked();
            forever {
                if (key_pressed("w")) {
                    change_y_by(10);
                }
                if (key_pressed("s")) {
                    change_y_by(-10);
                }
                if (key_pressed("a")) {
                    change_x_by(-10);
                }
                if (key_pressed("d")) {
                    change_x_by(10);
                }
            }
        );

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
                    .add_block_stacks(start)
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
