//! MGen - Mobile Generator Service
//!
//! A web service for generating iOS and Android project scaffolds.

mod generation;
mod github;
mod handlers;
mod state;
mod templates;
mod utils;

use axum::{
    routing::{delete, get, post},
    Router,
};
use std::env;
use tokio::net::TcpListener;

use handlers::{
    check_status, dashboard, delete_task, download_platform, download_project, generate, health,
    push_to_github, select_folder, show_form, sync_folders, task_card, task_detail, tasks_list,
    update_output_dir, verify_build,
};
use state::AppState;

#[tokio::main]
async fn main() {
    let shared_state = AppState::new();

    let app = Router::new()
        .route("/", get(dashboard))
        .route("/create", get(show_form))
        .route("/generate", post(generate))
        .route("/tasks", get(tasks_list))
        .route("/task/:id", get(task_detail))
        .route("/task/:id/card", get(task_card))
        .route("/task/:id/status", get(check_status))
        .route("/task/:id/verify", post(verify_build))
        .route("/task/:id", delete(delete_task))
        .route("/download/:id", get(download_project))
        .route("/download/:id/:platform", get(download_platform))
        .route("/sync-folders", post(sync_folders))
        .route("/api/select-folder", get(select_folder))
        .route("/api/update-output-dir", post(update_output_dir))
        .route("/github/push", post(push_to_github))
        .route("/health", get(health))
        .with_state(shared_state);

    let addr = env::var("SCAFFOLD_SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:3000".to_string());
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Failed to bind server address");

    println!("MGen server running at http://{}", addr);
    axum::serve(listener, app).await.expect("Server error");
}
