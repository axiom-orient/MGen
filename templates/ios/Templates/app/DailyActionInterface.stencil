import Dependencies
import Foundation

public protocol DailyActionRepository: Sendable {
    func fetchAll() async throws -> [DailyAction]
    func add(_ action: DailyAction) async throws
    func update(_ action: DailyAction) async throws
    func delete(_ id: UUID) async throws
}

public enum DailyActionRepositoryKey: TestDependencyKey {
    public static let testValue: any DailyActionRepository = UnimplementedDailyActionRepository()
}

public extension DependencyValues {
    var dailyActionRepository: any DailyActionRepository {
        get { self[DailyActionRepositoryKey.self] }
        set { self[DailyActionRepositoryKey.self] = newValue }
    }
}

struct UnimplementedDailyActionRepository: DailyActionRepository {
    func fetchAll() async throws -> [DailyAction] {
        unimplemented("DailyActionRepository.fetchAll")
        return []
    }
    func add(_ action: DailyAction) async throws {
        unimplemented("DailyActionRepository.add")
    }
    func update(_ action: DailyAction) async throws {
        unimplemented("DailyActionRepository.update")
    }
    func delete(_ id: UUID) async throws {
        unimplemented("DailyActionRepository.delete")
    }
}
