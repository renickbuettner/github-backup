# GitHub Backup Tool

A simple Rust-based tool for backing up all repositories from a GitHub user or organization as ZIP files.

![Screenshot](https://raw.githubusercontent.com/renickbuettner/github-backup/0e2da43dce82dba8df6582f75075592d60297fcd/assets/screenshot.png)

## Features

- 📦 Downloads repositories as ZIP archives
- 👤 Supports both user and organization repositories
- ⚡ Skip already downloaded repositories

## Installation

```bash
git clone https://github.com/renickbuettner/github-backup.git
cd github-backup
mise install
mise run build
```

## Usage

Set your GitHub token as an environment variable:
```bash
export GITHUB_TOKEN="your_github_token_here"
```

Run the backup:
```bash
cargo run --release -- --owner yourusername
cargo run --release -- --owner yourorg --owner-type org
```

### Options

| Option | Description |
|--------|-------------|
| `--token, -t` | GitHub personal access token (or use `GITHUB_TOKEN` env var) |
| `--owner, -o` | GitHub username or organization name (required) |
| `--owner-type` | `user` or `org` (default: `user`) |
| `--output` | Output directory (default: `data`) |

## GitHub Token

You need a GitHub Personal Access Token with `repo` scope (and `read:org` for organizations).

See [GITHUB_TOKEN.md](GITHUB_TOKEN.md) for detailed instructions.

## License

MIT

