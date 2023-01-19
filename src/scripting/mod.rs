use std::collections::HashMap;

use rs_sb3::block::Block;

use crate::uid::Uid;

pub mod arg;
pub mod script_builder;
pub mod typed_script_builder;

pub mod control;
pub mod event;
// mod looks;
pub mod motion;
pub mod operator;
// mod sensing;
// mod sound;
// mod variable;

// mod procedural;

#[test]
fn test() {
    use control::*;
    use event::*;
    use motion::*;
    use operator::*;

    #[rustfmt::skip]
    let stack = when_flag_clicked()
        .next(
            if_else(
                equals(1, 1),
                // if true
                move_steps(5),
                // if false
                turn_right(10)
            )
        );
    let (stack, _) = stack.into_untyped().build();
    let stack = uid_hashmap_to_string_hashmap(stack);
    println!("{}", serde_json::to_string_pretty(&stack).unwrap());
}

#[cfg(test)]
fn uid_hashmap_to_string_hashmap<V>(uid_hashmap: HashMap<Uid, V>) -> HashMap<String, V> {
    uid_hashmap
        .into_iter()
        .map(|(k, v)| (k.into_inner(), v))
        .collect()
}
