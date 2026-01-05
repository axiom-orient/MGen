// TMAndroid - Hilt Convention Plugin
// Adds Hilt dependency injection to a module

import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.kotlin.dsl.dependencies

class ConventionHiltPlugin : Plugin<Project> {
    override fun apply(target: Project) {
        with(target) {
            with(pluginManager) {
                apply("com.google.devtools.ksp")
                apply("com.google.dagger.hilt.android")
            }

            // Note: Dependencies should be added in each module's build.gradle.kts
            // buildSrc cannot access the root project's version catalog (LibrariesForLibs)
            // Example:
            //   dependencies {
            //       implementation(libs.hilt.android)
            //       ksp(libs.hilt.compiler)
            //   }
        }
    }
}
