use cereal::simulator::{run, Options};
use std::path::PathBuf;

#[test]
fn public_test_basic() {
    asm_trace_test(&["public-test_basic"]);
}

#[test]
fn public_branch0() {
    asm_trace_test(&["public-branch0"]);
}

#[test]
fn public_br_arith() {
    asm_trace_test(&["public-BR_arith"]);
}

#[test]
fn public_test_checkers_img() {
    asm_trace_test(&["public-test_checkers_img"]);
}

#[test]
fn wireframe() {
    asm_trace_test(&["wireframe", "os"]);
}

fn asm_trace_test(names: &[&str]) {
    let input_file_names: Vec<PathBuf> = names.iter().map(|name| {
        let mut full = String::from("data/asm/");
        full.push_str(name);
        full.push_str(".obj");
        full.into()
    }).collect();

    let mut input_file_name = String::from("data/asm/");
    let mut output_file_name = String::from("data/tests/asm/");
    input_file_name.push_str(names[0]);
    output_file_name.push_str(names[0]);
    let mut test_data = input_file_name.clone();
    input_file_name.push_str(".obj");
    output_file_name.push_str(".txt");
    test_data.push_str(".txt");
    
    let expected = std::fs::read_to_string(test_data).expect("Cannot find test data");
    let options = Options {
        trace_path: Some(output_file_name.clone().into()),
        input_paths: input_file_names,
        step_cap: Some(expected.lines().count() as u64),
        loader_trace: false,
    };
    
    run(options);
    
    let actual = std::fs::read_to_string(output_file_name).expect("Cannot open output file");
    
    compare_by_lines(&actual, &expected);
}

fn compare_by_lines(actual: &str, expected: &str) {
    actual
        .lines()
        .zip(expected.lines())
        .enumerate()
        .map(|(i, (a, e))| (i + 1, a, e))
        .for_each(|(line, actual, expected)| {
            if actual != expected {
                assert_eq!(actual, expected, "Mismatch on line {}", line);
            }
        });
    
    if actual.len() != expected.len() {
        panic!("The files are of different lengths");
    }
}
