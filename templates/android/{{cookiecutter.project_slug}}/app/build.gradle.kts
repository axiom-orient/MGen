// TMAndroid App Module
// Generated for {{ cookiecutter.project_name }}

plugins {
    id("convention.application")
}

android {
    namespace = "{{ cookiecutter.base_package }}"

    defaultConfig {
        applicationId = "{{ cookiecutter.base_package }}"
        versionCode = {{ cookiecutter.version_code }}
        versionName = "{{ cookiecutter.version_name }}"
    }
}

dependencies {
    // Feature Modules
    implementation(projects.feature.featureHome)

    // Core Modules
    implementation(projects.core.coreUi)
    implementation(projects.core.coreNetwork)
    implementation(projects.core.coreData)
    implementation(projects.core.coreDatabase)

    // Persistence
    implementation(libs.datastore.preferences)

    // Core Android
    implementation(libs.androidx.core.ktx)
    implementation(libs.androidx.core.splashscreen)
    implementation(libs.bundles.lifecycle)
    implementation(libs.androidx.activity.compose)

    // Material Design Components (for XML themes)
    implementation(libs.material)

    // Compose
    implementation(platform(libs.compose.bom))
    implementation(libs.bundles.compose)
    implementation(libs.androidx.navigation.compose)

    // Hilt
    implementation(libs.hilt.android)
    ksp(libs.hilt.compiler)
    implementation(libs.androidx.hilt.navigation.compose)

    // Kotlin
    implementation(libs.kotlinx.coroutines.core)
    implementation(libs.kotlinx.coroutines.android)

    // Testing
    testImplementation(libs.bundles.testing.unit)
    testRuntimeOnly(libs.junit.jupiter.engine)
    androidTestImplementation(libs.bundles.testing.android)
    androidTestImplementation(platform(libs.compose.bom))
    androidTestImplementation(libs.bundles.testing.compose)

    // Debug
    debugImplementation(libs.compose.ui.tooling)
    debugImplementation(libs.compose.ui.test.manifest)
    debugImplementation(libs.leakcanary.android)

    {% if cookiecutter.use_firebase == 'yes' -%}
    // Firebase
    implementation(platform(libs.firebase.bom))
    implementation(libs.firebase.analytics)
    implementation(libs.firebase.crashlytics)
    implementation(libs.firebase.config)
    {% endif -%}
}
