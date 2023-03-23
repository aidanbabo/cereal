use cereal::simulator::{run, Options};
use std::path::PathBuf;

#[test]
fn public_test_basic() {
    test_asm_from_script("public-test_basic");
}

#[test]
fn public_branch0() {
    test_asm_from_script("public-branch0");
}

#[test]
fn public_br_arith() {
    test_asm_from_script("public-BR_arith");
}

#[test]
fn public_test_checkers_img() {
    test_asm_from_script("public-test_checkers_img");
}

#[test]
fn wireframe() {
    test_asm_from_script("wireframe");
}

fn test_asm_from_script(name: &str) {
    let _ = std::fs::remove_file(&format!("data/tests/asm/{name}.txt"));
    std::process::Command::new("cargo")
        .args(&["r", "--", "-s", &format!("tests/scripts/{name}_script")])
        .spawn()
        .expect("Failed to start child process")
        .wait()
        .expect("Child process exited with an error");
    let expected = std::fs::read_to_string(&format!("data/asm/{name}.txt")).expect("Cannot open expected output file");
    let actual = std::fs::read_to_string(&format!("data/tests/asm/{name}.txt")).expect("Cannot open actual output file");
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

    assert_eq!(actual.lines().count(), expected.lines().count(), "The files have a different number of lines");
}
