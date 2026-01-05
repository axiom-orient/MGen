import Foundation

/// App Version - Semantic version comparison utility
struct AppVersion: Comparable, Sendable {
  let rawValue: String
  private let components: [Int]

  init?(_ rawValue: String) {
    let parts = rawValue.split(separator: ".").compactMap { Int($0) }
    guard !parts.isEmpty else {
      return nil
    }
    self.rawValue = rawValue
    self.components = parts
  }

  /// Current app version from bundle
  static var current: AppVersion? {
    guard let version = Bundle.main.infoDictionary?["CFBundleShortVersionString"] as? String else {
      return nil
    }
    return AppVersion(version)
  }

  static func < (lhs: AppVersion, rhs: AppVersion) -> Bool {
    let maxCount = max(lhs.components.count, rhs.components.count)
    for index in 0..<maxCount {
      let left = index < lhs.components.count ? lhs.components[index] : 0
      let right = index < rhs.components.count ? rhs.components[index] : 0
      if left != right {
        return left < right
      }
    }
    return false
  }
}
