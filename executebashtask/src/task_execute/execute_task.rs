use std::process::Command;

pub async fn execute_task(command: String) -> Result<(), String> {
    let output = Command::new("bash")
        .arg("-c")
        .arg(command)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("Output: {}", stdout);
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Error: {}", stderr);
        Err(stderr.to_string())
    }
}
