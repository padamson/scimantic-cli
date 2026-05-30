use std::process::Command;

const BIN: &str = env!("CARGO_BIN_EXE_scimantic");

#[test]
fn version_flag_prints_name_and_version() {
    let output = Command::new(BIN)
        .arg("--version")
        .output()
        .expect("failed to execute scimantic");

    assert!(output.status.success(), "exit status: {}", output.status);

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    let expected = format!("{} {}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    assert_eq!(stdout, expected);
}

#[test]
fn short_version_flag_matches_long() {
    let output = Command::new(BIN)
        .arg("-V")
        .output()
        .expect("failed to execute scimantic");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    let expected = format!("{} {}\n", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    assert_eq!(stdout, expected);
}

#[test]
fn unknown_argument_exits_nonzero() {
    let output = Command::new(BIN)
        .arg("--definitely-not-a-flag")
        .output()
        .expect("failed to execute scimantic");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr is utf-8");
    assert!(
        stderr.contains("unrecognized arguments"),
        "stderr: {stderr}"
    );
}

#[test]
fn no_arguments_prints_help_to_stdout() {
    let output = Command::new(BIN)
        .output()
        .expect("failed to execute scimantic");

    assert!(output.status.success(), "exit status: {}", output.status);

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("Usage: scimantic"), "stdout: {stdout}");
}

#[test]
fn help_flag_prints_help_to_stdout() {
    let output = Command::new(BIN)
        .arg("--help")
        .output()
        .expect("failed to execute scimantic");

    assert!(output.status.success(), "exit status: {}", output.status);

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("Usage: scimantic"), "stdout: {stdout}");
}

#[test]
fn short_help_flag_matches_long() {
    let output = Command::new(BIN)
        .arg("-h")
        .output()
        .expect("failed to execute scimantic");

    assert!(output.status.success(), "exit status: {}", output.status);

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("Usage: scimantic"), "stdout: {stdout}");
}
