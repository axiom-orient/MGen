// TMAndroid Core Data Module
// Data storage utilities

plugins {
    id("convention.core")
    id("convention.hilt")
}

android {
    namespace = "{{ cookiecutter.base_package }}.core.data"

    buildFeatures {
        buildConfig = true
    }
}

dependencies {
    // AndroidX Security
    implementation("androidx.security:security-crypto:1.1.0-alpha06")

    // DataStore
    implementation("androidx.datastore:datastore-preferences:1.1.1")

    // Kotlin
    implementation(libs.kotlinx.serialization.json)

    // Hilt
    implementation(libs.hilt.android)
    ksp(libs.hilt.compiler)

    // Google Play
    implementation(libs.play.app.update)

    // Firebase
    implementation(platform(libs.firebase.bom))
    implementation(libs.firebase.config)
}
