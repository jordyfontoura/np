use anyhow::{Context, Result, bail};
use std::process::Stdio;
use tokio::process::Command;

use crate::package_detector::PackageManager;

fn manager_to_bin(m: PackageManager) -> &'static str {
    match m {
        PackageManager::Npm => "npm",
        PackageManager::Yarn => "yarn",
        PackageManager::Pnpm => "pnpm",
    }
}

pub async fn execute_raw(manager: PackageManager, args: &[String]) -> Result<()> {
    let bin = manager_to_bin(manager).to_string();
    let status = Command::new(&bin)
        .args(args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .await
        .with_context(|| format!("Falha ao executar {}", bin))?;

    if !status.success() {
        let code = status.code().unwrap_or(-1);
        bail!("❌ Comando falhou com código {}", code);
    }
    Ok(())
}
