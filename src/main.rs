use clap::{Arg, Command};
use reqwest::{Client, header::RANGE};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use indicatif::ProgressBar;
use futures::stream::StreamExt;

async fn download_chunk(
    url: &str,
    client: &Client,
    part: usize,
    range_start: u64,
    range_end: u64,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let range_header_value = format!("bytes={}-{}", range_start, range_end);
    let response = client.get(url)
        .header(RANGE, range_header_value)
        .send()
        .await?;

    if !response.status().is_success() {
        eprintln!("Failed to download part {}: {}", part, response.status());
        return Err("Failed to download chunk".into());
    }

    let bytes = response.bytes().await?.to_vec();
    Ok(bytes)
}

async fn download_file(
    url: &str,
    client: &Client,
    total_size: u64,
    num_parts: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    let chunk_size = total_size / num_parts as u64;
    let pb = ProgressBar::new(total_size);

    let mut futures = Vec::new();
    for part in 0..num_parts {
        let range_start = part as u64 * chunk_size;
        let range_end = if part == num_parts - 1 {
            total_size - 1  // last part may be larger
        } else {
            range_start + chunk_size - 1
        };
        futures.push(download_chunk(url, client, part, range_start, range_end));
    }

    // download parts concurrently
    let mut parts_data = futures::future::join_all(futures).await;

    // handle any failed part downloads
    if parts_data.iter().any(|r| r.is_err()) {
        eprintln!("Some parts failed to download");
        return Err("Download failed".into());
    }

    // merge parts and save to a file
    let filename = "output_file.download";
    let mut file = File::create(filename).await?;

    for part_data in parts_data {
        let data = part_data?;
        file.write_all(&data).await?;
        pb.inc(data.len() as u64);
    }

    pb.finish_with_message("Download completed!");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // cli setup using clap 4.0+
    let matches = Command::new("Concurrent File Downloader")
        .version("1.0")
        .author("Aidan Blancas")
        .about("downloads multiple files concurrently in parts")
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .value_name("URL")
                .help("URL of the file to download")
                .required(true),
        )
        .arg(
            Arg::new("parts")
                .short('p')
                .long("parts")
                .value_name("NUM")
                .help("number of parts to split the download into")
                .default_value("4"),
        )
        .get_matches();

    let url = matches.get_one::<String>("url").unwrap();
    let num_parts: usize = matches.get_one::<String>("parts").unwrap().parse()?;

    let client = Client::new();

    // get total file size first
    let response = client.head(url).send().await?;
    let total_size = response
        .headers()
        .get("content-length")
        .ok_or("No content-length header")?
        .to_str()?
        .parse::<u64>()?;

    // download file in parts
    download_file(url, &client, total_size, num_parts).await?;

    Ok(())
}
