#!/usr/bin/env bash
set -euo pipefail

# TMAndroid Wrapper Script
# Wraps the TMAndroid cookiecutter template for MGen service integration

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
TMANDROID_DIR="${ROOT_DIR}/templates/android"


# Default values
APP_NAME=""
PACKAGE_NAME="com.example.app"
MIN_SDK="24"
TARGET_SDK="35"
COMPILE_SDK="35"
OUTPUT_BASE_DIR="${OUTPUT_BASE_DIR:-/tmp/scaffold-output}"

# TMAndroid parameters (from cookiecutter.json)
ORGANIZATION_NAME="MyOrg"
USE_FIREBASE="no"
ARCHITECTURE="MVI"

usage() {
  cat << USAGE
Usage:
  scaffold_android.sh --name <AppName> [options]

Options:
  --name <value>              App name (required)
  --package-name <value>      Package name (default: ${PACKAGE_NAME})
  --min-sdk <value>           Min SDK version (default: ${MIN_SDK})
  --target-sdk <value>        Target SDK version (default: ${TARGET_SDK})
  --output-dir <path>         Output directory (default: ${OUTPUT_BASE_DIR})
  --organization <value>      Organization name (default: ${ORGANIZATION_NAME})
  --use-firebase              Enable Firebase (default: no)
  (Room/Retrofit are enabled by default)
  -h, --help                  Show help

Note:
  - Python 3.8+ required (will be checked)
  - cookiecutter will be auto-installed if missing
USAGE
}

log() {
  echo "$1"
}


ensure_cookiecutter() {
  log "[INFO] Checking cookiecutter availability..."
  
  # Check if cookiecutter is in path
  if command -v cookiecutter &> /dev/null; then
    log "[INFO] cookiecutter is already installed"
    return 0
  fi
  
  log "[WARN] cookiecutter not found. Attempting installation via brew..."
  
  if command -v brew &> /dev/null; then
    if brew install cookiecutter; then
      log "[INFO] cookiecutter installed successfully via brew"
      return 0
    else
      log "[ERROR] brew installation failed"
      exit 1
    fi
  else
    log "[ERROR] brew not found. Please install Homebrew or install cookiecutter manually."
    exit 1
  fi
}

check_dependencies() {
  log "[INFO] Checking dependencies..."
  
  # Check Python (brew cookiecutter usually brings its own python or uses system, but we need python3 for hooks)
  if ! command -v python3 &> /dev/null; then
    log "[ERROR] Python 3 is required but not installed"
    log "[ERROR] Install: brew install python3"
    exit 1
  fi
  
  # Ensure cookiecutter is installed via brew
  ensure_cookiecutter
  
  # Check TMAndroid exists
  if [[ ! -d "$TMANDROID_DIR" ]]; then
    log "[ERROR] TMAndroid directory not found: ${TMANDROID_DIR}"
    exit 1
  fi
  
  log "[INFO] All dependencies ready"
}

parse_args() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --name)
        APP_NAME="$2"
        shift 2
        ;;
      --package-name)
        PACKAGE_NAME="$2"
        shift 2
        ;;
      --min-sdk)
        MIN_SDK="$2"
        shift 2
        ;;
      --target-sdk)
        TARGET_SDK="$2"
        shift 2
        ;;
      --output-dir)
        OUTPUT_BASE_DIR="$2"
        shift 2
        ;;
      --organization)
        ORGANIZATION_NAME="$2"
        shift 2
        ;;
      --use-firebase)
        USE_FIREBASE="yes"
        shift 1
        ;;
      -h|--help)
        usage
        exit 0
        ;;
      *)
        log "[ERROR] Unknown argument: $1"
        usage
        exit 1
        ;;
    esac
  done
}

validate_inputs() {
  if [[ -z "$APP_NAME" ]]; then
    log "[ERROR] --name is required"
    usage
    exit 1
  fi
  
  if [[ ! "$APP_NAME" =~ ^[A-Za-z][A-Za-z0-9]*$ ]]; then
    log "[ERROR] Invalid app name. Must start with a letter and contain only alphanumeric characters."
    exit 1
  fi
}

run_cookiecutter() {
  log "[INFO] Running TMAndroid cookiecutter..."
  log "[INFO] App Name: ${APP_NAME}"
  log "[INFO] Package: ${PACKAGE_NAME}"
  log "[INFO] Min SDK: ${MIN_SDK}"
  log "[INFO] Target SDK: ${TARGET_SDK}"
  local compile_sdk="${TARGET_SDK}"
  if [[ "$compile_sdk" =~ ^[0-9]+$ ]] && (( compile_sdk < 35 )); then
    compile_sdk="35"
  fi
  log "[INFO] Compile SDK: ${compile_sdk}"
  log "[INFO] Output: ${OUTPUT_BASE_DIR}"
  
  # CamelCase folder naming: {AppName}a (no special characters for package compatibility)
  local project_slug="${APP_NAME}a"
  local cookiecutter_package_name="${PACKAGE_NAME##*.}"
  
  # Create output directory if it doesn't exist
  mkdir -p "$OUTPUT_BASE_DIR"
  
  log "[INFO] Cookiecutter parameters:"
  log "[INFO]   project_name: ${APP_NAME}"
  log "[INFO]   project_slug: ${project_slug}"
  log "[INFO]   package_name: ${cookiecutter_package_name}"
  log "[INFO]   base_package: ${PACKAGE_NAME}"
  
  # Run cookiecutter in non-interactive mode with inline parameters
  local output
  if output=$(cookiecutter \
    --output-dir "$OUTPUT_BASE_DIR" \
    --no-input \
    "$TMANDROID_DIR" \
    project_name="$APP_NAME" \
    project_slug="$project_slug" \
    package_name="$cookiecutter_package_name" \
    base_package="$PACKAGE_NAME" \
    template_dir="$TMANDROID_DIR/templates" \
    min_sdk="$MIN_SDK" \
    target_sdk="$TARGET_SDK" \
    compile_sdk="$compile_sdk" \
    organization_name="$ORGANIZATION_NAME" \
    use_firebase="$USE_FIREBASE" \
    architecture_pattern="$ARCHITECTURE" \
    2>&1); then
    
    log ""
    log "$output"
    log ""
    log "[INFO] Android project generated successfully"
    
    # Folder created directly with project_slug name
    local project_dir="${OUTPUT_BASE_DIR}/${project_slug}"
    
    if [[ -d "$project_dir" ]]; then
      log "[INFO] Project path: ${project_dir}"
      
      # Make gradlew executable
      if [[ -f "${project_dir}/gradlew" ]]; then
        chmod +x "${project_dir}/gradlew"
      fi
      
      # Make scripts executable if any exist
      if [[ -d "${project_dir}/scripts" ]]; then
        chmod +x "${project_dir}/scripts"/*.sh 2>/dev/null || true
      fi
      
      # Show next steps
      log ""
      log "[INFO] Next steps:"
      log "[INFO]   cd ${project_dir}"
      log "[INFO]   ./gradlew build"
      
    else
      log "[WARN] Expected project directory not found: ${project_dir}"
    fi
    
  else
    log "[ERROR] Cookiecutter execution failed:"
    log "$output"
    exit 1
  fi
}



main() {
  parse_args "$@"
  
  if [[ -z "$APP_NAME" ]]; then
    log "[ERROR] --name is required"
    usage
    exit 1
  fi
  
  validate_inputs
  check_dependencies
  run_cookiecutter
}

main "$@"
