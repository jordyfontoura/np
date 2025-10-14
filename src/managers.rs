
pub struct PackageManagerInfo {
    pub name: &'static str,
    lockfiles: &'static [&'static str],
}

const PACKAGE_MANAGERS: &[PackageManagerInfo] = &[
    PackageManagerInfo {
        name: "npm",
        lockfiles: &["package-lock.json"],
    },
    PackageManagerInfo {
        name: "yarn",
        lockfiles: &["yarn.lock"],
    },
    PackageManagerInfo {
        name: "pnpm",
        lockfiles: &["pnpm-lock.yaml"],
    },
];

pub fn get_from_lockfile(file: &str) -> Option<&'static PackageManagerInfo> {
    PACKAGE_MANAGERS
        .iter()
        .find(|pm| pm.lockfiles.iter().any(|&f| f == file))
}

pub fn list_package_manager_names() -> Vec<&'static str> {
    PACKAGE_MANAGERS.iter().map(|pm| pm.name).collect()
}

pub fn get_package_manager_info_by_name(name: &str) -> Option<&'static PackageManagerInfo> {
    PACKAGE_MANAGERS.iter().find(|pm| pm.name == name)
}