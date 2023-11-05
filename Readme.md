# Dash

![dash](https://github.com/KunalBagaria/Dash/assets/61944452/aba1fc04-e76f-421b-983f-c8d7e421b595)

Dash is a command-line tool designed to streamline the setup of new applications, allowing developers to code more and configure less. With Dash, you can kickstart your projects with predefined configurations, making your development workflow faster and more efficient.


## Features

- Quick Initialization: Easily initialize new projects with `dash init`.
- Configuration Management: Open and edit your configurations with `dash config`.
- Project Creation: Create new projects with custom predefined setups using `dash create`.

## Installation

Ensure you have Rust and Cargo installed on your system. Then, install Dash using Cargo:

```
cargo install dash-create
```

## Usage

### Initializing Dash

To set up Dash, run:

```
dash init
```

This command initializes the Dash configuration with a default 'next-app' setup.

### Opening Configuration

To open and edit the Dash configuration file:

```
dash config
```

This opens the configuration file in VSCode or your default text editor.

### Creating a New Project

To create a new project:

```
dash create <alias> <project_name>
```

Replace `<alias>` with your configuration alias and `<project_name>` with the desired name of your new project.

## Custom Configuration

You can customize Dash's behavior by editing the `.dash-config.json` file in your home directory. This file contains the configurations for different aliases that Dash can use to create new projects.

## Contributing

Contributions to Dash are welcome! Feel free to fork the repository, make your changes, and create a pull request.

## License

This project is licensed under the GPL License - see the LICENSE file for details.

---
