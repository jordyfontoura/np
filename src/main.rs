mod command_executor;
mod package_detector;
mod script_handler;

use anyhow::Result;
use inquire::{Confirm, Select};
use std::env;
use std::path::PathBuf;

use crate::command_executor::execute_raw;
use crate::package_detector::{PackageManager, detect_package_manager};
use crate::script_handler::write_package_manager_to_package_json;

#[tokio::main]
async fn main() -> Result<()> {
    let cwd = PathBuf::from(env::current_dir()?);

    // Pure proxy: capture everything after the binary and forward as-is
    let raw: Vec<String> = env::args().skip(1).collect();

    if raw.is_empty() {
        println!(
            "Use 'np <command>' to forward to the detected package manager. Ex.: 'np -v', 'np add axios'."
        );
        return Ok(());
    }

    let manager = resolve_package_manager_interactive(&cwd)?;
    execute_raw(manager, &raw).await?;
    Ok(())
}

fn resolve_package_manager_interactive(cwd: &PathBuf) -> Result<PackageManager> {
    let managers = detect_package_manager(cwd);
    if managers.len() == 1 {
        return Ok(managers[0]);
    }
    if managers.len() > 1 {
        println!("Multiple package managers detected: {:?}", managers);
    }
    // Removed fallback to default manager via config

    println!("\n🤔 Could not determine the package manager.");
    let choice = Select::new(
        "Which package manager would you like to use?",
        vec!["npm".to_string(), "yarn".to_string(), "pnpm".to_string()],
    )
    .prompt()?;
    let selected = match choice.as_str() {
        "npm" => PackageManager::Npm,
        "yarn" => PackageManager::Yarn,
        _ => PackageManager::Pnpm,
    };

    // Ask if we can fill package.json -> packageManager
    let can_write = Confirm::new(
        "Would you like to save this choice in the 'packageManager' field in package.json?",
    )
    .with_default(true)
    .prompt()?;
    if can_write {
        // Ignore errors silently; this is just for convenience
        let _ = write_package_manager_to_package_json(cwd, selected);
    }
    Ok(selected)
}
