use std::{fs, io::Write, process::Command};

const RUNTIME_EXE: &str = "zvm";
const TEST_BYTECODE_LOCATION: &str = "debug/test";

pub fn read_file(file_name: &str) -> String {
    fs::read_to_string(file_name).expect("Files should be in the examples folder.")
}

pub fn run(bytes: Vec<u8>, expected_stdout: &str) -> Result<(), String> {
    let mut tmp_file = fs::File::create(TEST_BYTECODE_LOCATION).expect(&*format!(
        "Failed to open test file at: {}",
        TEST_BYTECODE_LOCATION
    ));
    tmp_file
        .write_all(&bytes)
        .expect("Failed to write to test tmp file.");

    let out = Command::new(RUNTIME_EXE)
        .arg(TEST_BYTECODE_LOCATION)
        .output()
        .expect("Failed to launch zvm runtime.");

    if !out.status.success() {
        return Err("Runtime failed to execute binary.".to_owned());
    }

    if out.stdout != expected_stdout.as_bytes() {
        return Err(
            "Runtime ran successfully, but returned different output than expected.".to_owned(),
        );
    }

    Ok(())
}
