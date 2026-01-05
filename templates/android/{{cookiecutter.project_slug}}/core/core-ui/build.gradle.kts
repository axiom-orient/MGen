// TMAndroid Core UI Module
// Shared UI components and theme

plugins {
    id("convention.core")
    id("convention.compose")
}

android {
    namespace = "{{ cookiecutter.base_package }}.core.ui"
}

dependencies {
    // Compose
    implementation(platform(libs.compose.bom))
    implementation(libs.bundles.compose)
}
