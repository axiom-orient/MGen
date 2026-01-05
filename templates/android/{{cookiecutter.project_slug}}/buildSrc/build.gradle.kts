// BuildSrc - Convention Plugins Build Script
// TMAndroid Generated

plugins {
    `kotlin-dsl`
}

// Repositories are now configured in settings.gradle.kts

dependencies {
    implementation("com.android.tools.build:gradle:{{ cookiecutter.agp_version }}")
    implementation("org.jetbrains.kotlin:kotlin-gradle-plugin:{{ cookiecutter.kotlin_version }}")
    implementation("org.jetbrains.kotlin:kotlin-serialization:{{ cookiecutter.kotlin_version }}")
    implementation("org.jetbrains.kotlin:compose-compiler-gradle-plugin:{{ cookiecutter.kotlin_version }}")
    implementation("com.google.devtools.ksp:com.google.devtools.ksp.gradle.plugin:{{ cookiecutter.kotlin_version }}-1.0.29")
    implementation("com.google.dagger:hilt-android-gradle-plugin:{{ cookiecutter.hilt_version }}")
}

gradlePlugin {
    plugins {
        register("conventionApplication") {
            id = "convention.application"
            implementationClass = "ConventionApplicationPlugin"
        }
        register("conventionLibrary") {
            id = "convention.library"
            implementationClass = "ConventionLibraryPlugin"
        }
        register("conventionFeature") {
            id = "convention.feature"
            implementationClass = "ConventionFeaturePlugin"
        }
        register("conventionDomainApi") {
            id = "convention.domain.api"
            implementationClass = "ConventionDomainApiPlugin"
        }
        register("conventionDomainImpl") {
            id = "convention.domain.impl"
            implementationClass = "ConventionDomainImplPlugin"
        }
        register("conventionServiceApi") {
            id = "convention.service.api"
            implementationClass = "ConventionServiceApiPlugin"
        }
        register("conventionServiceImpl") {
            id = "convention.service.impl"
            implementationClass = "ConventionServiceImplPlugin"
        }
        register("conventionCore") {
            id = "convention.core"
            implementationClass = "ConventionCorePlugin"
        }
        register("conventionCompose") {
            id = "convention.compose"
            implementationClass = "ConventionComposePlugin"
        }
        register("conventionHilt") {
            id = "convention.hilt"
            implementationClass = "ConventionHiltPlugin"
        }
        register("conventionTest") {
            id = "convention.test"
            implementationClass = "ConventionTestPlugin"
        }
    }
}
