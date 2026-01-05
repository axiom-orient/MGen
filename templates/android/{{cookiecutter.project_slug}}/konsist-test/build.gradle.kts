// Android Konsist Architecture Test Module
// Validates architecture rules

plugins {
    kotlin("jvm")
}

dependencies {
    testImplementation(libs.konsist)
    testImplementation(libs.junit.jupiter.api)
    testRuntimeOnly(libs.junit.jupiter.engine)
}

tasks.withType<Test> {
    useJUnitPlatform()
}
