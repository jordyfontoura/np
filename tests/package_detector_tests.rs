use std::fs;
use std::io::Write;
use std::path::Path;

use np::package_detector::{detect_package_manager, PackageManager};

fn touch(path: &Path, name: &str) {
    let mut f = fs::File::create(path.join(name)).unwrap();
    writeln!(f, "").unwrap();
}

#[test]
fn detect_npm() {
    let dir = tempfile::tempdir().unwrap();
    touch(dir.path(), "package-lock.json");
    assert_eq!(detect_package_manager(dir.path()), Some(PackageManager::Npm));
}

#[test]
fn detect_yarn() {
    let dir = tempfile::tempdir().unwrap();
    touch(dir.path(), "yarn.lock");
    assert_eq!(detect_package_manager(dir.path()), Some(PackageManager::Yarn));
}

#[test]
fn detect_pnpm() {
    let dir = tempfile::tempdir().unwrap();
    touch(dir.path(), "pnpm-lock.yaml");
    assert_eq!(detect_package_manager(dir.path()), Some(PackageManager::Pnpm));
}

#[test]
fn detect_none() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(detect_package_manager(dir.path()), None);
}


