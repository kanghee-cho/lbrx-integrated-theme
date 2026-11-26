# LBRX 통합 테마

스타일별 TOML 원본에서 9개 앱의 라이트·다크 테마를 생성한다. 생성기는 설치된 앱이나 사용자 설정을 수정하지 않는다. V1.1 배포 파일은 5개 스타일과 9개 앱을 포함한다.

## V1.1 정식 배포

[GitHub Releases의 V1.1](https://github.com/kanghee-cho/lbrx-integrated-theme/releases/tag/V1.1)에서 `lbrx-themes-1.1.0.zip`을 내려받아 압축을 푼다. 원하는 `<style>/lbrx` 아래 앱별 `README.md`를 따라 수동 설치한다. Obsidian 안내는 `obsidian/LBRX`, Vim 안내는 `vim/colors`에 있다. VS Code만 사용할 경우 `lbrx-theme-1.1.0-<style>.vsix` 중 원하는 스타일을 내려받아 **Install from VSIX**로 설치한다. VSIX ID는 모두 동일하므로 한 스타일을 선택해 설치하고 다른 스타일 설치 시 기존 스타일을 교체한다. `SHA256SUMS.txt`는 ZIP과 5개 VSIX의 SHA-256 확인용이다.

V1.1에는 Green, Lavender Mint, Sky Rose, Rose Slate, Sand Petrol이 포함된다. Typora 사이드바·YAML 속성의 모드별 대비와 터미널 ANSI 전경 대비를 개선하고, 선택 적용용 PowerShell 디렉터리 색 스니펫을 제공한다. 오렌지 스타일은 이번 배포에서 제외한다.

[V1.0](https://github.com/kanghee-cho/lbrx-integrated-theme/releases/tag/V1.0)은 변경하지 않고 유지한다. 이전 배포는 Studio 추가 전의 8개 앱과 단일 스타일을 포함한다.

배포 파일에는 글꼴과 앱 실행 파일이 포함되지 않는다. 글꼴은 아래 안내에 따라 별도로 준비한다. 테마 파일 생성과 실제 앱 화면 검증의 범위는 [검증 버전과 범위](05-languages-and-dependencies.md#대상-앱과-외부-도구-버전)를 따른다.

## 시작하기

`rustup`으로 [고정 Rust 도구체인](../rust-toolchain.toml)을 사용한다. Windows MSVC에서는 Visual Studio Build Tools의 C++ 빌드 도구와 Windows SDK도 필요하다. 저장소 루트에서 실행한다.

```powershell
cargo run --locked -- check
cargo run --locked -- build
cargo run --locked -- build --app obsidian --app typora
# 다른 원본을 지정할 때
cargo run --locked -- build .\themes\lavender-type\lbrx.toml
```

수정은 [공통·언어별·앱별 TOML](02-design-and-editor.md#언어별-구문색과-앱-전용-옵션)에서 한다. `check`는 파일을 생성하지 않는다. 기본 `check`와 `build`는 `themes`의 스타일 폴더를 이름순으로 처리하고, `build`는 `dist/<style>/lbrx`에 결과와 앱별 설치 안내를 기록한다.

### 여러 스타일 빌드

현재 아래 5개 스타일의 라이트·다크를 각각 생성한다. 새 스타일은 `themes/<style>/lbrx.toml`과 같은 폴더의 `editors`·`languages`에 작성한다. 스타일 간 설정은 공유하거나 섞지 않는다. ZIP 같은 일반 파일은 탐색에서 제외하며, 원본이 없는 하위 폴더와 링크 경로는 오류로 알린다.

| 스타일 이름 | 원본 폴더 | 배색 |
|---|---|---|
| Green | [green-type](../themes/green-type) | 기존 그린 |
| Lavender Mint | [lavender-type](../themes/lavender-type) | 라벤더·민트 |
| Sky Rose | [sky-type](../themes/sky-type) | 스카이 블루·로즈 |
| Rose Slate | [rose-type](../themes/rose-type) | 로즈·슬레이트 블루 |
| Sand Petrol | [sand-type](../themes/sand-type) | 샌드·페트롤 블루 |

스타일 이름은 원본 주석과 이 목록에서 구분한다. 출력 위치는 `dist/<style>/lbrx`이며 앱의 설치 이름은 아래 설명처럼 기존 계약을 유지한다. 새 4종은 그린 스타일의 글꼴·조판·언어·앱 설정과 사이드바·YAML 대비 개선을 유지하면서 공통 팔레트와 문서 강조색을 변경한다. 터미널 의미색은 UI 강조와 분리한다. 오렌지 스타일은 원본에서 제거했으며, 기존 생성 출력은 자동 삭제하지 않는다.

```powershell
# 모든 스타일·모든 앱
cargo run --locked -- build
# 한 스타일·한 앱 (파일 경로도 같은 출력 위치를 사용)
cargo run --locked -- build .\themes\green-type --app typora
cargo run --locked -- build .\themes\lavender-type\lbrx.toml --app windows-terminal
```

모든 선택 원본의 검사가 완료된 후 출력한다. 출력 교체는 **스타일별** 트랜잭션이며, 전체 스타일에 대한 일괄 원자적 교체는 아니다. 선택하지 않은 앱·스타일과 이전 `dist/lbrx`는 유지한다. 일반 사용자 지정 TOML 파일은 기존처럼 `dist/lbrx`로 출력한다.

설치 이름(`LBRX Light/Dark`, VSIX ID 등)은 스타일 간 동일하다. 각 스타일은 설치 시 선택하는 대안이며 같은 앱에 별도 이름으로 동시 설치되는 테마가 아니다. 앱별 가이드의 `lavender-type` 경로는 예시이므로 선택한 스타일로 바꿔 사용한다.

## 앱별 문서

| 앱 | 적용 가이드 | 옵션 대장 | 언어별 구문 강조 |
|---|---|---|---|
| Windows Terminal | [설치·복구](06-windows-terminal.md) | — | — |
| macOS Terminal.app | [설치·복구](14-macos-terminal.md) | [프로파일](15-macos-terminal-options.md) | — |
| iTerm2 | [설치·복구](20-iterm2.md) | [프리셋](21-iterm2-options.md) | — |
| Typora | [설치·복구](07-typora.md) | [문서·전용 기능](08-typora-options.md) | — |
| Obsidian | [설치·복구](09-obsidian.md) | [문서·전용 기능·인쇄](10-obsidian-options.md) | [코드 펜스](24-obsidian-language-highlighting.md) |
| Vim | [설치·복구](12-vim.md) | [하이라이트 그룹](13-vim-options.md) | [언어별 그룹](23-vim-language-highlighting.md) |
| VS Code | [설치·복구](18-vscode.md) | [작업 화면 색](19-vscode-options.md) | [언어별 토큰·HTML CSS](22-vscode-language-highlighting.md) |
| Helix | [설치·복구·설정 내보내기](16-helix.md) | [테마 스코프](17-helix-options.md) | — |
| LBRX Studio | [출력·적용·결정사항](25-lbrx-studio.md) | [전용 TOML](../themes/lavender-type/editors/lbrx-studio.toml) | [HTML 토큰 계약](25-lbrx-studio.md#공통언어별-구문-토큰) |

옵션 대장에서 `생성`은 빌드 결과를, `선택 생성`은 설정했을 때만 출력하는 항목을 뜻한다. 둘 다 실제 앱에서 표시가 확인됐다는 뜻은 아니다. [검증 버전과 범위](05-languages-and-dependencies.md#대상-앱과-외부-도구-버전)를 별도로 관리한다.

## 개발 문서

- [구조·출력 교체·옵션 확장](01-architecture.md)
- [팔레트·토큰·설정 우선순위](02-design-and-editor.md)
- [앱별 산출물과 수동 적용 경계](03-targets.md)
- [Rust·라이브러리·대상 앱 버전](05-languages-and-dependencies.md)
- [공통 역할과 문서 기능 ID](11-cross-app-options.md)

## 테마 글꼴 확인과 설치

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
