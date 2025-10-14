use crate::script_handler::read_package_manager_from_package_json;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PackageManager {
    Npm,
    Yarn,
    Pnpm,
}

impl PackageManager {
    pub fn as_str(&self) -> &'static str {
        match self {
            PackageManager::Npm => "npm",
            PackageManager::Yarn => "yarn",
            PackageManager::Pnpm => "pnpm",
        }
    }
}

pub fn detect_package_manager(cwd: &Path) -> Option<PackageManager> {
    // 1) Tenta via package.json -> packageManager
    if let Some(pm) = read_package_manager_from_package_json(cwd) {
        return Some(pm);
    }

    // 2) Fallback: lockfiles
    let npm_lock = cwd.join("package-lock.json");
    if npm_lock.exists() {
        return Some(PackageManager::Npm);
    }

    let yarn_lock = cwd.join("yarn.lock");
    if yarn_lock.exists() {
        return Some(PackageManager::Yarn);
    }

    let pnpm_lock = cwd.join("pnpm-lock.yaml");
    if pnpm_lock.exists() {
        return Some(PackageManager::Pnpm);
    }

    None
}
