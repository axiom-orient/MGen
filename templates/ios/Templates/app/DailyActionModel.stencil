import Foundation

public struct DailyAction: Equatable, Identifiable, Sendable, Codable {
    public let id: UUID
    public let title: String
    public let isCompleted: Bool
    public let createdAt: Date
    
    public init(id: UUID, title: String, isCompleted: Bool = false, createdAt: Date = Date()) {
        self.id = id
        self.title = title
        self.isCompleted = isCompleted
        self.createdAt = createdAt
    }
}
