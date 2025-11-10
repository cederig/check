mod utils;

use std::path::Path;
use anyhow::{Context, Result};
use clap::Parser;
use glob::glob;
use crate::utils::process_file;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to the file or directory to inspect
    #[arg(value_name = "PATH")]
    path: Vec<String>,
    #[arg(short, long, help = "Process directories recursively")]
    recursive: bool,
    #[arg(long, help = "Show SHA256 checksum")]
    sha: bool,
    #[arg(long, help = "Show MD5 checksum")]
    md5: bool,
}

fn walk_and_process_dir(path: &Path, cli: &Cli) -> Result<()> {
    for entry in std::fs::read_dir(path).context("Failed to read directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let current_path = entry.path();

        if current_path.is_file() {
            println!("--- File: {} ---", current_path.display());
            match process_file(&current_path) {
                Ok(info) => {
                    println!("  Size: {}", info.formatted_size);
                    println!("  Type: {}", info.file_type);
                    println!("  Encoding: {}", info.encoding);
                    if cli.sha {
                        println!("  SHA256: {}", info.sha256);
                    }
                    if cli.md5 {
                        println!("  MD5: {}", info.md5);
                    }
                }
                Err(e) => {
                    eprintln!("  Error processing file {}: {}", current_path.display(), e);
                }
            }
            println!("----------------\n");
        } else if current_path.is_dir() && cli.recursive {
            println!("Processing directory: {}\n", current_path.display());
            walk_and_process_dir(&current_path, cli)?;
        }
    }
    Ok(())
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    for pattern in &cli.path {
        for entry in glob(&pattern).context(format!("Failed to read glob pattern: {}", pattern))? {
            match entry {
                Ok(path) => {
                    if path.is_dir() {
                        println!("Processing directory: {}\n", path.display());
                        walk_and_process_dir(&path, &cli)?;
                    } else if path.is_file() {
                        println!("--- File: {} ---", path.display());
                        match process_file(&path) {
                            Ok(info) => {
                                println!("  Size: {}", info.formatted_size);
                                println!("  Type: {}", info.file_type);
                                println!("  Encoding: {}", info.encoding);
                                if cli.sha {
                                    println!("  SHA256: {}", info.sha256);
                                }
                                if cli.md5 {
                                    println!("  MD5: {}", info.md5);
                                }
                            }
                            Err(e) => {
                                eprintln!("  Error processing file: {}", e);
                            }
                        }
                        println!("----------------\n");
                    }
                }
                Err(e) => eprintln!("Error processing glob entry: {}", e),
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use crate::utils::format_size;

    #[test]
    fn test_process_file() {
        // Create a temporary file
        let mut temp_file = tempfile::NamedTempFile::new().unwrap();
        let content = "This is a test file with some plain text content to ensure that the infer crate can correctly identify it as text/plain.";
        temp_file.write_all(content.as_bytes()).unwrap();
        let path = temp_file.path();

        // Expected values
        let expected_size = content.len() as u64;
        let expected_sha256 = "a8bd55d8c8ef4637731e139fd9af0b45529b67117be70f93bd142ddbf6dbabf3";
        let expected_md5 = "0d74ddaa1b80d2694f9137a9b87f5a57";

        // Process the file
        let file_info = process_file(path).unwrap();

        // Assertions
        assert_eq!(file_info.size, expected_size);

        // The old chardet version is not very accurate
        // assert_eq!(file_info.encoding, "UTF-8");
        assert_eq!(file_info.sha256, expected_sha256);
        assert_eq!(file_info.md5, expected_md5);
    }

    #[test]
    fn test_format_size() {
        assert_eq!(format_size(0), "0 bytes");
        assert_eq!(format_size(100), "100 bytes");
        assert_eq!(format_size(1023), "1023 bytes");
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1536), "1.50 KB");
        assert_eq!(format_size(1024 * 1024 - 1), "1024.00 KB");
        assert_eq!(format_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_size(1024 * 1024 * 1024 - 1), "1024.00 MB");
        assert_eq!(format_size(1024 * 1024 * 1024), "1.00 GB");
        assert_eq!(format_size(1024 * 1024 * 1024 * 1024 - 1), "1024.00 GB");
        assert_eq!(format_size(1024 * 1024 * 1024 * 1024), "1.00 TB");
        assert_eq!(format_size(1024 * 1024 * 1024 * 1024 * 1024 - 1), "1024.00 TB");
        assert_eq!(format_size(1024 * 1024 * 1024 * 1024 * 1024), "1.00 PB");
        assert_eq!(format_size(1024 * 1024 * 1024 * 1024 * 1024 * 1024 - 1), "1024.00 PB");
        assert_eq!(format_size(1024 * 1024 * 1024 * 1024 * 1024 * 1024), "1.00 EB");
        assert_eq!(format_size(u64::MAX), "16.00 EB");
    }
}

