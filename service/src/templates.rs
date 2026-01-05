//! HTML templates and CSS styles for MGen UI

use uuid::Uuid;

/// Escape HTML special characters
pub fn escape_html(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}

/// Generate base HTML with navigation bar
pub fn base_html_with_nav(current_path: &str) -> String {
    let nav_item = |href: &str, label: &str| {
        let active = if current_path == href { "active" } else { "" };
        format!(
            r#"<a href="{}" class="nav-link {}">{}</a>"#,
            href, active, label
        )
    };

    format!(
        r#"<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8" />
  <meta name="viewport" content="width=device-width, initial-scale=1" />
  <title>MGen - Mobile Generator</title>
  <script src="https://unpkg.com/htmx.org@1.9.10"></script>
  <style>{}</style>
  <script>{}</script>
</head>
<body>
<nav class="nav-bar">
  <a href="/" class="nav-logo">MGen</a>
  <div class="nav-links">
    {}
    {}
    {}
  </div>
</nav>
"#,
        CSS_STYLES,
        JS_SCRIPTS,
        nav_item("/", "Home"),
        nav_item("/create", "New"),
        nav_item("/tasks", "History")
    )
}

/// Render polling UI for in-progress generation
pub fn render_polling(id: Uuid) -> String {
    format!(
        r#"
<div id="main-container" class="card" hx-get="/task/{}/status" hx-trigger="every 1s" hx-swap="outerHTML">
  <div class="polling-state">
    <div class="loading-spinner"></div>
    <h2>Generating Project</h2>
    <p>Please wait while your project is being created...</p>
    <span class="task-id-display">{}</span>
  </div>
</div>
"#,
        id, id
    )
}

/// JavaScript for platform field toggling
const JS_SCRIPTS: &str = r#"
function updatePlatformFields() {
  const platform = document.querySelector('input[name="platform"]:checked');
  if (!platform) return;
  const iosFields = document.getElementById('ios-fields');
  const androidFields = document.getElementById('android-fields');
  if (!iosFields || !androidFields) return;

  if (platform.value === 'ios') {
    iosFields.classList.remove('hidden');
    androidFields.classList.add('hidden');
  } else if (platform.value === 'android') {
    iosFields.classList.add('hidden');
    androidFields.classList.remove('hidden');
  } else if (platform.value === 'both') {
    iosFields.classList.remove('hidden');
    androidFields.classList.remove('hidden');
  }
}
"#;

/// All CSS styles for MGen UI (Apple-inspired minimal design)
const CSS_STYLES: &str = r#"
@import url('https://fonts.googleapis.com/css2?family=Inter:wght@400;500;600;700&display=swap');

:root {
  --bg: #F5F5F7;
  --card: #FFFFFF;
  --text: #1d1d1f;
  --muted: #5e5e63;
  --accent: #23545b; /* Transformative Teal (WGSN 2026) */
  --accent-hover: #1b4247;
  --border: #d2d2d7;
  --nav-bg: rgba(255, 255, 255, 0.8);
  --success: #34c759;
  --error: #ff3b30;
  --shadow-sm: 0 2px 4px rgba(0,0,0,0.04);
  --shadow-md: 0 4px 12px rgba(0,0,0,0.08);
  --radius: 18px;
}

* { box-sizing: border-box; -webkit-font-smoothing: antialiased; }

body {
  margin: 0;
  font-family: -apple-system, BlinkMacSystemFont, "Inter", sans-serif;
  color: var(--text);
  background: var(--bg);
  min-height: 100vh;
  line-height: 1.5;
}

/* Navigation */
.nav-bar {
  background: var(--nav-bg);
  backdrop-filter: saturate(180%) blur(20px);
  -webkit-backdrop-filter: saturate(180%) blur(20px);
  padding: 0 max(24px, env(safe-area-inset-right));
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 52px;
  position: sticky;
  top: 0;
  z-index: 100;
  border-bottom: 1px solid rgba(0,0,0,0.05);
}

.nav-logo {
  color: var(--text);
  font-size: 16px;
  font-weight: 600;
  text-decoration: none;
  letter-spacing: -0.01em;
}

.nav-links { display: flex; gap: 24px; }

.nav-link {
  color: var(--muted);
  text-decoration: none;
  font-size: 13px;
  transition: color 0.2s;
  font-weight: 500;
}

.nav-link:hover { color: var(--text); }
.nav-link.active { color: var(--accent); }

/* Dashboard */
.dashboard { max-width: 980px; margin: 0 auto; padding: 60px 24px; }

.hero { text-align: center; margin-bottom: 80px; }

.hero h1 { 
    font-size: 48px; 
    margin: 0 0 12px 0; 
    font-weight: 700; 
    letter-spacing: -0.02em;
    color: var(--text);
}

.hero .subtitle { 
    color: var(--muted); 
    font-size: 21px; 
    line-height: 1.4;
    margin: 0 0 40px 0; 
    font-weight: 400;
}

.meta-info {
    font-size: 12px;
    color: var(--muted);
}

.meta-info code {
    background: rgba(0,0,0,0.05);
    padding: 2px 6px;
    border-radius: 4px;
    font-family: "SF Mono", monospace;
}

.feature-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 24px;
  margin: 60px auto;
  max-width: 980px;
}

.feature-card {
  padding: 0;
  text-align: center;
  background: transparent;
  box-shadow: none;
}

.feature-card h3 { 
    margin: 0 0 8px 0; 
    font-size: 17px; 
    font-weight: 600; 
}

.feature-card p { 
    margin: 0; 
    font-size: 15px; 
    color: var(--muted); 
    line-height: 1.4;
}

.create-button { 
    font-size: 17px; 
    padding: 12px 26px; 
    background: var(--accent); 
    color: white; 
    border-radius: 980px; 
    transition: transform 0.2s, background-color 0.2s;
    text-decoration: none;
    display: inline-block;
}
.create-button:hover {
    background: var(--accent-hover);
    transform: scale(1.02);
}

/* Recent Section */
.recent-section {
  background: transparent;
  padding: 0;
  box-shadow: none;
}

.recent-header {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  margin-bottom: 24px;
  border-bottom: 1px solid var(--border);
  padding-bottom: 12px;
}

.recent-header h2 { 
    margin: 0; 
    font-size: 24px; 
    font-weight: 600; 
    letter-spacing: -0.01em;
}

.scan-button {
  background: transparent;
  color: var(--accent);
  border: none;
  font-size: 14px;
  cursor: pointer;
  padding: 0;
  font-weight: 500;
}
.scan-button:hover { text-decoration: underline; color: var(--accent-hover); }

.task-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 24px;
  margin-bottom: 30px;
}

/* Redesigned Project Card */
.project-card {
  background: var(--card);
  border-radius: var(--radius);
  padding: 24px;
  position: relative;
  transition: transform 0.3s cubic-bezier(0.2, 0.8, 0.2, 1), box-shadow 0.3s;
  box-shadow: var(--shadow-sm);
  border: 1px solid rgba(0,0,0,0.04);
  cursor: pointer;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.project-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 12px 30px rgba(0,0,0,0.08);
}

.card-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.card-platform {
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: var(--muted);
}

.card-name {
  color: var(--text);
  font-size: 20px;
  font-weight: 600;
  margin-bottom: 4px;
  letter-spacing: -0.01em;
}

.card-id {
  font-family: "SF Mono", monospace;
  font-size: 11px;
  color: var(--muted);
  margin-bottom: 20px;
}

.card-actions {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
  margin-top: auto;
}

.card-button {
  background: #f5f5f7;
  color: var(--text);
  padding: 8px 16px;
  border-radius: 980px;
  font-size: 12px;
  font-weight: 500;
  text-decoration: none;
  transition: all 0.2s;
  border: none;
}

.card-button:hover {
  background: #e8e8ed;
}

.card-button.verify-button {
  background: var(--accent);
  color: white;
}
.card-button.verify-button:hover {
  background: var(--accent-hover);
}

.card-button.delete-button {
    background: transparent;
    color: var(--muted);
    padding: 8px 12px;
}
.card-button.delete-button:hover {
    color: var(--error);
    background: rgba(255, 59, 48, 0.1);
}

/* Build Badges */
.build-badge {
    font-size: 10px;
    padding: 4px 10px;
    border-radius: 980px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.03em;
}
.build-badge.pending { background: #f5f5f7; color: var(--muted); }
.build-badge.verifying { background: #fffcf0; color: #f5a623; border: 1px solid #f5a623; animation: pulse 2s infinite; }
.build-badge.verified { background: #eaffef; color: #34c759; border: 1px solid #34c759; }
.build-badge.failed { background: #fff2f2; color: #ff3b30; border: 1px solid #ff3b30; }

@keyframes pulse { 0% { opacity: 1; } 50% { opacity: 0.5; } }


/* Forms (Apple Style) */
form { margin-top: 32px; }

.form-section {
    margin-bottom: 40px;
    border-bottom: 1px solid var(--border);
    padding-bottom: 32px;
}
.form-section:last-of-type { border-bottom: none; }

.section-title {
    font-size: 18px;
    font-weight: 600;
    margin: 0 0 24px 0;
    color: var(--text);
}

.form-group { margin-bottom: 24px; }

form label { 
    font-size: 14px; 
    font-weight: 600; 
    color: var(--text); 
    margin-bottom: 8px; 
    display: block; 
}
form label .required { color: var(--accent); }

input {
  background: white;
  border: 1px solid var(--border);
  border-radius: 12px;
  padding: 14px 16px;
  font-size: 16px;
  margin-bottom: 8px; /* For hint spacing */
  transition: all 0.2s;
  width: 100%;
}

input:focus {
    outline: none;
    border-color: var(--accent);
    box-shadow: 0 0 0 4px rgba(0, 113, 227, 0.1);
}
input.input-lg {
    font-size: 18px;
    padding: 16px 20px;
}

.hint { margin: 4px 0 0 0; font-size: 13px; color: var(--muted); line-height: 1.4; }

.grid-2 { 
    display: grid; 
    grid-template-columns: 1fr 1fr; 
    gap: 24px; 
}

/* Radio Cards */
.radio-group-large {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 16px;
}
.radio-card {
    position: relative;
    cursor: pointer;
}
.radio-card input {
    position: absolute;
    opacity: 0;
    width: 0;
    height: 0;
}
.radio-card-content {
    display: flex;
    flex-direction: column;
    padding: 20px;
    background: white;
    border: 1px solid var(--border);
    border-radius: 12px;
    transition: all 0.2s;
    height: 100%;
}
.radio-card:hover .radio-card-content {
    border-color: var(--text);
    background: #fbfbfd;
}
.radio-card input:checked + .radio-card-content {
    border-color: var(--accent);
    background: #f0f7ff;
    box-shadow: 0 0 0 1px var(--accent);
}
.radio-title {
    font-weight: 600;
    font-size: 16px;
    margin-bottom: 4px;
    color: var(--text);
}
.radio-card input:checked + .radio-card-content .radio-title { color: var(--accent); }
.radio-desc {
    font-size: 13px;
    color: var(--muted);
}

.platform-fields {
  margin-top: 24px;
  padding: 24px;
  background: #fbfbfd;
  border-radius: 12px;
  border: 1px solid var(--border);
}
.platform-heading {
    font-size: 16px;
    margin: 0 0 20px 0;
    color: var(--text);
    font-weight: 600;
}

/* Compact Form Styles */
.form-section.compact {
    border-bottom: none;
    padding-bottom: 0;
    margin-bottom: 24px;
}

.platform-fields.compact {
    padding: 20px;
    margin-top: 20px;
    background: #fbfbfd;
}

.form-group.compact { margin-bottom: 0; }
.form-group.compact label { font-size: 13px; margin-bottom: 4px; }
.form-group.compact input { 
    padding: 10px 12px; 
    font-size: 14px; 
    margin-bottom: 12px;
}

/* Compact Radio Cards */
.radio-card.compact .radio-card-content {
    padding: 12px 16px;
    text-align: center;
    border-radius: 10px;
}
.radio-card.compact .radio-title {
    font-size: 14px;
    margin-bottom: 2px;
}
.radio-card.compact .radio-desc {
    font-size: 11px;
}

/* Submit Button */
.submit-button {
    width: 100%;
    background: var(--accent); /* Updated to 2026 Color */
    color: white;
    padding: 18px;
    font-size: 17px;
    font-weight: 600;
    border-radius: 14px;
    margin-top: 12px;
    transition: transform 0.2s, background-color 0.2s;
    border: none;
    cursor: pointer;
}
.submit-button:hover {
    background: var(--accent-hover);
    transform: scale(1.01);
}
.submit-button:active { transform: scale(0.99); }

/* Timeline Item (History) */
.timeline-item {
  margin-bottom: 20px;
  cursor: pointer;
  transition: transform 0.2s;
}

.timeline-item:hover {
  transform: translateX(4px);
}

.timeline-item:hover .timeline-content {
  box-shadow: var(--shadow-md);
  border-color: rgba(0,0,0,0.08);
}

.timeline-content {
    background: white;
    border-radius: 12px;
    padding: 20px;
    border: 1px solid rgba(0,0,0,0.05);
    box-shadow: var(--shadow-sm);
    transition: all 0.2s;
}

.timeline-marker {
    background: white;
    border: 2px solid var(--border);
    box-shadow: 0 2px 4px rgba(0,0,0,0.05);
}

.timeline-marker.success { border-color: #34c759; color: #34c759; }

/* Detail Actions */
.detail-actions {
  display: flex;
  gap: 16px;
  justify-content: center;
  margin-bottom: 40px;
  flex-wrap: wrap;
}

.detail-button {
  display: inline-flex;
  align-items: center;
  gap: 8px;
  padding: 14px 28px;
  border-radius: 980px;
  font-size: 15px;
  font-weight: 600;
  text-decoration: none;
  transition: all 0.2s;
  border: none;
  cursor: pointer;
  background: white;
  color: var(--text);
  box-shadow: 0 2px 8px rgba(0,0,0,0.08);
  border: 1px solid var(--border);
}

.detail-button:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 20px rgba(0,0,0,0.12);
}

.detail-button.primary {
  background: var(--accent);
  color: white;
  border: none;
  box-shadow: 0 4px 12px rgba(35, 84, 91, 0.3);
}

.detail-button.primary:hover {
  background: var(--accent-hover);
  box-shadow: 0 6px 20px rgba(35, 84, 91, 0.4);
}

.detail-button.primary.large {
  padding: 16px 36px;
  font-size: 16px;
}

.detail-button.verify {
  background: linear-gradient(135deg, #34c759 0%, #30d158 100%);
  color: white;
  border: none;
  box-shadow: 0 4px 12px rgba(52, 199, 89, 0.3);
}

.detail-button.verify:hover {
  box-shadow: 0 6px 20px rgba(52, 199, 89, 0.4);
}

.button-icon {
  font-size: 16px;
}

.detail-footer {
  text-align: center;
  padding-top: 32px;
  border-top: 1px solid var(--border);
  margin-bottom: 32px;
}

.detail-link {
  color: var(--accent);
  text-decoration: none;
  font-size: 15px;
  font-weight: 500;
  transition: all 0.2s;
}

.detail-link:hover {
  color: var(--accent-hover);
  text-decoration: underline;
}

/* Detail Log */
.detail-log {
  background: white;
  border-radius: 14px;
  padding: 0;
  box-shadow: var(--shadow-md);
  border: 1px solid var(--border);
  overflow: hidden;
  margin-top: 0;
}

.detail-log summary {
  cursor: pointer;
  font-weight: 600;
  color: var(--text);
  padding: 20px 24px;
  user-select: none;
  background: linear-gradient(135deg, #fbfbfd 0%, #f8f9fa 100%);
  border-bottom: 1px solid var(--border);
  font-size: 15px;
  display: flex;
  justify-content: space-between;
  align-items: center;
  transition: all 0.2s;
}

.detail-log summary:hover {
  background: linear-gradient(135deg, #f5f5f7 0%, #f0f1f2 100%);
}

.detail-log[open] summary {
  background: var(--accent);
  color: white;
  border-bottom-color: var(--accent);
}

.detail-log[open] summary .log-summary-icon {
  transform: rotate(180deg);
}

.log-summary-text {
  font-weight: 600;
}

.log-summary-icon {
  font-size: 12px;
  transition: transform 0.3s;
}

.log-content {
  margin: 0;
  padding: 24px;
  background: #1d1d1f;
  color: #f5f5f7;
  overflow-x: auto;
  font-size: 13px;
  font-family: "SF Mono", Monaco, monospace;
  line-height: 1.7;
  border-radius: 0 0 14px 14px;
  max-height: 500px;
  overflow-y: auto;
}

.log-content::-webkit-scrollbar {
  width: 10px;
  height: 10px;
}

.log-content::-webkit-scrollbar-track {
  background: #2d2d2f;
}

.log-content::-webkit-scrollbar-thumb {
  background: #4d4d4f;
  border-radius: 5px;
}

.log-content::-webkit-scrollbar-thumb:hover {
  background: #5d5d5f;
}

/* Info Value */
.info-value {
  font-weight: 500;
  color: var(--text);
  font-size: 14px;
  text-align: right;
}

/* Page Content */
.page-content {
  max-width: 980px;
  margin: 0 auto;
  padding: 40px 20px;
}

.page-content.narrow {
  max-width: 800px;
}

.page-content.full-height {
  max-width: 100%;
  min-height: calc(100vh - 52px);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
}

.page-header {
  text-align: center;
  margin-bottom: 40px;
}

.page-subtitle {
  color: var(--muted);
  font-size: 15px;
}

/* Detail Container */
.detail-container {
  max-width: 900px;
  margin: 40px auto;
  padding: 48px;
  background: var(--card);
}

.detail-header {
  text-align: center;
  margin-bottom: 48px;
  padding-bottom: 32px;
  border-bottom: 1px solid var(--border);
}

.detail-status-icon {
  margin: 0 auto 24px;
  width: 80px;
  height: 80px;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 40px;
  border-radius: 50%;
  box-shadow: 0 4px 12px rgba(0,0,0,0.08);
}

.detail-status-icon.success {
  background: linear-gradient(135deg, #34c759 0%, #30d158 100%);
  color: white;
}

.detail-status-icon.failed {
  background: linear-gradient(135deg, #ff3b30 0%, #ff453a 100%);
  color: white;
}

.detail-title {
  font-size: 32px;
  margin: 0 0 12px 0;
  font-weight: 700;
  letter-spacing: -0.02em;
}

.detail-app-name {
  font-size: 19px;
  color: var(--muted);
  margin: 0;
  font-weight: 500;
}

/* Info Grid */
.detail-info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
  margin-bottom: 40px;
}

.info-card {
  background: #fbfbfd;
  border-radius: 12px;
  padding: 20px;
  border: 1px solid var(--border);
  transition: all 0.2s;
}

.info-card:hover {
  border-color: var(--accent);
  background: white;
  box-shadow: var(--shadow-sm);
}

.info-card-label {
  font-size: 12px;
  color: var(--muted);
  text-transform: uppercase;
  letter-spacing: 0.05em;
  font-weight: 600;
  margin-bottom: 8px;
}

.info-card-value {
  font-size: 17px;
  color: var(--text);
  font-weight: 600;
}

.info-card-value.mono {
  font-family: "SF Mono", Monaco, monospace;
  font-size: 14px;
  color: var(--accent);
}

/* Path Cards */
.detail-paths {
  margin-bottom: 40px;
}

.path-card {
  background: linear-gradient(135deg, #f8f9fa 0%, #ffffff 100%);
  border: 1px solid var(--border);
  border-radius: 14px;
  padding: 24px;
  margin-bottom: 16px;
  transition: all 0.3s;
}

.path-card:hover {
  transform: translateX(4px);
  box-shadow: var(--shadow-md);
  border-color: var(--accent);
}

.path-card:last-child {
  margin-bottom: 0;
}

.path-card-header {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-bottom: 12px;
}

.path-icon {
  font-size: 24px;
}

.path-label {
  font-size: 15px;
  font-weight: 600;
  color: var(--text);
}

.path-value {
  display: block;
  background: rgba(0,0,0,0.03);
  padding: 12px 16px;
  border-radius: 8px;
  font-family: "SF Mono", Monaco, monospace;
  font-size: 13px;
  color: var(--accent);
  word-break: break-all;
  border: 1px solid rgba(0,0,0,0.05);
}

.info-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 0;
  border-bottom: 1px solid var(--border);
}

.info-row:last-child {
  border-bottom: none;
}

.info-label {
  font-size: 14px;
  color: var(--muted);
  font-weight: 500;
}

/* Form Container */
.form-container {
  padding: 40px;
  width: 100%;
  max-width: 800px;
  margin: 0;
}

.form-header {
  text-align: center;
  margin-bottom: 32px;
}

.form-title {
  font-size: 28px;
  margin-bottom: 8px;
}

.form-subtitle {
  margin: 0;
}

/* Timeline */
.timeline {
  max-width: 100%;
}

/* Success Message */
.success-message {
  background: #d4edda;
  color: #155724;
  padding: 16px;
  border-radius: 8px;
}

.success-message a {
  color: #155724;
  font-weight: 600;
  text-decoration: underline;
}

/* Empty State */
.empty-state {
  text-align: center;
  padding: 60px 20px;
}

.empty-state p {
  color: var(--muted);
  font-size: 15px;
}

/* Utility Classes */
.hidden {
  display: none !important;
}

.label-muted {
  color: var(--muted) !important;
  font-weight: normal !important;
}

.label-small {
  font-size: 11px;
}

.section-label {
  margin-bottom: 12px;
  display: block;
}

.form-actions {
  margin-top: 24px;
}

/* Error Message */
.error {
  background: #fff2f2;
  color: var(--error);
  padding: 12px 16px;
  border-radius: 8px;
  margin-bottom: 20px;
  font-size: 14px;
  border: 1px solid var(--error);
}
"#;
