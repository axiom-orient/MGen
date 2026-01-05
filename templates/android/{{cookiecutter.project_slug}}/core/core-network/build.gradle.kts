// TMAndroid Core Network Module
// Network utilities and monitoring

plugins {
    id("convention.core")
    id("convention.hilt")
}

android {
    namespace = "{{ cookiecutter.base_package }}.core.network"
}

dependencies {
    // Network
    implementation(libs.bundles.network)

    // Kotlin
    implementation(libs.kotlinx.serialization.json)

    // Hilt
    implementation(libs.hilt.android)
    ksp(libs.hilt.compiler)
}
