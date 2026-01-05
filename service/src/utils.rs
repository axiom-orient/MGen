//! Utility functions for MGen service

use std::env;
use std::path::PathBuf;

/// Validate app name (alphanumeric, starts with letter)
pub fn valid_app_name(value: &str) -> bool {
    let mut chars = value.chars();
    match chars.next() {
        Some(first) if first.is_ascii_alphabetic() => (),
        _ => return false,
    }
    chars.all(|c| c.is_ascii_alphanumeric())
}

/// Check if name is a Java reserved keyword (case-insensitive)
pub fn is_java_reserved_keyword(name: &str) -> bool {
    const JAVA_KEYWORDS: &[&str] = &[
        "abstract", "continue", "for", "new", "switch", "assert", "default", "goto", "package",
        "synchronized", "boolean", "do", "if", "private", "this", "break", "double", "implements",
        "protected", "throw", "byte", "else", "import", "public", "throws", "case", "enum",
        "instanceof", "return", "transient", "catch", "extends", "int", "short", "try", "char",
        "final", "interface", "static", "void", "class", "finally", "long", "strictfp", "volatile",
        "const", "float", "native", "super", "while",
    ];

    let name_lower = name.to_lowercase();
    JAVA_KEYWORDS.contains(&name_lower.as_str())
}

/// Get default output directory
pub fn default_output_dir() -> String {
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let project_root = current_dir
        .parent()
        .map(|p| p.to_path_buf())
        .unwrap_or(current_dir);
    project_root.join("generated").to_string_lossy().to_string()
}
