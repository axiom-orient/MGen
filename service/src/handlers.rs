//! Route handlers for MGen service

use axum::extract::{Path as AxumPath, State};
use axum::response::{Html, IntoResponse};
use axum::Form;
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::generation::{run_android_generation, run_both_generation, run_ios_generation};
use crate::state::{save_log_to_file, AppState, Platform, TaskState, TaskStatus};
use crate::templates::{base_html_with_nav, escape_html, render_polling};
use crate::utils::{default_output_dir, is_java_reserved_keyword, valid_app_name};

// ============================================================================
// Form Structs
// ============================================================================

#[derive(Deserialize, Clone, Debug)]
pub struct ScaffoldForm {
    pub name: String,
    #[serde(default = "default_platform")]
    pub platform: String,
    pub bundle_id_prefix: Option<String>,
    pub team_id: Option<String>,
    pub organization_name: Option<String>,
    pub deployment_target: Option<String>,
    pub package_name: Option<String>,
    pub min_sdk_version: Option<String>,
    pub target_sdk_version: Option<String>,
    pub output_dir: Option<String>,
}

fn default_platform() -> String {
    "ios".to_string()
}

#[derive(Deserialize, Debug)]
pub struct GitHubPushForm {
    pub task_id: Uuid,
    pub repo_name: String,
    pub token: String,
}

// ============================================================================
// Health Check
// ============================================================================

pub async fn health() -> Html<&'static str> {
    Html("ok")
}

// ============================================================================
// Helper Functions
// ============================================================================

/// Render project cards with build verification status
/// Render a single project card
fn render_single_card(task: &TaskState, id: Uuid) -> String {
    use crate::state::BuildStatus;

    let (platform_str, card_class) = match task.platform {
        Platform::Ios => ("iOS", "ios-card"),
        Platform::Android => ("Android", "android-card"),
        Platform::Both => ("Both", "both-card"),
    };

    // Build verification badge & polling attributes
    let (build_badge, poll_attr) = match task.build_status {
        BuildStatus::NotStarted => (
            "",
            String::new(), // No polling if not started
        ),
        BuildStatus::Verifying => (
            r##"<span class="build-badge verifying">Verifying</span>"##,
            format!(
                r##"hx-get="/task/{}/card" hx-trigger="every 1s" hx-swap="outerHTML""##,
                id
            ),
        ),
        BuildStatus::Verified => (
            r##"<span class="build-badge verified">Verified</span>"##,
            String::new(),
        ),
        BuildStatus::Failed => (
            r##"<span class="build-badge failed">Build Failed</span>"##,
            String::new(),
        ),
    };

    let actions = if task.status == TaskStatus::Success {
        let download_links = match task.platform {
            Platform::Both => format!(
                r##"<a href="/download/{}/ios" class="card-button" onclick="event.stopPropagation()">iOS</a>
                   <a href="/download/{}/android" class="card-button" onclick="event.stopPropagation()">Android</a>"##,
                id, id
            ),
            _ => format!(
                r##"<a href="/download/{}" class="card-button" onclick="event.stopPropagation()">Download</a>"##,
                id
            ),
        };

        // Add Verification Button if not started or failed
        let verify_button = if task.build_status == BuildStatus::NotStarted
            || task.build_status == BuildStatus::Failed
        {
            format!(
                r##"<button class="card-button verify-button" 
                        hx-post="/task/{}/verify" 
                        hx-target="#project-{}" 
                        hx-swap="outerHTML"
                        onclick="event.stopPropagation()">
                    Verify Build
                </button>"##,
                id, id
            )
        } else {
            String::new()
        };

        format!(
            r##"{}
            {}
            <button class="card-button delete-button" 
                    onclick="event.stopPropagation(); showDeleteConfirm('{}')">
                Delete
            </button>"##,
            download_links, verify_button, id
        )
    } else {
        // Even for failed tasks, allow deletion
        format!(
            r##"<button class="card-button delete-button" 
                    onclick="event.stopPropagation(); showDeleteConfirm('{}')">
                Delete
            </button>"##,
            id
        )
    };

    format!(
        r##"<div class="project-card {}" id="project-{}" onclick="window.location.href='/task/{}'" {}>
            <div class="card-header">
                <span class="card-platform">{}</span>
                {}
            </div>
            <div class="card-name">{}</div>
            <div class="card-id">{}</div>
            <div class="card-actions">
                {}
            </div>
        </div>"##,
        card_class,
        id,
        id,
        poll_attr,
        platform_str,
        build_badge,
        escape_html(&task.app_name),
        &id.to_string()[..8],
        actions
    )
}

/// Render project cards with build verification status
fn render_project_cards(tasks: &[(Uuid, TaskState)]) -> String {
    if tasks.is_empty() {
        return r##"<div class="empty-state"><p>No projects yet. Create your first project!</p></div>"##
            .to_string();
    }

    tasks
        .iter()
        .map(|(id, task)| render_single_card(task, *id))
        .collect()
}

// ============================================================================
// Delete Task
// ============================================================================

pub async fn delete_task(
    AxumPath(id): AxumPath<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    if let Some((_, task)) = state.tasks.remove(&id) {
        // Delete directories
        if let Some(path) = task.ios_path {
            let _ = std::fs::remove_dir_all(&path);
        }
        if let Some(path) = task.android_path {
            let _ = std::fs::remove_dir_all(&path);
        }
        // Fallback or legacy paths
        if let Some(path) = task.output_path {
            if std::path::Path::new(&path).exists() {
                let _ = std::fs::remove_dir_all(&path);
            }
        }
    }

    // Return empty string to remove the element from DOM
    Html(String::new())
}

// ============================================================================
// Verify Build (Manual Trigger)
// ============================================================================

pub async fn verify_build(
    AxumPath(id): AxumPath<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    use crate::state::BuildStatus;

    if let Some(mut task_entry) = state.tasks.get_mut(&id) {
        // Don't start if already verifying
        if task_entry.build_status == BuildStatus::Verifying {
            return Html(render_single_card(&task_entry, id));
        }

        task_entry.build_status = BuildStatus::Verifying;

        let task = task_entry.clone();
        let state_clone = state.clone();
        let task_id = id;

        tokio::spawn(async move {
            use crate::generation::{verify_android_build, verify_ios_build};

            // Run build verification based on platform
            match task.platform {
                Platform::Ios => {
                    if let Some(ref ios_path) = task.ios_path {
                        let (build_status, build_log) = verify_ios_build(ios_path).await;

                        if let Some(mut entry) = state_clone.tasks.get_mut(&task_id) {
                            entry.build_status = build_status;
                            entry.build_log = build_log;
                        }
                    }
                }
                Platform::Android => {
                    if let Some(ref android_path) = task.android_path {
                        let (build_status, build_log) = verify_android_build(android_path).await;

                        if let Some(mut entry) = state_clone.tasks.get_mut(&task_id) {
                            entry.build_status = build_status;
                            entry.build_log = build_log;
                        }
                    }
                }
                Platform::Both => {
                    // Verify iOS first
                    if let Some(ref ios_path) = task.ios_path {
                        let (ios_status, ios_log) = verify_ios_build(ios_path).await;

                        // Then verify Android
                        let (final_status, combined_log) =
                            if let Some(ref android_path) = task.android_path {
                                let (android_status, android_log) =
                                    verify_android_build(android_path).await;

                                let status = if ios_status == BuildStatus::Verified
                                    && android_status == BuildStatus::Verified
                                {
                                    BuildStatus::Verified
                                } else {
                                    BuildStatus::Failed
                                };

                                let log = format!(
                                    "=== iOS Build ===\n{}\n\n=== Android Build ===\n{}",
                                    ios_log, android_log
                                );
                                (status, log)
                            } else {
                                (ios_status, ios_log)
                            };

                        if let Some(mut entry) = state_clone.tasks.get_mut(&task_id) {
                            entry.build_status = final_status;
                            entry.build_log = combined_log;
                        }
                    }
                }
            }
        });

        // Return the card immediately in Verifying state (which triggers polling)
        Html(render_single_card(&task_entry, id))
    } else {
        Html("Task not found".to_string())
    }
}

// ============================================================================
// Dashboard
// ============================================================================

pub async fn dashboard(State(state): State<Arc<AppState>>) -> Html<String> {
    let mut recent_tasks: Vec<(Uuid, TaskState)> = state
        .tasks
        .iter()
        .map(|entry| (*entry.key(), entry.value().clone()))
        .collect();
    recent_tasks.truncate(10);

    let tasks_html = render_project_cards(&recent_tasks);

    let html = format!(
        r##"{}
<div class="dashboard">
  <div id="hero-section" class="hero">
    {}
  </div>

  <div class="recent-section">
    <div class="recent-header">
      <h2>Recent Projects</h2>
      <button class="scan-button" hx-post="/sync-folders" hx-target=".recent-section" hx-swap="outerHTML">Sync Folders</button>
    </div>
    <div class="task-grid">{}</div>
  </div>
</div>
</body></html>"##,
        base_html_with_nav("/"),
        render_hero_section(&state.generated_dir.lock().unwrap()),
        tasks_html
    );

    Html(html)
}

/// Renders just the hero section (used for partial updates)
fn render_hero_section(generated_dir: &str) -> String {
    format!(
        r##"
    <h1>Mobile Generator</h1>
    <p class="subtitle">Create production-ready iOS and Android projects</p>
    <div class="meta-info-container">
        <p class="meta-info">Output Location: <code>{}</code></p>
        <button type="button" class="choose-btn-sm" onclick="chooseDashboardPath()">Choose</button>
    </div>
    
    <div class="feature-grid">
      <div class="feature-card">
        <h3>iOS</h3>
        <p>Tuist-based Xcode projects with modern architecture</p>
      </div>
      <div class="feature-card">
        <h3>Android</h3>
        <p>Kotlin-first Android projects with best practices</p>
      </div>
      <div class="feature-card">
        <h3>Export</h3>
        <p>Download as ZIP or push directly to GitHub</p>
      </div>
    </div>

    <a href="/create" class="button create-button">Create Project</a>
"##,
        escape_html(generated_dir)
    )
}

#[derive(Deserialize)]
pub struct UpdateDirForm {
    pub path: String,
}

/// Updates the global output directory and re-renders the hero section
pub async fn update_output_dir(
    State(state): State<Arc<AppState>>,
    Form(form): Form<UpdateDirForm>,
) -> Html<String> {
    // Basic validation
    let path = std::path::Path::new(&form.path);
    if path.exists() && path.is_dir() {
        // Update the state
        if let Ok(mut dir) = state.generated_dir.lock() {
            *dir = form.path.clone();
        }
        Html(render_hero_section(&state.generated_dir.lock().unwrap()))
    } else {
        // Return original if invalid
        Html(render_hero_section(&state.generated_dir.lock().unwrap()))
    }
}

// ============================================================================
// History
// ============================================================================

pub async fn tasks_list(State(state): State<Arc<AppState>>) -> Html<String> {
    let all_tasks: Vec<(Uuid, TaskState)> = state
        .tasks
        .iter()
        .map(|entry| (*entry.key(), entry.value().clone()))
        .collect();

    let timeline_html = if all_tasks.is_empty() {
        r##"<div class="empty-state"><p>No projects yet. Create your first project!</p></div>"##
            .to_string()
    } else {
        all_tasks
            .iter()
            .map(|(id, task)| {
                let status_icon = match task.status {
                    TaskStatus::Success => "✓",
                    TaskStatus::Failed => "✕",
                    TaskStatus::Running => "○",
                };
                let status_class = match task.status {
                    TaskStatus::Success => "success",
                    TaskStatus::Failed => "failed",
                    TaskStatus::Running => "running",
                };
                let platform_str = match task.platform {
                    Platform::Ios => "iOS",
                    Platform::Android => "Android",
                    Platform::Both => "iOS & Android",
                };

                let path_info = task
                    .output_path
                    .as_ref()
                    .or(task.ios_path.as_ref())
                    .or(task.android_path.as_ref())
                    .map(|p| {
                        format!(
                            r##"<div class="timeline-path"><code>{}</code></div>"##,
                            escape_html(p)
                        )
                    })
                    .unwrap_or_default();

                format!(
                    r##"<div class="timeline-item" onclick="window.location.href='/task/{}'">
                        <div class="timeline-marker {}">
                            <span class="timeline-icon">{}</span>
                        </div>
                        <div class="timeline-content">
                            <div class="timeline-header">
                                <h3 class="timeline-title">{}</h3>
                                <span class="timeline-platform">{}</span>
                            </div>
                            <div class="timeline-id">ID: {}</div>
                            {}
                        </div>
                    </div>"##,
                    id,
                    status_class,
                    status_icon,
                    escape_html(&task.app_name),
                    platform_str,
                    &id.to_string()[..8],
                    path_info
                )
            })
            .collect()
    };

    Html(format!(
        r##"{}
<div class="page-content narrow">
  <div class="page-header">
    <h1>Project History</h1>
    <p class="page-subtitle">{} total projects</p>
  </div>
  <div class="timeline">
    {}
  </div>
</div>
</body></html>"##,
        base_html_with_nav("/tasks"),
        all_tasks.len(),
        timeline_html
    ))
}

// ============================================================================
// Create Form
// ============================================================================

pub async fn show_form() -> Html<String> {
    Html(render_form(None, None))
}

pub fn render_form(error: Option<&str>, form: Option<&ScaffoldForm>) -> String {
    let default_output = default_output_dir();

    let (
        name,
        platform,
        bundle_id_prefix,
        team_id,
        organization_name,
        deployment_target,
        package_name,
        min_sdk_version,
        target_sdk_version,
        output_dir,
    ) = match form {
        Some(f) => (
            f.name.as_str(),
            f.platform.as_str(),
            f.bundle_id_prefix.as_deref().unwrap_or(""),
            f.team_id.as_deref().unwrap_or(""),
            f.organization_name.as_deref().unwrap_or(""),
            f.deployment_target.as_deref().unwrap_or(""),
            f.package_name.as_deref().unwrap_or(""),
            f.min_sdk_version.as_deref().unwrap_or(""),
            f.target_sdk_version.as_deref().unwrap_or(""),
            f.output_dir.as_deref().unwrap_or(""),
        ),
        None => (
            "",
            "ios",
            "com.axiomorient",
            "7WR76382QB",
            "axient",
            "17.0",
            "com.axiomorient.app",
            "24",
            "34",
            default_output.as_str(),
        ),
    };

    let ios_checked = if platform == "ios" { "checked" } else { "" };
    let android_checked = if platform == "android" { "checked" } else { "" };
    let both_checked = if platform == "both" { "checked" } else { "" };

    let error_html = error
        .map(|msg| format!(r##"<div class="error">{}</div>"##, escape_html(msg)))
        .unwrap_or_default();

    format!(
        r##"{}
<div class="page-content full-height">
  <div id="main-container" class="card form-container">
    <div class="form-header">
        <h1 class="form-title">New Project</h1>
        <p class="subtitle form-subtitle">Configure your mobile application</p>
    </div>

    {}
    <form hx-post="/generate" hx-target="#main-container" hx-swap="outerHTML">
      
      <!-- Essentials Row -->
      <div class="form-section compact">
        <div class="grid-2">
            <div class="form-group compact">
                <label>App Name <span class="required">*</span></label>
                <input name="name" value="{}" required placeholder="MyAwesomeApp" class="input-lg" />
            </div>
            <div class="form-group compact">
                <label class="label-muted">Output Location <span class="label-small">(Optional)</span></label>
                <div class="input-group">
                    <input name="output_dir" value="{}" placeholder="Default: generated/" />
                    <button type="button" id="choose-btn" class="choose-btn" onclick="browseFolder()">Choose</button>
                </div>
                <span id="folder-status" class="folder-status"></span>
            </div>
        </div>
      </div>

      <!-- Platform Selection -->
      <div class="form-section compact">
        <label class="section-label">Target Platform</label>
        <div class="radio-group-large">
          <label class="radio-card compact">
            <input type="radio" name="platform" value="ios" {} onchange="updatePlatformFields()">
            <span class="radio-card-content">
                <span class="radio-title">iOS</span>
                <span class="radio-desc">Xcode + Tuist</span>
            </span>
          </label>
          <label class="radio-card compact">
            <input type="radio" name="platform" value="android" {} onchange="updatePlatformFields()">
            <span class="radio-card-content">
                <span class="radio-title">Android</span>
                <span class="radio-desc">Kotlin + Gradle</span>
            </span>
          </label>
          <label class="radio-card compact">
            <input type="radio" name="platform" value="both" {} onchange="updatePlatformFields()">
            <span class="radio-card-content">
                <span class="radio-title">Both</span>
                <span class="radio-desc">Multi-platform</span>
            </span>
          </label>
        </div>
      </div>

      <!-- Configuration -->
      <div id="ios-fields" class="platform-fields compact">
        <h3 class="platform-heading">iOS Config</h3>
        <div class="grid-2">
          <div class="form-group compact">
            <label>Bundle ID Prefix</label>
            <input name="bundle_id_prefix" value="{}" placeholder="com.example" />
          </div>
          <div class="form-group compact">
            <label>Team ID</label>
            <input name="team_id" value="{}" placeholder="ABCD123456" />
          </div>
        </div>
        <div class="grid-2">
          <div class="form-group compact">
            <label>Organization</label>
            <input name="organization_name" value="{}" placeholder="MyCompany" />
          </div>
          <div class="form-group compact">
            <label>Target</label>
            <input name="deployment_target" value="{}" placeholder="17.0" />
          </div>
        </div>
      </div>

      <div id="android-fields" class="platform-fields compact hidden">
        <h3 class="platform-heading">Android Config</h3>
        <div class="form-group compact">
            <label>Package Name</label>
            <input name="package_name" value="{}" placeholder="com.example.app" />
        </div>
        <div class="grid-2">
          <div class="form-group compact">
            <label>Min SDK</label>
            <input name="min_sdk_version" value="{}" placeholder="24" />
          </div>
          <div class="form-group compact">
            <label>Target SDK</label>
            <input name="target_sdk_version" value="{}" placeholder="34" />
          </div>
        </div>
      </div>

      <div class="form-actions">
          <button type="submit" class="submit-button">Create Project</button>
      </div>
    </form>
  </div>
</div>
</body></html>"##,
        base_html_with_nav("/create"),
        error_html,
        escape_html(name),
        escape_html(output_dir),
        ios_checked,
        android_checked,
        both_checked,
        escape_html(bundle_id_prefix),
        escape_html(team_id),
        escape_html(organization_name),
        escape_html(deployment_target),
        escape_html(package_name),
        escape_html(min_sdk_version),
        escape_html(target_sdk_version)
    )
}

// ============================================================================
// Generate
// ============================================================================

pub async fn generate(
    State(state): State<Arc<AppState>>,
    Form(mut form): Form<ScaffoldForm>,
) -> Html<String> {
    use crate::state::Platform;

    let name = form.name.trim();

    if name.is_empty() {
        return Html(render_form(Some("App name is required."), Some(&form)));
    }
    if !valid_app_name(name) {
        return Html(render_form(
            Some("App name must start with a letter and contain only alphanumeric characters."),
            Some(&form),
        ));
    }

    // Check Java reserved keywords for Android/Both platforms
    let platform_str = form.platform.trim();
    if (platform_str == "android" || platform_str == "both") && is_java_reserved_keyword(name) {
        return Html(render_form(
            Some(&format!(
                "App name '{}' is a Java reserved keyword. Please choose a different name.",
                name
            )),
            Some(&form),
        ));
    }

    // Generate Android package name dynamically: com.axiomorient.app.{app_name_lowercase}
    if platform_str == "android" || platform_str == "both" {
        let app_name_lower = name.to_lowercase();
        let package_name = format!("com.axiomorient.app.{}", app_name_lower);
        form.package_name = Some(package_name);
    }

    let task_id = Uuid::new_v4();
    let name = name.to_string();
    let platform = Platform::from_str(&form.platform);
    let form_clone = form.clone();
    let state_clone = state.clone();

    // Create initial running task
    state
        .tasks
        .insert(task_id, TaskState::running(&name, platform.clone()));

    tokio::spawn(async move {
        use crate::state::BuildStatus;
        let platform_str = form_clone.platform.trim();

        let task = match platform_str {
            "ios" => {
                let (status, log, ios_path) = run_ios_generation(&name, &form_clone).await;
                TaskState {
                    status,
                    log,
                    app_name: name.clone(),
                    platform: Platform::Ios,
                    build_status: BuildStatus::NotStarted,
                    build_log: String::new(),
                    ios_path: ios_path.clone(),
                    android_path: None,
                    output_path: ios_path,
                }
            }
            "android" => {
                let (status, log, android_path) = run_android_generation(&name, &form_clone).await;
                TaskState {
                    status,
                    log,
                    app_name: name.clone(),
                    platform: Platform::Android,
                    build_status: BuildStatus::NotStarted,
                    build_log: String::new(),
                    ios_path: None,
                    android_path: android_path.clone(),
                    output_path: android_path,
                }
            }
            "both" => {
                let result = run_both_generation(&name, &form_clone).await;
                TaskState {
                    status: result.status,
                    log: result.log,
                    app_name: name.clone(),
                    platform: Platform::Both,
                    build_status: BuildStatus::NotStarted,
                    build_log: String::new(),
                    ios_path: result.ios_path.clone(),
                    android_path: result.android_path.clone(),
                    output_path: result.ios_path.or(result.android_path),
                }
            }
            _ => TaskState {
                status: TaskStatus::Failed,
                log: "Invalid platform".to_string(),
                app_name: name.clone(),
                platform: Platform::Ios,
                build_status: BuildStatus::NotStarted,
                build_log: String::new(),
                ios_path: None,
                android_path: None,
                output_path: None,
            },
        };

        // Save log to file if project was created successfully
        if task.status == TaskStatus::Success {
            if let Some(ref path) = task.output_path {
                save_log_to_file(path, &task.log);
            } else if let Some(ref ios_path) = task.ios_path {
                save_log_to_file(ios_path, &task.log);
            } else if let Some(ref android_path) = task.android_path {
                save_log_to_file(android_path, &task.log);
            }
        }

        state_clone.tasks.insert(task_id, task.clone());
    });

    Html(render_polling(task_id))
}

// ============================================================================
// Task Status
// ============================================================================

pub async fn task_card(
    AxumPath(id): AxumPath<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    if let Some(task) = state.tasks.get(&id) {
        Html(render_single_card(&task, id))
    } else {
        Html(String::new())
    }
}

pub async fn check_status(
    AxumPath(id): AxumPath<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let Some(task) = state.tasks.get(&id) else {
        return Html("Task not found".to_string());
    };

    match task.status {
        TaskStatus::Running => Html(render_polling(id)),
        TaskStatus::Success => Html(render_success_redirect()),
        TaskStatus::Failed => Html(render_result_with_task(&task, id)),
    }
}

/// Render success redirect to home page
fn render_success_redirect() -> String {
    r##"<script>window.location.href = '/';</script>"##.to_string()
}

/// Render result with separate platform downloads for Both
fn render_result_with_task(task: &TaskState, task_id: Uuid) -> String {
    use crate::state::{BuildStatus, Platform};

    let (status, status_icon) = match task.status {
        TaskStatus::Success => ("Project Created", "✓"),
        TaskStatus::Failed => ("Generation Failed", "✕"),
        TaskStatus::Running => ("Generating", "○"),
    };

    // Build verification status text
    let build_status_text = match task.build_status {
        BuildStatus::NotStarted => "Not Started",
        BuildStatus::Verifying => "Verifying...",
        BuildStatus::Verified => "✓ Verified",
        BuildStatus::Failed => "✕ Failed",
    };

    // Platform info
    let platform_name = match task.platform {
        Platform::Ios => "iOS",
        Platform::Android => "Android",
        Platform::Both => "iOS & Android",
    };

    // Build path info with beautiful cards
    let path_sections = if task.status == TaskStatus::Success {
        let mut sections = String::new();
        let has_paths = task.ios_path.is_some() || task.android_path.is_some();

        if has_paths {
            sections.push_str(r##"<div class="detail-paths">"##);

            if let Some(ref ios_path) = task.ios_path {
                sections.push_str(&format!(
                    r##"<div class="path-card">
                        <div class="path-card-header">
                            <span class="path-icon">📱</span>
                            <span class="path-label">iOS Project</span>
                        </div>
                        <code class="path-value">{}</code>
                    </div>"##,
                    escape_html(ios_path)
                ));
            }

            if let Some(ref android_path) = task.android_path {
                sections.push_str(&format!(
                    r##"<div class="path-card">
                        <div class="path-card-header">
                            <span class="path-icon">🤖</span>
                            <span class="path-label">Android Project</span>
                        </div>
                        <code class="path-value">{}</code>
                    </div>"##,
                    escape_html(android_path)
                ));
            }

            sections.push_str(r##"</div>"##);
        }
        sections
    } else {
        String::new()
    };

    // Build download buttons with verify button
    let action_buttons = if task.status == TaskStatus::Success {
        let mut buttons = String::new();

        // Download buttons
        match task.platform {
            Platform::Both => {
                if task.ios_path.is_some() {
                    buttons.push_str(&format!(
                        r##"<a href="/download/{}/ios" class="detail-button primary">
                            <span class="button-icon">⬇</span> Download iOS
                        </a>"##,
                        task_id
                    ));
                }
                if task.android_path.is_some() {
                    buttons.push_str(&format!(
                        r##"<a href="/download/{}/android" class="detail-button primary">
                            <span class="button-icon">⬇</span> Download Android
                        </a>"##,
                        task_id
                    ));
                }
            }
            _ => {
                buttons.push_str(&format!(
                    r##"<a href="/download/{}" class="detail-button primary large">
                        <span class="button-icon">⬇</span> Download Project
                    </a>"##,
                    task_id
                ));
            }
        }

        // Verify Build button if not started or failed
        if task.build_status == BuildStatus::NotStarted || task.build_status == BuildStatus::Failed
        {
            buttons.push_str(&format!(
                r##"<button class="detail-button verify"
                        hx-post="/task/{}/verify"
                        hx-target=".detail-container"
                        hx-swap="outerHTML">
                    <span class="button-icon">✓</span> Verify Build
                </button>"##,
                task_id
            ));
        }

        buttons
    } else {
        String::new()
    };

    // Verify Log - Show if build_log is not empty
    let verification_log_section = if !task.build_log.trim().is_empty() {
        format!(
            r##"<details class="detail-log verification-log">
        <summary>
            <span class="log-summary-text">Verification Log</span>
            <span class="log-summary-icon">▼</span>
        </summary>
        <pre class="log-content">{}</pre>
    </details>"##,
            escape_html(&task.build_log)
        )
    } else {
        String::new()
    };

    format!(
        r##"<div class="detail-container card">
    <!-- Status Header -->
    <div class="detail-header">
        <div class="detail-status-icon {}">{}</div>
        <h1 class="detail-title">{}</h1>
        <p class="detail-app-name">{}</p>
    </div>

    <!-- Project Info Grid -->
    <div class="detail-info-grid">
        <div class="info-card">
            <div class="info-card-label">Platform</div>
            <div class="info-card-value">{}</div>
        </div>
        <div class="info-card">
            <div class="info-card-label">Task ID</div>
            <div class="info-card-value mono">{}</div>
        </div>
        <div class="info-card">
            <div class="info-card-label">Build Status</div>
            <div class="info-card-value">{}</div>
        </div>
    </div>

    <!-- Paths Section -->
    {}

    <!-- Action Buttons -->
    <div class="detail-actions">
        {}
    </div>

    <div class="detail-footer">
        <a href="/" class="detail-link">← Back to Home</a>
    </div>

    <!-- Execution Log -->
    <details class="detail-log">
        <summary>
            <span class="log-summary-text">Execution Log</span>
            <span class="log-summary-icon">▼</span>
        </summary>
        <pre class="log-content">{}</pre>
    </details>

    <!-- Verification Log -->
    {}
</div>"##,
        if task.status == TaskStatus::Success {
            "success"
        } else {
            "failed"
        },
        status_icon,
        status,
        escape_html(&task.app_name),
        platform_name,
        &task_id.to_string()[..8],
        build_status_text,
        path_sections,
        action_buttons,
        escape_html(&task.log),
        verification_log_section
    )
}

// ============================================================================
// Task Detail
// ============================================================================

pub async fn task_detail(
    AxumPath(id): AxumPath<Uuid>,
    State(state): State<Arc<AppState>>,
) -> Html<String> {
    let Some(task) = state.tasks.get(&id) else {
        return Html(format!(
            r##"{}<div class="page-content"><div class="error">Task not found</div><a href="/tasks" class="button">Back</a></div></body></html>"##,
            base_html_with_nav("")
        ));
    };

    if task.status == TaskStatus::Running {
        return Html(format!(
            r##"{}<div class="page-content">{}</div></body></html>"##,
            base_html_with_nav(""),
            render_polling(id)
        ));
    }

    Html(format!(
        r##"{}<div class="page-content">{}</div></body></html>"##,
        base_html_with_nav(""),
        render_result_with_task(&task, id)
    ))
}

// ============================================================================
// Download
// ============================================================================

pub async fn download_project(
    AxumPath(id): AxumPath<Uuid>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    use axum::http::{header, StatusCode};
    use std::io::Write;
    use std::path::PathBuf;

    let task = match state.tasks.get(&id) {
        Some(t) => t.clone(),
        None => return (StatusCode::NOT_FOUND, "Task not found").into_response(),
    };

    if task.status != TaskStatus::Success {
        return (StatusCode::BAD_REQUEST, "Project not ready").into_response();
    }

    let output_path = match &task.output_path {
        Some(p) => PathBuf::from(p),
        None => return (StatusCode::INTERNAL_SERVER_ERROR, "No output path").into_response(),
    };

    if !output_path.exists() {
        return (StatusCode::NOT_FOUND, "Project folder not found").into_response();
    }

    // Create ZIP in memory
    let buffer = Vec::new();
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(buffer));
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    // Folders and files to exclude from ZIP (build artifacts, caches)
    let excluded = [
        ".build",
        ".DS_Store",
        "DerivedData",
        ".swiftpm",
        "xcuserdata",
    ];

    let should_exclude = |path: &std::path::Path| -> bool {
        path.components().any(|c| {
            if let std::path::Component::Normal(name) = c {
                excluded.iter().any(|ex| name.to_string_lossy() == *ex)
            } else {
                false
            }
        })
    };

    for entry in walkdir::WalkDir::new(&output_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Skip excluded folders/files
        if should_exclude(path) {
            continue;
        }

        let name = match path.strip_prefix(&output_path) {
            Ok(n) => n,
            Err(_) => continue,
        };

        if path.is_file() {
            if let Ok(contents) = std::fs::read(path) {
                #[allow(deprecated)]
                if zip.start_file(name.to_string_lossy(), options).is_ok() {
                    let _ = zip.write_all(&contents);
                }
            }
        } else if !name.as_os_str().is_empty() {
            #[allow(deprecated)]
            let _ = zip.add_directory(name.to_string_lossy(), options);
        }
    }

    let cursor = match zip.finish() {
        Ok(c) => c,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "ZIP creation failed").into_response()
        }
    };

    let buffer = cursor.into_inner();

    let folder_name = output_path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| format!("project_{}", id));

    let headers = [
        (header::CONTENT_TYPE, "application/zip".to_string()),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}.zip\"", folder_name),
        ),
    ];
    (headers, buffer).into_response()
}

/// Download specific platform (iOS or Android) from a Both task
pub async fn download_platform(
    AxumPath((id, platform)): AxumPath<(Uuid, String)>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    use axum::http::{header, StatusCode};
    use std::io::Write;
    use std::path::PathBuf;

    let task = match state.tasks.get(&id) {
        Some(t) => t.clone(),
        None => return (StatusCode::NOT_FOUND, "Task not found").into_response(),
    };

    if task.status != TaskStatus::Success {
        return (StatusCode::BAD_REQUEST, "Project not ready").into_response();
    }

    // Get platform-specific path
    let output_path = match platform.as_str() {
        "ios" => task.ios_path.as_ref().map(PathBuf::from),
        "android" => task.android_path.as_ref().map(PathBuf::from),
        _ => None,
    };

    let output_path = match output_path {
        Some(p) => p,
        None => return (StatusCode::NOT_FOUND, "Platform path not found").into_response(),
    };

    if !output_path.exists() {
        return (StatusCode::NOT_FOUND, "Project folder not found").into_response();
    }

    // Create ZIP (same logic as download_project)
    let buffer = Vec::new();
    let mut zip = zip::ZipWriter::new(std::io::Cursor::new(buffer));
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    let excluded = [
        ".build",
        ".DS_Store",
        "DerivedData",
        ".swiftpm",
        "xcuserdata",
        "build",
        ".gradle",
    ];
    let should_exclude = |path: &std::path::Path| -> bool {
        path.components().any(|c| {
            if let std::path::Component::Normal(name) = c {
                excluded.iter().any(|ex| name.to_string_lossy() == *ex)
            } else {
                false
            }
        })
    };

    for entry in walkdir::WalkDir::new(&output_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();
        if should_exclude(path) {
            continue;
        }
        let name = match path.strip_prefix(&output_path) {
            Ok(n) => n,
            Err(_) => continue,
        };

        if path.is_file() {
            if let Ok(contents) = std::fs::read(path) {
                #[allow(deprecated)]
                if zip.start_file(name.to_string_lossy(), options).is_ok() {
                    let _ = zip.write_all(&contents);
                }
            }
        } else if !name.as_os_str().is_empty() {
            #[allow(deprecated)]
            let _ = zip.add_directory(name.to_string_lossy(), options);
        }
    }

    let cursor = match zip.finish() {
        Ok(c) => c,
        Err(_) => {
            return (StatusCode::INTERNAL_SERVER_ERROR, "ZIP creation failed").into_response()
        }
    };

    let buffer = cursor.into_inner();
    let folder_name = output_path
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| format!("project_{}_{}", id, platform));

    let headers = [
        (header::CONTENT_TYPE, "application/zip".to_string()),
        (
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}.zip\"", folder_name),
        ),
    ];
    (headers, buffer).into_response()
}

// ============================================================================
// Sync Folders
// ============================================================================

pub async fn sync_folders(State(state): State<Arc<AppState>>) -> Html<String> {
    use crate::state::sync_tasks_with_filesystem;

    // Perform the sync
    sync_tasks_with_filesystem(&state);

    // Get recent tasks after sync
    let mut recent_tasks: Vec<(Uuid, TaskState)> = state
        .tasks
        .iter()
        .map(|entry| (*entry.key(), entry.value().clone()))
        .collect();
    recent_tasks.truncate(10);

    let tasks_html = render_project_cards(&recent_tasks);

    Html(format!(
        r##"<div class="recent-section">
    <div class="recent-header">
      <h2>Recent Projects</h2>
      <button class="scan-button" hx-post="/sync-folders" hx-target=".recent-section" hx-swap="outerHTML">Sync Folders</button>
    </div>
    <div class="task-grid">{}</div>
    <a href="/tasks" class="view-all-link">View All Projects</a>
  </div>"##,
        tasks_html
    ))
}

// ============================================================================
// GitHub Push
// ============================================================================

pub async fn push_to_github(
    State(state): State<Arc<AppState>>,
    Form(form): Form<GitHubPushForm>,
) -> Html<String> {
    use std::path::{Path, PathBuf};
    use std::process::Command;

    let task = match state.tasks.get(&form.task_id) {
        Some(t) => t,
        None => return Html("<div class='error'>Task not found</div>".to_string()),
    };

    if task.status != TaskStatus::Success {
        return Html("<div class='error'>Project not ready</div>".to_string());
    }

    let output_path = match &task.output_path {
        Some(p) => PathBuf::from(p),
        None => return Html("<div class='error'>No output path</div>".to_string()),
    };

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.github.com/user/repos")
        .header("Authorization", format!("token {}", form.token))
        .header("User-Agent", "mgen-scaffold")
        .json(&serde_json::json!({ "name": form.repo_name, "private": true }))
        .send()
        .await;

    let res = match res {
        Ok(r) => r,
        Err(e) => return Html(format!("<div class='error'>GitHub API Error: {}</div>", e)),
    };

    if !res.status().is_success() {
        let text = res.text().await.unwrap_or_default();
        return Html(format!("<div class='error'>Failed: {}</div>", text));
    }

    let repo_json: serde_json::Value = res.json().await.unwrap_or_default();
    let clone_url = repo_json["clone_url"].as_str().unwrap_or("");
    let html_url = repo_json["html_url"].as_str().unwrap_or("");

    if clone_url.is_empty() {
        return Html("<div class='error'>Failed to get clone URL</div>".to_string());
    }

    let auth_url = clone_url.replace("https://", &format!("https://oauth2:{}@", form.token));
    let path = output_path.clone();

    let git_result = tokio::task::spawn_blocking(move || -> Result<(), String> {
        fn run_git(path: &Path, args: &[&str]) -> Result<(), String> {
            let out = Command::new("git")
                .current_dir(path)
                .args(args)
                .output()
                .map_err(|e| e.to_string())?;
            if !out.status.success() {
                return Err(String::from_utf8_lossy(&out.stderr).to_string());
            }
            Ok(())
        }
        run_git(&path, &["init"])?;
        run_git(&path, &["add", "."])?;
        run_git(&path, &["commit", "-m", "Initial commit from MGen"])?;
        run_git(&path, &["branch", "-M", "main"])?;
        run_git(&path, &["remote", "add", "origin", &auth_url])?;
        run_git(&path, &["push", "-u", "origin", "main"])?;
        Ok(())
    })
    .await;

    match git_result {
        Ok(Ok(_)) => Html(format!(
            r##"<div class="success-message">
                <strong>Successfully pushed to GitHub!</strong><br>
                <a href="{}" target="_blank">Open Repository</a>
            </div>"##,
            html_url
        )),
        Ok(Err(e)) => Html(format!("<div class='error'>Git Error: {}</div>", e)),
        Err(e) => Html(format!("<div class='error'>Task Error: {}</div>", e)),
    }
}

// ============================================================================
// Native Folder Selection (macOS)
// ============================================================================

/// Opens a native folder picker dialog using osascript on macOS.
/// Returns the selected path as an HTML input field update.
pub async fn select_folder() -> impl IntoResponse {
    let script = r#"
        tell application "System Events"
            activate
        end tell
        set selectedFolder to choose folder with prompt "Select Output Location"
        return POSIX path of selectedFolder
    "#;

    let result = tokio::task::spawn_blocking(move || {
        std::process::Command::new("osascript")
            .args(["-e", script])
            .output()
    })
    .await;

    match result {
        Ok(Ok(output)) if output.status.success() => {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();

            // Check if the path is writable
            let is_writable = std::fs::metadata(&path)
                .map(|m| !m.permissions().readonly())
                .unwrap_or(false);

            if is_writable {
                // Return the path as JSON for the frontend to consume
                (
                    axum::http::StatusCode::OK,
                    [("Content-Type", "application/json")],
                    format!(
                        r#"{{"path":"{}","writable":true}}"#,
                        path.replace('"', "\\\"")
                    ),
                )
            } else {
                (
                    axum::http::StatusCode::OK,
                    [("Content-Type", "application/json")],
                    format!(
                        r#"{{"path":"{}","writable":false,"error":"This location is not writable. Please choose a different folder."}}"#,
                        path.replace('"', "\\\"")
                    ),
                )
            }
        }
        Ok(Ok(output)) => {
            // User cancelled or error
            let stderr = String::from_utf8_lossy(&output.stderr);
            if stderr.contains("User canceled") || stderr.contains("-128") {
                (
                    axum::http::StatusCode::OK,
                    [("Content-Type", "application/json")],
                    r#"{"cancelled":true}"#.to_string(),
                )
            } else {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    [("Content-Type", "application/json")],
                    format!(
                        r#"{{"error":"Failed to open folder picker: {}"}}"#,
                        stderr.replace('"', "\\\"")
                    ),
                )
            }
        }
        Ok(Err(e)) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            [("Content-Type", "application/json")],
            format!(
                r#"{{"error":"Failed to run osascript: {}"}}"#,
                e.to_string().replace('"', "\\\"")
            ),
        ),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            [("Content-Type", "application/json")],
            format!(
                r#"{{"error":"Task error: {}"}}"#,
                e.to_string().replace('"', "\\\"")
            ),
        ),
    }
}
