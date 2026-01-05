// TMAndroid - Feature Convention Plugin
// Feature modules contain UI (Compose), ViewModels, and navigation

import com.android.build.gradle.LibraryExtension
import internal.AndroidConfig
import internal.KotlinConfig
import org.gradle.api.JavaVersion
import org.gradle.api.Plugin
import org.gradle.api.Project
import org.gradle.kotlin.dsl.configure
import org.gradle.kotlin.dsl.dependencies
import org.jetbrains.kotlin.gradle.dsl.KotlinAndroidProjectExtension

class ConventionFeaturePlugin : Plugin<Project> {
    override fun apply(target: Project) {
        with(target) {
            with(pluginManager) {
                apply("convention.library")
                apply("org.jetbrains.kotlin.android")
                apply("convention.compose")
                apply("org.jetbrains.kotlin.plugin.serialization")
                apply("com.google.devtools.ksp")
                apply("convention.hilt")
            }

            extensions.configure<LibraryExtension> {
                // Namespace must be set by the module's build.gradle.kts
                compileSdk = AndroidConfig.COMPILE_SDK

                defaultConfig {
                    minSdk = AndroidConfig.MIN_SDK
                    testInstrumentationRunner = AndroidConfig.TEST_INSTRUMENTATION_RUNNER
                }

                buildTypes {
                    release {
                        isMinifyEnabled = false
                        proguardFiles(
                            getDefaultProguardFile("proguard-android-optimize.txt"),
                            "proguard-rules.pro"
                        )
                    }
                }

                compileOptions {
                    sourceCompatibility = JavaVersion.VERSION_17
                    targetCompatibility = JavaVersion.VERSION_17
                    isCoreLibraryDesugaringEnabled = true
                }

                buildFeatures {
                    compose = true
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
