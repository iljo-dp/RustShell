// main.rs

use std::process::exit;
use tokio::fs::create_dir_all;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use tokio::task;

mod prompt;
mod piping;
mod completion; // Import the completion module

const HISTORY_FILE: &str = ".local/share/rsh/history";

#[tokio::main]
async fn main() {
    // Ensure the history directory exists
    if let Some(mut home_dir) = dirs::home_dir() {
        home_dir.push(HISTORY_FILE);
        if let Some(parent) = home_dir.parent() {
            if !parent.exists() {
                create_dir_all(parent)
                    .await
                    .expect("Failed to create history directory");
            }
        }
    }

    let mut rl = Editor::<()>::new();
    if rl.load_history(&HISTORY_FILE).is_err() {
        eprintln!("No previous history");
    }

    loop {
        let readline = task::block_in_place(|| rl.readline(&prompt::get_prompt()));
        match readline {
            Ok(line) => {
                let mut line = line.trim().to_string();
                if line == "exit" {
                    exit(0);
                }

                rl.add_history_entry(line.clone());

                // Execute command after potential auto-completion
                let line_clone = line.clone();
                task::spawn(piping::execute_command(line_clone)).await.unwrap();
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    if let Err(err) = rl.save_history(&HISTORY_FILE) {
        eprintln!("Failed to save history: {}", err);
    }
}
