use std::process::Command;

pub fn setup() {
    let _ = Command::new("sh")
        .arg("./setup.sh")
        .spawn()
        .expect("failed to execute process");
}