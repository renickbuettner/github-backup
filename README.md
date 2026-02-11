# GitHub Backup Tool

A simple Rust-based tool for backing up all repositories from a GitHub user or organization as ZIP files.

## Features

- 🦀 Written in Rust for performance and reliability
- 📦 Downloads repositories as ZIP archives
- 👤 Supports both user and organization repositories
- 🔄 Automatic pagination for accounts with many repositories
- ⚡ Skip already downloaded repositories
- 🛠️ Easy setup with Mise

## Prerequisites

- [Mise](https://mise.jdx.dev/) - Development environment manager
- A GitHub Personal Access Token (see [GITHUB_TOKEN.md](GITHUB_TOKEN.md) for instructions)

## Installation

1. Clone this repository:
```bash
git clone https://github.com/renickbuettner/github-backup.git
cd github-backup
```

2. Install Rust using Mise:
```bash
mise install
```

3. Build the project:
```bash
mise run build
```

## Usage

### Using Environment Variables (Recommended)

Set your GitHub token as an environment variable:
```bash
export GITHUB_TOKEN="your_github_token_here"
```

Then run the backup for a user:
```bash
cargo run --release -- --owner yourusername
```

Or for an organization:
```bash
cargo run --release -- --owner yourorg --owner-type org
```

### Using Command Line Arguments

```bash
cargo run --release -- --token your_token --owner yourusername
```

### Options

- `--token, -t`: GitHub personal access token (can also use `GITHUB_TOKEN` env var)
- `--owner, -o`: GitHub username or organization name (required)
- `--owner-type`: Type of owner - either `user` or `org` (default: `user`)
- `--output`: Output directory for backups (default: `data`)

### Examples

Backup all repositories for a user:
```bash
cargo run --release -- --owner octocat
```

Backup all repositories for an organization:
```bash
cargo run --release -- --owner github --owner-type org
```

Backup to a custom directory:
```bash
cargo run --release -- --owner octocat --output /path/to/backups
```

## Development

### Build

```bash
cargo build
```

### Run in development mode

```bash
cargo run -- --owner yourusername
```

### Using Mise tasks

Build the project:
```bash
mise run build
```

Clean build artifacts:
```bash
mise run clean
```

## Output

All repository backups are saved as ZIP files in the `data/` directory (or your custom output directory) with the following naming convention:

```
{username}_{repository-name}_{last-update-date}.zip
```

For example:
```
octocat_Hello-World_2023-01-15.zip
```

## GitHub Token

You need a GitHub Personal Access Token with the following permissions:

- For **user repositories**: `repo` scope
- For **organization repositories**: `repo` and `read:org` scopes

See [GITHUB_TOKEN.md](GITHUB_TOKEN.md) for detailed instructions on how to generate a token.

## License

ISC

## Author

renickbuettner

