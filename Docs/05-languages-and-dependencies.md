# 사용 언어와 라이브러리 버전 관리

Rust 의존성은 [Cargo.toml](../Cargo.toml)의 버전 요구식과 [Cargo.lock](../Cargo.lock)의 실제 해석 버전으로 관리한다.

## 사용 언어와 파일 형식

| 구분 | 언어 / 형식 | 사용 범위와 기준 |
|---|---|---|
| 프로그램 | Rust, Edition 2024 | CLI, 토큰 검증, 앱별 변환을 단일 Cargo 패키지로 구현한다. 최소 Rust 버전 목표는 `1.85.0`이다. |
| 테마 원본 | TOML | `themes\lavender-type\lbrx.toml`의 팔레트·역할·서체·옵션. 원본 계약은 `schema_version = 1`부터 시작하며 TOML 라이브러리 버전과 별개다. |
| 문서형 앱 출력 | CSS | Typora와 Obsidian 테마. 대상 앱에서 확인한 선택자·속성만 사용한다. 별도 Sass·PostCSS 빌드는 두지 않는다. |
| 구조화된 출력 | JSON / TOML | VS Code 테마·manifest와 Windows Terminal 설정 조각은 JSON, Helix 테마 및 편집기·언어 설정은 TOML로 생성한다. |
| Vim 출력 | Vim script | 터미널 Vim 색상 정의만 생성한다. Vim9 전용 문법이나 Neovim Lua를 전제로 하지 않는다. |
| macOS 터미널 출력 | plist | iTerm2 `.itermcolors` 색 프리셋과 Terminal.app `.terminal` 프로파일을 각각 생성한다. 후자는 색을 바이너리 NSColor 아카이브로 포함한다. |
| 안내·기준 샘플 | Markdown | 한국어 문서와 한글·영문 혼합 미리보기 샘플을 관리한다. |

테마 생성 자체에는 JavaScript·TypeScript·Python이 필요하지 않다. 선택 기능인 [글꼴 도구](../scripts/theme-fonts.py)는 Python 3.11 이상과 표준 라이브러리를 사용하고, Linux에서는 Fontconfig의 `fc-list`와 `fc-cache`를 사용한다. VS Code용 산출물은 실행 코드 없는 색 테마 확장을 목표로 하므로 Node.js를 CLI 실행 필수 조건으로 두지 않는다. CSS·JSON·Vim script의 지원 범위는 **대상 앱의 검증 버전**으로 관리한다.

## Rust 직접 의존성

버전 열은 `Cargo.toml`의 요구식이다. 예를 들어 `~4.5.32`는 `4.5.32` 이상 `4.6.0` 미만을 허용한다. 실제 버전은 `Cargo.lock`에서 확인한다.

| 라이브러리 | 버전 요구식 | 필요한 기능 / 용도 | 도입 시점 |
|---|---|---|---|
| `serde` | `1` | `derive`: 테마 모델의 직렬화·역직렬화 | 원본 모델 |
| `toml` | `~0.8` | TOML 원본 읽기와 생성된 Helix 테마 재파싱 | 원본 검사·빌드 검증 |
| `clap` | `~4.5.32` | `derive`: `check` / `build` 명령과 인자 검사 | CLI |
| `serde_json` | `1` | JSON 테마·manifest·설정 조각 생성과 출력 재파싱 | JSON 어댑터 |
| `tempfile` | `3` | 미리보기 다시 읽기 및 읽기 전용 동작을 검증하는 임시 테스트 원본 | 테스트 전용 |
| `plist` | `~1.7` | iTerm2 `.itermcolors` XML 생성 및 재파싱 | iTerm2 어댑터 |
| `zip` | `~2.4` | VSIX 컨테이너 작성 및 다시 읽기. 압축 기능 없이 저장한다. | VS Code 어댑터 |

## 버전의 기준 파일과 변경 절차

| 관리 대상 | 구현 시 기준 파일 | 관리 규칙 |
|---|---|---|
| 개발·CI 도구체인 | `rust-toolchain.toml` | `1.85.0`, `rustfmt`, `clippy`로 고정한다. |
| 최소 Rust 버전·Edition | `Cargo.toml`의 `rust-version`, `edition` | 목표는 각각 `1.85`, `2024`. 고정 도구체인을 올려도 최소 버전은 별도로 검증한다. |
| 직접 의존성·feature | `Cargo.toml` | 위 표와 일치시킨다. 기본 feature를 포함해 실제 필요한 기능을 확인한다. |
| 실제 의존성 해석 결과 | `Cargo.lock` | CLI 애플리케이션이므로 저장소에 커밋한다. 문서에 전이 의존성을 복제하지 않는다. |
| 원본 스키마 | 테마 원본의 `schema_version`과 Rust 모델 | 라이브러리 업데이트와 분리한다. 기존 키의 의미·단위 변경은 마이그레이션 대상이다. |

1. 의존성 도입·갱신 시 변경 이유, 필요한 feature, 최소 Rust 버전 영향을 확인한다. 호환 범위 밖 업데이트는 API와 생성물 차이까지 검토한다.
2. `Cargo.toml`과 `Cargo.lock`을 함께 갱신하고 이 문서의 요구식·상태를 맞춘다. 단순히 최신 버전을 따라가기 위해 도구체인과 모든 의존성을 한꺼번에 올리지 않는다.
3. 저장소 루트에서 `cargo check --locked`, `cargo test --locked`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo fmt --check`를 통과시킨다. 최소 Rust 버전이 개발 도구체인과 다르면 그 최소 버전에서도 검사·테스트한다.
4. 어댑터 관련 변경은 양쪽 모드 출력 재파싱과 해당 앱의 실제 가져오기를 확인한다. 의도하지 않은 테마 값 변경은 허용하지 않는다.

## 대상 앱과 외부 도구 버전

대상 앱은 Rust 의존성과 다르다. 구현·검증 전에는 최소 지원 버전을 단정하지 않으며, 최초 가져오기 검증 시 아래 표를 채운다. 정확한 버전, OS, 뷰·호스트, 라이트·다크 결과와 확인일을 기록하고, 특정 버전에서 성공했다는 이유만으로 그 이상 모든 버전의 지원을 선언하지 않는다.

| 대상 | 검증 버전 / 환경 | 특히 확인할 항목 |
|---|---|---|
| Typora | 미검증 | 문서 CSS, 본문·코드 서체 |
| Google Antigravity IDE | 미검증; VSIX 엔진 `^1.90.0`, 표준 색 테마 구조 검사 | 실제 설치 및 에이전트 독자적 웹뷰의 색 상속은 별도 확인 |
| Antigravity CLI (agy) | Windows agy 1.5.18 버전·도움말 확인(2026-11-27), 대화 TUI 미검증 | 공식 colorScheme=terminal 스키마, --settings 세션 전용 플래그 |
| Claude Code CLI | 공식 사용자 정의 테마 문서 확인(2026-11-27), 실제 CLI 미검증 | ~/.claude/themes JSON, base·overrides 토큰과 터미널 배색 경계 |
| Obsidian | 초기 테마 가독성 사용자 확인(2026-11-23), 앱 버전 미기록 | [확인 범위](10-obsidian-options.md#검증-기록). 이후 기능별 화면·모바일·PDF 검증과는 별개 |
| VS Code | 1.147.0 (Windows, 임시 프로필) | VSIX 설치와 확장 등록 확인. 실제 화면 및 서체 적용은 미검증 |
| Helix | 25.07 규격·기본 언어 카탈로그 기준, 화면 미검증 | 스코프, 설정 병합, 호스트 터미널 |
| Vim | 9.2 (Windows, 비대화형 검증) | 라이트·다크 불러오기, 대표 RGB·`cterm` 값 확인. 실제 화면은 미검증 |
| Windows Terminal | `1.25.3131.0` x64 Portable: 라이트·다크 프로필 스키마 검증 및 두 창 실행 확인 (2026-11-19) | 사용자가 미리보기 화면을 직접 확인하고 긍정적으로 평가함. 항목별 결과는 별도 기록하지 않음. |
| macOS Terminal.app | 미검증 | macOS·Terminal.app 버전, 색상 데이터와 프로필 가져오기 |
| iTerm2 | 미검증 | macOS·iTerm2 버전, `.itermcolors` 가져오기와 실제 배색 |
| LBRX Studio | 개발 중인 React/Tauri 앱의 CSS 변수 계약 기준, 실제 앱 화면 미검증 | [루트 모드 선택·HTML 토큰·글꼴 변수](25-lbrx-studio.md). 앱 파일은 자동 수정하지 않음 |

VSIX는 Rust에서 직접 생성하며 `@vscode/vsce`는 필요하지 않다. [package.json](../package.json)과 [pnpm-lock.yaml](../pnpm-lock.yaml)은 별도의 아이콘 자산 버전을 관리하며, Rust CLI나 테마 설치의 요구 사항이 아니다. LINE Seed EN·KR·JP와 D2Coding ligature는 사용자가 설치하는 글꼴이다.
