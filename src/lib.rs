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

    // Empty block
    (@block {}) => {
        None
    };
    // Block
    (@block { $($block:tt)* }) => {
        Some(script!($($block)*))
    };

    // Stackable block that has no next block
    (@nextable ($($thing:tt)*) @next) => {
        $($thing)*
    };
    // Stackable block that has next block
    (@nextable ($($thing:tt)*) @next $($next:tt)* ) => {
        ($($thing)*).next(script!($($next)*))
    };

    // Repeat
    (repeat ( $($times:tt)* ) { $($repeat:tt)* } $($next:tt)*) => {
        script!(@nextable (
            repeat(
                script!($($times:tt)*),
                script!(@block { $($repeat)* })
            )
        ) @next $($next)*)
    };

    // Forever
    (forever { $($block:tt)* }) => {
        forever(script!(@block { $($block)* }))
    };

    // If
    (if ( $($cond:tt)* ) { $($then:tt)* } $($next:tt)*) => {
        script!(@nextable (
            if_(
                script!($($cond)*),
                script!(@block { $($then)* })
            )
        ) @next $($next)*)
    };

    // If else
    (if ( $($cond:tt)* ) { $($if_t:tt)* } else { $($if_f:tt)* } $($next:tt)*) => {
        script!(@nextable (if_else(
            script!($($cond)*),
            script!(@block { $($if_t)* }),
            script!(@block { $($if_f)* })
        )) @next $($next)*)
    };

    // Repeat until
    (repeat_until) => {

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

        let export_dir = var("EXPORT_DIR").expect("EXPORT_DIR env");
        let export_name = var("EXPORT_NAME").expect("EXPORT_NAME env");
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
