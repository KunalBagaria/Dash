mod commands;

use clap::{Arg, Command};
use commands::{create_project, init_config, open_config, ASCII_ART};

fn main() {
    let matches = Command::new("Dash")
        .version("1.1.3")
        .author("Kunal Bagaria")
        .about("Code More, Configure Less: The Smart Way to Start Your Projects.")
        .subcommand(Command::new("init").about("Initializes the Dash configuration"))
        .subcommand(
            Command::new("config").about("Opens the configuration file in VSCode or the default editor"),
        )
        .subcommand(
            Command::new("create")
                .about("Creates a new project based on a predefined configuration")
                .arg(
                    Arg::new("alias")
                        .help("Alias of the configuration to use")
                        .index(1)
                        .required(true),
                )
                .arg(
                    Arg::new("project_name")
                        .help("Name of the project to create")
                        .index(2)
                        .required(true),
                ),
        )
        .before_help(ASCII_ART)
        .get_matches();

    match matches.subcommand() {
        Some(("init", _)) => init_config(),
        Some(("config", _)) => open_config(),
        Some(("create", sub_matches)) => {
            let alias = sub_matches.get_one::<String>("alias").expect("required");
            let project_name = sub_matches
                .get_one::<String>("project_name")
                .expect("required");
            create_project(alias, project_name)
        }
        _ => println!("Invalid command or command not provided"),
    }
}