use np::package_detector::PackageManager;
use np::script_handler::{
    read_package_manager_from_package_json, write_package_manager_to_package_json,
};
use std::fs;

fn write_file(path: &std::path::Path, name: &str, content: &str) {
    fs::write(path.join(name), content).unwrap();
}

#[test]
fn read_package_manager_npm() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "package.json",
        r#"{"packageManager": "npm@10.0.0"}"#,
    );
    assert_eq!(
        read_package_manager_from_package_json(dir.path()),
        Some(PackageManager::Npm)
    );
}

#[test]
fn read_package_manager_yarn() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "package.json",
        r#"{"packageManager": "yarn@1.22.19"}"#,
    );
    assert_eq!(
        read_package_manager_from_package_json(dir.path()),
        Some(PackageManager::Yarn)
    );
}

#[test]
fn read_package_manager_pnpm() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "package.json",
        r#"{"packageManager": "pnpm@9.9.0"}"#,
    );
    assert_eq!(
        read_package_manager_from_package_json(dir.path()),
        Some(PackageManager::Pnpm)
    );
}

#[test]
fn read_package_manager_without_version() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "package.json", r#"{"packageManager": "npm"}"#);
    assert_eq!(
        read_package_manager_from_package_json(dir.path()),
        Some(PackageManager::Npm)
    );
}

#[test]
fn read_package_manager_no_field() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "package.json", r#"{"name": "test"}"#);
    assert_eq!(read_package_manager_from_package_json(dir.path()), None);
}

#[test]
fn read_package_manager_no_package_json() {
    let dir = tempfile::tempdir().unwrap();
    assert_eq!(read_package_manager_from_package_json(dir.path()), None);
}

#[test]
fn read_package_manager_invalid_json() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "package.json", r#"{"invalid json"#);
    assert_eq!(read_package_manager_from_package_json(dir.path()), None);
}

#[test]
fn read_package_manager_unknown_manager() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "package.json",
        r#"{"packageManager": "bun@1.0.0"}"#,
    );
    assert_eq!(read_package_manager_from_package_json(dir.path()), None);
}

#[test]
fn write_package_manager_npm() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "package.json", r#"{"name": "test"}"#);

    write_package_manager_to_package_json(dir.path(), PackageManager::Npm).unwrap();

    let content = fs::read_to_string(dir.path().join("package.json")).unwrap();
    assert!(content.contains("packageManager"));
    assert!(content.contains("npm"));
}

#[test]
fn write_package_manager_yarn() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "package.json", r#"{"name": "test"}"#);

    write_package_manager_to_package_json(dir.path(), PackageManager::Yarn).unwrap();

    let content = fs::read_to_string(dir.path().join("package.json")).unwrap();
    assert!(content.contains("packageManager"));
    assert!(content.contains("yarn"));
}

#[test]
fn write_package_manager_pnpm() {
    let dir = tempfile::tempdir().unwrap();
    write_file(dir.path(), "package.json", r#"{"name": "test"}"#);

    write_package_manager_to_package_json(dir.path(), PackageManager::Pnpm).unwrap();

    let content = fs::read_to_string(dir.path().join("package.json")).unwrap();
    assert!(content.contains("packageManager"));
    assert!(content.contains("pnpm"));
}

#[test]
fn write_package_manager_overwrites_existing() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "package.json",
        r#"{"packageManager": "npm@10.0.0"}"#,
    );

    write_package_manager_to_package_json(dir.path(), PackageManager::Yarn).unwrap();

    let content = fs::read_to_string(dir.path().join("package.json")).unwrap();
    assert!(content.contains("yarn"));
    assert!(!content.contains("npm"));
}

#[test]
fn write_package_manager_preserves_other_fields() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "package.json",
        r#"{"name": "test", "version": "1.0.0"}"#,
    );

    write_package_manager_to_package_json(dir.path(), PackageManager::Npm).unwrap();

    let content = fs::read_to_string(dir.path().join("package.json")).unwrap();
    assert!(content.contains("name"));
    assert!(content.contains("test"));
    assert!(content.contains("version"));
    assert!(content.contains("1.0.0"));
    assert!(content.contains("packageManager"));
}

#[test]
fn write_package_manager_no_package_json() {
    let dir = tempfile::tempdir().unwrap();
    // Não deve retornar erro, apenas não faz nada
    let result = write_package_manager_to_package_json(dir.path(), PackageManager::Npm);
    assert!(result.is_ok());
    assert!(!dir.path().join("package.json").exists());
}

#[test]
fn write_package_manager_maintains_json_structure() {
    let dir = tempfile::tempdir().unwrap();
    write_file(
        dir.path(),
        "package.json",
        r#"{
  "name": "test",
  "version": "1.0.0",
  "dependencies": {
    "axios": "^1.0.0"
  }
}"#,
    );

    write_package_manager_to_package_json(dir.path(), PackageManager::Pnpm).unwrap();

    let content = fs::read_to_string(dir.path().join("package.json")).unwrap();
    // Verifica que é um JSON válido
    let _: serde_json::Value = serde_json::from_str(&content).unwrap();
    assert!(content.contains("packageManager"));
    assert!(content.contains("pnpm"));
}
