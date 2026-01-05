//! Generation logic for iOS and Android projects

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::handlers::ScaffoldForm;
use crate::state::{BuildStatus, TaskStatus};

/// Run iOS project generation
pub async fn run_ios_generation(
    name: &str,
    form: &ScaffoldForm,
) -> (TaskStatus, String, Option<String>) {
    let args = build_args(name, form, "ios");
    let script_path = script_path();

    let output = tokio::task::spawn_blocking(move || run_script(&script_path, &args))
        .await
        .unwrap_or_else(|_| Ok((1, "Failed to join task".to_string())));

    process_output(output)
}

/// Run Android project generation
pub async fn run_android_generation(
    name: &str,
    form: &ScaffoldForm,
) -> (TaskStatus, String, Option<String>) {
    let args = build_args(name, form, "android");
    let script_path = android_script_path();

    let output = tokio::task::spawn_blocking(move || run_script(&script_path, &args))
        .await
        .unwrap_or_else(|_| Ok((1, "Failed to join task".to_string())));

    process_output(output)
}

/// Result for both platform generation
pub struct BothGenerationResult {
    pub status: TaskStatus,
    pub log: String,
    pub ios_path: Option<String>,
    pub android_path: Option<String>,
}

/// Run both iOS and Android generation
pub async fn run_both_generation(name: &str, form: &ScaffoldForm) -> BothGenerationResult {
    let mut combined_log = String::new();
    let mut all_success = true;
    let mut ios_path = None;
    let mut android_path = None;

    combined_log.push_str("=== iOS Generation ===\n\n");
    let (status, log, path) = run_ios_generation(name, form).await;
    combined_log.push_str(&log);
    if status != TaskStatus::Success {
        all_success = false;
        combined_log.push_str("\n❌ iOS generation failed\n\n");
    } else {
        combined_log.push_str("\n✓ iOS generation successful\n\n");
        ios_path = path;
    }

    combined_log.push_str("=== Android Generation ===\n\n");
    let (status, log, path) = run_android_generation(name, form).await;
    combined_log.push_str(&log);
    if status != TaskStatus::Success {
        all_success = false;
        combined_log.push_str("\n❌ Android generation failed\n\n");
    } else {
        combined_log.push_str("\n✓ Android generation successful\n\n");
        android_path = path;
    }

    let final_status = if all_success {
        TaskStatus::Success
    } else {
        TaskStatus::Failed
    };

    BothGenerationResult {
        status: final_status,
        log: combined_log,
        ios_path,
        android_path,
    }
}

fn run_script(script: &Path, args: &[String]) -> Result<(i32, String), String> {
    let script = script
        .canonicalize()
        .map_err(|e| format!("Script not found: {}", e))?;
    let mut cmd = Command::new(&script);
    cmd.args(args);

    // Add mise paths for tuist/cookiecutter detection
    if let Ok(home) = env::var("HOME") {
        let mise_shims = format!("{}/.local/share/mise/shims", home);
        let mise_bin = format!("{}/.local/bin", home);
        let current_path = env::var("PATH").unwrap_or_default();
        cmd.env(
            "PATH",
            format!("{}:{}:{}", mise_shims, mise_bin, current_path),
        );
    }

    if let Some(parent) = script.parent() {
        cmd.current_dir(parent);
    }

    let output = cmd.output().map_err(|e| e.to_string())?;
    let mut combined = String::new();
    combined.push_str(&String::from_utf8_lossy(&output.stdout));
    combined.push_str(&String::from_utf8_lossy(&output.stderr));

    Ok((output.status.code().unwrap_or(1), combined))
}

fn process_output(output: Result<(i32, String), String>) -> (TaskStatus, String, Option<String>) {
    match output {
        Ok((code, log)) => {
            let status = if code == 0 {
                TaskStatus::Success
            } else {
                TaskStatus::Failed
            };
            let path = extract_path(&log);
            (status, log, path)
        }
        Err(err) => (TaskStatus::Failed, err, None),
    }
}

fn extract_path(log: &str) -> Option<String> {
    log.lines()
        .find(|line| line.contains("[INFO] Project path:"))
        .and_then(|line| line.split("[INFO] Project path:").nth(1))
        .map(|s| s.trim().to_string())
}

fn build_args(name: &str, form: &ScaffoldForm, platform: &str) -> Vec<String> {
    let mut args = vec!["--name".to_string(), name.to_string()];

    // Skip build step for web service - only generate project files
    // Also skip install and generate for "Flash" generation
    if platform == "ios" || platform == "both" {
        args.push("--skip-install".to_string());
        args.push("--skip-generate".to_string());
        args.push("--skip-build".to_string());

        if let Some(ref v) = form.bundle_id_prefix {
            if !v.trim().is_empty() {
                args.extend(["--bundle-id-prefix".to_string(), v.clone()]);
            }
        }
        if let Some(ref v) = form.team_id {
            if !v.trim().is_empty() {
                args.extend(["--team-id".to_string(), v.clone()]);
            }
        }
        if let Some(ref v) = form.organization_name {
            if !v.trim().is_empty() {
                args.extend(["--organization-name".to_string(), v.clone()]);
            }
        }
        if let Some(ref v) = form.deployment_target {
            if !v.trim().is_empty() {
                args.extend(["--deployment-target".to_string(), v.clone()]);
            }
        }
    }

    if platform == "android" || platform == "both" {
        if let Some(ref v) = form.package_name {
            if !v.trim().is_empty() {
                args.extend(["--package-name".to_string(), v.clone()]);
            }
        }
        if let Some(ref v) = form.min_sdk_version {
            if !v.trim().is_empty() {
                args.extend(["--min-sdk".to_string(), v.clone()]);
            }
        }
        if let Some(ref v) = form.target_sdk_version {
            if !v.trim().is_empty() {
                args.extend(["--target-sdk".to_string(), v.clone()]);
            }
        }
    }

    if let Some(ref v) = form.output_dir {
        if !v.trim().is_empty() {
            args.extend(["--output-dir".to_string(), v.clone()]);
        }
    }

    args
}

fn script_path() -> PathBuf {
    env::var("SCAFFOLD_SCRIPT_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("../scripts/scaffold_ios.sh"))
}

fn android_script_path() -> PathBuf {
    env::var("ANDROID_SCAFFOLD_SCRIPT_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("../scripts/scaffold_android.sh"))
}

/// Run iOS build verification
pub async fn verify_ios_build(project_path: &str) -> (BuildStatus, String) {
    let verify_script = PathBuf::from("../templates/ios/Scripts/verify_build.sh");

    let script = match verify_script.canonicalize() {
        Ok(s) => s,
        Err(e) => {
            return (
                BuildStatus::Failed,
                format!("Verify script not found: {}", e),
            )
        }
    };

    let project_path = project_path.to_string();

    let output = tokio::task::spawn_blocking(move || {
        let mut cmd = Command::new(&script);
        cmd.arg(&project_path);

        // Add mise paths for tuist detection
        if let Ok(home) = env::var("HOME") {
            let mise_shims = format!("{}/.local/share/mise/shims", home);
            let mise_bin = format!("{}/.local/bin", home);
            let current_path = env::var("PATH").unwrap_or_default();
            cmd.env(
                "PATH",
                format!("{}:{}:{}", mise_shims, mise_bin, current_path),
            );
        }

        cmd.output()
    })
    .await;

    match output {
        Ok(Ok(result)) => {
            let mut log = String::new();
            log.push_str(&String::from_utf8_lossy(&result.stdout));
            log.push_str(&String::from_utf8_lossy(&result.stderr));

            if result.status.success() {
                (BuildStatus::Verified, log)
            } else {
                (BuildStatus::Failed, log)
            }
        }
        Ok(Err(e)) => (
            BuildStatus::Failed,
            format!("Build verification error: {}", e),
        ),
        Err(e) => (BuildStatus::Failed, format!("Task join error: {}", e)),
    }
}

/// Run Android build verification
pub async fn verify_android_build(project_path: &str) -> (BuildStatus, String) {
    // For now, Android build verification is a placeholder
    // TODO: Implement actual Android build verification (./gradlew build)
    let project_path = project_path.to_string();

    tokio::task::spawn_blocking(move || {
        let gradle_wrapper = PathBuf::from(&project_path).join("gradlew");

        if !gradle_wrapper.exists() {
            return (BuildStatus::Failed, "gradlew not found".to_string());
        }

        let mut cmd = Command::new(&gradle_wrapper);
        cmd.arg("assembleDebug").current_dir(&project_path);

        match cmd.output() {
            Ok(result) => {
                let mut log = String::new();
                log.push_str(&String::from_utf8_lossy(&result.stdout));
                log.push_str(&String::from_utf8_lossy(&result.stderr));

                if result.status.success() {
                    (BuildStatus::Verified, log)
                } else {
                    (BuildStatus::Failed, log)
                }
            }
            Err(e) => (BuildStatus::Failed, format!("Build error: {}", e)),
        }
    })
    .await
    .unwrap_or_else(|e| (BuildStatus::Failed, format!("Task error: {}", e)))
}
