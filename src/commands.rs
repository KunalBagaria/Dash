use serde::{Deserialize, Serialize};
use serde_json;
use std::io::{BufRead, BufReader};
use std::process::{Command as SystemCommand, Stdio};
use std::{fs, path::Path, path::PathBuf};

#[derive(Serialize, Deserialize)]
pub struct Config {
    working_directory: String,
    configs: std::collections::HashMap<String, Vec<String>>,
}

pub fn config_file_path() -> PathBuf {
    dirs::home_dir().unwrap().join(".dash-config.json")
}

pub fn init_config() {
    let config_path = config_file_path();
    let mut default_configs = std::collections::HashMap::new();

    // Adding default next-app configuration
    default_configs.insert("next-app".to_string(), vec![
      "yarn create next-app <project name> --ts --eslint --src-dir --use-yarn --no-tailwind --no-app --import-alias '@/*'".to_string(),
      "yarn add sass".to_string()
    ]);

    let config = Config {
        working_directory: std::env::current_dir()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
        configs: default_configs,
    };
    let config_str = serde_json::to_string_pretty(&config).unwrap();
    fs::write(config_path, config_str).expect("Unable to write config file");
    println!("Dash configuration initialized with default 'next-app' command.");
}

pub fn open_config() {
    let config_path = config_file_path();
    if !config_path.exists() {
        println!("Config file does not exist. Please run 'dash init' first.");
        return;
    }
    let status = std::process::Command::new("code")
        .arg(config_path.clone())
        .status()
        .expect("Failed to open config file in VSCode");

    if !status.success() {
        open::that(config_path).expect("Failed to open config file");
        println!("Opened config file in default editor.");
    } else {
        println!("Opened config file in VSCode.");
    }
}

pub fn create_project(alias: &str, project_name: &str) {
    let config_path = config_file_path();
    if !config_path.exists() {
        println!("Config file does not exist. Please run 'dash init' first.");
        return;
    }

    let config_str = fs::read_to_string(config_path).expect("Unable to read config file");
    let config: Config = serde_json::from_str(&config_str).expect("Unable to parse config file");

    if let Some(commands) = config.configs.get(alias) {
        for command in commands {
            let full_command = command.replace("<project name>", project_name);
            println!("Executing: {}", full_command);

            let process = SystemCommand::new("sh")
                .arg("-c")
                .arg(&full_command)
                .current_dir(&config.working_directory)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn();

            match process {
                Ok(mut child) => {
                    let stdout = child.stdout.take().expect("Failed to open stdout");
                    let stderr = child.stderr.take().expect("Failed to open stderr");

                    let stdout_reader = BufReader::new(stdout);
                    let stderr_reader = BufReader::new(stderr);

                    stdout_reader
                        .lines()
                        .filter_map(|line| line.ok())
                        .for_each(|line| println!("{}", line));

                    stderr_reader
                        .lines()
                        .filter_map(|line| line.ok())
                        .for_each(|line| eprintln!("{}", line));

                    if !child.wait().expect("Failed to wait on child").success() {
                        eprintln!("Failed to execute command: {}", full_command);
                        return;
                    }
                }
                Err(e) => {
                    eprintln!("Error executing command: {}", e);
                    return;
                }
            }
        }
        println!("Project '{}' created successfully.", project_name);
        let project_path = Path::new(&config.working_directory).join(project_name);
        let vscode_command = format!("code {}", project_path.to_str().unwrap());
        println!("Opening project in VSCode: {}", vscode_command);
        if let Err(e) = SystemCommand::new("sh")
            .arg("-c")
            .arg(&vscode_command)
            .spawn()
            .and_then(|mut child| child.wait())
        {
            eprintln!("Error opening project in VSCode: {}", e);
        } else {
            println!("Project '{}' opened in VSCode successfully.", project_name);
        }
    } else {
        eprintln!("Alias '{}' not found in configuration.", alias);
    }
}

pub const ASCII_ART: &str = r#"

██████╗  █████╗ ███████╗██╗  ██╗
██╔══██╗██╔══██╗██╔════╝██║  ██║
██║  ██║███████║███████╗███████║
██║  ██║██╔══██║╚════██║██╔══██║
██████╔╝██║  ██║███████║██║  ██║
╚═════╝ ╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝
"#;