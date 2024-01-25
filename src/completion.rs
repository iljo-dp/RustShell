// completion.rs

use rustyline::KeyEvent; // Import KeyEvent directly from rustyline

use std::fs;
use std::path::PathBuf;

/// Provides auto-completion suggestions based on user input.
pub fn autocomplete(input: &str) -> Option<String> {
    // Get the current directory
    let current_dir = std::env::current_dir().ok()?;

    // Get all entries in the current directory
    let entries = fs::read_dir(&current_dir).ok()?;
    
    // Collect matching entries
    let matches: Vec<PathBuf> = entries
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|path| {
            if let Some(file_name) = path.file_name() {
                if let Some(name) = file_name.to_str() {
                    return name.starts_with(input);
                }
            }
            false
        })
        .collect();

    // If there's only one match, return it
    if matches.len() == 1 {
        if let Some(file_name) = matches[0].file_name() {
            if let Some(name) = file_name.to_str() {
                return Some(name.to_string());
            }
        }
    }

    None
}

/// Handles key events, particularly the Tab key press event for auto-completion.
pub fn handle_key_event(input: &mut String, key: KeyEvent) {
    if key == KeyEvent::Tab {
        if let Some(autocompleted) = autocomplete(input) {
            *input = autocompleted;
        }
    }
}