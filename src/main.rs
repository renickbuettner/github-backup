use anyhow::{Context, Result};
use clap::Parser;
use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;
use std::fs::{self, OpenOptions};
use std::io::{Write, Read};
use std::path::Path;
use std::time::Instant;

// Constants
const PER_PAGE: u32 = 100;
const LOG_FILENAME: &str = "transition.log";

/// GitHub Backup Tool - Downloads all repositories from a user or organization
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// GitHub personal access token
    #[arg(short, long, env = "GITHUB_TOKEN")]
    token: Option<String>,

    /// GitHub username or organization name
    #[arg(short, long)]
    owner: String,

    /// Type of owner (user or org)
    #[arg(short = 't', long, default_value = "user")]
    owner_type: String,

    /// Output directory for backups
    #[arg(short, long, default_value = "data")]
    output: String,
}

#[derive(Debug, Deserialize)]
struct Repository {
    name: String,
    full_name: String,
    updated_at: String,
    #[serde(rename = "default_branch")]
    default_branch: String,
}

/// Utility function to log messages to both console and log file
fn log_message(message: &str, is_error: bool) {
    // Display to console (errors in red)
    if is_error {
        eprintln!("{}", message.red());
    } else {
        println!("{}", message);
    }

    // Write to log file
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILENAME)
    {
        if let Err(e) = writeln!(file, "{}", message) {
            eprintln!("Warning: Failed to write to log file: {}", e);
        }
    }
}

fn create_http_client(token: &str) -> Result<reqwest::blocking::Client> {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("token {}", token))?,
    );
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("github-backup-rust/1.0"),
    );
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/vnd.github.v3+json"),
    );

    reqwest::blocking::Client::builder()
        .default_headers(headers)
        .build()
        .context("Failed to create HTTP client")
}

fn fetch_repositories(client: &reqwest::blocking::Client, owner: &str, owner_type: &str) -> Result<Vec<Repository>> {
    let mut all_repos = Vec::new();
    let mut page = 1;

    loop {
        let url = match owner_type {
            "org" => format!(
                "https://api.github.com/orgs/{}/repos?per_page={}&page={}&sort=updated&direction=desc",
                owner, PER_PAGE, page
            ),
            _ => format!(
                "https://api.github.com/users/{}/repos?per_page={}&page={}&sort=updated&direction=desc",
                owner, PER_PAGE, page
            ),
        };

        log_message(&format!("[Backup] Fetching page {} of repositories...", page), false);

        let response = client
            .get(&url)
            .send()
            .context("Failed to fetch repositories")?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(
                "GitHub API request failed with status: {}",
                response.status()
            ));
        }

        let repos: Vec<Repository> = response.json().context("Failed to parse JSON response")?;

        if repos.is_empty() {
            break;
        }

        all_repos.extend(repos);
        page += 1;
    }

    Ok(all_repos)
}

fn download_repository_zip(
    client: &reqwest::blocking::Client,
    owner: &str,
    repo_name: &str,
    default_branch: &str,
    output_dir: &Path,
    updated_at: &str,
) -> Result<()> {
    let date = updated_at.split('T').next().unwrap();
    let safe_repo_name = repo_name.replace('/', "_");
    let filename = format!("{}_{}_{}.zip", owner, safe_repo_name, date);
    let output_path = output_dir.join(&filename);

    // Skip if file already exists
    if output_path.exists() {
        log_message(&format!("[Backup] Skipping {} (already exists)", filename), false);
        return Ok(());
    }

    let url = format!(
        "https://api.github.com/repos/{}/{}/zipball/{}",
        owner, repo_name, default_branch
    );

    let mut response = client
        .get(&url)
        .send()
        .context(format!("Failed to download repository: {}", repo_name))?;

    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to download {}: HTTP {}",
            repo_name,
            response.status()
        ));
    }

    // Get content length if available
    let total_size = response.content_length().unwrap_or(0);
    
    // Create progress bar for this download
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("  [{bar:30.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}) {msg}")
            .expect("Failed to create download progress bar template")
            .progress_chars("=>-"),
    );
    pb.set_message(filename.clone());

    // Download with progress tracking
    let start_time = Instant::now();
    let mut downloaded: u64 = 0;
    let mut file = fs::File::create(&output_path)
        .context(format!("Failed to create file: {}", filename))?;

    let mut buffer = [0u8; 8192];
    loop {
        let bytes_read = response.read(&mut buffer)
            .context("Failed to read response body")?;
        
        if bytes_read == 0 {
            break;
        }
        
        file.write_all(&buffer[..bytes_read])
            .context("Failed to write to file")?;
        
        downloaded += bytes_read as u64;
        pb.set_position(downloaded);
    }

    pb.finish_and_clear();

    let elapsed = start_time.elapsed().as_secs_f64();
    let speed_mbps = if elapsed > 0.0 {
        (downloaded as f64 / 1_048_576.0) / elapsed
    } else {
        0.0
    };

    log_message(
        &format!(
            "[Backup] ✓ Downloaded {} ({} bytes, {:.2} MB/s)",
            filename, downloaded, speed_mbps
        ),
        false,
    );

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Get token from args or environment variable
    let token = args.token.ok_or_else(|| {
        anyhow::anyhow!("GitHub token is required. Set GITHUB_TOKEN environment variable or use --token flag")
    })?;

    log_message("[Backup] Starting GitHub backup", false);
    log_message(&format!("[Backup] Owner: {}", args.owner), false);
    log_message(&format!("[Backup] Owner Type: {}", args.owner_type), false);
    log_message(&format!("[Backup] Output Directory: {}", args.output), false);

    // Create output directory if it doesn't exist
    fs::create_dir_all(&args.output)
        .context(format!("Failed to create output directory: {}", args.output))?;

    // Create HTTP client
    let client = create_http_client(&token)?;

    // Fetch all repositories
    log_message("[Backup] Fetching repositories...", false);
    let repositories = fetch_repositories(&client, &args.owner, &args.owner_type)?;

    log_message(
        &format!("[Backup] Found {} repositories to backup", repositories.len()),
        false,
    );

    let output_path = Path::new(&args.output);

    // Create progress bar
    let progress_bar = ProgressBar::new(repositories.len() as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .expect("Failed to create progress bar template")
            .progress_chars("=>-"),
    );

    // Download each repository
    for repo in repositories.iter() {
        progress_bar.set_message(format!("Processing {}", repo.full_name));

        if let Err(e) = download_repository_zip(
            &client,
            &args.owner,
            &repo.name,
            &repo.default_branch,
            output_path,
            &repo.updated_at,
        ) {
            let error_msg = format!("[Error] Failed to backup {}: {}", repo.full_name, e);
            log_message(&error_msg, true);
            // Continue with next repository instead of failing completely
        }

        progress_bar.inc(1);
    }

    progress_bar.finish_with_message("Complete");

    log_message("[Backup] ✓ Backup complete!", false);

    Ok(())
}
