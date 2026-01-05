import Foundation

/// Generic feature variants for A/B testing
public enum FeatureVariant: String, Sendable, Codable, Equatable {
  case control      // Default experience
  case variantA     // First test variant
  case variantB     // Second test variant
}

public enum RemoteConfigValue: Equatable, Sendable {
  case bool(Bool)
  case string(String)
  case number(Double)
  case data(Data)

  public var stringValue: String? {
    switch self {
    case let .string(value):
      return value
    case let .number(value):
      return String(value)
    case let .bool(value):
      return value ? "true" : "false"
    case .data:
      return nil
    }
  }

  public var boolValue: Bool? {
    switch self {
    case let .bool(value):
      return value
    case let .string(value):
      return Bool(value)
    case let .number(value):
      return value != 0
    case .data:
      return nil
    }
  }

  public var numberValue: Double? {
    switch self {
    case let .number(value):
      return value
    case let .string(value):
      return Double(value)
    case let .bool(value):
      return value ? 1 : 0
    case .data:
      return nil
    }
  }
}
