//! Exercise compilation and test running

use std::path::Path;
use std::process::Command;

pub fn compile(path: &Path) -> Result<(), String> {
    let output = Command::new("seqc")
        .arg("lint")
        .arg(path)
        .output()
        .map_err(|e| format!("Failed to run seqc: {}. Is seq installed and in PATH?", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        Err(format!("{}{}", stdout, stderr))
    }
}

pub fn run_tests(path: &Path) -> Result<String, String> {
    let temp_dir = std::env::temp_dir();
    let file_name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("exercise.seq");
    let temp_name = if file_name.starts_with("test-") {
        file_name.to_string()
    } else {
        format!("test-{}", file_name)
    };
    let temp_path = temp_dir.join(&temp_name);

    std::fs::copy(path, &temp_path)
        .map_err(|e| format!("Failed to copy exercise to temp file: {}", e))?;

    let output = Command::new("seqc")
        .arg("test")
        .arg(&temp_path)
        .output()
        .map_err(|e| format!("Failed to run seqc: {}. Is seq installed and in PATH?", e))?;

    let _ = std::fs::remove_file(&temp_path);

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();

    if output.status.success() {
        Ok(format!("{}{}", stdout, stderr))
    } else {
        Err(format!("{}{}", stdout, stderr))
    }
}
