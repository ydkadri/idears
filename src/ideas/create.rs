use crate::config::read::Config;
use inquire::{InquireError, Text};
use shellexpand;
use std::fs::{File, create_dir_all};
use std::io::{Error, Write};
use std::process::Command;

fn ensure_ideas_dir_exists(ideas_dir: &str) -> Result<(), Error> {
    if !std::path::Path::new(ideas_dir).exists() {
        println!("Creating new ideas directory at: {}", ideas_dir);
        create_dir_all(ideas_dir)?;
    }

    Ok(())
}

fn create_idea(ideas_dir: &str, editor: &str) -> Result<(), InquireError> {
    match Text::new("Enter the title of your idea:").prompt() {
        Ok(title) => {
            // Construct the file title and path
            let snake_case_title = title.to_lowercase().replace(" ", "_").replace("-", "_");

            let idea_path = format!("{}/{}.md", ideas_dir, snake_case_title);

            // Create the file with a title
            let mut idea_file = File::create(&idea_path).expect("Failed to create idea file");

            // Write the title to the file
            let header = format!("# {}", title);
            idea_file
                .write_all(header.as_bytes())
                .expect("Failed to write to idea file");

            // Open the file in the editor
            let status = Command::new(editor)
                .arg(&idea_path)
                .status()
                .expect("Failed to open editor");

            println!("Created idea file at: {} [{}]", idea_path, status);
        }
        Err(e) => {
            println!("Error: {}", e);
            return Err(e);
        }
    }

    Ok(())
}

pub fn create(config: &Config) -> Result<(), Error> {
    let ideas_dir = shellexpand::tilde(&config.ideas_dir);

    // Ensure the ideas directory exists
    ensure_ideas_dir_exists(&ideas_dir)?;
    create_idea(&ideas_dir, &config.editor).unwrap();

    Ok(())
}
