#!/usr/bin/env python3
"""
TMAndroid Post-generation Hook

이 스크립트는 Cookiecutter가 프로젝트를 생성한 직후에 실행됩니다.
불필요한 파일을 제거하고, Git 저장소를 초기화하며, Gradle 권한을 설정합니다.
"""

import os
import shutil
import subprocess
import sys
import urllib.request
from pathlib import Path
from typing import List

# Cookiecutter 변수
PROJECT_NAME = "{{ cookiecutter.project_name }}"
PROJECT_SLUG = "{{ cookiecutter.project_slug }}"
BASE_PACKAGE = "{{ cookiecutter.base_package }}"
USE_FIREBASE = "{{ cookiecutter.use_firebase }}" == "yes"
USE_ROOM = True
USE_RETROFIT = True

# 프로젝트 루트
PROJECT_ROOT = Path.cwd()


def print_step(message: str) -> None:
    """단계 메시지 출력"""
    print(f"\n🔧 {message}...")


def print_success(message: str) -> None:
    """성공 메시지 출력"""
    print(f"✅ {message}")


def print_warning(message: str) -> None:
    """경고 메시지 출력"""
    print(f"⚠️  {message}")


def print_error(message: str) -> None:
    """에러 메시지 출력"""
    print(f"❌ {message}", file=sys.stderr)


def remove_file_or_dir(path: Path) -> None:
    """파일 또는 디렉토리 제거"""
    if not path.exists():
        return

    try:
        if path.is_dir():
            shutil.rmtree(path)
        else:
            path.unlink()
        print(f"   Removed: {path.relative_to(PROJECT_ROOT)}")
    except Exception as e:
        print_warning(f"Failed to remove {path}: {e}")


def remove_unused_firebase_files() -> None:
    """Firebase 미사용 시 관련 파일 제거"""
    if USE_FIREBASE:
        return

    print_step("Removing Firebase files")

    firebase_paths: List[Path] = [
        PROJECT_ROOT / "app" / "google-services.json.template",
        PROJECT_ROOT / "app" / "src" / "main" / "kotlin" / BASE_PACKAGE.replace(".", "/") / "firebase",
        PROJECT_ROOT / "app" / "src" / "main" / "kotlin" / BASE_PACKAGE.replace(".", "/") / "di" / "FirebaseModule.kt",
        PROJECT_ROOT / "core" / "core-data" / "src" / "main" / "kotlin" / BASE_PACKAGE.replace(".", "/") / "core" / "remoteconfig" / "FirebaseRemoteConfigRepository.kt",
    ]

    for path in firebase_paths:
        remove_file_or_dir(path)

    print_success("Firebase files removed")


def download_gradle_wrapper_jar() -> None:
    """Gradle wrapper JAR 다운로드"""
    print_step("Downloading Gradle wrapper JAR")

    wrapper_jar_path = PROJECT_ROOT / "gradle" / "wrapper" / "gradle-wrapper.jar"
    wrapper_jar_url = "https://raw.githubusercontent.com/gradle/gradle/v8.11.1/gradle/wrapper/gradle-wrapper.jar"

    try:
        wrapper_jar_path.parent.mkdir(parents=True, exist_ok=True)
        urllib.request.urlretrieve(wrapper_jar_url, wrapper_jar_path)
        print_success("Gradle wrapper JAR downloaded")
    except Exception as e:
        print_warning(f"Failed to download gradle-wrapper.jar: {e}")
        print_warning("Run './gradlew wrapper' manually after generation")


def create_package_directories() -> None:
    """패키지 디렉토리 구조 생성"""
    print_step("Creating package directory structure")

    package_path = BASE_PACKAGE.replace(".", "/")

    # App 모듈 패키지
    app_packages = [
        f"app/src/main/kotlin/{package_path}",
        f"app/src/main/kotlin/{package_path}/di",
        f"app/src/test/kotlin/{package_path}",
        f"app/src/androidTest/kotlin/{package_path}",
    ]

    for pkg in app_packages:
        (PROJECT_ROOT / pkg).mkdir(parents=True, exist_ok=True)

    print_success("Package directories created")


def generate_source_files() -> None:
    """템플릿에서 소스 파일 생성"""
    print_step("Generating source files from templates")

    package_path = BASE_PACKAGE.replace(".", "/")
    
    # Try to read template path from .mgen_template_info file
    template_info_file = Path(".mgen_template_info")
    if template_info_file.exists():
        templates_base_path = template_info_file.read_text().strip()
        templates_base = Path(templates_base_path)
        # Clean up the info file
        template_info_file.unlink()
    else:
        # Fallback (unlikely to work in this setup but useful if run manually in place)
        templates_base = Path(__file__).parent.parent / "templates"
        print_step(f"Using inferred templates path: {templates_base}")

    if not templates_base.exists():
        print_warning(f"Templates directory NOT found at {templates_base}")

    # Source files to generate (template_path: output_path)
    source_files = {
        # App module - Core
        "app/MainActivity.kt.j2": f"app/src/main/kotlin/{package_path}/MainActivity.kt",
        "app/MainApplication.kt.j2": f"app/src/main/kotlin/{package_path}/MainApplication.kt",
        "app/AppState.kt.j2": f"app/src/main/kotlin/{package_path}/AppState.kt",
        "app/AppConstants.kt.j2": f"app/src/main/kotlin/{package_path}/AppConstants.kt",
        "app/di/AppModule.kt.j2": f"app/src/main/kotlin/{package_path}/di/AppModule.kt",
        "app/google-services.json.j2": "app/google-services.json",

        # App module - Lifecycle
        "app/lifecycle/AppStartupState.kt.j2": f"app/src/main/kotlin/{package_path}/lifecycle/AppStartupState.kt",
        "app/lifecycle/AppLifecycleManager.kt.j2": f"app/src/main/kotlin/{package_path}/lifecycle/AppLifecycleManager.kt",


        # App module - UI
        "app/ui/SplashScreen.kt.j2": f"app/src/main/kotlin/{package_path}/ui/SplashScreen.kt",
        "app/ui/ForceUpdateScreen.kt.j2": f"app/src/main/kotlin/{package_path}/ui/ForceUpdateScreen.kt",
        "app/ui/MaintenanceScreen.kt.j2": f"app/src/main/kotlin/{package_path}/ui/MaintenanceScreen.kt",
        "app/ui/ErrorScreen.kt.j2": f"app/src/main/kotlin/{package_path}/ui/ErrorScreen.kt",

        # App module - Deep Link
        "app/deeplink/DeepLinkRoute.kt.j2": f"app/src/main/kotlin/{package_path}/deeplink/DeepLinkRoute.kt",
        "app/deeplink/DeepLinkParser.kt.j2": f"app/src/main/kotlin/{package_path}/deeplink/DeepLinkParser.kt",

        # Core modules - UI
        "core/ui/Theme.kt.j2": f"core/core-ui/src/main/kotlin/{package_path}/core/ui/theme/Theme.kt",

        # Core modules - Network
        "core/network/NetworkMonitor.kt.j2": f"core/core-network/src/main/kotlin/{package_path}/core/network/NetworkMonitor.kt",
        "core/network/di/NetworkModule.kt.j2": f"core/core-network/src/main/kotlin/{package_path}/core/network/di/NetworkModule.kt",

        # Core modules - Data
        "core/data/SecureStorage.kt.j2": f"core/core-data/src/main/kotlin/{package_path}/core/data/SecureStorage.kt",

        # Core modules - Remote Config
        "core/remoteconfig/RemoteConfigRepository.kt.j2": f"core/core-data/src/main/kotlin/{package_path}/core/remoteconfig/RemoteConfigRepository.kt",
        "core/remoteconfig/FirebaseRemoteConfigRepository.kt.j2": f"core/core-data/src/main/kotlin/{package_path}/core/remoteconfig/FirebaseRemoteConfigRepository.kt",
        "core/remoteconfig/LocalRemoteConfigRepository.kt.j2": f"core/core-data/src/main/kotlin/{package_path}/core/remoteconfig/LocalRemoteConfigRepository.kt",
        "core/remoteconfig/UpdateChecker.kt.j2": f"core/core-data/src/main/kotlin/{package_path}/core/remoteconfig/UpdateChecker.kt",
        "core/remoteconfig/LocalizedRemoteConfig.kt.j2": f"core/core-data/src/main/kotlin/{package_path}/core/remoteconfig/LocalizedRemoteConfig.kt",
        "core/remoteconfig/di/RemoteConfigModule.kt.j2": f"core/core-data/src/main/kotlin/{package_path}/core/remoteconfig/di/RemoteConfigModule.kt",

        # Core modules - Database
        "core/database/AppDatabase.kt.j2": f"core/core-database/src/main/kotlin/{package_path}/core/database/AppDatabase.kt",
        "core/database/di/DatabaseModule.kt.j2": f"core/core-database/src/main/kotlin/{package_path}/core/database/di/DatabaseModule.kt",
        "core/database/dao/SampleDao.kt.j2": f"core/core-database/src/main/kotlin/{package_path}/core/database/dao/SampleDao.kt",
        "core/database/entity/SampleEntity.kt.j2": f"core/core-database/src/main/kotlin/{package_path}/core/database/entity/SampleEntity.kt",

        # Example Feature module (Home)
        "feature/build.gradle.kts.j2": f"feature/feature-home/build.gradle.kts",
        "feature/FeatureViewModel.kt.j2": f"feature/feature-home/src/main/kotlin/{package_path}/feature/home/HomeViewModel.kt",
        "feature/FeatureScreen.kt.j2": f"feature/feature-home/src/main/kotlin/{package_path}/feature/home/HomeScreen.kt",
        "feature/FeatureState.kt.j2": f"feature/feature-home/src/main/kotlin/{package_path}/feature/home/HomeState.kt",
        "feature/FeatureAction.kt.j2": f"feature/feature-home/src/main/kotlin/{package_path}/feature/home/HomeAction.kt",
        "feature/FeatureSideEffect.kt.j2": f"feature/feature-home/src/main/kotlin/{package_path}/feature/home/HomeSideEffect.kt",
    }

    try:
        from jinja2 import Environment, FileSystemLoader

        env = Environment(loader=FileSystemLoader(str(templates_base)))

        for template_path, output_path in source_files.items():
            try:
                template = env.get_template(template_path)

                # Template variables
                template_vars = {
                    "cookiecutter": {"base_package": BASE_PACKAGE, "project_name": PROJECT_NAME},
                    "base_package": BASE_PACKAGE,
                    "project_name": PROJECT_NAME,
                    "feature_name": "Home",
                    "feature_name_lower": "home",
                }

                content = template.render(**template_vars)

                output_file = PROJECT_ROOT / output_path
                output_file.parent.mkdir(parents=True, exist_ok=True)
                output_file.write_text(content)
                print(f"   Generated: {output_path}")
            except Exception as e:
                print_warning(f"Failed to generate {template_path}: {e}")

        print_success("Source files generated")

    except ImportError:
        print_warning("Jinja2 not installed. Source files not generated.")
        print_warning("Run: pip install jinja2")
    except Exception as e:
        print_error(f"Failed to generate source files: {e}")


def make_gradlew_executable() -> None:
    """gradlew 파일에 실행 권한 부여"""
    print_step("Setting Gradle wrapper permissions")

    gradlew = PROJECT_ROOT / "gradlew"
    if gradlew.exists():
        os.chmod(gradlew, 0o755)
        print_success("gradlew is now executable")
    else:
        print_warning("gradlew not found")


def make_scripts_executable() -> None:
    """scripts 폴더의 모든 스크립트에 실행 권한 부여"""
    print_step("Setting script permissions")

    scripts_dir = PROJECT_ROOT / "scripts"
    if not scripts_dir.exists():
        return

    for script in scripts_dir.glob("*.sh"):
        os.chmod(script, 0o755)
        print(f"   Executable: {script.name}")

    print_success("Scripts are now executable")


def init_git_repository() -> None:
    """Git 저장소 초기화"""
    print_step("Initializing Git repository")

    try:
        # Git 초기화
        subprocess.run(
            ["git", "init"],
            cwd=PROJECT_ROOT,
            check=True,
            capture_output=True,
            text=True
        )

        # Git add
        subprocess.run(
            ["git", "add", "."],
            cwd=PROJECT_ROOT,
            check=True,
            capture_output=True,
            text=True
        )

        # 초기 커밋
        subprocess.run(
            ["git", "commit", "-m", "Initial commit"],
            cwd=PROJECT_ROOT,
            check=True,
            capture_output=True,
            text=True
        )

        print_success("Git repository initialized with initial commit")

    except subprocess.CalledProcessError as e:
        print_warning(f"Git initialization failed: {e.stderr}")
    except FileNotFoundError:
        print_warning("Git not found. Skipping repository initialization.")


def create_local_properties() -> None:
    """local.properties 템플릿 생성"""
    print_step("Creating local.properties template")

    local_properties = PROJECT_ROOT / "local.properties"
    sdk_dir = os.environ.get("ANDROID_SDK_ROOT") or os.environ.get("ANDROID_HOME")
    if not sdk_dir:
        default_sdk = Path.home() / "Library" / "Android" / "sdk"
        if default_sdk.exists():
            sdk_dir = str(default_sdk)

    if not sdk_dir:
        sdk_dir = "/Users/YOUR_USERNAME/Library/Android/sdk"

    content = f"""# This file is automatically generated by Android Studio.
# Do not modify this file -- YOUR CHANGES WILL BE ERASED!
#
# This file should *NOT* be checked into Version Control Systems,
# as it contains information specific to your local configuration.
#
# Location of the Android SDK:
sdk.dir={sdk_dir}

# (Optional) NDK location:
# ndk.dir=/Users/YOUR_USERNAME/Library/Android/sdk/ndk/25.1.8937393
"""
    local_properties.write_text(content)
    print_success("local.properties template created")


def create_module_guide() -> None:
    """모듈 생성 가이드 문서 생성"""
    print_step("Creating module scaffolding guide")

    guide_content = f"""# 📦 Module Scaffolding Guide

## 🏗️ Project: {PROJECT_NAME}

This document provides instructions for creating new modules in this project.

---

## 🚀 Quick Start

### Prerequisites
- Ensure you're in the project root directory
- All scripts are in `./scripts/` directory

### Create New Feature Module

Feature modules contain UI screens, ViewModels, and navigation logic.

```bash
./scripts/scaffold-feature.sh FeatureName

# Example:
./scripts/scaffold-feature.sh Home
./scripts/scaffold-feature.sh UserProfile
```

**Generated structure:**
```
feature/
└── feature-featurename/
    ├── src/
    │   ├── main/kotlin/{BASE_PACKAGE}/feature/featurename/
    │   │   ├── FeatureNameState.kt
    │   │   ├── FeatureNameAction.kt
    │   │   ├── FeatureNameSideEffect.kt
    │   │   ├── FeatureNameViewModel.kt
    │   │   ├── ui/FeatureNameScreen.kt
    │   │   └── navigation/FeatureNameNavigation.kt
    │   └── test/kotlin/
    └── build.gradle.kts
```

---

### Create New Domain Module

Domain modules contain business logic (UseCases) and domain models.

```bash
./scripts/scaffold-domain.sh DomainName

# Example:
./scripts/scaffold-domain.sh User
./scripts/scaffold-domain.sh Product
```

**Generated structure:**
```
domain/
├── domain-domainname-api/
│   ├── src/main/kotlin/{BASE_PACKAGE}/domain/domainname/
│   │   ├── model/
│   │   ├── repository/DomainNameRepository.kt
│   │   └── usecase/GetDomainNameUseCase.kt
│   └── build.gradle.kts
└── domain-domainname-impl/
    ├── src/
    │   ├── main/kotlin/{BASE_PACKAGE}/domain/domainname/impl/
    │   │   ├── repository/DefaultDomainNameRepository.kt
    │   │   └── usecase/DefaultGetDomainNameUseCase.kt
    │   └── test/kotlin/
    └── build.gradle.kts
```

---

### Create New Service Module

Service modules handle external system boundaries (APIs, databases, etc).

```bash
./scripts/scaffold-service.sh ServiceName

# Example:
./scripts/scaffold-service.sh Auth
./scripts/scaffold-service.sh Analytics
```

**Generated structure:**
```
service/
├── service-servicename-api/
│   ├── src/main/kotlin/{BASE_PACKAGE}/service/servicename/
│   │   ├── ServiceNameService.kt (interface)
│   │   └── model/
│   └── build.gradle.kts
└── service-servicename-impl/
    ├── src/
    │   ├── main/kotlin/{BASE_PACKAGE}/service/servicename/impl/
    │   │   ├── LiveServiceNameService.kt
    │   │   ├── network/ServiceNameApi.kt
    │   │   └── di/ServiceNameModule.kt
    │   └── test/kotlin/
    └── build.gradle.kts
```

---

### Create New Core Module

Core modules provide shared utilities used across all layers.

```bash
./scripts/scaffold-core.sh CoreName

# Example:
./scripts/scaffold-core.sh Utils
./scripts/scaffold-core.sh Analytics
```

**Generated structure:**
```
core/
└── core-corename/
    ├── src/main/kotlin/{BASE_PACKAGE}/core/corename/
    └── build.gradle.kts
```

---

## 📐 Architecture Rules

### Dependency Flow

```
App (Composition Root)
 ├── Feature Modules
 │   ├── → Domain API
 │   ├── → Service API
 │   └── → Core Modules
 ├── Domain Impl → Domain API
 └── Service Impl → Service API
```

### ✅ Allowed Dependencies

- Feature → Domain API
- Feature → Service API
- Feature → Core
- Domain Impl → Domain API
- Service Impl → Service API
- App → All Impl modules

### ❌ Forbidden Dependencies

- Feature → Feature (cross-feature dependencies)
- Domain → Feature
- Service → Domain
- API → Impl

---

## 🧪 Testing

### Run All Tests
```bash
./gradlew test
```

### Run Architecture Validation (Konsist)
```bash
./gradlew :konsist-test:test
```

### Run Specific Module Tests
```bash
./gradlew :feature:feature-home:test
./gradlew :domain:domain-user-impl:test
```

---

## 🏗️ Build Commands

### Build Entire Project
```bash
./gradlew build
```

### Build Specific Module
```bash
./gradlew :app:build
./gradlew :feature:feature-home:build
```

### Clean Build
```bash
./gradlew clean build
```

---

## 📦 Module Naming Conventions

| Module Type | Naming Pattern | Example |
|-------------|---------------|---------|
| Feature | `feature-<name>` | `feature-home` |
| Domain API | `domain-<name>-api` | `domain-user-api` |
| Domain Impl | `domain-<name>-impl` | `domain-user-impl` |
| Service API | `service-<name>-api` | `service-auth-api` |
| Service Impl | `service-<name>-impl` | `service-auth-impl` |
| Core | `core-<name>` | `core-ui` |

---

## 🎯 Next Steps After Module Creation

1. **Sync Gradle**: `./gradlew tasks`
2. **Add dependencies** in `build.gradle.kts`
3. **Implement business logic** in ViewModel/UseCase
4. **Write tests** in `src/test/kotlin/`
5. **Register navigation** in AppNavHost

---

## 📚 Additional Resources

- [Android Architecture Guide](https://developer.android.com/topic/architecture)
- [Jetpack Compose Documentation](https://developer.android.com/jetpack/compose)
- [Hilt Dependency Injection](https://developer.android.com/training/dependency-injection/hilt-android)
- [Kotlin Coroutines Guide](https://kotlinlang.org/docs/coroutines-guide.html)

---

**Generated by TMAndroid**
"""

    guide_path = PROJECT_ROOT / "MODULE_SCAFFOLD_GUIDE.md"
    guide_path.write_text(guide_content)
    print_success("Module scaffolding guide created")


def print_final_summary() -> None:
    """최종 완료 메시지 출력"""
    print("\n" + "=" * 60)
    print("🎉 Project Generation Complete!")
    print("=" * 60)
    print(f"\n📁 Project: {PROJECT_NAME}")
    print(f"📦 Package: {BASE_PACKAGE}")
    print(f"🏗️  Architecture: {{ cookiecutter.architecture_pattern }}")
    print(f"\n📚 Features:")
    print(f"   Firebase:      {'✅' if USE_FIREBASE else '❌'}")
    print(f"   Room:          {'✅' if USE_ROOM else '❌'}")
    print(f"   Retrofit:      {'✅' if USE_RETROFIT else '❌'}")
    print("\n🚀 Next Steps:")
    print(f"   1. cd {PROJECT_SLUG}")
    if USE_FIREBASE:
        print("   2. Add google-services.json to app/ directory")
        print("      (Download from Firebase Console)")
        print("   3. ./gradlew build")
        print("   4. Open in Android Studio")
        print("   5. Read MODULE_SCAFFOLD_GUIDE.md for creating new modules")
    else:
        print("   2. ./gradlew build")
        print("   3. Open in Android Studio")
        print("   4. Read MODULE_SCAFFOLD_GUIDE.md for creating new modules")
    print("\n" + "=" * 60 + "\n")


def main() -> None:
    """메인 실행 함수"""
    try:
        download_gradle_wrapper_jar()
        # remove_unused_firebase_files() - Firebase is now standard
        create_package_directories()
        generate_source_files()
        make_gradlew_executable()
        make_scripts_executable()
        create_local_properties()
        create_module_guide()
        init_git_repository()
        print_final_summary()

    except Exception as e:
        print_error(f"Post-generation failed: {e}")
        import traceback
        traceback.print_exc()
        sys.exit(1)


if __name__ == '__main__':
    main()
