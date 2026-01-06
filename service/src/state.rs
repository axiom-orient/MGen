use dashmap::DashMap;
use serde::Serialize;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

/// Task execution status
#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum TaskStatus {
    Running,
    Success,
    Failed,
}

/// Build verification status
#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum BuildStatus {
    NotStarted,
    Verifying,
    Verified,
    Failed,
}

/// Platform type for generation
#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum Platform {
    Ios,
    Android,
    Both,
}

impl Platform {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "android" => Platform::Android,
            "both" => Platform::Both,
            _ => Platform::Ios,
        }
    }
}

/// State for a single generation task
#[derive(Clone, Debug, Serialize)]
pub struct TaskState {
    pub status: TaskStatus,
    pub log: String,
    pub app_name: String,
    pub platform: Platform,
    /// Build verification status
    pub build_status: BuildStatus,
    /// Build verification log
    pub build_log: String,
    /// iOS project path (if generated)
    pub ios_path: Option<String>,
    /// Android project path (if generated)
    pub android_path: Option<String>,
    /// Legacy single path (for backward compatibility)
    pub output_path: Option<String>,
}

impl TaskState {
    /// Create a new running task
    pub fn running(app_name: &str, platform: Platform) -> Self {
        Self {
            status: TaskStatus::Running,
            log: String::new(),
            app_name: app_name.to_string(),
            platform,
            build_status: BuildStatus::NotStarted,
            build_log: String::new(),
            ios_path: None,
            android_path: None,
            output_path: None,
        }
    }

    /// Create from existing project folder (for startup scan)
    pub fn from_existing(app_name: &str, platform: Platform, path: &str) -> Self {
        let (ios_path, android_path) = match platform {
            Platform::Ios => (Some(path.to_string()), None),
            Platform::Android => (None, Some(path.to_string())),
            Platform::Both => (None, None), // Will be set separately
        };

        // Try to load log from file
        let log = load_log_from_file(path)
            .unwrap_or_else(|| "Project loaded from existing folder".to_string());

        Self {
            status: TaskStatus::Success,
            log,
            app_name: app_name.to_string(),
            platform,
            build_status: BuildStatus::NotStarted,
            build_log: String::new(),
            ios_path,
            android_path,
            output_path: Some(path.to_string()),
        }
    }
}

/// Shared application state
pub struct AppState {
    pub tasks: DashMap<Uuid, TaskState>,
    pub generated_dir: Mutex<String>,
}

impl AppState {
    pub fn new() -> Arc<Self> {
        let generated_dir = default_generated_dir();
        let state = Arc::new(Self {
            tasks: DashMap::new(),
            generated_dir: Mutex::new(generated_dir.clone()),
        });

        // Scan existing projects on startup
        scan_existing_projects(&state, &generated_dir);

        state
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            tasks: DashMap::new(),
            generated_dir: Mutex::new(default_generated_dir()),
        }
    }
}

/// Get default generated directory path (PROJECT_ROOT/generated)
fn default_generated_dir() -> String {
    // When running from service/, the generated folder is at ../generated
    let generated_path = std::env::current_dir()
        .ok()
        .and_then(|cwd| {
            // Try direct ../generated first
            let direct = cwd.join("../generated");
            if direct.exists() {
                return direct.canonicalize().ok();
            }
            // Try parent/generated
            cwd.parent().map(|p| p.join("generated"))
        })
        .unwrap_or_else(|| std::path::PathBuf::from("../generated"));

    generated_path.to_string_lossy().to_string()
}

/// Scan existing project folders and populate tasks
fn scan_existing_projects(state: &Arc<AppState>, dir: &str) {
    let path = std::path::Path::new(dir);
    if !path.exists() {
        return;
    }

    let Ok(entries) = std::fs::read_dir(path) else {
        return;
    };

    for entry in entries.flatten() {
        let folder_name = entry.file_name().to_string_lossy().to_string();

        // Skip hidden files and non-directories
        if folder_name.starts_with('.') || !entry.path().is_dir() {
            continue;
        }

        // Parse folder name: {name}_{platform} or legacy formats
        let (app_name, platform) = parse_folder_name(&folder_name);

        let full_path = entry.path().to_string_lossy().to_string();
        let task = TaskState::from_existing(&app_name, platform, &full_path);

        state.tasks.insert(Uuid::new_v4(), task);
    }
}

/// Sync tasks with filesystem: remove tasks with deleted folders, add new folders
pub fn sync_tasks_with_filesystem(state: &Arc<AppState>) {
    // Get current generated directory
    let current_dir = {
        let lock = state.generated_dir.lock().unwrap();
        lock.clone()
    };

    // Step 1: Remove tasks whose folders no longer exist OR are not in the current directory
    let tasks_to_remove: Vec<Uuid> = state
        .tasks
        .iter()
        .filter_map(|entry| {
            let task = entry.value();
            let path_opt = task
                .output_path
                .as_ref()
                .or(task.ios_path.as_ref())
                .or(task.android_path.as_ref());

            let should_keep = if let Some(p) = path_opt {
                let path = std::path::Path::new(p);
                // Check existence AND that it belongs to current dir
                path.exists() && p.starts_with(&current_dir)
            } else {
                false
            };

            if !should_keep {
                Some(*entry.key())
            } else {
                None
            }
        })
        .collect();

    for task_id in tasks_to_remove {
        state.tasks.remove(&task_id);
    }

    // Step 2: Scan for new folders
    let path = std::path::Path::new(&current_dir);
    if !path.exists() {
        return;
    }

    let Ok(entries) = std::fs::read_dir(path) else {
        return;
    };

    // Get all existing paths in tasks
    let existing_paths: Vec<String> = state
        .tasks
        .iter()
        .filter_map(|entry| {
            let task = entry.value();
            task.output_path
                .clone()
                .or(task.ios_path.clone())
                .or(task.android_path.clone())
        })
        .collect();

    // Add new folders that aren't tracked
    for entry in entries.flatten() {
        let folder_name = entry.file_name().to_string_lossy().to_string();

        if folder_name.starts_with('.') || !entry.path().is_dir() {
            continue;
        }

        let full_path = entry.path().to_string_lossy().to_string();

        // Check if this path is already tracked
        if existing_paths.iter().any(|p| p == &full_path) {
            continue;
        }

        // New folder found - add it
        let (app_name, platform) = parse_folder_name(&folder_name);
        let task = TaskState::from_existing(&app_name, platform, &full_path);
        state.tasks.insert(Uuid::new_v4(), task);
    }
}

/// Save generation log to project folder
pub fn save_log_to_file(project_path: &str, log: &str) {
    let log_file = std::path::Path::new(project_path).join(".mgen_log.txt");
    let _ = std::fs::write(log_file, log);
}

/// Load generation log from project folder
fn load_log_from_file(project_path: &str) -> Option<String> {
    let log_file = std::path::Path::new(project_path).join(".mgen_log.txt");
    std::fs::read_to_string(log_file).ok()
}

/// Parse folder name to extract app name and platform
fn parse_folder_name(name: &str) -> (String, Platform) {
    // CamelCase format: {AppName}Ios, {AppName}Android (new format)
    if name.ends_with("Ios") {
        let app_name = name.strip_suffix("Ios").unwrap_or(name);
        return (app_name.to_string(), Platform::Ios);
    }
    if name.ends_with("Android") {
        let app_name = name.strip_suffix("Android").unwrap_or(name);
        return (app_name.to_string(), Platform::Android);
    }

    // Legacy format: {name}_ios, {name}_android (backward compatibility)
    if name.ends_with("_ios") {
        let app_name = name.strip_suffix("_ios").unwrap_or(name);
        return (app_name.to_string(), Platform::Ios);
    }
    if name.ends_with("_android") {
        let app_name = name.strip_suffix("_android").unwrap_or(name);
        return (app_name.to_string(), Platform::Android);
    }

    // Legacy format with timestamp: {name}_{timestamp}
    if let Some(idx) = name.rfind('_') {
        let potential_timestamp = &name[idx + 1..];
        if potential_timestamp.len() >= 8 && potential_timestamp.chars().all(|c| c.is_ascii_digit())
        {
            let app_name = &name[..idx];
            return (app_name.to_string(), Platform::Ios);
        }
    }

    // Default: use whole name as app name, assume iOS
    (name.to_string(), Platform::Ios)
}
