# TmaTemplates – Tuist Modular Architecture Plugin

TMA는 **The Composable Architecture (TCA)**를 사용하는 iOS 프로젝트를 위한 전문 Tuist 플러그인입니다. `tuist scaffold` 한 번으로 App, Feature, Service, Domain, Shared 모듈을 동일한 규칙과 테스트 기반으로 생성하며, 현대적인 iOS 개발 워크플로우에 최적화된 구조를 제공합니다.

---

## 📖 핵심 철학 및 원칙 (Core Principles)

TMA는 **μFeatures(Micro-Features)** 아키텍처와 **TCA 공식 패턴**을 결합하여 대규모 프로젝트에서도 유지보수 가능한 구조를 지향합니다.

- **일관된 타겟 구조**: 
    - **Domain/Service**: 2-target (Interface + Sources). 
    - **Feature**: 2-target (Feature + Tests). 
    - **Shared**: 1-target (Internal logical separation).
- **TCA 공식 관리 패턴**: `TestDependencyKey`는 Interface에, `liveValue`는 App Composition Root에서 주입하여 의존성을 철저히 분리합니다.
- **Portability (Self-contained)**: 모든 템플릿은 독립적입니다. 외부 Helper 유틸리티 없이 각 `Project.swift` 내에 설정이 완결적으로 포함되어 있어 높은 휴대성을 가집니다.
- **Swift 6 Readiness**: 전 모듈에 대해 Swift 6의 Concurrency 모델을 준수하도록 기본 설정되어 있습니다.

---

## 🛠️ 요구 사항 및 설치 (Setup)

### 요구 사항
- **Tuist 4.119.1 이상** (Xcode 16 `buildableFolders` 기능 필수)
- **Xcode 16.0 이상**
- **iOS 15.0 이상** (기본 Deployment Target)

### 설치 방법
`Tuist.swift` (프로젝트 루트)에 플러그인을 등록합니다.

```swift
import ProjectDescription

let tuist = Tuist(
    plugins: [
        .git(url: "https://github.com/axiom-orient/tma", tag: "2.0.0")
    ]
)
```

이후 터미널에서 `tuist install`을 실행하여 플러그인을 활성화합니다.

---

## 🚀 빠른 시작 (Quick Start)

### 1. 외부 의존성 관리
`Tuist/Package.swift`에 라이브러리를 정의하고 `tuist install`을 통해 동기화합니다. `Dependencies.swift`는 더 이상 사용하지 않습니다.

### 2. 모듈 생성 명령어
명령어 한 줄로 아키텍처 가이드라인에 맞는 모듈을 즉시 스캐폴딩할 수 있습니다.

| 템플릿 | 용도 | 명령어 예시 |
| :--- | :--- | :--- |
| **App** | 진입점, Firebase, DI Root | `tuist scaffold app --name MyApp --bundle-id-prefix com.example` |
| **Feature** | UI 스크린 및 비즈니스 로직 | `tuist scaffold feature --name Home` |
| **Domain** | 순수 비즈니스 로직 및 모델 | `tuist scaffold domain --name User` |
| **Service** | 앱 외부 시스템 경계 (API 등) | `tuist scaffold service --name Auth` |
| **Shared** | 공용 유틸리티, 디자인 시스템 | `tuist scaffold shared --name UIComponents` |

---

## 🏛️ 아키텍처 상세 설계 (Architecture Deep Dive)

### 1. 모듈별 타겟 상세

| 모듈 유형 | 타겟 구성 | 종속성 규칙 |
| :--- | :--- | :--- |
| **Feature** | Feature, Tests | Domain, Service, Shared |
| **Domain** | Interface, Sources | Foundation, Dependencies |
| **Service** | Interface, Sources, Tests | Interface, Shared |
| **Shared** | Sources (Internal organization) | Foundation |

> [!TIP]
> **권장 Import 패턴**: 가능한 한 `Interface`만 import 하세요 (예: `import UserDomainInterface`). 구체적인 구현체(`Sources`)는 App 타겟에서만 링크됩니다.

### 2. 의존성 규칙 (Dependency Rules)

```mermaid
graph TD
    App[App Composition Root] --> FeatureSources
    App --> DomainSources
    App --> ServiceSources
    
    FeatureSources --> DomainInterface
    FeatureSources --> ServiceInterface
    FeatureSources --> Shared
    
    DomainSources --> DomainInterface
    ServiceSources --> ServiceInterface
    ServiceSources --> Shared
```

- ✅ **GOOD**: `Interface`만 Import (예: `import UserDomainInterface`).
- ❌ **BAD**: `Sources` 직접 Import (예: `import UserDomainSources`).
- **Composition Root**: 오직 `App` 타겟만이 실체 구현체(`Sources`)를 알고 연결합니다.

---

## 🔍 TMA App 모듈 분석 (App Module Deep Dive)

TMA App 모듈은 **μFeatures(Micro-Features)** 아키텍처와 **TCA(The Composable Architecture)**를 기반으로 설계되었습니다. 이 문서는 각 파일의 역할과 앱이 시작될 때 발생하는 시퀀스를 코드 수준에서 분석한 결과입니다.

### 1. 아키텍처 레이어 구성

#### Layer 1: Entry & Lifecycle (진입점 및 관리)
- **App.stencil**: `@main`이 정의된 최상위 진입점입니다. Root Store를 초기화하고 의존성 주입을 시작합니다.
- **AppDelegate.stencil**: Firebase SDK 초기화 및 플랫폼 레벨의 이벤트(Deep Link)를 수신하여 `DeepLinkStore`로 전달합니다.
- **ApplicationLifecycle.stencil**: 앱의 부팅 시퀀스(RemoteConfig 페치, Analytics 설정 등)와 씬 상태(Background/Foreground)를 관리하는 도메인 모듈입니다.

#### Layer 2: Core Logic (비즈니스 로직 및 DI)
- **AppReducer.stencil**: 앱의 전역 상태(`State`)와 액션(`Action`)을 정의합니다. 라이프사이클과 연동하여 앱의 준비 상태를 결정합니다.
- **AppComposition.stencil**: 의존성 주입의 핵심인 Composition Root입니다. 모든 도메인/서비스/기능 모듈의 실제 구현체(`liveValue`)를 TCA 의존성 시스템에 등록합니다.
- **AppConstants.stencil**: RemoteConfig 키 등 전역 설정값을 관리합니다.

#### Layer 3: Infrastructure Services (기술 스택)
- **RemoteConfigServices**: Firebase Remote Config를 통해 런타임에 앱 설정을 변경합니다.
- **AnalyticsServices**: 사용자 이벤트를 추적하고 기록합니다.
- **DeepLinkServices**: URL을 해석하여 특정 화면으로 이동시키는 비즈니스 로직을 수행합니다.

### 2. 앱 시작 시퀀스 (Startup Sequence)

앱이 실행되면 다음과 같은 단계로 초기화가 진행됩니다.

1.  **의존성 구성 (Static Stage)**
    - `App.init()` 내부에서 `AppComposition.configureAll()`이 호출됩니다.
    - 이때 `NoopRemoteConfigService` 또는 `FirebaseRemoteConfigAdapter` 등이 주입되며, `RemoteConfigDefaults.plist`의 초기값이 로드됩니다.

2.  **시스템 초기화 (System Stage)**
    - `AppDelegate`가 생성되며 `FirebaseApp.configure()`가 실행됩니다.
    - `GoogleService-Info.plist`의 존재 여부를 검사하여 Firebase 기능 활성화 여부를 결정합니다.

3.  **TCA 부팅 (Boot Stage)**
    - 최상위 뷰가 나타나면 `AppReducer`에게 `.appDidLaunch` 액션을 보냅니다.
    - `AppReducer`는 이를 전문 리듀서인 `ApplicationLifecycle`로 위임합니다.

4.  **리소스 로딩 (Loading Stage)**
    - `ApplicationLifecycle`이 `remoteConfigService.fetchAndActivate()`를 호출합니다.
    - 새로운 설정값이 반영되면 `analyticsService`의 활성화 여부를 업데이트합니다.
    - 모든 과정이 완료되면 `didFinishColdStart`를 Root Reducer에 알립니다.

5.  **화면 전환 (View Transition Stage)**
    - `isAppReady` 상태가 `true`로 변경됩니다.
    - `SplashView` 애니메이션이 종료(`onComplete`)되면 최종적으로 `MainScreenView`가 사용자에게 노출됩니다.

---

## 📦 공용 모듈 가이드 (Shared Modules Guide)

Shared 모듈은 디자인 시스템이나 유틸리티 등 모든 레이어에서 공통으로 사용하는 기능을 제공합니다.

### 1. SharedCore (Infrastructure)

`tuist scaffold shared --name SharedCore` 명령으로 생성되며, 다음 컴포넌트를 포함합니다.

#### NetworkMonitor
Actor 기반의 네트워크 상태 모니터링 유틸리티입니다.

```swift
// Interface
public protocol NetworkMonitoring: Sendable {
    var status: NetworkStatus { get async }
    func start() async
}

// Usage in Feature
@Dependency(\.networkMonitor) var networkMonitor
// ...
await networkMonitor.start()
```

#### KeychainStorage
안전한 데이터 저장을 위한 Actor 기반 래퍼입니다.

```swift
// Interface
public protocol SecureStoring: Sendable {
    func save<T: Codable>(_ value: T, forKey key: String) async throws
    func load<T: Codable>(forKey key: String, as type: T.Type) async throws -> T?
}

// Usage
@Dependency(\.secureStorage) var secureStorage
try await secureStorage.save("token123", forKey: "authToken")
```

### 2. DesignSystem (UI)

`tuist scaffold shared --name DesignSystem` 명령으로 생성합니다.

#### ColorToken & Theme
시맨틱 컬러와 테마 시스템을 제공합니다.

```swift
// Usage
@Dependency(\.themeProvider) var theme
Text("Hello").foregroundStyle(theme.token(for: .textPrimary).swiftUIColor)
```

---

## 💡 개발 가이드라인 및 베스트 프랙티스

### 1. Xcode 16 `buildableFolders` 활용
Tuist 4.62.0부터 지원되는 파일 시스템 동기화 기능을 적극 활용합니다. 파일 추가/삭제 시 `tuist generate`를 매번 실행할 필요가 없어 AI 도우미와의 협업에 최적화되어 있습니다.

### 2. TCA Linking 일관성
기본적으로 모든 모듈을 `.staticFramework`로 유지하세요. 중복 심볼 오류가 발생할 경우에만 예외적으로 전체 모듈을 `.framework`로 전환하는 것을 검토합니다.

### 3. CI/CD 및 보안
- GitHub Actions 사용 시 `TUIST_TOKEN` 노출 대신 **OIDC 인증**을 연동하세요.
- CI 환경에서는 `tuist install --force-resolved-versions`를 사용하여 빌드 결정론을 확보합니다.

---

## 📋 검증 체크리스트

새로운 버전을 배포하거나 모듈을 추가할 때 다음을 확인하세요:
1.  `tuist scaffold` 명령어가 에러 없이 완료되는가?
2.  생성된 모듈의 `Project.swift`가 `tuist edit`에서 유효한가?
3.  Swift 6 Strict Concurrency 경고가 없는가?
4.  App 모듈의 `AppComposition.swift`에 신규 모듈의 `liveValue`가 등록되었는가?

---

## 라이선스
이 프로젝트는 [MIT License](LICENSE)를 따릅니다.
## Troubleshooting

### Q: Why do I see duplicate static library linking warnings?

> Target '...' has been linked from target '...' and target '...', it is a static product so may introduce unwanted side effects.

**Cause:**
In Tuist 4.x and Swift Package Manager, if a static library (e.g., `CombineSchedulers` from TCA) is a transitive dependency of multiple targets (e.g., `SharedCore` and `App`), and those targets are linked together or dynamically, the static library code might be copied multiple times into the final binary. This causes duplicate symbols and increased app size.

**Solution:**
We use `PackageSettings` in `Tuist/Package.swift` to force these shared dependencies to be built as **Dynamic Frameworks**. This ensures they are compiled once and shared across all targets.

The `scaffold_ios.sh` script automatically configures this:

```swift
#if TUIST
import ProjectDescription

let packageSettings = PackageSettings(
    productTypes: [
        "ComposableArchitecture": .framework,
        "Dependencies": .framework,
        "Sharing": .framework,
        // ... transitive dependencies ...
        "CombineSchedulers": .framework,
        "ConcurrencyExtras": .framework,
        // ...
    ]
)
#endif
```

If you see these warnings again (e.g., after adding a new library), add the specific target name to the `productTypes` list in `Tuist/Package.swift` (via `scaffold_ios.sh` or manually) and set it to `.framework`.
