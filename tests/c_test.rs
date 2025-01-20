use std::env;
use std::process::Command;

#[test]
fn run_c_tests() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let status = Command::new("sh")
        .arg("tests/compile_and_run_c_test.sh")
        .current_dir(&manifest_dir)
        .status()
        .expect("Failed to run test script");

    assert!(status.success(), "C tests failed");
}
