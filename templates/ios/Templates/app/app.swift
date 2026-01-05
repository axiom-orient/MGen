import ProjectDescription

private let appNameAttribute: Template.Attribute = .required("name")
private let appOrganizationAttribute: Template.Attribute = .optional("organizationName", default: "axient")
private let appBundleIdAttribute: Template.Attribute = .optional("bundleIdPrefix", default: "com.axiomorient")
private let appTeamAttribute: Template.Attribute = .optional("teamId", default: "")
private let appDeploymentTargetAttribute: Template.Attribute = .optional("deploymentTarget", default: "17.0")
private let splashSubtitleAttribute: Template.Attribute = .optional("splashSubtitle", default: "Build Your Time")
private let appStoreIdAttribute: Template.Attribute = .optional("appStoreId", default: "")

let templateApp = Template(
    description: "TMA App module with production-ready deep linking, lifecycle management, and Firebase integration",
    attributes: [
        appNameAttribute,
        appOrganizationAttribute,
        appBundleIdAttribute,
        appTeamAttribute,
        appDeploymentTargetAttribute,
        splashSubtitleAttribute,
        appStoreIdAttribute
    ],
    items: [
        // ============================================================
        // SharedCore Module - Common utilities
        // ============================================================
        .file(path: "Projects/Shared/SharedCore/Project.swift", templatePath: "SharedCoreProject.stencil"),
        .file(path: "Projects/Shared/SharedCore/Sources/SharedCore.swift", templatePath: "SharedCore.stencil"),
        
        // SharedCore Utility Components
        .file(path: "Projects/Shared/SharedCore/Sources/Components/AnySendableError.swift", templatePath: "AnySendableError.stencil"),
        .file(path: "Projects/Shared/SharedCore/Sources/Components/KeychainStorage.swift", templatePath: "KeychainStorage.stencil"),
        .file(path: "Projects/Shared/SharedCore/Sources/Components/NetworkMonitor.swift", templatePath: "NetworkMonitor.stencil"),

        // ============================================================
        // App Module
        // ============================================================
        // Project Definition
        .file(path: "Projects/App/Project.swift", templatePath: "Project.stencil"),

        // Main App Files
        .file(path: "Projects/App/Sources/App.swift", templatePath: "App.stencil"),
        .file(path: "Projects/App/Sources/AppReducer.swift", templatePath: "AppReducer.stencil"),
        .file(path: "Projects/App/Sources/AppConstants.swift", templatePath: "AppConstants.stencil"),
        .file(path: "Projects/App/Sources/AppDelegate.swift", templatePath: "AppDelegate.stencil"),
        .file(path: "Projects/App/Sources/AppState.swift", templatePath: "AppState.stencil"),
        .file(path: "Projects/App/Sources/MainScreenView.swift", templatePath: "MainScreenView.stencil"),
        .file(path: "Projects/App/Sources/SplashView.swift", templatePath: "SplashView.stencil"),
        .file(path: "Projects/App/Sources/ForceUpdateView.swift", templatePath: "ForceUpdateView.stencil"),
        .file(path: "Projects/App/Sources/MaintenanceView.swift", templatePath: "MaintenanceView.stencil"),

        // Core Files - Deep Link System
        .file(path: "Projects/App/Sources/Core/DeepLink.swift", templatePath: "DeepLink.stencil"),

        // Core Files - Lifecycle & Config
        .file(path: "Projects/App/Sources/Core/ApplicationLifecycle.swift", templatePath: "ApplicationLifecycle.stencil"),
        .file(path: "Projects/App/Sources/Core/AppVersion.swift", templatePath: "AppVersion.stencil"),
        .file(path: "Projects/App/Sources/Core/UpdateChecker.swift", templatePath: "UpdateChecker.stencil"),
        .file(path: "Projects/App/Sources/Core/StoreVersionChecker.swift", templatePath: "StoreVersionChecker.stencil"),
        .file(path: "Projects/App/Sources/Core/RemoteConfigModels.swift", templatePath: "RemoteConfigModels.stencil"),
        .file(path: "Projects/App/Sources/Core/LocalizedRemoteConfig.swift", templatePath: "LocalizedRemoteConfig.stencil"),

        // Dependencies
        .file(path: "Projects/App/Sources/Dependencies/AppComposition.swift", templatePath: "AppComposition.stencil"),
        .file(path: "Projects/App/Sources/Dependencies/AnalyticsServices.swift", templatePath: "AnalyticsServices.stencil"),
        .file(path: "Projects/App/Sources/Dependencies/LifecycleServices.swift", templatePath: "LifecycleServices.stencil"),
        .file(path: "Projects/App/Sources/Dependencies/RemoteConfigServices.swift", templatePath: "RemoteConfigServices.stencil"),
        .file(path: "Projects/App/Sources/Dependencies/DeepLinkServices.swift", templatePath: "DeepLinkServices.stencil"),

        // Resources
        .file(path: "Projects/App/Resources/RemoteConfigDefaults.plist", templatePath: "RemoteConfigDefaults.stencil"),
        .file(path: "Projects/App/Resources/GoogleService-Info.plist", templatePath: "RemoteConfigGoogleServiceInfo.stencil"),


        // Assets
        .file(path: "Projects/App/Resources/Assets.xcassets/Contents.json", templatePath: "Assets.xcassets/Contents.json"),
        .file(path: "Projects/App/Resources/Assets.xcassets/AppIcon.appiconset/Contents.json", templatePath: "Assets.xcassets/AppIcon.appiconset/Contents.json"),
        .file(path: "Projects/App/Resources/Assets.xcassets/AccentColor.colorset/Contents.json", templatePath: "Assets.xcassets/AccentColor.colorset/Contents.json"),

        // Localization
        .file(path: "Projects/App/Resources/en.lproj/Localizable.strings", templatePath: "Resources/en.lproj/Localizable.strings"),
        .file(path: "Projects/App/Resources/ko.lproj/Localizable.strings", templatePath: "Resources/ko.lproj/Localizable.strings"),

        // Entitlements
        .file(path: "Projects/App/App.entitlements", templatePath: "App.entitlements"),

        // Documentation
        .file(path: "Projects/App/MODULE_SCAFFOLD_GUIDE.md", templatePath: "MODULE_SCAFFOLD_GUIDE.stencil"),
        .file(path: "Projects/App/SWIFT6_TCA_INTEGRATION.md", templatePath: "SWIFT6_TCA_INTEGRATION.md.stencil"),

        // ============================================================
        // Sample Domain Module (DailyAction)
        // ============================================================
        .file(path: "Projects/Domains/DailyAction/Project.swift", templatePath: "DailyActionDomainProject.stencil"),
        .file(path: "Projects/Domains/DailyAction/Interface/DailyAction.swift", templatePath: "DailyActionModel.stencil"),
        .file(path: "Projects/Domains/DailyAction/Interface/DailyActionRepository.swift", templatePath: "DailyActionInterface.stencil"),

        // ============================================================
        // Sample Feature Module (DailyActionList)
        // ============================================================
        .file(path: "Projects/Features/DailyActionList/Project.swift", templatePath: "DailyActionFeatureProject.stencil"),
        .file(path: "Projects/Features/DailyActionList/Sources/DailyActionListFeature.swift", templatePath: "DailyActionFeatureSource.stencil"),

        // ============================================================
        // AppDataService Module - sqlite-data CRUD Repository
        // ============================================================
        .file(path: "Projects/Service/AppDataService/Project.swift", templatePath: "AppDataServiceProject.stencil"),
        .file(path: "Projects/Service/AppDataService/Interface/AppDataRepositoryInterface.swift", templatePath: "AppDataServiceInterface.stencil"),
        .file(path: "Projects/Service/AppDataService/Sources/AppDataRepository.swift", templatePath: "AppDataServiceSources.stencil"),
        .file(path: "Projects/Service/AppDataService/Tests/AppDataRepositoryTests.swift", templatePath: "AppDataServiceTests.stencil")
    ]
)
