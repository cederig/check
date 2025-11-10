use std::fs;
use std::io::{Read};
use std::path::Path;
use anyhow::{Context, Result};
use sha2::Sha256;
use md5::{Digest, Md5};
use infer;
use charset_normalizer_rs::from_bytes;

const INFER_BUFFER_SIZE: usize = 4096;

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    pub size: u64,
    pub formatted_size: String,
    pub file_type: String,
    pub encoding: String,
    pub sha256: String,
    pub md5: String,
}

pub fn format_size(bytes: u64) -> String {
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
        format!("{:.2} PB", bytes as f64 / EB as f64)
    } else {
        format!("{:.2} EB", bytes as f64 / EB as f64)
    }
}

pub fn process_file(path: &Path) -> Result<FileInfo> {
    let mut file = fs::File::open(path).context("Failed to open file")?;
    let metadata = file.metadata().context("Failed to read metadata")?;
    let size = metadata.len();
    let formatted_size = format_size(size);

    let mut sha256_hasher = Sha256::new();
    let mut md5_hasher = Md5::new();

    // Read a small chunk for inferring file type and encoding
    let mut infer_buffer = vec![0; INFER_BUFFER_SIZE];
    let bytes_read = file.read(&mut infer_buffer).context("Failed to read file chunk for inference")?;
    infer_buffer.truncate(bytes_read); // Adjust buffer size to actual bytes read

    // Update hashers with the initial buffer used for inference
    sha256_hasher.update(&infer_buffer);
    md5_hasher.update(&infer_buffer);

    // File Type
    let file_type = infer::get(&infer_buffer)
        .map_or_else(|| "unknown".to_string(), |t| t.mime_type().to_string());

    // Encoding
    let result = from_bytes(&infer_buffer, None);
    let best_guess = result.unwrap();
    let get_best = best_guess.get_best();
    let name = match get_best {
        Some(m) => m.encoding(),
        None => "unknown",
    };
    let encoding_name = format!("{:?}", name);

    // Continue reading the rest of the file in chunks for checksums
    let mut buffer = vec![0; INFER_BUFFER_SIZE];
    loop {
        let bytes_read = file.read(&mut buffer).context("Failed to read file chunk for hashing")?;
        if bytes_read == 0 {
            break;
        }
        sha256_hasher.update(&buffer[..bytes_read]);
        md5_hasher.update(&buffer[..bytes_read]);
    }

    // Hashes
    let sha256 = format!("{:x}", sha256_hasher.finalize());
    let md5 = format!("{:x}", md5_hasher.finalize());

    Ok(FileInfo {
        size,
        formatted_size,
        file_type,
        encoding: encoding_name,
        sha256,
        md5,
    })
}
