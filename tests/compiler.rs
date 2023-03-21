use cereal::simulator::{run, Options};

macro_rules! simple_compiler_test {
    ( $( $name:ident $result:expr,)* ) => {
        $(
            #[test]
            fn $name() {
                let input = concat!("data/c/", stringify!($name), ".c");
                let output = concat!("data/tests/c/", stringify!($name), ".obj");
                let result = _simple_compiler_test(&[input], output);
                assert_eq!(result, $result);
            }
        )*
    }
}

simple_compiler_test! {
    nice 69,
    precedence 30, precedence2 30,
    grouping 50, grouping2 50,
    negation -10,
    bit 1,
    stack_variable 5,
    global_variable 5,
    procedure_call 5,
    procedure_call_with_args 5,
}

fn _simple_compiler_test(input: &[&str], output: &str) -> i16 {
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
        input_paths: vec![output.into()],
        step_cap: Some(5000),
        headless: true,
        ..Default::default()
    };

    run(options)
}
