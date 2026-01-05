// TMAndroid - Test Convention Plugin
// Standardizes testing configuration across modules

import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.api.tasks.testing.Test
import org.gradle.kotlin.dsl.dependencies
import org.gradle.kotlin.dsl.withType

class ConventionTestPlugin : Plugin<Project> {
    override fun apply(target: Project) {
        with(target) {
            // Note: Dependencies should be added in each module's build.gradle.kts
            // buildSrc cannot access the root project's version catalog (LibrariesForLibs)
            // Example:
            //   dependencies {
            //       implementation(libs.hilt.android)
            //       ksp(libs.hilt.compiler)
            //   }

            // Configure JUnit 5
            tasks.withType<Test> {
                useJUnitPlatform()

                testLogging {
                    events("passed", "skipped", "failed")
                    showStandardStreams = false
                    showExceptions = true
                    showCauses = true
                    showStackTraces = true
                }

                // Parallel execution
                maxParallelForks = Runtime.getRuntime().availableProcessors()
            }
        }
    }
}
