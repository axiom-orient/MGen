// TMAndroid - Android Configuration Constants

package internal

object AndroidConfig {
    const val COMPILE_SDK = {{ cookiecutter.compile_sdk }}
    const val MIN_SDK = {{ cookiecutter.min_sdk }}
    const val TARGET_SDK = {{ cookiecutter.target_sdk }}

    const val VERSION_CODE = {{ cookiecutter.version_code }}
    const val VERSION_NAME = "{{ cookiecutter.version_name }}"

    const val APPLICATION_ID = "{{ cookiecutter.base_package }}"
    const val NAMESPACE_PREFIX = "{{ cookiecutter.base_package }}"

    const val TEST_INSTRUMENTATION_RUNNER = "androidx.test.runner.AndroidJUnitRunner"
    const val TEST_INSTRUMENTATION_RUNNER_HILT = "{{ cookiecutter.base_package }}.HiltTestRunner"

    // Core Library Desugaring
    const val DESUGAR_JDK_LIBS_VERSION = "2.1.4"

    val COMMON_PROGUARD_FILES = listOf(
        "proguard-rules.pro",
        "proguard-android-optimize.txt"
    )
}

object KotlinConfig {
    const val JVM_TARGET = "17"

    val COMMON_COMPILER_ARGS = listOf(
        "-opt-in=kotlin.RequiresOptIn",
        "-opt-in=kotlinx.coroutines.ExperimentalCoroutinesApi",
        "-opt-in=kotlinx.coroutines.FlowPreview"
    )

    val COMPOSE_COMPILER_ARGS = listOf(
        "-opt-in=androidx.compose.material3.ExperimentalMaterial3Api",
        "-opt-in=androidx.compose.foundation.ExperimentalFoundationApi",
        "-opt-in=androidx.compose.ui.ExperimentalComposeUiApi"
    )
}

object JavaConfig {
    const val JAVA_VERSION = 17
}
