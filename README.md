Concurrent Multi-part File Downloader

Concurrent Multi-part File Downloader is a Rust-based command-line tool that downloads large files from the internet by splitting them into parts and downloading them concurrently. This approach speeds up the download process and efficiently utilizes available bandwidth.

Features:
* Multi-part Downloads: Download large files in multiple parts for faster and more efficient transfers.
* Progress Tracking: Visual feedback with a progress bar showing overall download progress.
  * Customizable: Specify the number of parts to split the file into and the file URL via command-line arguments.
* Error Handling: Gracefully handles errors, with retries for failed parts.
Getting Started

Prerequisites
Rust and Cargo installed. If you don’t have Rust installed, you can get it here.
Installation
Clone this repository:
bash
Copy code
git clone https://github.com/yourusername/concurrent-file-downloader.git
cd concurrent-file-downloader
Build the project:
bash
Copy code
cargo build --release
(Optional) Run tests:
bash
Copy code
cargo test
Usage
To download a large file in parts, run the following command:

bash
Copy code
cargo run --release -- -u <URL> -p <parts>
Example:

bash
Copy code
cargo run --release -- -u "https://www.w3.org/WAI/ER/tests/xhtml/testfiles/resources/pdf/dummy.pdf" -p 4
-u: The URL of the file to download.
-p: The number of parts to split the file into (default is 4).
Example Outputs:
bash
Copy code
[ 100/100 ] Downloading part 1
[ 500/500 ] Downloading part 2
[ 750/750 ] Downloading part 3
[ 1000/1000 ] Downloading part 4
Download completed!
File Location
The downloaded file will be saved in the current directory under the name output_file.download.
