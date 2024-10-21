# Concurrent Multi-part File Downloader

**Concurrent Multi-part File Downloader** is a Rust-based command-line tool that downloads large files from the internet by splitting them into parts and downloading them concurrently. This approach speeds up the download process and efficiently utilizes available bandwidth.

## Features:
- **Multi-part Downloads**: Download large files in multiple parts for faster and more efficient transfers.
- **Progress Tracking**: Visual feedback with a progress bar showing overall download progress.
- **Customizable**: Specify the number of parts to split the file into and the file URL via command-line arguments.
- **Error Handling**: Gracefully handles errors, with retries for failed parts.

## Getting Started

### Prerequisites
- Rust and Cargo installed. If you don’t have Rust installed, you can get it [here](https://www.rust-lang.org/tools/install).

### Installation
1. Clone this repository:
    ```bash
    git clone https://github.com/yourusername/concurrent-file-downloader.git
    cd concurrent-file-downloader
    ```

2. Build the project:
    ```bash
    cargo build --release
    ```

3. (Optional) Run tests:
    ```bash
    cargo test
    ```

## Usage

To download a large file in parts, run the following command:

```bash
cargo run --release -- -u <URL> -p <parts>
