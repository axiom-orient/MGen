// TMAndroid - Service Implementation Convention Plugin
// Service Impl modules contain concrete service implementations (Retrofit, Room, etc)

import com.android.build.gradle.LibraryExtension
import internal.AndroidConfig
import internal.KotlinConfig
import org.gradle.api.JavaVersion
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.kotlin.dsl.configure
import org.gradle.kotlin.dsl.dependencies
import org.jetbrains.kotlin.gradle.dsl.KotlinAndroidProjectExtension

class ConventionServiceImplPlugin : Plugin<Project> {
    override fun apply(target: Project) {
        with(target) {
            with(pluginManager) {
                apply("convention.library")
                apply("convention.hilt")
                apply("org.jetbrains.kotlin.plugin.serialization")
                apply("com.google.devtools.ksp")
                // The original instruction had a malformed line: apply("convention.hilt")brains.kotlin.android")
                // Assuming the intent was to replace "org.jetbrains.kotlin.android" with "convention.hilt"
                // and keep the other plugins as they were, or that "convention.hilt" implicitly handles
                // "com.google.dagger.hilt.android".
                // For now, I'm replacing "org.jetbrains.kotlin.android" with "convention.hilt"
                // and keeping "com.google.dagger.hilt.android" as it was in the original code,
                // as the instruction did not explicitly remove it.
                apply("com.google.dagger.hilt.android")
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
                }

                buildFeatures {
                    buildConfig = true // Services often need BuildConfig
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
