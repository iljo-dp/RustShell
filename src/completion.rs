use std::path::{Path, PathBuf};
use tokio::fs;

pub async fn autocomplete(input: &str) -> Option<String> {
    let current_dir = std::env::current_dir().ok()?;
    let parent_dir = Path::new(input).parent().unwrap_or(&current_dir);

    let mut entries = fs::read_dir(parent_dir).await.ok()?.filter_map(|entry| {
        entry.ok().and_then(|e| {
            e.file_name().into_string().ok()
        })
    });

    let mut matching_files = Vec::new();
    while let Some(entry) = entries.next().await {
        if entry.starts_with(input) {
            matching_files.push(entry);
        }
    }

    if matching_files.len() == 1 {
        Some(matching_files[0].clone())
    } else {
        None
    }
}
pub fn handle_key_event(input: &mut String, key: rustyline::KeyEvent) {
    match key.code {
        rustyline::KeyCode::Tab => {
            if let Some(autocompleted) = futures::executor::block_on(autocomplete(input)) {
                *input = autocompleted;
            }
        }
        _ => {}
    }
}

