use std::process::Command;

#[test]
fn runs() {
    let mut command = Command::new("ls");
    let result = command.output();
    assert!(result.is_ok());
}
