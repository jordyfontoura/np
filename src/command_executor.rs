use std::process::Stdio;
use tokio::process::Command;
use anyhow::{Result, bail, Context};
use crate::managers::PackageManagerInfo;

pub async fn execute_raw(manager_info: &PackageManagerInfo, args: &[String]) -> Result<()> {
    let bin = manager_info.name;
    let status = Command::new(bin)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await
        .with_context(|| format!("Failed to execute {}", bin))?;

    if !status.success() {
        let code = status.code().unwrap_or(-1);
        bail!("❌ Command failed with exit code {}", code);
    }
    Ok(())
}