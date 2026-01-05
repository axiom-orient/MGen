import ProjectDescription

private let domainNameAttribute: Template.Attribute = .required("name")
private let domainOrganizationAttribute: Template.Attribute = .optional("organizationName", default: "axient")
private let domainBundleIdAttribute: Template.Attribute = .optional("bundleIdPrefix", default: "com.axiomorient")
private let domainTeamAttribute: Template.Attribute = .optional("teamId", default: "")
private let domainDeploymentTargetAttribute: Template.Attribute = .optional("deploymentTarget", default: "17.0")

let templateDomain = Template(
    description: "TMA Domain module with Interface/Sources structure (2-target with TestDependencyKey)",
    attributes: [
        domainNameAttribute,
        domainOrganizationAttribute,
        domainBundleIdAttribute,
        domainTeamAttribute,
        domainDeploymentTargetAttribute
    ],
    items: [
        .file(path: "Projects/Domains/{{ name }}/Project.swift", templatePath: "Project.stencil"),
        .file(path: "Projects/Domains/{{ name }}/Interface/{{ name }}Models.swift", templatePath: "DomainModels.stencil"),
        .file(path: "Projects/Domains/{{ name }}/Interface/{{ name }}UseCase.swift", templatePath: "DomainUseCase.stencil"),
        .file(path: "Projects/Domains/{{ name }}/Sources/Default{{ name }}UseCase.swift", templatePath: "DomainSources.stencil")
    ]
)
