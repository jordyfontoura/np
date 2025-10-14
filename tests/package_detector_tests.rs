use std::fs;
use std::io::Write;
use std::path::Path;

use np::package_detector::{PackageManager, detect_package_manager};

fn touch(path: &Path, name: &str) {
    let mut f = fs::File::create(path.join(name)).unwrap();
    writeln!(f, "").unwrap();
}

fn write_file(path: &Path, name: &str, content: &str) {
    fs::write(path.join(name), content).unwrap();
}

#[test]
fn detect_npm() {
    let dir = tempfile::tempdir().unwrap();
    touch(dir.path(), "package-lock.json");
    assert_eq!(
        detect_package_manager(dir.path()),
        vec![PackageManager::Npm]
    );
}

#[test]
fn detect_yarn() {
    let dir = tempfile::tempdir().unwrap();
    touch(dir.path(), "yarn.lock");
    assert_eq!(
        detect_package_manager(dir.path()),
        vec![PackageManager::Yarn]
    );
}

#[test]
fn detect_pnpm() {
    let dir = tempfile::tempdir().unwrap();
    touch(dir.path(), "pnpm-lock.yaml");
    assert_eq!(
        detect_package_manager(dir.path()),
        vec![PackageManager::Pnpm]
    );
}

#[test]
fn detect_none() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(detect_package_manager(dir.path()), vec![]);
}

#[test]
fn detect_multiple_lockfiles() {
    let dir = tempfile::tempdir().unwrap();
    touch(dir.path(), "package-lock.json");
    touch(dir.path(), "yarn.lock");
    let managers = detect_package_manager(dir.path());
    // Should detect multiple package managers
    assert!(managers.len() > 1);
    assert!(managers.contains(&PackageManager::Npm));
    assert!(managers.contains(&PackageManager::Yarn));
}

#[test]
fn detect_from_package_json_npm() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "package.json",
        r#"{"packageManager": "npm@10.0.0"}"#,
    );
    // Also create lockfiles to ensure package.json has priority
    touch(dir.path(), "yarn.lock");
    assert_eq!(
        detect_package_manager(dir.path()),
        vec![PackageManager::Npm]
    );
}

#[test]
fn detect_from_package_json_yarn() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "package.json",
        r#"{"packageManager": "yarn@1.22.19"}"#,
    );
    touch(dir.path(), "package-lock.json");
    assert_eq!(
        detect_package_manager(dir.path()),
        vec![PackageManager::Yarn]
    );
}

#[test]
fn detect_from_package_json_pnpm() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "package.json",
        r#"{"packageManager": "pnpm@9.9.0"}"#,
    );
    touch(dir.path(), "yarn.lock");
    assert_eq!(
        detect_package_manager(dir.path()),
        vec![PackageManager::Pnpm]
    );
}

#[test]
fn detect_from_package_json_without_version() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "package.json", r#"{"packageManager": "npm"}"#);
    assert_eq!(
        detect_package_manager(dir.path()),
        vec![PackageManager::Npm]
    );
}

#[test]
fn package_manager_as_str() {
    assert_eq!(PackageManager::Npm.as_str(), "npm");
    assert_eq!(PackageManager::Yarn.as_str(), "yarn");
    assert_eq!(PackageManager::Pnpm.as_str(), "pnpm");
}
