// TMAndroid Feature Home Module
// Example feature demonstrating MVI pattern with Compose

plugins {
    id("convention.feature")
}

android {
    namespace = "{{ cookiecutter.base_package }}.feature.home"
}

dependencies {
    // Core Modules
    implementation(projects.core.coreUi)
    implementation(projects.core.coreNetwork)
    implementation(projects.core.coreData)

    // Domain Modules (uncomment when created)
    // implementation(projects.domain.domainUserApi)

    // Service Modules (uncomment when created)
    // implementation(projects.service.serviceAuthApi)

    // AndroidX Core
    implementation(libs.androidx.core.ktx)
    implementation(libs.bundles.lifecycle)

    // Compose
    implementation(platform(libs.compose.bom))
    implementation(libs.bundles.compose)
    implementation(libs.androidx.navigation.compose)

    // Hilt
    implementation(libs.hilt.android)
    ksp(libs.hilt.compiler)
    implementation(libs.androidx.hilt.navigation.compose)

    // Coroutines
    implementation(libs.kotlinx.coroutines.core)
    implementation(libs.kotlinx.coroutines.android)

    // Testing
    testImplementation(libs.bundles.testing.unit)
    testRuntimeOnly(libs.junit.jupiter.engine)
    androidTestImplementation(libs.bundles.testing.android)
    androidTestImplementation(platform(libs.compose.bom))
    androidTestImplementation(libs.bundles.testing.compose)
}
