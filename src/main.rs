use ignore::{
    gitignore::{Gitignore, GitignoreBuilder},
    WalkBuilder,
};
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

fn should_skip_file(file_name: &str) -> bool {
    matches!(file_name, ".gitignore" | ".flattenignore")
}

fn load_ignore_file(dir: &Path, filename: &str) -> Option<Gitignore> {
    let ignore_path = dir.join(filename);
    if ignore_path.exists() {
        let mut builder = GitignoreBuilder::new(dir);
        match fs::read_to_string(&ignore_path) {
            Ok(content) => {
                for line in content.lines() {
                    let _ = builder.add_line(None, line);
                }
                builder.build().ok()
            }
            Err(_) => None,
        }
    } else {
        None
    }
}

fn is_ignored(
    path: &Path,
    gitignore: &Option<Gitignore>,
    flattenignore: &Option<Gitignore>,
) -> bool {
    // Check gitignore
    if let Some(gi) = gitignore {
        if gi
            .matched_path_or_any_parents(path, path.is_dir())
            .is_ignore()
        {
            return true;
        }
    }

    // Check flattenignore
    if let Some(fi) = flattenignore {
        if fi
            .matched_path_or_any_parents(path, path.is_dir())
            .is_ignore()
        {
            return true;
        }
    }

    false
}

fn main() -> io::Result<()> {
    // Get project path from args or use current directory
    let args: Vec<String> = env::args().collect();
    
    // Check for help flag
    if args.len() > 1 && args[1] == "--help" {
        println!("Usage: flatten [DIRECTORY]");
        println!("\nFlatten copies all files from a directory (and its subdirectories) into a single 'flattened' directory.");
        println!("\nArguments:");
        println!("  DIRECTORY    Optional. Path to the directory to flatten. If not provided, uses current directory.");
        println!("\nIgnore Files:");
        println!("  .gitignore      Files matching patterns in this file will be ignored");
        println!("  .flattenignore  Additional patterns for files to ignore during flattening");
        return Ok(());
    }

    let source_dir = if args.len() > 1 {
        PathBuf::from(&args[1])
    } else {
        env::current_dir()?
    };

    if !source_dir.exists() {
        eprintln!("Error: Directory '{}' does not exist", source_dir.display());
        std::process::exit(1);
    }

    println!("Processing directory: {}", source_dir.display());

    // Load both ignore files
    let gitignore = load_ignore_file(&source_dir, ".gitignore");
    let flattenignore = load_ignore_file(&source_dir, ".flattenignore");

    // Handle existing flattened directory
    let flattened_dir = source_dir.join("flattened");
    if flattened_dir.exists() {
        println!("Removing existing 'flattened' directory...");
        fs::remove_dir_all(&flattened_dir)?;
    }

    // Create new flattened directory
    fs::create_dir(&flattened_dir)?;

    // Set up the walker
    let mut builder = WalkBuilder::new(&source_dir);
    builder
        .hidden(false) // Include hidden files
        .git_ignore(false) // We'll handle ignore files manually
        .require_git(false); // Don't require a git repo

    let walker = builder.build();

    // Process all files
    let mut seen_names: HashSet<String> = HashSet::new();
    let mut processed_count = 0;

    for result in walker {
        match result {
            Ok(entry) => {
                let path = entry.path();

                // Skip the flattened directory itself
                if path.starts_with(&flattened_dir) {
                    continue;
                }

                // Only process files
                if path.is_file() {
                    // Skip ignore files and check ignore patterns
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if should_skip_file(file_name) {
                            continue;
                        }
                    }

                    // Check if file should be ignored
                    if is_ignored(path, &gitignore, &flattenignore) {
                        println!("Ignoring {}", path.display());
                        continue;
                    }

                    // Generate the new flattened filename
                    let relative_path = path
                        .strip_prefix(&source_dir)
                        .unwrap_or(path)
                        .to_string_lossy()
                        .replace(std::path::MAIN_SEPARATOR, "_");

                    // Remove the leading underscore if present
                    let mut flattened_name = relative_path.trim_start_matches('_').to_string();

                    // Handle filename collisions
                    let mut counter = 1;
                    let original_name = flattened_name.clone();
                    while seen_names.contains(&flattened_name) {
                        let file_stem = Path::new(&original_name)
                            .file_stem()
                            .unwrap_or_default()
                            .to_string_lossy();
                        let extension = Path::new(&original_name)
                            .extension()
                            .unwrap_or_default()
                            .to_string_lossy();
                        flattened_name = if extension.is_empty() {
                            format!("{}_{}", file_stem, counter)
                        } else {
                            format!("{}_{}.{}", file_stem, counter, extension)
                        };
                        counter += 1;
                    }

                    seen_names.insert(flattened_name.clone());
                    let new_path = flattened_dir.join(&flattened_name);
                    println!("Copying {} to {}", path.display(), new_path.display());
                    fs::copy(path, new_path)?;
                    processed_count += 1;
                }
            }
            Err(err) => {
                eprintln!("Error walking directory: {}", err);
            }
        }
    }

    println!(
        "Project has been flattened into: {}",
        flattened_dir.display()
    );
    println!("Processed {} files", processed_count);
    Ok(())
}
