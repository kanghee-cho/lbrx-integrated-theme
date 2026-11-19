# LBRX 통합 테마 설계

**상태: 원본 검증 `check`와 Windows Terminal·Typora용 `build` 구현.** 목적은 일곱 앱을 똑같이 보이게 하는 것이 아니라, 한 곳에서 바꾼 디자인 의도가 각 앱에서 같은 의미로 보이게 하는 것이다.

저장소 루트에서 `cargo run --locked -- check`를 실행하면 [예제 테마](../themes/lbrx.toml)를 변경하지 않고 검사한다. 다른 파일은 `check` 뒤에 경로를 전달한다. 스키마 버전, 팔레트·역할 키 일치, HEX와 참조, ANSI 16색, 서체, 대비를 확인한다. 대비는 본문/기본 배경 7:1, 보조·문서·구문·상태 색/기본 배경 및 선택 글자/선택 배경 4.5:1, 포커스/기본 배경·표면 3:1을 기준으로 한다. ANSI는 슬롯과 참조를 검사하며 임의의 ANSI 조합에는 대비 기준을 적용하지 않는다. `cargo test --locked`로 테스트를 실행한다.

```powershell
cargo run --locked -- check
cargo run --locked -- build
# 다른 원본을 지정할 때
cargo run --locked -- build .\themes\lbrx.toml
```

`build`는 같은 원본 검사를 거친 뒤 `dist\lbrx\resolved.toml`에 해석된 토큰을 쓴다. `dist\lbrx\windows-terminal`에는 배색표와 선택 적용용 글꼴 설정을, `dist\lbrx\typora`에는 라이트·다크 CSS를 생성한다. 사용자 설정은 수정하지 않는다. [Windows Terminal 적용 방법](06-windows-terminal.md), [Typora 적용 방법](07-typora.md), [출력 교체·복구 규칙](01-architecture.md)을 참고한다.

| 확인한 요구사항 | 설계 결정 |
|---|---|
| 장시간 글쓰기·읽기, 2026년 감성의 녹색 계열 | 저채도 배경과 청록 강조색을 쓰는 라이트·다크 한 쌍. 실제 색감은 시안이다. |
| 글꼴·크기·줄 간격도 관리 | 본문과 고정폭 글꼴을 분리한다. 앱이 지원하지 않는 값은 억지로 변환하지 않는다. |
| 색을 보면서 수정하는 Rust TUI | 색상 견본과 실제 문장·코드 미리보기를 함께 제공한다. 원본 파일도 직접 편집할 수 있다. |
| 한 명령으로 생성, 수동 가져오기 | 생성기는 앱 설정에 쓰지 않는다. 사용자는 원하는 앱의 산출물만 가져간다. |

1. [원본과 생성 구조](01-architecture.md) — 무엇을 어디서 바꾸며, 변경이 어떻게 일곱 앱에 전파되는가.
2. [팔레트와 편집 경험](02-design-and-editor.md) — 색상·서체 시안, TUI 조작, 읽기 편한지 판단하는 기준.
3. [앱별 적용 계약](03-targets.md) — 가져올 파일과 설정 위치, 상속·제한 사항, 구현 완료 조건.
4. [옵션 표시와 확장 전략](04-option-strategy.md) — 공통값과 앱별 예외가 보이는 화면, 같은 샘플의 앱별 차이, 새 옵션의 도입·승격 기준.
5. [사용 언어와 라이브러리 버전 관리](05-languages-and-dependencies.md) — 구현 언어·출력 형식, Rust 의존성 도입 기준, 버전 고정·갱신 절차와 대상 앱 검증 버전.
6. [Windows Terminal 적용 방법](06-windows-terminal.md) — 기존 설정에 배색표를 추가하고, 글꼴은 선택 적용하는 방법.
7. [Typora 적용 방법](07-typora.md) — CSS 복사, 모드 선택, 실제 화면 검증 방법.
8. [Typora 옵션 체크리스트](08-typora-option-checklist.md) — 항목별 구현 상태, 실제 앱 검증 기록, Obsidian 읽기 뷰·라이브 프리뷰 연계.

**향후 사용 흐름:** TUI에서 다크 모드의 청록 견본을 바꾸고 저장한 뒤 `lbrx-theme build`로 패키지를 다시 만든다. 사용자가 가져오기 전까지 앱 화면은 변하지 않는다. `edit`와 Windows Terminal·Typora 이외의 앱별 출력은 아직 제공하지 않는다.

## 개발 환경과 다음 단계

### 테마 글꼴 확인과 설치

선택 도구 [theme-fonts.py](../scripts/theme-fonts.py)는 본문 글꼴을 `LINE Seed Sans`(EN) → `LINE Seed Sans KR` → `LINE Seed JP_TTF` 순서로 확인한 뒤, 코드용 `D2Coding ligature`를 확인합니다. 이미 인식되는 외부 설치는 그대로 두고 `sans-serif`, `monospace` 같은 대체 글꼴은 건너뜁니다. Python 3.11 이상이 필요하며 Linux에서는 시스템의 Fontconfig 도구(`fc-list`, `fc-cache`)가 필요합니다.

```powershell
py -3.11 .\scripts\theme-fonts.py status
py -3.11 .\scripts\theme-fonts.py install --dry-run
py -3.11 .\scripts\theme-fonts.py install
```

macOS·Linux에서는 각각 `python3 scripts/theme-fonts.py status`, `python3 scripts/theme-fonts.py install --dry-run`, `python3 scripts/theme-fonts.py install`로 실행합니다.

압축을 푼 원본 파일을 저장소의 `fonts\assets`에 **하위 폴더 없이** 복사합니다. 본문은 `LINESeedSans_*.ttf`(EN), `LINESeedKR-*.ttf`(KR), `LINESeedJP_TTF_*.ttf`(JP) 중 [카탈로그](../fonts/catalog.toml)에 등록된 파일을 씁니다. 코드는 `D2Coding-Ver1.4.0-20261003-all.ttc`(일반·ligature를 함께 담은 컬렉션)를 씁니다. 이미 설치된 패밀리의 파일은 준비할 필요가 없습니다. 로컬 글꼴 파일은 Git에 포함하지 않습니다.

`status`는 OS에서 설치 여부를 조회합니다. `install --dry-run`은 필요한 로컬 파일의 경로와 해시를 확인하고, `install`만 검증된 파일을 현재 사용자에게 설치합니다. 어느 명령도 다운로드하지 않습니다. 필요한 파일이 없거나 해시가 다르면 설치를 중단합니다. 관리자 권한, 자동 업데이트·제거는 지원하지 않습니다. 설치 후 이미 열린 앱은 다시 실행해야 글꼴을 인식할 수 있습니다.

| OS | 사용자 글꼴 위치 | 설치 기록 기준 위치 (`lbrx-theme/fonts` 하위) |
|---|---|---|
| Windows | `%LOCALAPPDATA%\Microsoft\Windows\Fonts` | `%LOCALAPPDATA%` |
| macOS | `~/Library/Fonts` | `~/Library/Application Support` |
| Linux | `$XDG_DATA_HOME/fonts/lbrx-theme` (기본 `~/.local/share/fonts/lbrx-theme`) | `$XDG_STATE_HOME` (기본 `~/.local/state`) |

`external`은 기존 설치, `managed`는 도구 설치, `missing`은 설치 가능, `drifted`는 설치 기록과 실제 상태가 다름, `unknown`은 자동 판정·설치 불가를 의미합니다. `drifted`, `unknown` 또는 `pending.json`이 있으면 자동 설치를 중단합니다. 중단 기록이 남았다면 실행 중인 설치가 없는지 먼저 확인하고, 기록에 나온 경로와 해시를 `state.json` 및 실제 파일과 대조합니다. 상태를 확정하기 전에는 기록이나 파일을 삭제하지 않습니다.

### Rust 개발

`rustup`으로 [고정된 Rust 도구체인](../rust-toolchain.toml)을 사용한다. Windows MSVC에서는 Visual Studio Build Tools의 C++ 빌드 도구와 Windows SDK가 필요하다. 자세한 검사 명령은 [버전 관리 기준](05-languages-and-dependencies.md)을 참고한다.

Windows Terminal의 첫 가져오기를 확인할 때는 라이트·다크 배색표, 전경·배경·커서·선택 영역과 글꼴 조각을 각각 확인한다. 확인한 버전은 [버전 관리 기준](05-languages-and-dependencies.md)에 기록한다.
