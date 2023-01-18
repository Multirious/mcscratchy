mod arg;
mod script_builder;
mod typed_script_builder;

mod control;
mod event;
// mod looks;
mod motion;
mod operator;
// mod sensing;
// mod sound;
// mod variable;

// mod procedural;

#[test]
fn test() {
    use event::*;
    use motion::*;
    use operator::*;
    use control::*;

    let stack = event::when_flag_clicked()
        .next(if_else())
    let (stack, _) = stack.into_untyped().build();
    println!("{}", serde_json::to_string_pretty(&stack).unwrap());
}
