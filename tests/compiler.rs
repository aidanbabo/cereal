use cereal::simulator::{run, Options};

#[test]
fn nice() {
    let result = simple_compiler_test(&["data/c/nice.c"], "data/tests/c/nice.obj");
    assert_eq!(result, 69);
}

#[test]
fn precedence() {
    let result = simple_compiler_test(&["data/c/precedence.c"], "data/tests/c/precedence.obj");
    assert_eq!(result, 30);
}

#[test]
fn precedence2() {
    let result = simple_compiler_test(&["data/c/precedence2.c"], "data/tests/c/precedence2.obj");
    assert_eq!(result, 30);
}

fn simple_compiler_test(input: &[&str], output: &str) -> i16 {
    
    let mut inputs = Vec::new();
    inputs.push("data/c/simple_libc.asm".into());
    for input in input {
        inputs.push(input.into())
    }
    inputs.push("data/c/simple_os.asm".into());
    
    let options = cereal::Options {
        output_path: output.into(),
        debug_info: false,
        input_paths: inputs,
    };
    
    cereal::compile(options).expect("Compilation success");

    let options = Options {
        trace_path: None,
        input_paths: vec![output.into()],
        step_cap: None,
        loader_trace: false,
    };
    
    run(options)
}

