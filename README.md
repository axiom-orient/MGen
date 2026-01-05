# MGen (Mobile Generator)

Multi-platform mobile project generator handling generic scaffolding for iOS (via Tuist) and Android (via TMAndroid/Cookiecutter).

## Project Structure

```
MGen/
├── service/                 # Rust Web Server (Application Code)
│   ├── Cargo.toml
│   └── src/
├── templates/               # Project Templates
│   └── android/             # Android Cookiecutter Template
├── scripts/                 # Scaffold Wrapper Scripts
├── generated/               # Output Directory (Gitignored)
└── logs/                    # Generation Logs
```

## Prerequisites

### Common Requirements

- **Rust**: Required for running the web server
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### iOS Project Generation

- **macOS**: Required for iOS development
- **Tuist**: Recommended installation via `mise`
  ```bash
  # Install mise (if not already installed)
  brew install mise
  
  # Tuist will be auto-installed by the scaffold script
  # Or manually install:
  mise use --global tuist@latest
  ```

### Android Project Generation

- **Cookiecutter**: Auto-installs via Homebrew if missing
  ```bash
  # Manual installation (optional):
  brew install cookiecutter
  ```
- **Python 3.8+**: Required for cookiecutter hooks
  ```bash
  brew install python3
  ```

## Features

### 🚀 Phase 3 Enhancements

MGen includes advanced async features for improved user experience:

- **Asynchronous Generation**: Projects are generated in the background, allowing you to monitor progress without blocking the UI
- **Real-time Status Polling**: HTMX-powered polling shows live generation status updates every second
- **ZIP Download**: Download generated projects as compressed archives directly from the result dashboard
- **GitHub Integration**: Automatically create a private GitHub repository and push your generated project with a single click
- **Task State Management**: Thread-safe task tracking with UUID-based identification


## Usage

### Web Interface (Recommended)

Start the MGen server from the `service` directory.

```bash
cd service
cargo run
# Open http://127.0.0.1:3000
```

### Command Line: Android

Generates a new Android project using TMAndroid template. Dependency (cookiecutter) is auto-installed via brew if missing.

```bash
./scripts/scaffold_android.sh \
  --name MyApp \
  --package-name com.example.myapp \
  --output-dir ./projects
```

**Options:**
- `--name`: App name (Required)
- `--package-name`: App package (Default: `com.example.app`)
- `--min-sdk`: Minimum SDK version (Default: `24`)
- `--target-sdk`: Target SDK version (Default: `35`)
- `--output-dir`: Output directory (Default: `/tmp/scaffold-output`)

### Command Line: iOS

Generates a new iOS project using Tuist templates.

```bash
./scripts/scaffold_ios.sh \
  --name MyApp \
  --bundle-id-prefix com.example \
  --team-id TEAM1234
```

**Options:**
- `--name`: App name (Required)
- `--bundle-id-prefix`: Bundle ID prefix (Default: `com.axiomorient`)
- `--team-id`: Apple Team ID
- `--deployment-target`: iOS version (Default: `17.0`)

## Requirements

- **Common**: `cargo` (for web server)
- **iOS**: `mise` (for tuist installation), macOS
- **Android**: `brew` (for cookiecutter installation), generic OS (script checks brew)

## Logs

All generation logs are saved to `./logs/` with timestamps.

## Troubleshooting

### iOS Generation Fails: "Tuist not found"

**Solution**: Install Tuist via mise
```bash
mise use --global tuist@latest
```

### Android Generation Fails: "Cookiecutter not found"

**Solution**: The script should auto-install via Homebrew. If it fails:
```bash
brew install cookiecutter
```

### "Failed to bind server address"

**Solution**: Port 3000 is already in use. Either:
- Stop the other service using port 3000
- Set a custom port:
  ```bash
  SCAFFOLD_SERVER_ADDR=127.0.0.1:8080 cargo run
  ```

### Generated Project Path Not Found

**Solution**: Check the output directory setting in the web form (default: `../generated`). Ensure the path is accessible and has write permissions.

### GitHub Push Fails: "Authentication failed"

**Solution**: Ensure your GitHub Personal Access Token has `repo` scope:
1. Go to GitHub Settings → Developer settings → Personal access tokens
2. Generate a new token with `repo` scope
3. Use the token in the web interface
