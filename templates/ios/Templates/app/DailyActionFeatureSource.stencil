import ComposableArchitecture
import SwiftUI
import DailyAction

@Reducer
public struct DailyActionListFeature {
    public init() {}
    
    @ObservableState
    public struct State: Equatable, Sendable {
        public var actions: [DailyAction] = []
        public var isLoading: Bool = false
        public init() {}
    }
    
    public enum Action: BindableAction, Sendable {
        case binding(BindingAction<State>)
        case onAppear
        case loadActionsResponse(Result<[DailyAction], Error>)
        case addActionButtonTapped
        case deleteButtonTapped(IndexSet)
        case toggleCompletion(DailyAction)
    }
    
    @Dependency(\.dailyActionRepository) var repository
    
    public var body: some ReducerOf<Self> {
        BindingReducer()
        Reduce { state, action in
            switch action {
            case .onAppear:
                state.isLoading = true
                return .run { send in
                    do {
                        let actions = try await repository.fetchAll()
                        await send(.loadActionsResponse(.success(actions)))
                    } catch {
                        await send(.loadActionsResponse(.failure(error)))
                    }
                }
                
            case let .loadActionsResponse(.success(actions)):
                state.isLoading = false
                state.actions = actions
                return .none
                
            case .loadActionsResponse(.failure):
                state.isLoading = false
                return .none
                
            case .addActionButtonTapped:
                let newAction = DailyAction(id: UUID(), title: "New Action \(Date().formatted(date: .omitted, time: .shortened))")
                return .run { send in
                    try await repository.add(newAction)
                    await send(.onAppear)
                }
                
            case let .deleteButtonTapped(indexSet):
                let idsToDelete = indexSet.map { state.actions[$0].id }
                return .run { send in
                    for id in idsToDelete {
                        try await repository.delete(id)
                    }
                    await send(.onAppear)
                }
                
            case let .toggleCompletion(action):
                // Create a copy with toggled status
                // updated.isCompleted.toggle() // Model is immutable?
                // Need to create copy.
                let newAction = DailyAction(id: action.id, title: action.title, isCompleted: !action.isCompleted, createdAt: action.createdAt)
                return .run { send in
                    try await repository.update(newAction)
                    await send(.onAppear)
                }
                
            case .binding:
                return .none
            }
        }
    }
}

public struct DailyActionListView: View {
    @Bindable var store: StoreOf<DailyActionListFeature>
    
    public init(store: StoreOf<DailyActionListFeature>) {
        self.store = store
    }
    
    public var body: some View {
        List {
            if store.isLoading {
                ProgressView()
            } else if store.actions.isEmpty {
                Text("No actions yet. Tap + to add one.")
                    .foregroundStyle(.secondary)
            } else {
                ForEach(store.actions) { action in
                    HStack {
                        Text(action.title)
                        Spacer()
                        if action.isCompleted {
                            Image(systemName: "checkmark.circle.fill")
                                .foregroundStyle(.green)
                        } else {
                            Image(systemName: "circle")
                                .foregroundStyle(.gray)
                        }
                    }
                    .contentShape(Rectangle())
                    .onTapGesture {
                        store.send(.toggleCompletion(action))
                    }
                }
                .onDelete { indexSet in
                    store.send(.deleteButtonTapped(indexSet))
                }
            }
        }
        .navigationTitle("Daily Actions")
        .toolbar {
            ToolbarItem(placement: .primaryAction) {
                Button(action: { store.send(.addActionButtonTapped) }) {
                    Image(systemName: "plus")
                }
            }
        }
        .onAppear {
            store.send(.onAppear)
        }
    }
}
