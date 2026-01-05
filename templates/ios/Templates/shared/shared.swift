import ProjectDescription

private let sharedNameAttribute: Template.Attribute = .required("name")
private let sharedOrganizationAttribute: Template.Attribute = .optional("organizationName", default: "axient")
private let sharedBundleIdAttribute: Template.Attribute = .optional("bundleIdPrefix", default: "com.axiomorient")
private let sharedTeamAttribute: Template.Attribute = .optional("teamId", default: "")
private let sharedDeploymentTargetAttribute: Template.Attribute = .optional("deploymentTarget", default: "17.0")

let templateShared = Template(
    description: "TMA Shared module (1-target with internal Interface structure)",
    attributes: [
        sharedNameAttribute,
        sharedOrganizationAttribute,
        sharedBundleIdAttribute,
        sharedTeamAttribute,
        sharedDeploymentTargetAttribute
    ],
    items: [
        .file(path: "Projects/Shared/{{ name }}/Project.swift", templatePath: "Project.stencil"),

        // Interface (Internal Organization)
        .file(path: "Projects/Shared/{{ name }}/Sources/Interface/{{ name }}Interface.swift", templatePath: "SharedInterface.stencil"),

        // InterfaceKeys (Internal Organization)
        .file(path: "Projects/Shared/{{ name }}/Sources/InterfaceKeys/{{ name }}InterfaceKeys.swift", templatePath: "SharedInterfaceKeys.stencil"),

        // Sources - Main file
        .file(path: "Projects/Shared/{{ name }}/Sources/{{ name }}.swift", templatePath: "SharedSources.stencil"),

        // Sources - Component files
        .file(path: "Projects/Shared/{{ name }}/Sources/Components/PublishedUserDefaults.swift", templatePath: "PublishedUserDefaults.stencil"),
        .file(path: "Projects/Shared/{{ name }}/Sources/Components/NetworkMonitor.swift", templatePath: "NetworkMonitor.stencil"),
        .file(path: "Projects/Shared/{{ name }}/Sources/Components/KeychainStorage.swift", templatePath: "KeychainStorage.stencil"),
        .file(path: "Projects/Shared/{{ name }}/Sources/Components/AnySendableError.swift", templatePath: "AnySendableError.stencil")
    ]
)
