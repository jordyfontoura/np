use np::command_executor::execute_raw;
use np::package_detector::PackageManager;

// Integration tests to verify command execution
// Note: These tests execute real commands, so they require package managers to be installed

#[tokio::test]
async fn execute_npm_version() {
    // Check if npm is available
    if !is_command_available("npm") {
        println!("npm is not installed, skipping test");
        return;
    }

    let args = vec!["--version".to_string()];
    let result = execute_raw(PackageManager::Npm, &args).await;
    assert!(result.is_ok(), "npm --version should execute successfully");
}

#[tokio::test]
async fn execute_yarn_version() {
    if !is_command_available("yarn") {
        println!("yarn is not installed, skipping test");
        return;
    }

    let args = vec!["--version".to_string()];
    let result = execute_raw(PackageManager::Yarn, &args).await;
    assert!(result.is_ok(), "yarn --version should execute successfully");
}

#[tokio::test]
async fn execute_pnpm_version() {
    if !is_command_available("pnpm") {
        println!("pnpm is not installed, skipping test");
        return;
    }

    let args = vec!["--version".to_string()];
    let result = execute_raw(PackageManager::Pnpm, &args).await;
    assert!(result.is_ok(), "pnpm --version should execute successfully");
}

#[tokio::test]
async fn execute_invalid_command() {
    if !is_command_available("npm") {
        println!("npm is not installed, skipping test");
        return;
    }

    // Invalid command that should fail
    let args = vec!["this-is-not-a-valid-command".to_string()];
    let result = execute_raw(PackageManager::Npm, &args).await;
    assert!(result.is_err(), "Invalid command should return error");
}

#[tokio::test]
async fn execute_with_multiple_args() {
    if !is_command_available("npm") {
        println!("npm is not installed, skipping test");
        return;
    }

    let args = vec!["help".to_string()];
    let result = execute_raw(PackageManager::Npm, &args).await;
    assert!(result.is_ok(), "npm help should execute successfully");
}

#[tokio::test]
async fn execute_npm_help() {
    if !is_command_available("npm") {
        println!("npm is not installed, skipping test");
        return;
    }

    let args = vec!["help".to_string()];
    let result = execute_raw(PackageManager::Npm, &args).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn execute_yarn_help() {
    if !is_command_available("yarn") {
        println!("yarn is not installed, skipping test");
        return;
    }

    let args = vec!["help".to_string()];
    let result = execute_raw(PackageManager::Yarn, &args).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn execute_pnpm_help() {
    if !is_command_available("pnpm") {
        println!("pnpm is not installed, skipping test");
        return;
    }

    let args = vec!["help".to_string()];
    let result = execute_raw(PackageManager::Pnpm, &args).await;
    assert!(result.is_ok());
}

// Helper function to check if a command is available
fn is_command_available(cmd: &str) -> bool {
    std::process::Command::new(cmd)
        .arg("--version")
        .output()
        .is_ok()
}

#[test]
fn package_manager_binaries() {
    // Test that binary names are correct
    assert_eq!(PackageManager::Npm.as_str(), "npm");
    assert_eq!(PackageManager::Yarn.as_str(), "yarn");
    assert_eq!(PackageManager::Pnpm.as_str(), "pnpm");
}
