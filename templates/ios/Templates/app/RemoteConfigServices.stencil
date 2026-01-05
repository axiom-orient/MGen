@preconcurrency import Foundation
import OSLog

#if canImport(Network)
import Network
#endif

#if canImport(FirebaseRemoteConfig)
@preconcurrency import FirebaseRemoteConfig
#endif

#if canImport(FirebaseCore)
@preconcurrency import FirebaseCore
#endif

#if canImport(FirebaseInstallations)
@preconcurrency import FirebaseInstallations
#endif

// MARK: - Remote Config Contracts

public enum RemoteConfigError: Error, LocalizedError {
    case fetchFailed(String)
    
    public var errorDescription: String? {
        switch self {
        case let .fetchFailed(reason):
            return "Remote Config fetch failed: \(reason)"
        }
    }
}

public protocol RemoteConfigService: Sendable {
    func fetchAndActivate() async throws
    func getString(forKey key: String) -> String
    func getBool(forKey key: String) -> Bool
    func getInt(forKey key: String) -> Int
    func getDouble(forKey key: String) -> Double
    func getData(forKey key: String) -> Data
    func getFeatureVariant(forKey key: String) async -> FeatureVariant
}

// MARK: - Noop Remote Config Service

public final class NoopRemoteConfigService: RemoteConfigService {
    private let logger = Logger(subsystem: "com.test.testmgenapp", category: "RemoteConfig.Noop")
    private let values: [String: RemoteConfigValue]
    
    public init(defaults: [String: RemoteConfigValue] = [:]) {
        var mutableValues = defaults

        if mutableValues[AppConstants.RemoteConfig.welcomeMessageKey] == nil {
            mutableValues[AppConstants.RemoteConfig.welcomeMessageKey] = .string("Welcome to TestMGenApp!")
        }
        self.values = mutableValues
    }
    
    public func fetchAndActivate() async throws {
        let keysDescription = values.keys.sorted().joined(separator: ", ")
        logger.info("Serving static Remote Config defaults for keys: \(keysDescription, privacy: .public)")
    }
    
    public func getString(forKey key: String) -> String {
        values[key]?.stringValue ?? ""
    }
    
    public func getBool(forKey key: String) -> Bool {
        values[key]?.boolValue ?? false
    }
    
    public func getInt(forKey key: String) -> Int {
        Int(values[key]?.numberValue ?? 0)
    }
    
    public func getDouble(forKey key: String) -> Double {
        values[key]?.numberValue ?? 0
    }
    
    public func getData(forKey key: String) -> Data {
        if case let .data(data) = values[key] {
            return data
        }
        return Data()
    }
    
    public func getFeatureVariant(forKey key: String) async -> FeatureVariant {
        .control
    }
}

// MARK: - Firebase Remote Config Adapter

#if canImport(FirebaseRemoteConfig)
public enum FirebaseRemoteConfigAdapterError: Error, LocalizedError {
    case firebaseAppNotConfigured
    
    public var errorDescription: String? {
        switch self {
        case .firebaseAppNotConfigured:
            return "FirebaseApp is not configured. Verify GoogleService-Info.plist or Firebase setup."
        }
    }
}

public final class FirebaseRemoteConfigAdapter: RemoteConfigService, @unchecked Sendable {
    private let remoteConfig: RemoteConfig
    private let logger = Logger(subsystem: "com.test.testmgenapp", category: "RemoteConfig")
    private let fetchTimeoutSeconds: Double
    private let fetchTimeoutNanoseconds: UInt64
    private let minimumFetchInterval: TimeInterval
    
    public init(
        defaults: [String: any Sendable] = [:],
        minimumFetchInterval: TimeInterval = 600, // 10 minutes
        fetchTimeout: TimeInterval = 60
    ) throws {
        if Thread.isMainThread {
#if canImport(FirebaseCore)
            if FirebaseApp.app() == nil {
                FirebaseApp.configure()
            }
#endif
        } else {
            DispatchQueue.main.sync {
#if canImport(FirebaseCore)
                if FirebaseApp.app() == nil {
                    FirebaseApp.configure()
                }
#endif
            }
        }
        
#if canImport(FirebaseCore)
        guard FirebaseApp.app() != nil else {
            throw FirebaseRemoteConfigAdapterError.firebaseAppNotConfigured
        }
#endif
        
        let remoteConfig = RemoteConfig.remoteConfig()
        let settings = RemoteConfigSettings()
        let sanitizedTimeout = max(fetchTimeout, 1)
        settings.minimumFetchInterval = minimumFetchInterval
        settings.fetchTimeout = sanitizedTimeout
        remoteConfig.configSettings = settings
        if !defaults.isEmpty {
            let nsObjectDefaults = defaults.compactMapValues { $0 as? NSObject }
            remoteConfig.setDefaults(nsObjectDefaults)
        }
        
        self.remoteConfig = remoteConfig
        self.minimumFetchInterval = minimumFetchInterval
        self.fetchTimeoutSeconds = sanitizedTimeout
        self.fetchTimeoutNanoseconds = UInt64((sanitizedTimeout * 1_000_000_000).rounded())
        
#if canImport(Network)
        RemoteConfigNetworkMonitor.shared.startIfNeeded(logger: logger)
#endif
        
#if canImport(FirebaseInstallations)
        Task.detached { [logger] in
            do {
                let installationID = try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<String?, Error>) in
                    Installations.installations().installationID { id, error in
                        if let error {
                            continuation.resume(throwing: error)
                            return
                        }
                        continuation.resume(returning: id)
                    }
                }
                logger.info("🆔 Firebase Installations ID resolved: \(installationID ?? "nil", privacy: .public)")
            } catch {
                logger.error("❌ Failed to obtain Firebase Installations ID: \(error.localizedDescription, privacy: .public)")
            }
        }
#endif
    }
    
    public func fetchAndActivate() async throws {
#if canImport(FirebaseRemoteConfig)
        try await withCheckedThrowingContinuation { (continuation: CheckedContinuation<Void, Error>) in
            let completionLock = OSAllocatedUnfairLock(initialState: false)
            let logger = self.logger
            let timeoutSeconds = self.fetchTimeoutSeconds
            let timeoutNanoseconds = self.fetchTimeoutNanoseconds
            let remoteConfig = self.remoteConfig
            let adapter = self
            let startTime = Date()
            
            let timeoutTask = Task<Void, Never> {
                do {
                    try await Task.sleep(nanoseconds: timeoutNanoseconds)
                } catch {
                    logger.debug("⏱️ Timeout task cancelled")
                    return
                }
                
                let shouldResume = completionLock.withLock { completed -> Bool in
                    if completed {
                        return false
                    }
                    completed = true
                    return true
                }
                
                guard shouldResume else {
                    logger.debug("⏱️ Timeout occurred but already completed")
                    return
                }
                
                let elapsed = Date().timeIntervalSince(startTime)
                logger.error("❌ Remote Config fetch timed out after \(elapsed, privacy: .public)s (expected: \(timeoutSeconds, privacy: .public)s)")
                logger.error("❌ Timeout details: lastFetchStatus=\(String(describing: remoteConfig.lastFetchStatus), privacy: .public)")
                await adapter.diagnoseConnectivity()
                continuation.resume(
                    throwing: RemoteConfigError.fetchFailed(
                        "Remote Config fetch timed out after \(Int(timeoutSeconds)) seconds"
                    )
                )
            }
            
            DispatchQueue.main.async {
                logger.info("📞 Invoking Firebase RemoteConfig.fetchAndActivate on main thread at \(Date(), privacy: .public)")
                
                remoteConfig.fetchAndActivate { status, error in
                    let elapsed = Date().timeIntervalSince(startTime)
                    logger.info("📞 Firebase RemoteConfig.fetchAndActivate callback invoked after \(elapsed, privacy: .public)s")
                    
                    let shouldResume = completionLock.withLock { completed -> Bool in
                        if completed {
                            return false
                        }
                        completed = true
                        return true
                    }
                    
                    guard shouldResume else {
                        logger.warning("⚠️ Callback invoked but already completed (likely after timeout)")
                        return
                    }
                    
                    timeoutTask.cancel()
                    
                    if let error {
                        logger.error("❌ Remote Config fetch failed: \(error.localizedDescription, privacy: .public)")
                        logger.error("❌ Error domain: \((error as NSError).domain, privacy: .public), code: \((error as NSError).code, privacy: .public)")
                        Task { await adapter.diagnoseConnectivity() }
                        continuation.resume(throwing: RemoteConfigError.fetchFailed(error.localizedDescription))
                        return
                    }
                    
                    if status == .error {
                        logger.error("❌ Remote Config fetch completed with status .error without an error payload")
                        Task { await adapter.diagnoseConnectivity() }
                        continuation.resume(throwing: RemoteConfigError.fetchFailed("Remote Config fetch returned status .error"))
                        return
                    }
                    
                    let remoteKeys = remoteConfig.allKeys(from: .remote).sorted().joined(separator: ", ")
                    let defaultKeys = remoteConfig.allKeys(from: .default).sorted().joined(separator: ", ")
                    logger.info(
            """
            ✅ Remote Config fetch completed after \(elapsed, privacy: .public)s
            - status: \(status.rawValue, privacy: .public) (\(String(describing: status), privacy: .public))
            - remoteKeys: [\(remoteKeys, privacy: .public)]
            - defaultKeys: [\(defaultKeys, privacy: .public)]
            """
                    )
                    continuation.resume(returning: ())
                }
                
                logger.debug("📞 fetchAndActivate method returned (async operation started)")
            }
        }
#else
        throw RemoteConfigError.fetchFailed("FirebaseRemoteConfig not available on this build")
#endif
    }

    public func getString(forKey key: String) -> String {
        remoteConfig.configValue(forKey: key).stringValue
    }
    
    public func getBool(forKey key: String) -> Bool {
        remoteConfig.configValue(forKey: key).boolValue
    }
    
    public func getInt(forKey key: String) -> Int {
        let number = remoteConfig.configValue(forKey: key).numberValue
        return number.intValue
    }
    
    public func getDouble(forKey key: String) -> Double {
        remoteConfig.configValue(forKey: key).numberValue.doubleValue
    }
    
    public func getData(forKey key: String) -> Data {
        remoteConfig.configValue(forKey: key).dataValue
    }
    
    public func getFeatureVariant(forKey key: String) async -> FeatureVariant {
        if let rawValue = getString(forKey: key).lowercased().trimmingCharacters(in: .whitespaces) as String?,
           let variant = FeatureVariant(rawValue: rawValue) {
            return variant
        }
        return .control
    }
    
    private func diagnoseConnectivity() async {
#if canImport(FirebaseCore)
        if let options = FirebaseApp.app()?.options {
            logger.info("🌐 Remote Config diagnostics starting. projectID=\(options.projectID ?? "nil", privacy: .public) apiKey=\(options.apiKey ?? "nil", privacy: .private(mask: .hash))")
        }
#endif
        
        await withTaskGroup(of: Void.self) { group in
            if let remoteURL = URL(string: "https://firebaseremoteconfig.googleapis.com/"), remoteURL.scheme == "https" {
                group.addTask { await self.probeEndpoint(remoteURL, label: "RemoteConfig") }
            }
            if let installationsURL = URL(string: "https://firebaseinstallations.googleapis.com/"), installationsURL.scheme == "https" {
                group.addTask { await self.probeEndpoint(installationsURL, label: "Installations") }
            }
        }
    }
    
    private func probeEndpoint(_ url: URL, label: String) async {
        var request = URLRequest(url: url)
        request.httpMethod = "HEAD"
        request.timeoutInterval = fetchTimeoutSeconds
        
        let session = URLSession(configuration: .ephemeral)
        
        do {
            let (_, response) = try await session.data(for: request)
            if let httpResponse = response as? HTTPURLResponse {
                logger.info("🌐 \(label) endpoint reachable: status=\(httpResponse.statusCode)")
            } else {
                logger.info("🌐 \(label) endpoint reachable: response=\(String(describing: response), privacy: .public)")
            }
        } catch {
            let nsError = error as NSError
            logger.error("🌐 \(label) endpoint check failed: \(error.localizedDescription, privacy: .public) domain=\(nsError.domain, privacy: .public) code=\(nsError.code, privacy: .public)")
        }
    }
}
#endif

// MARK: - Network Monitor

#if canImport(Network)
private final class RemoteConfigNetworkMonitor {
    static let shared = RemoteConfigNetworkMonitor()
    
    private var monitor: NWPathMonitor?
    private var hasStarted = false
    private let lock = NSLock()
    
    func startIfNeeded(logger: Logger) {
        lock.lock()
        defer { lock.unlock() }
        
        guard !hasStarted else { return }
        
        let monitor = NWPathMonitor()
        let queue = DispatchQueue(label: "com.test.testmgenapp.remoteconfig.network")
        
        monitor.pathUpdateHandler = { path in
            logger.info(
        """
        Remote Config network status updated: status=\(path.status.logDescription, privacy: .public) \
        expensive=\(path.isExpensive, privacy: .public) constrained=\(path.isConstrained, privacy: .public)
        """
            )
        }
        
        monitor.start(queue: queue)
        self.monitor = monitor
        hasStarted = true
        
        logger.debug("Remote Config network monitor started on queue \(queue.label, privacy: .public)")
    }
}

private extension NWPath.Status {
    var logDescription: String {
        switch self {
        case .satisfied:
            return "satisfied"
        case .unsatisfied:
            return "unsatisfied"
        case .requiresConnection:
            return "requiresConnection"
        @unknown default:
            return "unknown"
        }
    }
}
#endif

// MARK: - Dependency Key

import ComposableArchitecture
import Dependencies

private enum RemoteConfigServiceKey: DependencyKey {
    static let liveValue: any RemoteConfigService = NoopRemoteConfigService()
}

public extension DependencyValues {
    var remoteConfigService: any RemoteConfigService {
        get { self[RemoteConfigServiceKey.self] }
        set { self[RemoteConfigServiceKey.self] = newValue }
    }
}
