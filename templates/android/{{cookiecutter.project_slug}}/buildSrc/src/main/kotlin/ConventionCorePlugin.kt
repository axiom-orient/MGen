// TMAndroid - Core Convention Plugin
// Core modules provide shared utilities across all layers

import com.android.build.gradle.LibraryExtension
import internal.AndroidConfig
import internal.KotlinConfig
import org.gradle.api.JavaVersion
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.kotlin.dsl.configure
import org.gradle.kotlin.dsl.dependencies
import org.jetbrains.kotlin.gradle.dsl.KotlinAndroidProjectExtension

class ConventionCorePlugin : Plugin<Project> {
    override fun apply(target: Project) {
        with(target) {
            with(pluginManager) {
                apply("com.android.library")
                apply("org.jetbrains.kotlin.android")
                apply("org.jetbrains.kotlin.plugin.serialization")
            }

            extensions.configure<LibraryExtension> {
                compileSdk = AndroidConfig.COMPILE_SDK

                defaultConfig {
                    minSdk = AndroidConfig.MIN_SDK
                    testInstrumentationRunner = AndroidConfig.TEST_INSTRUMENTATION_RUNNER
                }

                compileOptions {
                    sourceCompatibility = JavaVersion.VERSION_17
                    targetCompatibility = JavaVersion.VERSION_17
                    isCoreLibraryDesugaringEnabled = true
                }

                buildFeatures {
                    buildConfig = false
                }

                lint {
                    abortOnError = false
                }
            }

            extensions.configure<KotlinAndroidProjectExtension> {
                jvmToolchain(17)
            }

            tasks.withType(org.jetbrains.kotlin.gradle.tasks.KotlinCompile::class.java) {
                compilerOptions {
                    jvmTarget.set(org.jetbrains.kotlin.gradle.dsl.JvmTarget.fromTarget(KotlinConfig.JVM_TARGET))
                    freeCompilerArgs.addAll(KotlinConfig.COMMON_COMPILER_ARGS)
                }
            }

            // Add coreLibraryDesugaring dependency automatically
            dependencies {
                add("coreLibraryDesugaring", "com.android.tools:desugar_jdk_libs:${AndroidConfig.DESUGAR_JDK_LIBS_VERSION}")
            }

            // Note: Other dependencies should be added in each module's build.gradle.kts
            // buildSrc cannot access the root project's version catalog (LibrariesForLibs)
            // Example:
            //   dependencies {
            //       implementation(libs.hilt.android)
            //       ksp(libs.hilt.compiler)
            //   }
        }
    }
}
