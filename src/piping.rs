use std::io::{Error, ErrorKind};
use tokio::io::{self, AsyncBufReadExt, AsyncWriteExt};

pub async fn execute_command(line: String) {
    let mut parts = line.trim().split_whitespace();
    let mut commands: Vec<&str> = Vec::new();

    while let Some(command) = parts.next() {
        if command == "|" {
            execute_pipeline(&commands).await;
            commands.clear();
        } else {
            commands.push(command);
        }
    }

    if !commands.is_empty() {
        execute_pipeline(&commands).await;
    }
}

pub async fn execute_pipeline(commands: &[&str]) {
    if commands.is_empty() {
        return;
    }

    let command = commands[0];

    if let Err(err) = is_command_available(command).await {
        eprintln!("Command '{}' not found: {}", command, err);
        return;
    }

    let child = tokio::process::Command::new(command)
        .args(&commands[1..])
        .stdout(std::process::Stdio::piped())
        .spawn();

    match child {
        Ok(mut child) => {
            if let Some(stdout) = child.stdout.take() {
                let mut stdout_reader = io::BufReader::new(stdout);
                let mut stdout_writer = io::stdout();

                if let Err(err) = io::copy(&mut stdout_reader, &mut stdout_writer).await {
                    eprintln!("Failed to copy output: {}", err);
                }
            }

            if let Err(err) = child.wait().await {
                eprintln!("Command failed with exit code: {}", err);
            }
        }
        Err(err) => {
            eprintln!("Failed to start command '{}': {}", command, err);
        }
    }
}

async fn is_command_available(command: &str) -> Result<(), Error> {
    let command_path = match which::which(command) {
        Ok(path) => path,
        Err(_) => {
            return Err(Error::new(
                ErrorKind::NotFound,
                format!("Command '{}' not found", command),
            ));
        }
    };

    if command_path.to_str() == Some("") {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("Command '{}' not found", command),
        ));
    }

    Ok(())
}

