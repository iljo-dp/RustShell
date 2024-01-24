// piping.rs

use tokio::io::{self, BufReader, BufWriter};

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
    if !commands.is_empty() {
        let mut child = tokio::process::Command::new(commands[0])
            .args(&commands[1..])
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to start command");

        if let Some(stdout) = child.stdout.take() {
            let stdin_writer = io::stdout();
            let mut stdout_reader = BufReader::new(stdout);
            let mut stdin_writer = BufWriter::new(stdin_writer);

            let _ = tokio::spawn(async move {
                if let Err(err) = io::copy(&mut stdout_reader, &mut stdin_writer).await {
                    eprintln!("Failed to copy: {}", err);
                }
            })
            .await;

            if let Err(err) = child.wait().await {
                eprintln!("Command failed with exit code: {}", err);
            }
        }
    }
}

