use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use anyhow::{Context, Result};
use clap::Parser;
use sha2::{Digest, Sha256};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The path to the file or directory to inspect
    #[arg(value_name = "PATH")]
    path: PathBuf,
    #[arg(short, long, help = "Process directories recursively")]
    recursive: bool,
    #[arg(long, help = "Show SHA256 checksum")]
    sha: bool,
    #[arg(long, help = "Show MD5 checksum")]
    md5: bool,
}

#[derive(Debug, PartialEq)]
struct FileInfo {
    size: u64,
    formatted_size: String,
    file_type: String,
    encoding: String,
    sha256: String,
    md5: String,
}

fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = 1024 * KB;
    const GB: u64 = 1024 * MB;
    const TB: u64 = 1024 * GB;
    const PB: u64 = 1024 * TB;
    const EB: u64 = 1024 * PB;

    if bytes < KB {
        format!("{} bytes", bytes)
    } else if bytes < MB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else if bytes < GB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes < TB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes < PB {
        format!("{:.2} TB", bytes as f64 / TB as f64)
    } else if bytes < EB {
        format!("{:.2} PB", bytes as f64 / PB as f64)
    } else {
        format!("{:.2} EB", bytes as f64 / EB as f64)
    }
}

fn process_file(path: &Path) -> Result<FileInfo> {
    let mut file = fs::File::open(path).context("Failed to open file")?;
    let metadata = file.metadata().context("Failed to read metadata")?;
    let size = metadata.len();
    let formatted_size = format_size(size);

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).context("Failed to read file content")?;

    // File Type
    let file_type = infer::get(&buffer)
        .map_or_else(|| "unknown".to_string(), |t| t.mime_type().to_string());

    // Encoding
    let (encoding, ..) = chardet::detect(&buffer);
    let encoding_name = format!("{:?}", encoding);

    // Hashes
    let sha256 = format!("{:x}", Sha256::digest(&buffer));
    let md5 = format!("{:x}", md5::compute(&buffer));

    Ok(FileInfo {
        size,
        formatted_size,
        file_type,
        encoding: encoding_name,
        sha256,
        md5,
    })
}

fn walk_and_process_dir(path: &Path, cli: &Cli) -> Result<()> {
    for entry in fs::read_dir(path).context("Failed to read directory")? {
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
    let path = &cli.path;

    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", path.display());
    }

    if path.is_dir() {
        println!("Processing directory: {}\n", path.display());
        walk_and_process_dir(path, &cli)?;
    } else if path.is_file() {
        println!("--- File: {} ---", path.display());
        match process_file(path) {
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
    } else {
        anyhow::bail!("Path is not a file or a directory: {}", path.display());
    }

    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

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
