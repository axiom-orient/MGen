// TMAndroid Core Database Module
// Room database setup

plugins {
    id("convention.core")
    id("convention.hilt")
}

android {
    namespace = "{{ cookiecutter.base_package }}.core.database"
}

dependencies {
    // Room
    implementation(libs.room.runtime)
    implementation(libs.room.ktx)
    ksp(libs.room.compiler)

    // Hilt
    implementation(libs.hilt.android)
    ksp(libs.hilt.compiler)
}
