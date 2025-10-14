use anyhow::{Context, Result};
use serde_json::Value;
use std::path::Path;
use std::process::Command;

use crate::package_detector::PackageManager;

pub fn read_package_manager_from_package_json(cwd: &Path) -> Option<PackageManager> {
    let package_json_path = cwd.join("package.json");
    if !package_json_path.exists() {
        return None;
    }

    let content = std::fs::read_to_string(&package_json_path).ok()?;
    let v: Value = serde_json::from_str(&content).ok()?;
    let pm_str = v.get("packageManager")?.as_str()?.to_string();

    // Valores comuns: "pnpm@9.9.0", "npm@10", "yarn@1.22.19".
    let name = pm_str.split('@').next().unwrap_or("");
    match name {
        "npm" => Some(PackageManager::Npm),
        "yarn" => Some(PackageManager::Yarn),
        "pnpm" => Some(PackageManager::Pnpm),
        _ => None,
    }
}

pub fn write_package_manager_to_package_json(cwd: &Path, manager: PackageManager) -> Result<()> {
    let package_json_path = cwd.join("package.json");
    if !package_json_path.exists() {
        // Se não existe, nada a fazer silenciosamente
        return Ok(());
    }

    let content = std::fs::read_to_string(&package_json_path)
        .with_context(|| format!("Falha ao ler package.json em {:?}", package_json_path))?;
    let mut v: Value =
        serde_json::from_str(&content).with_context(|| "Falha ao fazer parse do package.json")?;

    let name = manager.as_str();
    // Tenta detectar a versão instalada do gerenciador (ex.: yarn@3.6.4)
    let pm_value = detect_package_manager_version(manager)
        .map(|ver| format!("{}@{}", name, ver))
        .unwrap_or_else(|| name.to_string());
    v["packageManager"] = Value::String(pm_value);

    let serialized = serde_json::to_string_pretty(&v)?;
    std::fs::write(&package_json_path, serialized)
        .with_context(|| format!("Falha ao salvar package.json em {:?}", package_json_path))?;
    Ok(())
}

fn detect_package_manager_version(manager: PackageManager) -> Option<String> {
    let bin = manager.as_str();

    fn run(bin: &str, flag: &str) -> Option<String> {
        let output = Command::new(bin).arg(flag).output().ok()?;
        if !output.status.success() {
            return None;
        }
        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if text.is_empty() { None } else { Some(text) }
    }

    run(bin, "--version").or_else(|| run(bin, "-v"))
}
