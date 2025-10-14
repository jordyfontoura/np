

use anyhow::Result;
use std::env;
use std::path::{PathBuf};

use crate::command_executor::execute_raw;

mod config;
mod command_executor;
mod managers;
mod cli;

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

    let manager = cli::resolve_package_manager(&cwd)?;
    execute_raw(manager, &raw).await?;
    Ok(())
}