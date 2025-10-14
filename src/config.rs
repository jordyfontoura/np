use std::{path::Path, process::Command};
use anyhow::Context;
use serde_json::Value;

use crate::managers::{get_from_lockfile, get_package_manager_info_by_name, get_possible_lockfiles, PackageManagerInfo};

pub fn read_package_manager_from_package_json(cwd: &Path) -> Option<&'static PackageManagerInfo> {
    let path = cwd.join("package.json");
    let content = std::fs::read_to_string(&path).ok()?;
    let v: Value = serde_json::from_str(&content).ok()?;
    let pm_str = v.get("packageManager")?.as_str()?;
    let name = pm_str.split('@').next().unwrap_or("");

    get_package_manager_info_by_name(name)
}

pub fn detect_package_managers(cwd: &Path) -> Vec<&'static PackageManagerInfo> {
    if let Some(pm) = read_package_manager_from_package_json(cwd) {
        return vec![pm];
    }

    get_possible_lockfiles()
        .into_iter()
        .filter(|lock| cwd.join(lock).exists())
        .filter_map(|lock| get_from_lockfile(lock))
        .collect()
}

pub fn write_package_manager_to_package_json(
    cwd: &Path,
    manager_info: &PackageManagerInfo,
) -> anyhow::Result<()> {
    let package_json_path = cwd.join("package.json");
    if !package_json_path.exists() {
        return Err(anyhow::anyhow!("package.json not found at {:?}", package_json_path));
    }

    let content = std::fs::read_to_string(&package_json_path)
        .with_context(|| format!("Failed to read package.json at {:?}", package_json_path))?;
    let mut v: Value = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse package.json at {:?}", package_json_path))?;

    let name = manager_info.name;
    if !name.is_empty() {
        // Detect version if possible
        let pm_value = detect_package_manager_version(manager_info)
            .map(|ver| format!("{}@{}", name, ver))
            .unwrap_or_else(|| name.to_string());

        v["packageManager"] = Value::String(pm_value);

        let serialized = serde_json::to_string_pretty(&v)
            .with_context(|| "Failed to serialize package.json")?;
        std::fs::write(&package_json_path, serialized)
            .with_context(|| format!("Failed to write package.json at {:?}", package_json_path))?;
    }

    Ok(())
}

fn detect_package_manager_version(manager: &PackageManagerInfo) -> Option<String> {
    let bin = manager.name;

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
