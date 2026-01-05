import XCTest
import Dependencies
@testable import AppDataService
@testable import AppDataServiceInterface

// MARK: - AppDataRepository Tests

final class AppDataRepositoryTests: XCTestCase {
    
    // MARK: - Test Create
    
    func testInsertItem() async throws {
        try await withDependencies {
            $0.appDataRepository = .testValue
        } operation: {
            @Dependency(\.appDataRepository) var repository
            
            let item = AppDataItem(
                title: "Test Item",
                content: "Test Content"
            )
            
            // Should not throw
            try await repository.insert(item)
        }
    }
    
    func testInsertBatch() async throws {
        try await withDependencies {
            $0.appDataRepository = .testValue
        } operation: {
            @Dependency(\.appDataRepository) var repository
            
            let items = [
                AppDataItem(title: "Item 1"),
                AppDataItem(title: "Item 2"),
                AppDataItem(title: "Item 3")
            ]
            
            try await repository.insertBatch(items)
        }
    }
    
    // MARK: - Test Read
    
    func testFetchAll() async throws {
        try await withDependencies {
            $0.appDataRepository = .testValue
        } operation: {
            @Dependency(\.appDataRepository) var repository
            
            let items = try await repository.fetchAll()
            XCTAssertEqual(items.count, 0, "Test value should return empty array")
        }
    }
    
    func testFetchById() async throws {
        try await withDependencies {
            $0.appDataRepository = .testValue
        } operation: {
            @Dependency(\.appDataRepository) var repository
            
            let item = try await repository.fetchById(UUID())
            XCTAssertNil(item, "Test value should return nil")
        }
    }
    
    func testCount() async {
        await withDependencies {
            $0.appDataRepository = .testValue
        } operation: {
            @Dependency(\.appDataRepository) var repository
            
            let count = await repository.count()
            XCTAssertEqual(count, 0, "Test value should return 0")
        }
    }
    
    // MARK: - Test Model
    
    func testAppDataItemInitialization() {
        let item = AppDataItem(
            title: "Test",
            content: "Content",
            metadata: ["key": "value"]
        )
        
        XCTAssertFalse(item.id.uuidString.isEmpty)
        XCTAssertEqual(item.title, "Test")
        XCTAssertEqual(item.content, "Content")
        XCTAssertEqual(item.metadata["key"], "value")
    }
    
    func testAppDataItemEquality() {
        let id = UUID()
        let date = Date()
        
        let item1 = AppDataItem(
            id: id,
            title: "Test",
            createdAt: date,
            updatedAt: date
        )
        
        let item2 = AppDataItem(
            id: id,
            title: "Test",
            createdAt: date,
            updatedAt: date
        )
        
        XCTAssertEqual(item1, item2)
    }
}
