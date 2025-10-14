use std::path::Path;

use inquire::{Confirm, Select};
use anyhow::Result;

use crate::{config::{detect_package_managers, write_package_manager_to_package_json}, managers::{get_package_manager_info_by_name, list_package_manager_names, PackageManagerInfo}};


fn set_default_package_manager(project_path: &Path) -> Result<&'static PackageManagerInfo> {
    let names: Vec<&str> = list_package_manager_names();

    let choice = Select::new(
        "Which package manager would you like to use?",
        names.clone(),
    )
    .prompt()?;

    let selected = get_package_manager_info_by_name(choice)
        .expect(&format!("Unexpected package manager chosen: {:?}", choice));

    let can_write = Confirm::new(
        "Would you like to save this choice in the 'packageManager' field in package.json?",
    )
    .with_default(true)
    .prompt()?;

    if can_write {
        let _ = write_package_manager_to_package_json(project_path, selected)
            .inspect_err(|e| println!("Failed to save 'packageManager' field into package.json: {e}"));
    }

    Ok(selected)
}

pub fn resolve_package_manager(cwd: &Path) -> Result<&'static PackageManagerInfo> {
    match detect_package_managers(cwd).as_slice() {
        [single] => Ok(*single),
        [] => {
            println!("\n🤔 Could not determine the package manager.");
            set_default_package_manager(cwd)
        }
        multiple => {
            let names: Vec<&str> = multiple.iter().map(|pm| pm.name).collect();
            println!("Multiple package managers detected: {:?}", names);
            set_default_package_manager(cwd)
        }
    }
}
