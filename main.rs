use std::process::Command;
use std::io::{self, Write};

fn main() {
    println!("Welcome to Rust YouTube Downloader!");
    
    // Ask for the YouTube video URL
    let mut url = String::new();
    print!("Enter the YouTube video URL: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut url).unwrap();
    let url = url.trim();

    // Use yt-dlp (or youtube-dl) to download the video
    println!("Downloading video...");
    let output = Command::new("yt-dlp")
        .arg("-f")
        .arg("best") // Best video format
        .arg("-o")
        .arg("%(title)s.%(ext)s") // Save with video title as filename
        .arg(url)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                println!("Download complete!");
            } else {
                eprintln!(
                    "Error downloading video: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        Err(e) => eprintln!("Failed to run yt-dlp: {}", e),
    }
}
