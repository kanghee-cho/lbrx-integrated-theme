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

## 설정을 수정한 뒤 생성하기

```powershell
cargo run --locked -- build
cargo run --locked -- build --app typora --app obsidian
```

`build`는 저장한 원본을 검증한 뒤 기본적으로 일곱 앱의 테마를 생성한다. `--app`으로 필요한 앱만 선택할 수 있고, 다른 원본은 `build` 뒤에 파일 경로를 전달한다. 빌드 결과에 표시된 앱별 `README.md`에 따라 수동으로 설치하고 실제 앱에서 확인한다.

파일을 생성하지 않고 검사만 하려면 `check`를 사용한다. 공통 대비 기준은 본문 7:1, 나머지 문서·구문색과 선택 조합 4.5:1, 포커스 3:1이며 임의의 ANSI 조합이나 호스트 설정까지 보장하지 않는다.

## 문서 속 코드 블록 배경

Typora·Obsidian 본문에서 드래그한 텍스트는 `prose_selection_background`를 사용한다(라이트 `#F3D3AB`, 다크 `#60402A`). 코드에서 드래그한 텍스트는 기존 `selection_background`(라이트 `#D8E6DA`, 다크 `#304138`)를 유지한다. 선택 글자색은 두 경우 모두 `selection_text`다. Obsidian에서 CodeMirror가 자체 레이어로 그리는 선택은 기존 앱 배색을 유지하며, 커서만 놓인 줄의 강조색은 변경하지 않는다.

`light.roles.code_block_background`와 `dark.roles.code_block_background`는 Typora와 Obsidian에서 여러 줄 코드 블록의 배경만 정한다. 기본은 라이트 `#E8EDE4`, 다크 `#25332C`이며, 기존 원본에서 이 역할이 없으면 문서 배경을 사용한다. 인라인 코드와 콜아웃 자체의 스타일, VS Code·Vim·Helix의 편집기 배경은 바뀌지 않는다. `check`는 지정된 코드 블록 배경과 기본 코드 글자·공통 및 언어별 구문색의 대비가 4.5:1 이상인지 검사한다.

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

`check`와 `build`는 지정한 원본 **파일 옆**의 `languages`와 `editors` 디렉터리에서 `.toml` 파일을 읽는다. 다른 이름의 원본이라도 같은 디렉터리에 있으면 설정을 공유한다. 두 디렉터리 없이 기존 단일 파일만 사용해도 된다. 언어 ID는 소문자 영문으로 시작하고 소문자·숫자·하이픈을 허용한다. 새 언어 파일을 추가해도 해당 앱의 문법 또는 HTML 렌더러가 실제로 제공하는 토큰만 색칠된다.
