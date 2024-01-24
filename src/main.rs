use std::io::{self, Write};
use std::process::exit;
use std::fs::create_dir_all;
use rustyline::Editor;
use tokio::task;

mod prompt;

const HISTORY_FILE: &str = ".local/share/rsh/history";

async fn execute_command(line: String) {
    let mut parts = line.trim().split_whitespace();
    if let Some(command) = parts.next() {
        let args: Vec<&str> = parts.collect();

        let status = tokio::process::Command::new(command)
            .args(args)
            .status()
            .await;

        match status {
            Ok(status) => {
                if !status.success() {
                    eprintln!("Command failed with exit code: {}", status);
                }
            }
            Err(err) => {
                eprintln!("Failed to execute command: {}", err);
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
                create_dir_all(parent).expect("Failed to create history directory");
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

                // Clone the line before passing it to the asynchronous task
                let line_clone = line.clone();
                // Spawn a Tokio task to execute the command asynchronously
                task::spawn(execute_command(line_clone)).await.unwrap();
            }
            Err(_) => {
                // Handle Ctrl+C or Ctrl+D (EOF)
                break;
            }
        }
    }

    if let Err(err) = rl.save_history(&HISTORY_FILE) {
        eprintln!("Failed to save history: {}", err);
    }
}

