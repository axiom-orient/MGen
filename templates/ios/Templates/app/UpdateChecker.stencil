import Foundation

// MARK: - Update Checker Protocol

/// UpdateChecker - Single Responsibility for version comparison.
/// Decoupled from Remote Config and UI concerns.
public protocol UpdateChecker: Sendable {
    /// Check if the current app version requires a forced update.
    ///
    /// - Parameters:
    ///   - currentVersion: The current app version (e.g., "1.2.3")
    ///   - minimumVersion: The minimum required version from Remote Config
    /// - Returns: true if update is required (current < minimum)
    func isUpdateRequired(currentVersion: String, minimumVersion: String) -> Bool
    
    /// Get the App Store URL for this app.
    var storeURL: URL { get }
}

// MARK: - Default Implementation

/// Default implementation using semantic versioning comparison.
public final class DefaultUpdateChecker: UpdateChecker {
    private let appStoreId: String

    public init(appStoreId: String = AppConstants.App.appStoreId) {
        self.appStoreId = appStoreId
    }

    public var storeURL: URL {
        if appStoreId.isEmpty {
            // Generic App Store URL - will open App Store app
            return URL(string: "https://apps.apple.com")!
        }
        return URL(string: "https://apps.apple.com/app/id\(appStoreId)")!
    }
    
    public func isUpdateRequired(currentVersion: String, minimumVersion: String) -> Bool {
        guard !minimumVersion.isEmpty else { return false }
        
        let currentParts = parseVersion(currentVersion)
        let minimumParts = parseVersion(minimumVersion)
        
        let maxLength = max(currentParts.count, minimumParts.count)
        
        for i in 0..<maxLength {
            let current = i < currentParts.count ? currentParts[i] : 0
            let minimum = i < minimumParts.count ? minimumParts[i] : 0
            
            if current < minimum { return true }
            if current > minimum { return false }
        }
        
        return false // Versions are equal
    }
    
    private func parseVersion(_ version: String) -> [Int] {
        return version
            .split(separator: ".")
            .compactMap { part -> Int? in
                // Handle versions like "1.2.3-beta" by taking only numeric part
                let numericPart = part.prefix(while: { $0.isNumber })
                return Int(numericPart)
            }
    }
}

// MARK: - Dependency Key

import ComposableArchitecture
import Dependencies

private enum UpdateCheckerKey: DependencyKey {
    static let liveValue: any UpdateChecker = DefaultUpdateChecker()
}

public extension DependencyValues {
    var updateChecker: any UpdateChecker {
        get { self[UpdateCheckerKey.self] }
        set { self[UpdateCheckerKey.self] = newValue }
    }
}
