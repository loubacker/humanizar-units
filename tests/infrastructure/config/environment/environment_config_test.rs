#![forbid(unsafe_code)]

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus};
use std::time::{SystemTime, UNIX_EPOCH};

use humanizar_units::infrastructure::config::EnvironmentConfig;

const SCENARIO_ENV: &str = "HUMANIZAR_ENV_TEST_SCENARIO";

#[test]
fn environment_process_helper() {
    let Ok(scenario) = std::env::var(SCENARIO_ENV) else {
        return;
    };

    match scenario.as_str() {
        "missing" => EnvironmentConfig::load().expect("arquivo ausente deve ser aceito"),
        "invalid" => assert!(EnvironmentConfig::load().is_err()),
        "precedence" => {
            EnvironmentConfig::load().expect("arquivo válido deve ser carregado");
            assert_eq!(
                "process",
                std::env::var("ENVIRONMENT_PRECEDENCE").expect("a variável deve existir")
            );
        }
        scenario => panic!("cenário desconhecido: {scenario}"),
    }
}

#[test]
fn missing_dotenv_is_valid_for_injected_environments() {
    let directory = temporary_directory("missing");
    fs::create_dir_all(&directory).expect("o diretório temporário deve ser criado");

    let status = run_scenario("missing", &directory, &[]);

    cleanup(&directory);
    assert!(status.success());
}

#[test]
fn invalid_dotenv_is_rejected() {
    let directory = temporary_directory("invalid");
    fs::create_dir_all(&directory).expect("o diretório temporário deve ser criado");
    fs::write(directory.join(".env"), "INVALID LINE WITHOUT ASSIGNMENT\n")
        .expect("o .env inválido deve ser criado");

    let status = run_scenario("invalid", &directory, &[]);

    cleanup(&directory);
    assert!(status.success());
}

#[test]
fn process_environment_has_precedence_over_dotenv() {
    let directory = temporary_directory("precedence");
    fs::create_dir_all(&directory).expect("o diretório temporário deve ser criado");
    fs::write(directory.join(".env"), "ENVIRONMENT_PRECEDENCE=file\n")
        .expect("o .env deve ser criado");

    let status = run_scenario(
        "precedence",
        &directory,
        &[("ENVIRONMENT_PRECEDENCE", "process")],
    );

    cleanup(&directory);
    assert!(status.success());
}

#[test]
fn example_exposes_placeholders_and_local_dotenv_is_ignored() {
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    let example = fs::read_to_string(manifest.join(".env.example"))
        .expect("o contrato .env.example deve existir");
    let gitignore =
        fs::read_to_string(manifest.join(".gitignore")).expect("o .gitignore deve existir");

    assert!(example.contains("DB_PASSWORD=change_me"));
    assert!(example.contains("DATABASE_RETRY_MAX_RETRIES=2"));
    assert!(gitignore.lines().any(|line| line.trim() == ".env"));
}

fn run_scenario(scenario: &str, directory: &Path, environment: &[(&str, &str)]) -> ExitStatus {
    let mut command =
        Command::new(std::env::current_exe().expect("o teste deve possuir executável"));
    command
        .arg("--exact")
        .arg("environment_process_helper")
        .arg("--nocapture")
        .current_dir(directory)
        .env(SCENARIO_ENV, scenario);

    for (name, value) in environment {
        command.env(name, value);
    }

    command.status().expect("o processo filho deve executar")
}

fn temporary_directory(scenario: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("o relógio deve estar após o epoch")
        .as_nanos();

    std::env::temp_dir().join(format!(
        "humanizar-units-environment-{scenario}-{}-{timestamp}",
        std::process::id()
    ))
}

fn cleanup(directory: &Path) {
    fs::remove_dir_all(directory).expect("o diretório temporário deve ser removido");
}
