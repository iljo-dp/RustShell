use std::process::exit;
use tokio::fs::create_dir_all;
use rustyline::Editor;
use tokio::task;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt, BufReader, BufWriter};

mod prompt;

const HISTORY_FILE: &str = ".local/share/rsh/history";

async fn execute_command(line: String) {
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

async fn execute_pipeline(commands: &[&str]) {
    if !commands.is_empty() {
        let mut child = tokio::process::Command::new(commands[0])
            .args(&commands[1..])
            .stdout(std::process::Stdio::piped())
            .spawn()
            .expect("Failed to start command");

        if let Some(stdout) = child.stdout.take() {
            let mut stdin = io::stdin();
            let mut stdin_writer = io::stdout();

            let mut stdout_reader = BufReader::new(stdout);
            let mut stdin_writer = BufWriter::new(stdin_writer);

            tokio::spawn(async move {
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

#[tokio::main]
async fn main() {
    // Ensure the history directory exists
    if let Some(mut home_dir) = dirs::home_dir() {
        home_dir.push(HISTORY_FILE);
        if let Some(parent) = home_dir.parent() {
            if !parent.exists() {
                create_dir_all(parent).await.expect("Failed to create history directory");
            }
        }
    }

    let mut rl = Editor::<()>::new();
    if rl.load_history(&HISTORY_FILE).is_err() {
        eprintln!("No previous history");
    }

    loop {
        let readline = rl.readline(&prompt::get_prompt());
        match readline {
            Ok(line) => {
                if line.trim() == "exit" {
                    exit(0);
                }

                rl.add_history_entry(line.clone());

                let line_clone = line.clone();
                task::spawn(execute_command(line_clone)).await.unwrap();
            }
            Err(_) => {
                break;
            }
        }
    }

    if let Err(err) = rl.save_history(&HISTORY_FILE) {
        eprintln!("Failed to save history: {}", err);
    }
}

