# TMAndroid

> **Tuist-inspired Android project generator with modular architecture, convention plugins, and automated scaffolding**

TMAndroid is the Android counterpart to iOS's [TmaTemplates](../TmaTemplates), providing enterprise-grade project scaffolding for modern Android development. Generate production-ready Android projects with clean architecture, Jetpack Compose, Hilt DI, and MVI pattern in seconds.

---

## 📖 Overview

TMAndroid automates the creation of Android projects following best practices:

- 🏗️ **Modular Architecture**: Feature, Domain, Service, Core module separation
- 🎨 **Jetpack Compose**: Modern declarative UI with Material 3
- 💉 **Hilt DI**: Type-safe dependency injection
- 🔄 **MVI Pattern**: Unidirectional data flow (similar to TCA)
- 🚀 **Convention Plugins**: Centralized build logic in `buildSrc`
- 📦 **Version Catalogs**: Centralized dependency management
- 🧪 **Konsist Testing**: Automated architecture validation
- ⚡ **Gradle Optimizations**: Build cache, configuration cache, parallel execution

---

## 🚀 Quick Start

### Prerequisites

- Python 3.8+ with `cookiecutter` installed
- JDK 17+
- Android Studio Ladybug or later

### Installation

```bash
# Install Cookiecutter
pip install cookiecutter

# Generate a new Android project
cookiecutter gh:your-org/TMAndroid

# Follow the prompts:
# project_name: MyAwesomeApp
# base_package: com.example.myapp
# use_firebase: yes
# architecture_pattern: MVI

# Navigate to the project
cd myawesomeapp

# Build the project
./gradlew build

# Open in Android Studio
```

---

## 🏛️ Architecture

### Module Types

| Module Type | Purpose | Example |
|------------|---------|---------|
| **App** | Application entry point, DI composition root | `:app` |
| **Feature** | UI screens, ViewModels, navigation | `:feature:feature-home` |
| **Domain API** | Business logic interfaces, models | `:domain:domain-user-api` |
| **Domain Impl** | Domain logic implementation | `:domain:domain-user-impl` |
| **Service API** | External system interfaces (API, DB) | `:service:service-auth-api` |
| **Service Impl** | Service implementation (Retrofit, Room) | `:service:service-auth-impl` |
| **Core** | Shared utilities, UI components | `:core:core-ui`, `:core:core-network`, `:core:core-data`, `:core:core-database` |

### Dependency Flow

```
App (Composition Root)
 ├── Feature Modules
 │   ├── → Domain API
 │   ├── → Service API
 │   └── → Core
 ├── Domain Impl → Domain API
 └── Service Impl → Service API
```

**Rules**:
- ✅ Feature → Domain API, Service API, Core
- ✅ Domain Impl → Domain API
- ✅ Service Impl → Service API
- ✅ App → All Impl modules
- ❌ Feature ↔ Feature (no cross-feature dependencies)
- ❌ Domain → Feature
- ❌ API → Impl

---

## 📦 Generated Project Structure

```
myawesomeapp/
├── app/                          # Application module
│   ├── src/main/kotlin/
│   │   ├── MainActivity.kt
│   │   ├── TmaApplication.kt
│   │   ├── di/
│   │   ├── navigation/
│   │   └── ui/
│   └── build.gradle.kts
├── feature/                      # Feature modules
│   └── feature-*/
├── domain/                       # Domain modules
│   ├── domain-*-api/
│   └── domain-*-impl/
├── service/                      # Service modules
│   ├── service-*-api/
│   └── service-*-impl/
├── core/                         # Core modules
│   ├── core-ui/
│   ├── core-network/
│   ├── core-data/
│   └── core-database/
├── buildSrc/                     # Convention Plugins
│   └── src/main/kotlin/
│       ├── TmaApplicationPlugin.kt
│       ├── TmaFeaturePlugin.kt
│       ├── TmaDomainApiPlugin.kt
│       └── ...
├── gradle/
│   └── libs.versions.toml        # Version Catalog
├── konsist-test/                 # Architecture validation
├── build.gradle.kts              # Root build script (includes scaffolding tasks)
├── settings.gradle.kts
└── gradle.properties
```

---

## 🛠️ Creating Modules

TMAndroid provides Gradle tasks for instant module generation.

### Create Feature Module

```bash
./gradlew scaffoldFeature -PmoduleName=Home
```

**Generates**:
```
feature/feature-home/
├── src/main/kotlin/{package}/feature/home/
│   ├── HomeState.kt
│   ├── HomeAction.kt
│   ├── HomeSideEffect.kt
│   ├── HomeViewModel.kt
│   ├── ui/HomeScreen.kt
│   └── navigation/HomeNavigation.kt
├── src/test/kotlin/
│   └── HomeViewModelTest.kt
└── build.gradle.kts (with plugin: tma.feature)
```

### Create Domain Module

```bash
./gradlew scaffoldDomain -PmoduleName=User
```

**Generates**:
```
domain/
├── domain-user-api/
│   ├── src/main/kotlin/{package}/domain/user/
│   │   ├── model/
│   │   ├── repository/UserRepository.kt
│   │   └── usecase/GetUserUseCase.kt
│   └── build.gradle.kts (plugin: tma.domain.api)
└── domain-user-impl/
    ├── src/main/kotlin/{package}/domain/user/impl/
    │   ├── repository/DefaultUserRepository.kt
    │   └── usecase/DefaultGetUserUseCase.kt
    ├── src/test/kotlin/
    └── build.gradle.kts (plugin: tma.domain.impl)
```

### Create Service Module

```bash
./gradlew scaffoldService -PmoduleName=Auth
```

**Generates**:
```
service/
├── service-auth-api/
│   ├── src/main/kotlin/{package}/service/auth/
│   │   ├── AuthService.kt
│   │   └── model/
│   └── build.gradle.kts (plugin: tma.service.api)
└── service-auth-impl/
    ├── src/main/kotlin/{package}/service/auth/impl/
    │   ├── LiveAuthService.kt
    │   ├── network/AuthApi.kt
    │   └── di/AuthModule.kt
    ├── src/test/kotlin/
    └── build.gradle.kts (plugin: tma.service.impl)
```

### Create Core Module

```bash
./scripts/scaffold-core.sh Analytics
```

---

## 🎨 MVI Pattern (TCA-inspired)

TMAndroid uses MVI (Model-View-Intent) pattern, similar to iOS's TCA.

```kotlin
// State (immutable)
data class HomeState(
    val isLoading: Boolean = false,
    val users: List<User> = emptyList(),
    val error: String? = null
)

// Action (user intent)
sealed interface HomeAction {
    data object LoadUsers : HomeAction
    data class UserClicked(val userId: String) : HomeAction
}

// SideEffect (one-time events)
sealed interface HomeSideEffect {
    data class NavigateToDetail(val userId: String) : HomeSideEffect
    data class ShowToast(val message: String) : HomeSideEffect
}

// ViewModel (Store in TCA)
@HiltViewModel
class HomeViewModel @Inject constructor(
    private val getUsersUseCase: GetUsersUseCase
) : ViewModel() {

    private val _state = MutableStateFlow(HomeState())
    val state: StateFlow<HomeState> = _state.asStateFlow()

    private val _sideEffect = Channel<HomeSideEffect>()
    val sideEffect = _sideEffect.receiveAsFlow()

    fun dispatch(action: HomeAction) {
        when (action) {
            is HomeAction.LoadUsers -> loadUsers()
            is HomeAction.UserClicked -> navigateToDetail(action.userId)
        }
    }

    private fun loadUsers() {
        viewModelScope.launch {
            _state.update { it.copy(isLoading = true) }

            getUsersUseCase()
                .onSuccess { users ->
                    _state.update { it.copy(isLoading = false, users = users) }
                }
                .onFailure { error ->
                    _state.update { it.copy(isLoading = false, error = error.message) }
                }
        }
    }
}

// Composable Screen
@Composable
fun HomeScreen(viewModel: HomeViewModel = hiltViewModel()) {
    val state by viewModel.state.collectAsStateWithLifecycle()

    LaunchedEffect(Unit) {
        viewModel.sideEffect.collect { effect ->
            when (effect) {
                is HomeSideEffect.NavigateToDetail -> { /* navigate */ }
                is HomeSideEffect.ShowToast -> { /* show toast */ }
            }
        }
    }

    HomeContent(state = state, onAction = viewModel::dispatch)
}
```

---

## 🧪 Architecture Validation

TMAndroid includes Konsist tests to enforce architecture rules.

```bash
# Run architecture tests
./gradlew :konsist-test:test
```

**Example Rules**:
```kotlin
@Test
fun `all ViewModels must be in feature layer`() {
    Konsist.scopeFromProject()
        .classes()
        .withNameEndingWith("ViewModel")
        .assertTrue { it.resideInPackage("..feature..") }
}

@Test
fun `feature layer cannot depend on other features`() {
    // Validates no cross-feature dependencies
}

@Test
fun `API modules cannot depend on impl modules`() {
    // Ensures clean architecture boundaries
}
```

---

## 📋 Convention Plugins

All build logic is centralized in `buildSrc` as reusable plugins.

| Plugin | Purpose | Applied To |
|--------|---------|-----------|
| `tma.application` | App module configuration | `:app` |
| `tma.feature` | Feature module with Compose + Hilt | Feature modules |
| `tma.domain.api` | Domain interface module | Domain API |
| `tma.domain.impl` | Domain implementation with Hilt | Domain Impl |
| `tma.service.api` | Service interface module | Service API |
| `tma.service.impl` | Service implementation with Hilt | Service Impl |
| `tma.core` | Core utility module | Core modules |
| `tma.compose` | Jetpack Compose support | Composable modules |
| `tma.hilt` | Hilt DI support | DI modules |
| `tma.test` | Testing configuration | Test modules |

**Usage**:
```kotlin
// feature/feature-home/build.gradle.kts
plugins {
    alias(libs.plugins.tma.feature)
}

android {
    namespace = "com.example.myapp.feature.home"
}

dependencies {
    implementation(projects.domain.domainUserApi)
    implementation(projects.core.coreUi)
}
```

---

## 🎯 Comparison: iOS TmaTemplates vs TMAndroid

| Aspect | iOS (TmaTemplates) | Android (TMAndroid) |
|--------|-------------------|---------------------|
| **Generator** | Tuist | Cookiecutter |
| **Build System** | Tuist + Xcode | Gradle + Kotlin DSL |
| **Architecture** | TCA | MVI + Clean Architecture |
| **UI Framework** | SwiftUI | Jetpack Compose |
| **DI** | TCA Dependencies | Hilt (Dagger) |
| **State Management** | Reducer + Store | ViewModel + StateFlow |
| **Concurrency** | Swift Concurrency | Kotlin Coroutines |
| **Module Creation** | `tuist scaffold feature` | `./scripts/scaffold-feature.sh` |
| **Project Generation** | `tuist generate` | (Not needed - Gradle handles it) |
| **Convention Sharing** | ProjectDescriptionHelpers | Convention Plugins (buildSrc) |
| **Dependency Management** | Package.swift (SPM) | libs.versions.toml (Version Catalog) |
| **Architecture Validation** | Tuist Graph | Konsist |

---

## 🔧 Gradle Commands

```bash
# Build entire project
./gradlew build

# Run all tests
./gradlew test

# Run architecture validation
./gradlew :konsist-test:test

# Clean build
./gradlew clean

# Build specific module
./gradlew :feature:feature-home:build

# Print dependency tree
./gradlew printDependencyTree

# Validate architecture
./gradlew validateArchitecture
```

---

## 📚 Documentation

- **[IMPLEMENTATION_PLAN.md](./IMPLEMENTATION_PLAN.md)**: Detailed technical design
- **[MODULE_SCAFFOLD_GUIDE.md](./MODULE_SCAFFOLD_GUIDE.md)**: Module creation guide (generated in each project)
- **[Konsist Testing](https://docs.konsist.lemonappdev.com/)**: Architecture validation docs

---

## ⚙️ Configuration

### Cookiecutter Variables

When generating a project, you'll be prompted for:

| Variable | Description | Default |
|----------|-------------|---------|
| `project_name` | Project display name | MyAwesomeApp |
| `base_package` | Java package (e.g., com.example.app) | com.example.myapp |
| `min_sdk` | Minimum Android SDK | 24 |
| `target_sdk` | Target Android SDK | 35 |
| `use_firebase` | Include Firebase integration | yes |
| `architecture_pattern` | MVI or MVVM | MVI |

Room/Retrofit are enabled by default. Update the base URL in `app/di/AppModule.kt` and replace the sample Room schema in `core/core-database`.

### Generated Project Customization

After generation, customize:

1. **Version Catalog** (`gradle/libs.versions.toml`): Update dependency versions
2. **Convention Plugins** (`buildSrc/`): Modify build logic
3. **Gradle Properties** (`gradle.properties`): Adjust build performance settings
4. **ProGuard Rules** (`app/proguard-rules.pro`): Add obfuscation rules

---

## 🚧 Roadmap

- [ ] **Phase 1**: Core scaffolding and convention plugins ✅
- [ ] **Phase 2**: Module templates (Feature, Domain, Service, Core)
- [ ] **Phase 3**: Automation scripts (scaffold-*.sh)
- [ ] **Phase 4**: Konsist architecture tests
- [ ] **Phase 5**: CI/CD integration (GitHub Actions)
- [ ] **Phase 6**: IntelliJ IDEA plugin (GUI-based module creation)
- [ ] **Phase 7**: Kotlin Multiplatform (KMP) support
- [ ] **Phase 8**: Compose Multiplatform templates

---

## 🤝 Contributing

Contributions are welcome! Please read our [Contributing Guide](./CONTRIBUTING.md) (TODO).

---

## 📄 License

MIT License - see [LICENSE](./LICENSE) for details.

---

## 🙏 Acknowledgments

- Inspired by [TmaTemplates](../TmaTemplates) (iOS/Tuist)
- Architecture patterns from [Now in Android](https://github.com/android/nowinandroid)
- Convention plugins approach from [Gradle Best Practices](https://docs.gradle.org/current/userguide/organizing_gradle_projects.html)

---

**Generated by TMAndroid** | [Report Issues](https://github.com/your-org/TMAndroid/issues)
