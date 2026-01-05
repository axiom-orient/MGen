// TMAndroid Root Build Script
// Generated for {{ cookiecutter.project_name }}

plugins {
    // Note: AGP, Kotlin, KSP, and Hilt plugins are already in buildSrc classpath
    // Only declare plugins that are NOT in buildSrc/build.gradle.kts
    {% if cookiecutter.use_firebase == 'yes' -%}
    alias(libs.plugins.google.services) apply false
    alias(libs.plugins.firebase.crashlytics) apply false
    {% endif -%}
    alias(libs.plugins.detekt) apply false
    alias(libs.plugins.ktlint) apply false
}

// Global configuration for all projects
allprojects {
    // Apply detekt for code quality
    apply(plugin = "io.gitlab.arturbosch.detekt")
    apply(plugin = "org.jlleitschuh.gradle.ktlint")
}

// Clean task
tasks.register("clean", Delete::class) {
    delete(rootProject.layout.buildDirectory)
}

// Custom task: Print dependency tree
tasks.register("printDependencyTree") {
    group = "tma"
    description = "Prints the dependency tree of all modules"

    doLast {
        println("\n=== Dependency Tree ===")
        subprojects {
            println("${project.path}")
            configurations.findByName("implementation")?.dependencies?.forEach { dep ->
                println("  └── ${dep.group}:${dep.name}:${dep.version}")
            }
        }
    }
}

// Custom task: Validate architecture (runs Konsist tests)
tasks.register("validateArchitecture") {
    group = "tma"
    description = "Validates project architecture using Konsist"
    dependsOn(":konsist-test:test")
}

// Scaffolding tasks
abstract class ScaffoldTask : DefaultTask() {
    @get:Input
    abstract val moduleName: Property<String>

    @get:Input
    abstract val moduleType: Property<String>

    @get:Input
    abstract val basePackage: Property<String>

    @TaskAction
    fun scaffold() {
        val name = moduleName.get()
        val type = moduleType.get()
        val pkg = basePackage.get()
        val nameLower = name.lowercase()

        when (type) {
            "feature" -> scaffoldFeature(name, nameLower, pkg)
            "domain" -> scaffoldDomain(name, nameLower, pkg)
            "service" -> scaffoldService(name, nameLower, pkg)
            else -> throw GradleException("Unknown module type: $type")
        }
    }

    private fun scaffoldFeature(name: String, nameLower: String, pkg: String) {
        val moduleDir = project.file("feature/feature-$nameLower")
        val srcDir = moduleDir.resolve("src/main/kotlin/${pkg.replace('.', '/')}/feature/$nameLower")

        moduleDir.mkdirs()
        srcDir.mkdirs()

        // Create build.gradle.kts
        moduleDir.resolve("build.gradle.kts").writeText("""
            // Feature Module: $name

            plugins {
                id("convention.feature")
            }

            android {
                namespace = "$pkg.feature.$nameLower"
            }

            dependencies {
                // Add domain/service dependencies here
            }
        """.trimIndent())

        // Create State
        srcDir.resolve("${name}State.kt").writeText("""
            package $pkg.feature.$nameLower

            data class ${name}State(
                val isLoading: Boolean = false,
                val data: String? = null,
                val error: String? = null
            )
        """.trimIndent())

        // Create Action
        srcDir.resolve("${name}Action.kt").writeText("""
            package $pkg.feature.$nameLower

            sealed interface ${name}Action {
                data object Load : ${name}Action
                data object Refresh : ${name}Action
            }
        """.trimIndent())

        // Create SideEffect
        srcDir.resolve("${name}SideEffect.kt").writeText("""
            package $pkg.feature.$nameLower

            sealed interface ${name}SideEffect {
                data class ShowToast(val message: String) : ${name}SideEffect
            }
        """.trimIndent())

        // Create ViewModel
        srcDir.resolve("${name}ViewModel.kt").writeText("""
            package $pkg.feature.$nameLower

            import androidx.lifecycle.ViewModel
            import androidx.lifecycle.viewModelScope
            import dagger.hilt.android.lifecycle.HiltViewModel
            import kotlinx.coroutines.channels.Channel
            import kotlinx.coroutines.flow.MutableStateFlow
            import kotlinx.coroutines.flow.StateFlow
            import kotlinx.coroutines.flow.asStateFlow
            import kotlinx.coroutines.flow.receiveAsFlow
            import kotlinx.coroutines.flow.update
            import kotlinx.coroutines.launch
            import javax.inject.Inject

            @HiltViewModel
            class ${name}ViewModel @Inject constructor() : ViewModel() {
                private val _state = MutableStateFlow(${name}State())
                val state: StateFlow<${name}State> = _state.asStateFlow()

                private val _sideEffect = Channel<${name}SideEffect>()
                val sideEffect = _sideEffect.receiveAsFlow()

                fun onAction(action: ${name}Action) {
                    when (action) {
                        is ${name}Action.Load -> load()
                        is ${name}Action.Refresh -> refresh()
                    }
                }

                private fun load() {
                    viewModelScope.launch {
                        _state.update { it.copy(isLoading = true) }
                        _state.update { it.copy(isLoading = false, data = "$name loaded") }
                    }
                }

                private fun refresh() {
                    load()
                }
            }
        """.trimIndent())

        // Create Screen
        srcDir.resolve("${name}Screen.kt").writeText("""
            package $pkg.feature.$nameLower

            import androidx.compose.foundation.layout.Box
            import androidx.compose.foundation.layout.fillMaxSize
            import androidx.compose.material3.CircularProgressIndicator
            import androidx.compose.material3.Scaffold
            import androidx.compose.material3.Text
            import androidx.compose.runtime.Composable
            import androidx.compose.runtime.LaunchedEffect
            import androidx.compose.runtime.getValue
            import androidx.compose.ui.Alignment
            import androidx.compose.ui.Modifier
            import androidx.hilt.navigation.compose.hiltViewModel
            import androidx.lifecycle.compose.collectAsStateWithLifecycle

            @Composable
            fun ${name}Screen(
                viewModel: ${name}ViewModel = hiltViewModel()
            ) {
                val state by viewModel.state.collectAsStateWithLifecycle()

                LaunchedEffect(Unit) {
                    viewModel.onAction(${name}Action.Load)
                }

                Scaffold { paddingValues ->
                    Box(
                        modifier = Modifier.fillMaxSize(),
                        contentAlignment = Alignment.Center
                    ) {
                        when {
                            state.isLoading -> CircularProgressIndicator()
                            else -> Text(text = state.data ?: "No data")
                        }
                    }
                }
            }
        """.trimIndent())

        // Update settings.gradle.kts
        val settingsFile = project.rootProject.file("settings.gradle.kts")
        val settingsContent = settingsFile.readText()
        if (!settingsContent.contains(":feature:feature-$nameLower")) {
            settingsFile.appendText("\ninclude(\":feature:feature-$nameLower\")\n")
        }

        println("✅ Feature module 'feature-$nameLower' created successfully!")
        println("📁 Location: feature/feature-$nameLower")
        println("🔄 Run './gradlew sync' to load the new module")
    }

    private fun scaffoldDomain(name: String, nameLower: String, pkg: String) {
        val apiDir = project.file("domain/domain-$nameLower-api")
        val implDir = project.file("domain/domain-$nameLower-impl")

        // Create API module
        val apiSrcDir = apiDir.resolve("src/main/kotlin/${pkg.replace('.', '/')}/domain/$nameLower")
        apiSrcDir.mkdirs()

        apiDir.resolve("build.gradle.kts").writeText("""
            plugins {
                alias(libs.plugins.tma.domain.api)
            }

            android {
                namespace = "$pkg.domain.$nameLower"
            }
        """.trimIndent())

        apiSrcDir.resolve("model").mkdirs()
        apiSrcDir.resolve("model/${name}.kt").writeText("""
            package $pkg.domain.$nameLower.model

            data class $name(
                val id: String,
                val name: String
            )
        """.trimIndent())

        apiSrcDir.resolve("repository").mkdirs()
        apiSrcDir.resolve("repository/${name}Repository.kt").writeText("""
            package $pkg.domain.$nameLower.repository

            import $pkg.domain.$nameLower.model.$name
            import kotlinx.coroutines.flow.Flow

            interface ${name}Repository {
                fun getAll(): Flow<List<$name>>
                suspend fun getById(id: String): $name?
            }
        """.trimIndent())

        apiSrcDir.resolve("usecase").mkdirs()
        apiSrcDir.resolve("usecase/Get${name}UseCase.kt").writeText("""
            package $pkg.domain.$nameLower.usecase

            import $pkg.domain.$nameLower.repository.${name}Repository
            import javax.inject.Inject

            class Get${name}UseCase @Inject constructor(
                private val repository: ${name}Repository
            ) {
                operator fun invoke() = repository.getAll()
            }
        """.trimIndent())

        // Create Impl module
        val implSrcDir = implDir.resolve("src/main/kotlin/${pkg.replace('.', '/')}/domain/$nameLower/impl")
        implSrcDir.mkdirs()

        implDir.resolve("build.gradle.kts").writeText("""
            plugins {
                alias(libs.plugins.tma.domain.impl)
            }

            android {
                namespace = "$pkg.domain.$nameLower.impl"
            }

            dependencies {
                implementation(projects.domain.domain${name.replaceFirstChar { if (it.isLowerCase()) it.titlecase(java.util.Locale.getDefault()) else it.toString() }}Api)
            }
        """.trimIndent())

        implSrcDir.resolve("repository").mkdirs()
        implSrcDir.resolve("repository/Default${name}Repository.kt").writeText("""
            package $pkg.domain.$nameLower.impl.repository

            import $pkg.domain.$nameLower.model.$name
            import $pkg.domain.$nameLower.repository.${name}Repository
            import kotlinx.coroutines.flow.Flow
            import kotlinx.coroutines.flow.flowOf
            import javax.inject.Inject

            class Default${name}Repository @Inject constructor() : ${name}Repository {
                override fun getAll(): Flow<List<$name>> = flowOf(emptyList())
                override suspend fun getById(id: String): $name? = null
            }
        """.trimIndent())

        implSrcDir.resolve("di").mkdirs()
        implSrcDir.resolve("di/${name}DomainModule.kt").writeText("""
            package $pkg.domain.$nameLower.impl.di

            import $pkg.domain.$nameLower.repository.${name}Repository
            import $pkg.domain.$nameLower.impl.repository.Default${name}Repository
            import dagger.Binds
            import dagger.Module
            import dagger.hilt.InstallIn
            import dagger.hilt.components.SingletonComponent
            import javax.inject.Singleton

            @Module
            @InstallIn(SingletonComponent::class)
            abstract class ${name}DomainModule {
                @Binds
                @Singleton
                abstract fun bind${name}Repository(impl: Default${name}Repository): ${name}Repository
            }
        """.trimIndent())

        // Update settings.gradle.kts
        val settingsFile = project.rootProject.file("settings.gradle.kts")
        val settingsContent = settingsFile.readText()
        if (!settingsContent.contains(":domain:domain-$nameLower-api")) {
            settingsFile.appendText("\ninclude(\":domain:domain-$nameLower-api\")\n")
            settingsFile.appendText("include(\":domain:domain-$nameLower-impl\")\n")
        }

        println("✅ Domain modules created successfully!")
        println("📁 API: domain/domain-$nameLower-api")
        println("📁 Impl: domain/domain-$nameLower-impl")
        println("🔄 Run './gradlew sync' to load the new modules")
    }

    private fun scaffoldService(name: String, nameLower: String, pkg: String) {
        val apiDir = project.file("service/service-$nameLower-api")
        val implDir = project.file("service/service-$nameLower-impl")

        // Create API module
        val apiSrcDir = apiDir.resolve("src/main/kotlin/${pkg.replace('.', '/')}/service/$nameLower")
        apiSrcDir.mkdirs()

        apiDir.resolve("build.gradle.kts").writeText("""
            plugins {
                alias(libs.plugins.tma.service.api)
            }

            android {
                namespace = "$pkg.service.$nameLower"
            }
        """.trimIndent())

        apiSrcDir.resolve("${name}Service.kt").writeText("""
            package $pkg.service.$nameLower

            import $pkg.service.$nameLower.model.${name}Response

            interface ${name}Service {
                suspend fun fetch(): Result<${name}Response>
            }
        """.trimIndent())

        apiSrcDir.resolve("model").mkdirs()
        apiSrcDir.resolve("model/${name}Response.kt").writeText("""
            package $pkg.service.$nameLower.model

            data class ${name}Response(
                val id: String,
                val data: String
            )
        """.trimIndent())

        // Create Impl module
        val implSrcDir = implDir.resolve("src/main/kotlin/${pkg.replace('.', '/')}/service/$nameLower/impl")
        implSrcDir.mkdirs()

        implDir.resolve("build.gradle.kts").writeText("""
            plugins {
                alias(libs.plugins.tma.service.impl)
            }

            android {
                namespace = "$pkg.service.$nameLower.impl"
            }

            dependencies {
                implementation(projects.service.service${name.replaceFirstChar { if (it.isLowerCase()) it.titlecase(java.util.Locale.getDefault()) else it.toString() }}Api)
            }
        """.trimIndent())

        implSrcDir.resolve("Live${name}Service.kt").writeText("""
            package $pkg.service.$nameLower.impl

            import $pkg.service.$nameLower.${name}Service
            import $pkg.service.$nameLower.model.${name}Response
            import javax.inject.Inject

            class Live${name}Service @Inject constructor() : ${name}Service {
                override suspend fun fetch(): Result<${name}Response> {
                    return try {
                        Result.success(${name}Response("1", "data"))
                    } catch (e: Exception) {
                        Result.failure(e)
                    }
                }
            }
        """.trimIndent())

        implSrcDir.resolve("di").mkdirs()
        implSrcDir.resolve("di/${name}ServiceModule.kt").writeText("""
            package $pkg.service.$nameLower.impl.di

            import $pkg.service.$nameLower.${name}Service
            import $pkg.service.$nameLower.impl.Live${name}Service
            import dagger.Binds
            import dagger.Module
            import dagger.hilt.InstallIn
            import dagger.hilt.components.SingletonComponent
            import javax.inject.Singleton

            @Module
            @InstallIn(SingletonComponent::class)
            abstract class ${name}ServiceModule {
                @Binds
                @Singleton
                abstract fun bind${name}Service(impl: Live${name}Service): ${name}Service
            }
        """.trimIndent())

        // Update settings.gradle.kts
        val settingsFile = project.rootProject.file("settings.gradle.kts")
        val settingsContent = settingsFile.readText()
        if (!settingsContent.contains(":service:service-$nameLower-api")) {
            settingsFile.appendText("\ninclude(\":service:service-$nameLower-api\")\n")
            settingsFile.appendText("include(\":service:service-$nameLower-impl\")\n")
        }

        println("✅ Service modules created successfully!")
        println("📁 API: service/service-$nameLower-api")
        println("📁 Impl: service/service-$nameLower-impl")
        println("🔄 Run './gradlew sync' to load the new modules")
    }
}

// Task to scaffold a new feature module
tasks.register<ScaffoldTask>("scaffoldFeature") {
    group = "tma"
    description = "Scaffold a new feature module (e.g., ./gradlew scaffoldFeature -PmoduleName=Home)"

    moduleName.set(project.findProperty("moduleName")?.toString() ?: "Example")
    moduleType.set("feature")
    basePackage.set("{{ cookiecutter.base_package }}")
}

// Task to scaffold a new domain module
tasks.register<ScaffoldTask>("scaffoldDomain") {
    group = "tma"
    description = "Scaffold a new domain module (e.g., ./gradlew scaffoldDomain -PmoduleName=User)"

    moduleName.set(project.findProperty("moduleName")?.toString() ?: "Example")
    moduleType.set("domain")
    basePackage.set("{{ cookiecutter.base_package }}")
}

// Task to scaffold a new service module
tasks.register<ScaffoldTask>("scaffoldService") {
    group = "tma"
    description = "Scaffold a new service module (e.g., ./gradlew scaffoldService -PmoduleName=Auth)"

    moduleName.set(project.findProperty("moduleName")?.toString() ?: "Example")
    moduleType.set("service")
    basePackage.set("{{ cookiecutter.base_package }}")
}
