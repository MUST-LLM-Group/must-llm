use clap::{Arg, Command};
use std::process::Command as StdCommand;

fn main() {
    let matches = Command::new("must-llm")
        .version("0.1.0")
        .author("Your Name <your.email@example.com>")
        .about("Executes the given command or runs diagnostic commands")
        .arg(
            Arg::new("command")
                .help("The command to execute")
                .required(false) // No longer required because we have another subcommand
                .num_args(1..), // Accepts one or more values
        )
        .subcommand(
            Command::new("doctor")
                .about("Runs diagnostic commands")
            // No arguments for doctor subcommand for now
        )
        .get_matches();

    match matches.subcommand() {
        Some(("doctor", _)) => {
            // Run the screenfetch command
            execute_command(&["screenfetch".to_string()]);
        }
        _ => {
            if let Some(command) = matches.get_many::<String>("command") {
                let command_parts: Vec<String> = command.map(|s| s.to_string()).collect();
                execute_command(&command_parts);
            }
        }
    }
}

fn execute_command(command_parts: &[String]) {
    // Execute the command
    let status = StdCommand::new(&command_parts[0])
        .args(&command_parts[1..])
        .spawn()
        .expect("Failed to execute command")
        .wait()
        .expect("Failed to wait on child");

    // Check if the command was successful
    if status.success() {
        println!("Command executed successfully");
    } else {
        eprintln!("Command failed with status: {}", status);
    }
}
