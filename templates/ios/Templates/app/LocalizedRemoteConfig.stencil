import Foundation

/// Localized Remote Config value parser.
///
/// **IMPORTANT**: All Remote Config text values MUST be in JSON format with language codes as keys.
///
/// Example JSON format:
/// ```json
/// {
///   "ko": "한국어 메시지",
///   "en": "English message",
///   "ja": "日本語メッセージ"
/// }
/// ```
///
/// Usage:
/// ```swift
/// let rawValue = remoteConfigService.getString(forKey: "maintenance_message")
/// let languageCode = await appState.effectiveLanguageCode()
/// let localized = LocalizedRemoteConfig.localize(rawValue, forLanguage: languageCode)
/// // Returns the message in the specified language, or nil if not found
/// ```
public enum LocalizedRemoteConfig {

    /// Get all available languages from a localized Remote Config value.
    ///
    /// - Parameter rawValue: The raw JSON string from Remote Config
    /// - Returns: Array of language codes, or empty array if not valid JSON format
    public static func availableLanguages(_ rawValue: String) -> [String] {
        guard !rawValue.isEmpty,
              let data = rawValue.data(using: .utf8),
              let json = try? JSONSerialization.jsonObject(with: data) as? [String: String] else {
            return []
        }

        return Array(json.keys).sorted()
    }

    /// Get a specific language version from a localized Remote Config value.
    ///
    /// - Parameters:
    ///   - rawValue: The raw string from Remote Config
    ///   - languageCode: The desired language code (e.g., "ko", "en", "ja")
    /// - Returns: The localized string, or nil if not found
    public static func localize(_ rawValue: String, forLanguage languageCode: String) -> String? {
        guard !rawValue.isEmpty,
              let data = rawValue.data(using: .utf8),
              let json = try? JSONSerialization.jsonObject(with: data) as? [String: String] else {
            return nil
        }

        return json[languageCode]
    }
}
