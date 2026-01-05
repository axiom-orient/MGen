#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

APP_NAME=""
BUNDLE_PREFIX="com.axiomorient"
TEAM_ID="7WR76382QB"
ORG_NAME="axient"
DEPLOYMENT_TARGET="17.0"

OUTPUT_BASE_DIR="${OUTPUT_BASE_DIR:-/tmp/scaffold-output}"
TUIST_VERSION="${TUIST_VERSION:-latest}"
TUIST_VERSION_FILE="${TUIST_VERSION_FILE:-${TUIST_VERSION}}"
CREATE_ROOT_PACKAGE="${CREATE_ROOT_PACKAGE:-false}"
PLUGIN_GIT_URL="${PLUGIN_GIT_URL:-https://github.com/axiom-orient/TmaTemplates}"
PLUGIN_GIT_TAG="${PLUGIN_GIT_TAG:-}"
PLUGIN_GIT_SHA="${PLUGIN_GIT_SHA:-5731b6b9e9e036a484164f0fb8dbcea5130b002b}"
FORCE_TUIST_FILES=false

DRY_RUN=false
SKIP_BUILD=false
SKIP_GENERATE=false
SKIP_INSTALL=false

RESUME=false
START_INDEX=1
START_STEP_LABEL=""
PROJECT_DIR_OVERRIDE=""
PROJECT_DIR=""
TUIST_CMD=(tuist)

usage() {
  cat << USAGE
Usage:
  scaffold_project.sh --name <AppName> [options]

Options:
  --name <value>                 App name (required)
  --bundle-id-prefix <value>     Bundle ID prefix (default: ${BUNDLE_PREFIX})
  --team-id <value>              Apple Team ID (default: empty)
  --organization-name <value>    Organization name (default: ${ORG_NAME})
  --deployment-target <value>    iOS deployment target (default: ${DEPLOYMENT_TARGET})
  --output-dir <path>            Base output directory (default: ${OUTPUT_BASE_DIR})
  --tuist-version <value>        Tuist version for mise (default: ${TUIST_VERSION})
  --tuist-version-file <value>   .tuist-version contents (default: ${TUIST_VERSION_FILE})
  --plugin-git-url <value>       Plugin git URL (default: ${PLUGIN_GIT_URL})
  --plugin-git-tag <value>       Plugin git tag (default: ${PLUGIN_GIT_TAG})
  --plugin-git-sha <value>       Plugin git sha (default: ${PLUGIN_GIT_SHA})
  --force-tuist-files            Overwrite Tuist/Tuist.swift and Tuist/Package.swift
  --project-dir <path>           Use an existing project directory
  --start-step <step>            Start from a specific step (name or number)
  --resume-from <step>           Alias for --start-step
  --resume                       Resume from last completed step (requires project dir or latest match)
  --skip-install                 Skip tuist install
  --skip-generate                Skip tuist generate
  --skip-build                   Skip tuist build
  --dry-run                      Print steps without running commands
  -h, --help                     Show help
USAGE
}

log() {
  echo "$1"
}


resolve_step_index() {
  local step="$1"
  case "$step" in
    1|validate) echo 1 ;;
    2|create|create_dir|create-dir) echo 2 ;;
    3|ensure_tuist|ensure-tuist) echo 3 ;;
    4|ensure_files|ensure-files) echo 4 ;;
    5|workspace|workspace_file|workspace-file) echo 5 ;;
    6|install) echo 6 ;;
    7|scaffold) echo 7 ;;
    8|generate) echo 8 ;;
    9|build) echo 9 ;;
    *) return 1 ;;
  esac
}

step_name_for_index() {
  case "$1" in
    1) echo "validate" ;;
    2) echo "create_dir" ;;
    3) echo "ensure_tuist" ;;
    4) echo "ensure_files" ;;
    5) echo "workspace" ;;
    6) echo "install" ;;
    7) echo "scaffold" ;;
    8) echo "generate" ;;
    9) echo "build" ;;
    *) echo "unknown" ;;
  esac
}

write_state() {
  local index="$1"
  local name
  name="$(step_name_for_index "$index")"

  if [[ -z "$PROJECT_DIR" || ! -d "$PROJECT_DIR" ]]; then
    return 0
  fi

  if $DRY_RUN; then
    return 0
  fi

  cat > "${PROJECT_DIR}/.scaffold_state" << EOF
last_completed_index=${index}
last_completed_name=${name}
last_updated=$(date +"%Y-%m-%dT%H:%M:%S%z")
EOF
}

read_state() {
  local state_file="${PROJECT_DIR}/.scaffold_state"
  if [[ ! -f "$state_file" ]]; then
    log "[ERROR] State file not found: ${state_file}"
    log "[ERROR] Use --start-step with --project-dir to resume manually."
    exit 1
  fi

  local last_index
  last_index="$(grep -E \"^last_completed_index=\" \"$state_file\" | head -n 1 | cut -d= -f2 || true)"
  if [[ -z "$last_index" ]]; then
    log "[ERROR] Invalid state file: ${state_file}"
    log "[ERROR] Use --start-step with --project-dir to resume manually."
    exit 1
  fi

  START_INDEX=$((last_index + 1))
  START_STEP_LABEL="$(step_name_for_index "$START_INDEX")"
  log "[INFO] Resuming from step ${START_INDEX} (${START_STEP_LABEL})"
}

run_step() {
  local index="$1"
  local title="$2"
  local func="$3"

  if (( index < START_INDEX )); then
    log ""
    log "## Step ${index} - ${title} (skipped)"
    return 0
  fi

  log ""
  log "## Step ${index} - ${title}"
  "$func"
  write_state "$index"
}


resolve_project_dir() {
  if [[ -n "$PROJECT_DIR_OVERRIDE" ]]; then
    PROJECT_DIR="$PROJECT_DIR_OVERRIDE"
    return 0
  fi

  if ! $RESUME; then
    return 0
  fi

  local matches=()
  local latest
  shopt -s nullglob
  matches=("${OUTPUT_BASE_DIR}/${APP_NAME}i"*)
  shopt -u nullglob

  if (( ${#matches[@]} == 0 )); then
    log "[ERROR] No existing project directories found for ${APP_NAME} in ${OUTPUT_BASE_DIR}"
    exit 1
  fi

  latest="$(printf '%s\n' "${matches[@]}" | sort | tail -n 1)"
  PROJECT_DIR="$latest"
  log "[INFO] Using latest project dir: ${PROJECT_DIR}"
}

ensure_project_dir_for_resume() {
  if (( START_INDEX > 2 )); then
    if [[ -z "$PROJECT_DIR" ]]; then
      log "[ERROR] project directory is required to start from step ${START_INDEX}"
      log "[ERROR] Use --project-dir or --resume (with existing output)."
      exit 1
    fi
    if [[ ! -d "$PROJECT_DIR" ]]; then
      log "[ERROR] project directory not found: ${PROJECT_DIR}"
      exit 1
    fi
  fi
}

format_cmd() {
  printf '%q ' "$@"
}

run_cmd() {
  local formatted
  formatted="$(format_cmd "$@")"
  log "[CMD] ${formatted}"

  if $DRY_RUN; then
    log "[DRY RUN] skipped"
    return 0
  fi

  "$@"
}

run_in_project() {
  if $DRY_RUN; then
    local formatted
    formatted="$(format_cmd "$@")"
    log "[CMD] (cd \"${PROJECT_DIR}\" && ${formatted})"
    log "[DRY RUN] skipped"
    return 0
  fi

  (cd "$PROJECT_DIR" && run_cmd "$@")
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

create_project_dir() {
  if [[ -n "$PROJECT_DIR_OVERRIDE" ]]; then
    PROJECT_DIR="$PROJECT_DIR_OVERRIDE"
  else
    # CamelCase folder naming: {AppName}i (no special characters)
    PROJECT_DIR="${OUTPUT_BASE_DIR}/${APP_NAME}i"
  fi

  run_cmd mkdir -p "$PROJECT_DIR"
  log "[INFO] Project dir: ${PROJECT_DIR}"
}

ensure_mise() {
  if ! command -v mise >/dev/null 2>&1; then
    log "[ERROR] mise is required to install tuist but not found in PATH."
    exit 1
  fi
}

ensure_tuist_installed() {
  if command -v tuist >/dev/null 2>&1; then
    log "[INFO] tuist found in PATH"
    TUIST_CMD=(tuist)
    return 0
  fi

  if $DRY_RUN; then
    log "[DRY RUN] tuist not found. Would install via mise"
    TUIST_CMD=(tuist)
    return 0
  fi

  log "[WARN] tuist not found. Attempting install via mise"
  ensure_mise

  local mise_file="${PROJECT_DIR}/mise.toml"
  if [[ ! -f "$mise_file" ]]; then
    log "[INFO] Creating mise.toml"
    cat > "$mise_file" << EOF
[tools]
tuist = "${TUIST_VERSION}"
EOF
  else
    log "[INFO] Using existing mise.toml: ${mise_file}"
  fi

  run_in_project mise install

  if command -v tuist >/dev/null 2>&1; then
    TUIST_CMD=(tuist)
    return 0
  fi

  if mise which tuist >/dev/null 2>&1; then
    local tuist_path
    tuist_path="$(mise which tuist)"
    TUIST_CMD=("${tuist_path}")
    log "[INFO] Using tuist from mise: ${tuist_path}"
    return 0
  fi

  log "[ERROR] tuist install via mise failed"
  exit 1
}

ensure_tuist_files() {
  local tuist_dir="${PROJECT_DIR}/Tuist"
  local tuist_swift="${PROJECT_DIR}/Tuist.swift"
  local package_swift="${PROJECT_DIR}/Package.swift"
  local tuist_package_swift="${tuist_dir}/Package.swift"
  local tuist_version_file="${PROJECT_DIR}/.tuist-version"

  run_cmd mkdir -p "$tuist_dir"

  if [[ ! -f "$tuist_swift" || "$FORCE_TUIST_FILES" == "true" ]]; then
    log "[INFO] Writing Tuist.swift"
    if $DRY_RUN; then
      log "[DRY RUN] would write ${tuist_swift}"
    else
      cat > "$tuist_swift" << EOF
import ProjectDescription

let tuist = Tuist(
    project: .tuist(
        plugins: [
            .local(path: "${ROOT_DIR}/templates/ios")
        ],
        generationOptions: .options(
            resolveDependenciesWithSystemScm: true,
            disableSandbox: true
        )
    )
)
EOF
    fi
  else
    log "[INFO] Tuist.swift exists"
  fi

  if [[ ! -f "$tuist_package_swift" || "$FORCE_TUIST_FILES" == "true" ]]; then
    log "[INFO] Writing Tuist/Package.swift"
    if $DRY_RUN; then
      log "[DRY RUN] would write ${tuist_package_swift}"
    else
      cat > "$tuist_package_swift" << 'EOF'
// swift-tools-version: 6.0
import PackageDescription

#if TUIST
import ProjectDescription

let packageSettings = PackageSettings(
    productTypes: [
        "ComposableArchitecture": .framework,
        "Dependencies": .framework,
        "Sharing": .framework,
        "Clocks": .framework,
        "CombineSchedulers": .framework,
        "ConcurrencyExtras": .framework,
        "CustomDump": .framework,
        "IdentifiedCollections": .framework,
        "InternalCollectionsUtilities": .framework,
        "IssueReporting": .framework,
        "IssueReportingPackageSupport": .framework,
        "OrderedCollections": .framework,
        "PerceptionCore": .framework,
        "XCTestDynamicOverlay": .framework,
        "SQLiteData": .framework,
        "GRDB": .framework
    ]
)
#endif

let package = Package(
    name: "ProjectDependencies",
    dependencies: [
        // TCA & Dependencies
        .package(url: "https://github.com/pointfreeco/swift-composable-architecture", exact: "1.23.1"),
        .package(url: "https://github.com/pointfreeco/swift-dependencies", from: "1.10.0"),
        // Persistence
        .package(url: "https://github.com/pointfreeco/swift-sharing", from: "2.7.4"),
        // Database
        .package(url: "https://github.com/pointfreeco/sqlite-data", from: "1.4.2"),
        // Firebase
        .package(url: "https://github.com/firebase/firebase-ios-sdk", from: "11.0.0")
    ]
)
EOF
    fi
  else
    log "[INFO] Tuist/Package.swift exists"
  fi

  if [[ ! -f "$tuist_version_file" || "$FORCE_TUIST_FILES" == "true" ]]; then
    log "[INFO] Writing .tuist-version"
    if $DRY_RUN; then
      log "[DRY RUN] would write ${tuist_version_file}"
    else
      echo "${TUIST_VERSION_FILE}" > "$tuist_version_file"
    fi
  else
    log "[INFO] .tuist-version exists"
  fi

  if [[ "$CREATE_ROOT_PACKAGE" == "true" ]]; then
    if [[ ! -f "$package_swift" ]]; then
      log "[INFO] Creating root Package.swift"
      if $DRY_RUN; then
        log "[DRY RUN] would write ${package_swift}"
      else
        cat > "$package_swift" << 'EOF'
// swift-tools-version: 6.0
import PackageDescription

let package = Package(
    name: "GeneratedProject",
    dependencies: [
        // TCA & Dependencies
        .package(url: "https://github.com/pointfreeco/swift-composable-architecture", exact: "1.23.0"),
        .package(url: "https://github.com/pointfreeco/swift-dependencies", from: "1.10.0"),
        // Persistence
        .package(url: "https://github.com/pointfreeco/swift-sharing", from: "2.7.4"),
        // Database
        .package(url: "https://github.com/pointfreeco/sqlite-data", from: "0.1.0"),
        // Firebase
        .package(url: "https://github.com/firebase/firebase-ios-sdk", from: "11.0.0")
    ]
)
EOF
      fi
    else
      log "[INFO] root Package.swift exists"
    fi
  fi

}

ensure_workspace_file() {
  local workspace_swift="${PROJECT_DIR}/Workspace.swift"

  if [[ ! -f "$workspace_swift" ]]; then
    log "[INFO] Creating Workspace.swift"
    if $DRY_RUN; then
      log "[DRY RUN] would write ${workspace_swift}"
    else
      cat > "$workspace_swift" << EOF
import ProjectDescription

let workspace = Workspace(
    name: "${APP_NAME}",
    projects: [
        "Projects/**"
    ]
)
EOF
    fi
  else
    log "[INFO] Workspace.swift exists"
  fi
}

run_scaffold() {
  log "[INFO] Running tuist scaffold app"
  local args=(scaffold app \
    --name "$APP_NAME" \
    --bundle-id-prefix "$BUNDLE_PREFIX" \
    --organization-name "$ORG_NAME" \
    --deployment-target "$DEPLOYMENT_TARGET")

  if [[ -n "$TEAM_ID" ]]; then
    args+=(--team-id "$TEAM_ID")
  fi

  run_in_project "${TUIST_CMD[@]}" "${args[@]}"
}

run_install() {
  if $SKIP_INSTALL; then
    log "[INFO] Skip tuist install"
    return 0
  fi

  log "[INFO] Running tuist install"
  run_in_project "${TUIST_CMD[@]}" install
}

run_generate() {
  if $SKIP_GENERATE; then
    log "[INFO] Skip tuist generate"
    return 0
  fi

  log "[INFO] Running tuist generate"
  run_in_project "${TUIST_CMD[@]}" generate --no-open
}

run_build() {
  if $SKIP_BUILD; then
    log "[INFO] Skip tuist build"
    return 0
  fi

  log "[INFO] Running tuist build (target: ${APP_NAME})"
  run_in_project "${TUIST_CMD[@]}" build "$APP_NAME" --clean
}



parse_args() {
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --name)
        APP_NAME="$2"
        shift 2
        ;;
      --bundle-id-prefix)
        BUNDLE_PREFIX="$2"
        shift 2
        ;;
      --team-id)
        TEAM_ID="$2"
        shift 2
        ;;
      --organization-name)
        ORG_NAME="$2"
        shift 2
        ;;
      --deployment-target)
        DEPLOYMENT_TARGET="$2"
        shift 2
        ;;

      --output-dir)
        OUTPUT_BASE_DIR="$2"
        shift 2
        ;;
      --tuist-version)
        TUIST_VERSION="$2"
        shift 2
        ;;
      --tuist-version-file)
        TUIST_VERSION_FILE="$2"
        shift 2
        ;;
      --plugin-git-url)
        PLUGIN_GIT_URL="$2"
        shift 2
        ;;
      --plugin-git-tag)
        PLUGIN_GIT_TAG="$2"
        shift 2
        ;;
      --plugin-git-sha)
        PLUGIN_GIT_SHA="$2"
        shift 2
        ;;
      --force-tuist-files)
        FORCE_TUIST_FILES=true
        shift 1
        ;;
      --project-dir)
        PROJECT_DIR_OVERRIDE="$2"
        shift 2
        ;;
      --start-step|--resume-from)
        if ! START_INDEX="$(resolve_step_index "$2")"; then
          log "[ERROR] Invalid start step: $2"
          usage
          exit 1
        fi
        START_STEP_LABEL="$(step_name_for_index "$START_INDEX")"
        shift 2
        ;;
      --resume)
        RESUME=true
        shift 1
        ;;
      --skip-install)
        SKIP_INSTALL=true
        shift 1
        ;;
      --skip-generate)
        SKIP_GENERATE=true
        shift 1
        ;;
      --skip-build)
        SKIP_BUILD=true
        shift 1
        ;;
      --dry-run)
        DRY_RUN=true
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


main() {
  parse_args "$@"

  if [[ -z "$APP_NAME" ]]; then
    log "[ERROR] --name is required"
    usage
    exit 1
  fi

  resolve_project_dir

  if $RESUME; then
    if [[ -n "$START_STEP_LABEL" ]]; then
      log "[WARN] --resume overrides --start-step"
    fi
    read_state
  fi

  ensure_project_dir_for_resume

  run_step 1 "Validate inputs" validate_inputs
  run_step 2 "Create project directory" create_project_dir
  run_step 3 "Ensure tuist is installed" ensure_tuist_installed
  run_step 4 "Ensure Tuist/Package files" ensure_tuist_files
  run_step 5 "Ensure Workspace.swift" ensure_workspace_file
  run_step 6 "Install dependencies" run_install
  run_step 7 "Run tuist scaffold" run_scaffold
  run_step 8 "Generate Xcode project" run_generate
  run_step 9 "Build verification" run_build

  log ""
  log "[INFO] Done"
  log "[INFO] Project path: ${PROJECT_DIR}"
}

main "$@"
