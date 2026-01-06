// TMAndroid Generated Project
// Settings configuration for {{ cookiecutter.project_name }}

pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}

plugins {
    // Auto-download JDKs required by toolchains if missing.
    id("org.gradle.toolchains.foojay-resolver-convention") version "0.8.0"
}

dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
    }
}

rootProject.name = "{{ cookiecutter.project_name }}"

// Enable Type-Safe Project Accessors
enableFeaturePreview("TYPESAFE_PROJECT_ACCESSORS")

// Enable Configuration Cache is now stable and handled in gradle.properties


// Build Cache Configuration
buildCache {
    local {
        directory = File(rootDir, "build-cache")
    }
}

// App Module
include(":app")

// Feature Modules
include(":feature:feature-home")  // Example feature module

// Additional Feature Modules (uncomment when created)
// include(":feature:feature-profile")
// include(":feature:feature-settings")

// Domain Modules (uncomment when created)
// include(":domain:domain-user-api")
// include(":domain:domain-user-impl")

// Service Modules (uncomment when created)
// include(":service:service-auth-api")
// include(":service:service-auth-impl")

// Core Modules
include(":core:core-ui")
include(":core:core-network")
include(":core:core-data")
include(":core:core-database")

// Architecture Testing
include(":konsist-test")
