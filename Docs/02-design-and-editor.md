# 팔레트와 앱별 미리보기

청록 계열의 라이트·다크 한 쌍을 사용한다. 아래 값은 프로젝트 시안이며, 장문은 중립색으로 두고 링크·포커스 등 시선이 필요한 곳에 강조색을 쓴다.

| 팔레트 키 | 라이트 | 다크 | 대표 용도 |
|---|---|---|---|
| `paper` | `#F3F4EC` | `#19231F` | 배경 |
| `ink` | `#24362F` | `#DCE6DB` | 본문 |
| `muted` | `#52655B` | `#A3B5A9` | 주석·인용 |
| `pine` | `#246958` | `#84C7AC` | 강조·링크·포커스·문자열 |
| `slate` | `#43617B` | `#97BBD6` | 함수 |
| `plum` | `#80516A` | `#D4A8BD` | 키워드 |
| `ochre` | `#815919` | `#D6B378` | 숫자·경고 |
| `clay` | `#A44540` | `#E59E91` | 오류 |

본문 서체는 LINE Seed EN → KR → JP → 시스템 sans-serif, 코드는 D2Coding ligature → 시스템 monospace 순서로 시작한다. 글꼴 폴백은 지원 문자를 찾는 순서이지 자동 언어 판별이 아니다.

## 원본 파일에서 편집

`themes` 아래에 TOML을 추가하거나 수정할 때는 파일이 다루는 범위를 상단에 적고, 모든 설정값과 비활성 예시에 용도·단위·참조 대상을 해당하는 만큼 한국어 주석으로 설명한다.

[themes/lbrx.toml](../themes/lbrx.toml)을 원하는 편집기로 열어 수정한다. 공통 색 자체를 바꾸려면 모드별 `palette`의 HEX를 수정한다. 같은 색을 쓰는 역할 중 하나만 바꾸려면 해당 모드의 `roles`에서 참조를 바꾼다. 예를 들어 다크 `accent`만 변경하려면 `dark.roles.accent`를 다른 키에 연결한다. 새 팔레트 키를 만들 때는 양쪽 모드에 모두 추가해야 하지만 반대 모드의 역할 연결을 바꿀 필요는 없다. 글꼴은 `typography.prose`와 `typography.mono`에서 관리한다.

## TUI는 읽기 전용 미리보기

```powershell
cargo run --locked -- preview
cargo run --locked -- preview --app typora --app obsidian
```

기본은 현재 지원하는 일곱 앱이며, `--app`을 반복하면 그 앱만 미리보기 탭에 표시한다. 테마 원본을 바꾸려면 `preview` 뒤에 파일 경로를 전달한다. True Color 터미널(최소 50열 × 19행)을 권장한다.

조작 키 안내는 화면 맨 아래에 표시한다. 선택한 앱 탭은 미리보기 모드와 관계없이 항상 라이트 색으로 표시한다. 탭 아래, 미리보기 제목 아래, 조작 키 안내 위에 가로 구분선을 표시한다. 미리보기 제목은 글자색을 유지하고, 배경만 현재 모드의 보조 배경색(`surface`)으로 채운다.

| 키 | 동작 |
|---|---|
| `Tab` / `Shift+Tab` | 앱 탭 이동 |
| `M` | 라이트·다크 전환 |
| `L` | Python → Rust → PowerShell → TOML → TypeScript 견본을 순환. 앱·모드를 바꿔도 선택을 유지하며 터미널 앱에서는 숨김 |
| `I` | 색상 견본과 적용 근거를 전환 |
| `R` | 외부 편집기에서 저장한 원본 다시 읽기. 오류가 있으면 이유를 표시하고 마지막 정상 화면을 유지한다. |
| `Esc` / `Q` | 종료 |

Windows Terminal과 iTerm2는 프롬프트·커서·ANSI 16색을 보여 주며, iTerm2 견본에는 선택 글자색도 표시한다. Typora는 문서 제목·링크·코드, Obsidian은 내부 링크·기본 강조색·코드를 보여 준다. 빈 공간을 포함해 샘플 영역 전체에 해당 앱·모드의 배경색을 사용한다. 글꼴 값은 참고 정보이며 TUI 호스트 글꼴을 바꾸지 않는다. 이 화면은 색상 샘플이며 앱의 렌더링·사용자 설정을 재현하거나 자동으로 바꾸지 않는다.

적용 근거에는 견본에 나온 토큰과 앱 전용 색상의 최종 HEX, 설정 파일명, 역할을 표시한다. HEX가 같아도 언어 예외와 공통값의 출처를 구분한다. Obsidian 언어 견본은 읽기 뷰, Helix는 공통 구문색 기준이다. 언어 예외 개수는 현재 견본의 토큰만 집계하며, 앱에 매핑이 없는 예외는 미반영으로 표시한다.

`preview`는 원본이나 배포 파일을 작성하지 않는다. `check`로 원본을 검사하고, `build --app …`으로 필요한 앱만 생성한 후 수동으로 적용한다. 공통 대비 기준은 본문 7:1, 나머지 문서·구문색과 선택 조합 4.5:1, 포커스 3:1을 기준으로 하며 임의의 ANSI 색 조합이나 호스트 사용자 설정까지 보장하지 않는다.

## 문서 속 코드 블록 배경

`light.roles.code_block_background`와 `dark.roles.code_block_background`는 Typora와 Obsidian에서 여러 줄 코드 블록의 배경만 정한다. 기본은 라이트 `#E8EDE4`, 다크 `#25332C`이며, 기존 원본에서 이 역할이 없으면 문서 배경을 사용한다. 인라인 코드와 콜아웃 자체의 스타일, VS Code·Vim·Helix의 편집기 배경은 바뀌지 않는다. TUI에서는 Typora와 Obsidian의 코드 견본에만 이 배경을 표시한다. `check`는 지정된 코드 블록 배경과 기본 코드 글자·공통 및 언어별 구문색의 대비가 4.5:1 이상인지 검사한다.

## 언어별 구문색과 앱 전용 옵션

| 바꾸려는 대상 | 편집할 파일 |
|---|---|
| 전체 팔레트·역할·ANSI·서체 | [lbrx.toml](../themes/lbrx.toml) |
| 모든 언어의 구문색 | [languages/common.toml](../themes/languages/common.toml)의 `[tokens]` |
| 특정 언어의 구문색 | [languages/](../themes/languages/)의 `<언어 ID>.toml`. 기본 제공: `python`, `rust`, `powershell`, `toml`, `typescript` |
| 특정 앱에서만 다르게 보일 색 | [editors/](../themes/editors/)의 `<앱 ID>.toml`. 각 파일의 `[colors]`에 지원 키와 기본 역할이 있다. |

언어 파일의 왼쪽은 `comment`, `keyword`, `string`, `function`, `number`, `error` 중 하나, 오른쪽은 **팔레트 색 이름이나 HEX가 아닌 역할 이름**이다. 같은 키는 **개별 언어 → 공통 언어 → 같은 이름의 기본 역할** 순서로 찾고, 선택한 역할의 라이트·다크 값은 각각 원본에서 읽는다. 예를 들어 [python.toml](../themes/languages/python.toml)에 다음 한 줄을 활성화하면 Python 함수만 `accent`를 사용한다. `string` 등 나머지 키를 복사할 필요는 없다.

```toml
[tokens]
function = "accent"
```

앱 파일은 위 구문색을 전역으로 덮어쓰지 않는다. 예를 들어 [vscode.toml](../themes/editors/vscode.toml)의 `[colors]` 아래 `"sideBar.background" = "background"`는 VS Code 사이드바만 바꾼다. Vim의 `Comment`처럼 구문 그룹을 직접 설정한 경우에도 명시된 언어 전용 하위 그룹이 우선한다. 키를 삭제하면 어댑터 기본값으로 돌아간다.

| 대상 | 언어 예외가 출력되는 범위 |
|---|---|
| [VS Code](22-vscode-language-highlighting.md) | 기본 5종 문법의 TextMate 상위 scope, 언어 ID 한정 semantic 선택자(`error` 제외), 동반 HTML CSS의 `.language-<id>` |
| [Vim](23-vim-language-highlighting.md) | Vim 문법이 제공하는 명명된 그룹만 적용. 예: `pythonFunction`, `rustKeyword`, `ps1Cmdlet`, `tomlInteger`, `typescriptNumber` |
| Typora · [Obsidian](24-obsidian-language-highlighting.md) | Typora는 `lang`이 있는 코드 펜스, Obsidian은 읽기 뷰의 `code.language-<id>` 내부 Prism 토큰만 적용. Obsidian 라이브 프리뷰에 읽기 뷰 규칙을 적용하지 않는다. |
| Helix | 단일 색 테마는 언어 ID를 구분하지 않으므로 공통 구문색만 적용 |
| Windows Terminal · iTerm2 | 언어를 분석하지 않는다. 터미널 자체 배색에는 각 앱의 전용 파일만 적용 |

`check`, `build`, `preview`(`R` 포함)는 지정한 원본 **파일 옆**의 `languages`와 `editors` 디렉터리에서 `.toml` 파일을 읽는다. 다른 이름의 원본이라도 같은 디렉터리에 있으면 설정을 공유한다. 두 디렉터리 없이 기존 단일 파일만 사용해도 된다. 언어 ID는 소문자 영문으로 시작하고 소문자·숫자·하이픈을 허용한다. 새 언어 파일을 추가해도 해당 앱의 문법 또는 HTML 렌더러가 실제로 제공하는 토큰만 색칠된다.
