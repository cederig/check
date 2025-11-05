# check

`check` is a simple and fast command-line tool written in Rust to get information about a file or all files in a directory.

## Features

- Get the size of a file.
- Identify the file's MIME type.
- Detect the character encoding of a file.
- Calculate SHA256 and MD5 checksums.
- Process a single file or all files within a directory.

## Dependencies

The project uses the following main Rust crates:

- `clap` for command-line argument parsing.
- `anyhow` and `thiserror` for error handling.
- `sha2` and `md5` for calculating checksums.
- `infer` for detecting file types.
- `chardet` for detecting character encodings.

## Installation

### Prerequisites

Make sure you have Rust and Cargo installed on your system. You can install them by following the instructions on the official Rust website: [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)

### Compiling for Linux (from Linux)
1.  Clone this repository:
    ```sh
    git clone https://github.com/cederig/check.git
    cd check
    ```
2.  Compile the project:
    ```sh
    cargo build --release
    ```
    The executable will be located in `target/release/check`.

### Compiling for Windows (from Linux/macOS)

To cross-compile this project for Windows from another operating system (like Linux or macOS), you will need the Rust target for Windows.

1.  Add the Windows target to your Rust installation:
    ```sh
    rustup target add x86_64-pc-windows-gnu
    ```

2.  Compile the project for the Windows target:
    ```sh
    cargo build --release --target=x86_64-pc-windows-gnu
    ```

The Windows executable will be located in `target/x86_64-pc-windows-gnu/release/check.exe`.

## Usage

Run the program from the command line, passing the path to a file or a directory as an argument.

### Arguments

- `<PATH>`: The path to the file or directory to inspect.

### Options

The tool supports the default options from `clap`:

- `-h`, `--help`: Print help information.
- `-V`, `--version`: Print version information.
- `-r`, `--recursive`: Process directories recursively.
- `--sha`: Show SHA256 checksum.
- `--md5`: Show MD5 checksum.

## Examples

### 1. Checking a single file (with SHA256)

```sh
./target/release/check --sha ./my_file.txt
```

**Example Output:**

```
--- File: ./my_file.txt ---
  Size: 1.21 KB
  Type: text/plain
  Encoding: UTF-8
  SHA256: <sha256_hash>
--------------------
```

### 2. Checking all files in a directory (with both hashes)

```sh
./target/release/check --sha --md5 ./my_directory
```

**Example Output:**

```
Processing directory: ./my_directory

--- File: ./my_directory/file1.jpg ---
  Size: 5.54 KB
  Type: image/jpeg
  Encoding: ASCII
  SHA256: <sha256_hash_1>
  MD5: <md5_hash_1>
--------------------

--- File: ./my_directory/document.pdf ---
  Size: 88.88 KB
  Type: application/pdf
  Encoding: ASCII
  SHA256: <sha256_hash_2>
  MD5: <md5_hash_2>
--------------------

```

### 3. Checking all files in a directory recursively

```sh
./target/release/check -r --sha --md5 ./my_directory
```

**Example Output:**

```
Processing directory: ./my_directory

Processing directory: ./my_directory/subdir1

--- File: ./my_directory/subdir1/file_in_subdir.txt ---
  Size: 1.21 KB
  Type: text/plain
  Encoding: UTF-8
  SHA256: <sha256_hash_3>
  MD5: <md5_hash_3>
--------------------

Processing directory: ./my_directory/subdir2

--- File: ./my_directory/file1.jpg ---
  Size: 5.54 KB
  Type: image/jpeg
  Encoding: ASCII
  SHA256: <sha256_hash_1>
  MD5: <md5_hash_1>
--------------------

--- File: ./my_directory/document.pdf ---
  Size: 88.88 KB
  Type: application/pdf
  Encoding: ASCII
  SHA256: <sha256_hash_2>
  MD5: <md5_hash_2>
--------------------

```
