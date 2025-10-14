mod command_executor;
mod package_detector;
mod script_handler;

use anyhow::Result;
use inquire::{Confirm, Select};
use std::env;
use std::path::PathBuf;

use crate::command_executor::execute_raw;
use crate::package_detector::{PackageManager, detect_package_manager};
use crate::script_handler::write_package_manager_to_package_json;

#[tokio::main]
async fn main() -> Result<()> {
    let cwd = PathBuf::from(env::current_dir()?);

    // Proxy puro: captura tudo após o binário e repassa como está
    let raw: Vec<String> = env::args().skip(1).collect();

    if raw.is_empty() {
        println!(
            "Use 'np <comando>' para encaminhar ao gerenciador detectado. Ex.: 'np -v', 'np add axios'."
        );
        return Ok(());
    }

    let manager = resolve_package_manager_interactive(&cwd)?;
    execute_raw(manager, &raw).await?;
    Ok(())
}

fn resolve_package_manager_interactive(cwd: &PathBuf) -> Result<PackageManager> {
    if let Some(d) = detect_package_manager(cwd) {
        return Ok(d);
    }
    // Removido fallback para gerenciador padrão via config

    println!("\n🤔 Não foi possível determinar o gerenciador de pacotes.");
    let choice = Select::new(
        "Qual gerenciador de pacotes você deseja usar?",
        vec!["npm".to_string(), "yarn".to_string(), "pnpm".to_string()],
    )
    .prompt()?;
    let selected = match choice.as_str() {
        "npm" => PackageManager::Npm,
        "yarn" => PackageManager::Yarn,
        _ => PackageManager::Pnpm,
    };

    // Pergunta se pode preencher package.json -> packageManager
    let can_write =
        Confirm::new("Deseja preencher o campo 'packageManager' no package.json com esta escolha?")
            .with_default(true)
            .prompt()?;
    if can_write {
        // Ignora erros silenciosamente; intenção é conveniência
        let _ = write_package_manager_to_package_json(cwd, selected);
    }
    Ok(selected)
}
