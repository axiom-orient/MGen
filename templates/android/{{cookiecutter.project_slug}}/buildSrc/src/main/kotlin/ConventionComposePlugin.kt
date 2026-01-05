// TMAndroid - Compose Convention Plugin
// Adds Jetpack Compose support to a module

import com.android.build.gradle.LibraryExtension
import internal.KotlinConfig
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.kotlin.dsl.configure

class ConventionComposePlugin : Plugin<Project> {
    override fun apply(target: Project) {
        with(target) {
            pluginManager.apply("org.jetbrains.kotlin.plugin.compose")

            extensions.configure<LibraryExtension> {
                buildFeatures {
                    compose = true
                }
            }

            // Apply Compose-specific compiler args
            tasks.withType(org.jetbrains.kotlin.gradle.tasks.KotlinCompile::class.java) {
                compilerOptions {
                    freeCompilerArgs.addAll(KotlinConfig.COMPOSE_COMPILER_ARGS)
                }
            }

            // Note: Dependencies should be added in each module's build.gradle.kts
            // Example:
            //   dependencies {
            //       implementation(platform(libs.compose.bom))
            //       implementation(libs.bundles.compose)
            //       debugImplementation(libs.compose.ui.tooling)
            //   }
        }
    }
}
