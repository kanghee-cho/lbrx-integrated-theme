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

`build`는 저장한 원본을 검증한 뒤 기본적으로 지원하는 모든 앱의 테마를 생성한다. `--app`으로 필요한 앱만 선택할 수 있고, 다른 원본은 `build` 뒤에 파일 경로를 전달한다. 빌드 결과에 표시된 앱별 `README.md`에 따라 수동으로 설치하고 실제 앱에서 확인한다.

파일을 생성하지 않고 검사만 하려면 `check`를 사용한다. 공통 대비 기준은 본문 7:1, 나머지 문서·구문색과 선택 조합 4.5:1, 포커스 3:1이며 임의의 ANSI 조합이나 호스트 설정까지 보장하지 않는다.

## 문서 속 코드 블록 배경

Typora·Obsidian 본문에서 드래그한 텍스트는 `prose_selection_background`를 사용한다(라이트 `#F3D3AB`, 다크 `#60402A`). 코드에서 드래그한 텍스트는 기존 `selection_background`(라이트 `#D8E6DA`, 다크 `#304138`)를 유지한다. 선택 글자색은 두 경우 모두 `selection_text`다. Obsidian에서 CodeMirror가 자체 레이어로 그리는 선택은 기존 앱 배색을 유지하며, 커서만 놓인 줄의 강조색은 변경하지 않는다.

`light.roles.code_block_background`와 `dark.roles.code_block_background`는 Typora와 Obsidian에서 여러 줄 코드 블록의 배경만 정한다. 기본은 라이트 `#E8EDE4`, 다크 `#25332C`이며, 기존 원본에서 이 역할이 없으면 문서 배경을 사용한다. 인라인 코드와 콜아웃 자체의 스타일, VS Code·Vim·Helix의 편집기 배경은 바뀌지 않는다. `check`는 지정된 코드 블록 배경과 기본 코드 글자·공통 및 언어별 구문색의 대비가 4.5:1 이상인지 검사한다.

## 언어별 구문색과 앱 전용 옵션

| 바꾸려는 대상 | 편집할 파일 |
|---|---|
| 전체 팔레트·역할·ANSI·서체 | [lbrx.toml](../themes/lbrx.toml) |
| 모든 언어의 구문색 | [languages/common.toml](../themes/languages/common.toml)의 `[tokens]` |
| 특정 언어의 구문색·스타일 | [languages/](../themes/languages/)의 `<언어 ID>.toml`. 공통 설정만 사용하는 언어도 파일을 둘 수 있다. |
| 특정 앱에서만 다르게 보일 색 | [editors/](../themes/editors/)의 `<앱 ID>.toml`. 각 파일의 `[colors]`에 지원 키와 기본 역할이 있다. |

`[tokens]` 값은 **팔레트 색 이름이나 HEX가 아닌 역할 이름**이다. 공통 16종 외에도 기존 `keyword`(세분화되지 않은 키워드)와 `error`를 지원한다. 같은 키는 **개별 언어 → 공통 언어 → 기본 역할** 순서로 찾는다. 세부 토큰 설정이 없으면 상위 토큰 또는 기본 전경색을 사용한다. `[styles]`에는 선택적으로 `normal`, `bold`, `italic`을 지정한다. 점이 들어간 토큰 키는 `"keyword.control"`처럼 따옴표로 감싼다. 예를 들어 [python.toml](../themes/languages/python.toml)에 다음을 지정하면 지원하는 에디터에서 Python 함수만 `accent`를 사용한다.

```toml
[tokens]
function = "accent"
```

앱 파일은 위 구문색을 전역으로 덮어쓰지 않는다. 예를 들어 [vscode.toml](../themes/editors/vscode.toml)의 `[colors]` 아래 `"sideBar.background" = "background"`는 VS Code 사이드바만 바꾼다. Vim의 `Comment`처럼 구문 그룹을 직접 설정한 경우에도 명시된 언어 전용 하위 그룹이 우선한다. 키를 삭제하면 어댑터 기본값으로 돌아간다.

| 대상 | 언어 예외가 출력되는 범위 |
|---|---|
| [VS Code](22-vscode-language-highlighting.md) | 알려진 TextMate 문법 루트와 일부 semantic 선택자. 동반 HTML CSS는 `.tok-<토큰>` 마크업을 제공하는 렌더러에서 사용하며, 토큰 이름의 점은 하이픈으로 바꾼다. |
| [Vim](23-vim-language-highlighting.md) | Vim 문법이 제공하는 명명된 그룹만 적용. 예: `pythonFunction`, `rustKeyword`, `ps1Cmdlet`, `tomlInteger`, `typescriptNumber` |
| Typora · [Obsidian](24-obsidian-language-highlighting.md) | Typora는 `lang`이 있는 코드 펜스, Obsidian은 읽기 뷰의 `code.language-<id>` 내부 Prism 토큰만 적용. Obsidian 라이브 프리뷰에 읽기 뷰 규칙을 적용하지 않는다. |
| Helix | 단일 색 테마는 언어 ID를 구분하지 않으므로 공통 구문색만 적용 |
| Windows Terminal · iTerm2 | 언어를 분석하지 않는다. 터미널 자체 배색에는 각 앱의 전용 파일만 적용 |

`check`와 `build`는 지정한 원본 **파일 옆**의 `languages`와 `editors` 디렉터리에서 `.toml` 파일을 읽는다. 다른 이름의 원본이라도 같은 디렉터리에 있으면 설정을 공유한다. 두 디렉터리 없이 기존 단일 파일만 사용해도 된다. 언어 ID는 소문자 영문으로 시작하고 소문자·숫자·하이픈을 허용한다. 새 언어 파일을 추가해도 해당 앱의 문법 또는 HTML 렌더러가 실제로 제공하는 토큰만 색칠된다.

## 구문 강조 공통 토큰 16종 설계

아래 기준은 [공통 TOML](../themes/languages/common.toml)과 [팔레트](../themes/lbrx.toml)에 반영된다. 색상은 라이트·다크 각각의 기본 배경과 여러 줄 코드 블록 배경에 대해 WCAG sRGB 상대 휘도 대비로 계산했다. 표의 두 수치는 **편집기 / 코드 블록** 순이며 단위는 `:1`이다. 기준 배경은 라이트 `#F3F4EC` / `#E8EDE4`, 다크 `#19231F` / `#25332C`다.

| 공통 토큰 | 라이트 HEX · 대비 | 다크 HEX · 대비 | 스타일 | 색상 선정 이유 |
|---|---|---|---|---|
| `keyword.control` | `#80516A` · 5.74 / 5.35 | `#D4A8BD` · 7.79 / 6.38 | Bold | 자주색과 굵기로 분기·반복·반환 등 실행 흐름의 전환점을 먼저 찾게 한다. |
| `keyword.declaration` | `#654878` · 6.89 / 6.42 | `#BFA9DF` · 7.65 / 6.27 | Bold | 보라색으로 제어 흐름과 구별하고, 굵기로 선언·접근 제어의 시작점을 표시한다. |
| `string` | `#246958` · 5.86 / 5.46 | `#84C7AC` · 8.26 / 6.76 | Normal | 청록색으로 리터럴 범위를 묶되 긴 문자열이 코드 구조보다 튀지 않게 한다. |
| `string.escape` | `#8A4D27` · 5.98 / 5.57 | `#F3B184` · 8.78 / 7.20 | Bold | 따뜻한 주황과 굵기로 이스케이프·보간 구문·정규식을 일반 문자열에서 분리한다. |
| `number` | `#815919` · 5.62 / 5.23 | `#D6B378` · 8.13 / 6.66 | Normal | 황토색을 수치 데이터의 기준색으로 삼아 문자열과 빠르게 구별한다. |
| `constant.builtin` | `#794C3B` · 6.52 / 6.07 | `#E7AA96` · 8.14 / 6.67 | Bold | 적갈색과 굵기로 불리언·널 등 언어가 제공하는 상수를 숫자와 구별한다. |
| `function` | `#43617B` · 5.86 / 5.46 | `#97BBD6` · 7.99 / 6.54 | Normal | 남회색으로 함수·메서드의 이름을 변수와 구별하면서 반복 호출은 과도하게 강조하지 않는다. |
| `function.macro` | `#6F497F` · 6.46 / 6.02 | `#CDB0EB` · 8.46 / 6.93 | Bold | 선명한 보라색과 굵기로 매크로·특수 내장 호출을 일반 호출에서 분리한다. |
| `type` | `#365F55` · 6.48 / 6.04 | `#91C6BC` · 8.46 / 6.93 | Bold | 녹청색과 굵기로 자료형·클래스·인터페이스를 구조를 읽는 기준점으로 삼는다. |
| `variable` | `#24362F` · 11.54 / 10.75 | `#DCE6DB` · 12.59 / 10.31 | Normal | 가장 자주 나오는 식별자는 기본 전경색에 둬 전체 코드의 가독성을 유지한다. |
| `variable.parameter` | `#5B567F` · 6.15 / 5.73 | `#BFB9EC` · 8.73 / 7.15 | Italic | 회보라와 기울임으로 함수의 입력을 일반 변수와 구별한다. |
| `property` | `#3D6070` · 6.11 / 5.69 | `#9FC7DB` · 8.96 / 7.34 | Normal | 푸른 회색으로 필드·키와 값의 경계를 보여 주되, 반복되는 키를 굵게 만들지 않는다. |
| `operator` | `#536057` · 5.97 / 5.56 | `#AABDB1` · 8.16 / 6.69 | Normal | 녹회색으로 연산 관계를 읽을 수 있게 하면서 피연산자보다 튀지 않게 한다. |
| `attribute` | `#78602D` · 5.40 / 5.03 | `#DCC28D` · 9.33 / 7.64 | Italic | 금갈색과 기울임으로 데코레이터·어노테이션·HTML 속성명을 메타정보로 구별한다. |
| `comment` | `#52655B` · 5.62 / 5.24 | `#A3B5A9` · 7.48 / 6.13 | Italic | 저채도 녹회색과 기울임으로 설명을 실행 코드에서 물러나게 하되 읽을 대비는 유지한다. |
| `punctuation` | `#596B61` · 5.12 / 4.77 | `#99ADA1` · 6.80 / 5.57 | Normal | 구분 기호는 가장 낮은 대비로 두되 괄호와 구문 경계가 흐려지지 않게 한다. |

### 렌더러 매핑 원칙

- **VS Code·Helix:** 실제로 구분되는 TextMate/semantic scope 또는 Tree-sitter 캡처에 세부 토큰을 연결한다. VS Code의 semantic 분류가 없으면 TextMate 분류를 사용한다. 같은 글자에 둘 다 적용될 때 설정이 충돌하지 않도록 매핑한다.
- **Typora·Obsidian·Vim:** Typora CodeMirror의 토큰 클래스, Obsidian 읽기 뷰 Prism 토큰, Vim의 문법 그룹에 각각 연결한다. 문법이 `keyword.control`과 `keyword.declaration`을 한 그룹으로만 제공한다면 기존 `keyword` 역할로 합친다. `string.escape` → `string`, `function.macro` → `function`, `variable.parameter` → `variable`도 같은 원칙이다. 별도 분류가 없는 연산자·속성·구분 기호 등은 기본 전경색을 유지한다. 보간 내부의 식별자는 렌더러가 구분할 경우 `variable` 등 원래 역할을 유지한다.
- **글꼴 스타일:** Normal은 별도 강조 없음, Bold는 굵게, Italic은 기울임이다. 스타일을 지원하지 않는 환경은 색만 적용한다. 분류가 없는 토큰을 문자열 검색으로 추정하거나 일반 본문까지 재분류하지 않는다.
- **터미널:** Windows Terminal과 iTerm2는 ANSI 팔레트만 적용하며 이 토큰을 직접 분류하지 않는다. `error`는 기존 오류 토큰을 유지하되 공통 구문 16종에는 포함하지 않는다.

빌드한 에디터별 설치 안내의 **구문 강조 설정 목록**은 입력 TOML과 적용 경로를 표시한다. 파일이 있다는 사실이 호스트의 문법 지원을 보장하지는 않는다. VS Code의 Bash는 `shellscript` 문법을 공유하므로 `shell.toml`을 적용하고, Bash 코드 펜스는 별도로 `bash.toml`을 사용한다. MD는 `markdown.toml`, C#은 `csharp.toml`을 사용한다. 일반 텍스트는 전경색만 사용한다.

[2025 Stack Overflow 개발자 설문](https://survey.stackoverflow.co/2025/technology#most-popular-technologies-language)을 참고해 PHP, Kotlin, Lua, Ruby, Dart, Swift, R을 추가하고, 수치 계산·시스템·인프라·API 용도로 Julia, Zig, HCL, GraphQL도 추가했다. TOML에서 색과 스타일을 설정할 수 있지만, Kotlin·Zig·HCL·GraphQL처럼 VS Code 설치본에 문법이 없는 경우 별도 문법 확장이 필요하다. 확장마다 scope가 다르면 공통 scope만 적용된다.

### Rust 구문 매핑 기준

Rust는 새 색 역할을 추가하지 않고 공통 팔레트를 재사용한다. 아래는 렌더러별 매핑을 구현할 때 적용할 기준이다.

| Rust 구문 | 적용할 역할과 경계 |
|---|---|
| `println!`, `vec!`, `format!` | 이름과 후행 `!`를 함께 `function.macro`로 표시한다. `std::println!`에서는 최종 이름 `println!`만 해당한다. 인수 괄호에는 `punctuation`, `!ready`의 부정 및 `-> !`의 never 타입에는 각각 `operator`, `type`을 적용한다. 렌더러가 매크로의 `!`를 문맥상 식별하지 못하면 식별된 이름만 매핑하고, 모든 `!`를 일괄 변경하지 않는다. |
| `'a`, `'static`, `'loop_label` | 선행 `'`까지 포함해 `attribute`를 적용한다. 라벨 선언 `'loop_label:`과 `break 'loop_label`의 참조는 같은 색을 쓰고, 선언 뒤 `:`는 포함하지 않는다. 문자 리터럴 `'c'`, 이스케이프가 있는 `'\n'`, 바이트 문자 `b'c'`는 라이프타임이나 라벨이 아니다. 문법이 이를 구분하지 못하면 기존 분류를 유지하며 `'`만으로 판정하지 않는다. |
| `#[derive(Debug)]`, `#![allow(...)]` | `#`, 내부 어트리뷰트의 `!`, 대괄호와 어트리뷰트 이름은 `attribute`를 적용한다. `derive(Debug)`의 `Debug`는 **렌더러가 트레잇으로 식별할 때만** `type`으로 중첩 적용한다. `allow(dead_code)`의 `dead_code`나 임의의 대문자 인자를 타입으로 추정하지 않는다. 전체가 하나의 토큰이라면 블록 전체에 `attribute`를 적용한다. |
| `std::fmt`, `&mut value` | `::`는 경로 구조를 드러내는 `punctuation`으로 통일한다. 참조의 `&`는 `operator`, 가변성을 나타내는 `mut`는 `keyword.declaration`이다. `&mut`가 단일 토큰으로만 제공되면 가변성이 묻히지 않도록 `keyword.declaration`을 적용한다. |

검증할 때는 `println!`과 `!ready`, `'a`와 `'c'`, `#[derive(Debug)]`와 `#![allow(dead_code)]`, `std::fmt`와 `&mut value`를 같은 Rust 샘플에서 비교한다. 매크로 이름과 `!`가 서로 다른 토큰으로 나오더라도 매크로 문맥을 확인할 수 있으면 같은 색과 스타일을 적용한다.

### Common Lisp 구문 매핑 기준

Common Lisp는 공통 팔레트를 재사용한다. 다른 Lisp 방언에는 해당 문법이 같은 의미로 분류하는 구문에만 이 기준을 적용한다.

| Lisp 구문 | 적용할 역할과 경계 |
|---|---|
| 괄호와 호출 위치 | 기본은 괄호를 `punctuation`에 두고, **호출로 식별된 목록의 첫 심볼**을 강조한다. 일반 호출은 `function` 색에 Bold를 적용하고, 매크로·특수 형식은 각각 `function.macro`와 해당 키워드 역할을 유지한다. 인용된 목록, 백쿼트 템플릿의 데이터 부분, 인자·바인딩 목록에는 위치만 보고 함수색을 칠하지 않는다. 언쿼트 안에서 실제 호출로 분류된 형식에는 일반 호출 규칙을 적용한다. |
| 선택적 3색 괄호 | 렌더러가 짝과 깊이를 구분할 때만 선택적으로 켠다. 바깥쪽부터 `function` → `keyword.control` → `string` 색을 순환하며, 여는 괄호와 닫는 괄호는 같은 색·Normal로 표시한다. 깊이 정보가 없으면 기본 `punctuation`을 유지하고 문자열·주석 속 괄호는 순환에 포함하지 않는다. |
| 쿼트 `'`, 백쿼트 `` ` ``, 언쿼트 `,`, 스플라이싱 `,@` | 리더가 데이터와 평가할 식을 구분하는 기호이므로 `keyword.control`의 색과 Bold를 적용한다. `,@`는 한 단위로 취급한다. 문자열·주석 내부의 같은 문자는 재분류하지 않는다. |
| 키워드 심볼 `:name`, `:key` | 콜론까지 포함해 `constant.builtin`으로 표시한다. 자기 평가 심볼이라는 의미를 우선하며, 키워드 인자 위치에서도 `property`로 바꾸지 않는다. `pkg:name`의 패키지 구분 콜론은 이 규칙에 해당하지 않는다. |
| `*global-var*`, `+constant+` | 앞뒤 기호까지 하나의 심볼로 취급한다. 선언 정보가 없으면 `variable`을 유지하고, 이름만 보고 동적 바인딩이나 상수 여부를 추정하지 않는다. `(* x 2)`나 `(+ x 1)`에서 독립 심볼로 분류된 산술 함수만 `operator`로 표시한다. |

검증 샘플에서는 `(list :name *global-var* +constant+)`와 `'(+ x 1)`을 비교하고, 백쿼트 안의 `,(+ x 1)`과 `,@items`를 함께 확인한다. 렌더러가 호출과 데이터를 구별하지 못하면 첫 심볼을 임의로 강조하지 않는다. 리더 구문 참고: [Common Lisp HyperSpec — Backquote](https://www.lispworks.com/documentation/HyperSpec/Body/02_df.htm).

### PowerShell 구문 매핑 기준

| 구문 | 적용할 역할과 경계 |
|---|---|
| `$var`, `$env:PATH`, `$_` | `$`와 범위·드라이브 접두사를 포함한 전체 변수 참조를 `variable`로 표시한다. 렌더러가 자동 변수로 구분하는 `$_`, `$true`, `$null` 등은 `constant.builtin`을 적용한다. 이는 강조 분류이며 모든 자동 변수가 불변이라는 뜻은 아니다. |
| `Get-ChildItem`, `Set-ExecutionPolicy` | 명령으로 파싱된 이름 전체에 `function`을 적용한다. 이름 안의 `-`는 떼어 내지 않고, `$a - $b`의 뺄셈과 `-eq` 같은 비교 연산자는 `operator`로 유지한다. |
| `-Path`, `-Force`, `-Recurse` | 명령 인자 위치에서 파라미터로 파싱되는 이름은 `-`까지 `variable.parameter`로 표시한다. `-Path:$dir`의 값 `$dir`은 별도로 `variable`을 적용한다. |
| `"Hello $name"`, `"Size: $(1 + 2)"` | 큰따옴표 문자열의 `$name`은 변수 역할을 적용하고, `$(`와 닫는 `)`는 `string.escape`로 표시한다. 하위 식에는 일반 코드 규칙을 적용한다. `'Hello $name'`의 `$name`은 보간하지 않고 `string`으로 유지한다. |

### Python 구문 매핑 기준

| 구문 | 적용할 역할과 경계 |
|---|---|
| `self`, `cls` | 메서드의 매개변수로 식별되면 `variable.parameter`와 Italic을 적용한다. 예약어는 아니므로 일반 변수나 속성으로 쓰인 같은 이름을 덮어쓰지 않는다. |
| `@dataclass`, `@app.route(...)` | 데코레이터로 파싱된 `@`부터 점으로 연결된 이름 끝까지 `attribute`를 적용한다. 호출 괄호와 인자는 일반 코드로 두며, 행렬 곱셈의 `@`는 `operator`다. |
| `f"val: {x}"`, `r"\d+"`, `b"bytes"` | 문자열로 파싱된 `f`, `r`, `b`와 유효한 조합 접두사는 `string.escape`다. f-string의 치환 경계 `{`·`}`는 `punctuation`, 내부 식은 일반 코드로 표시한다. `{{`·`}}`는 치환 경계가 아닌 리터럴 이스케이프로, raw 문자열의 백슬래시는 일반 이스케이프로 재해석하지 않는다. |
| `__init__`, `__str__` | 알려진 특수 메서드로 식별되는 이름은 시각적 강조 역할인 `function.macro`를 재사용한다. 임의의 `__name__`이나 일반 함수는 이름 모양만 보고 특수 메서드로 올리지 않는다. |

### TypeScript·TSX 구문 매핑 기준

| 구문 | 적용할 역할과 경계 |
|---|---|
| 타입 공간과 값 공간 | `interface`·`type` 선언 키워드, 선언된 타입 이름, 제네릭 타입 매개변수 `T`, 주석 타입 `: string`의 `string`에는 `type`을 적용한다. `<`·`>`와 `:` 같은 구분자는 `punctuation`으로 두며, 같은 철자가 런타임 값일 때는 값 문맥의 역할을 유지한다. |
| `Partial<T>`, `Record<K, V>`, `any`, `unknown` | 유틸리티 타입, 원시 타입, 사용자 정의 타입을 동일한 `type` 색과 스타일로 표시한다. `number` 타입 이름을 숫자 리터럴로 분류하지 않는다. |
| `<Component prop={val} />`, `<div />` | TSX에서 값 기반 컴포넌트 태그는 `function`, 소문자 내장 태그는 `keyword.declaration`으로 분리한다. 속성명은 `attribute`, `{val}` 내부는 일반 표현식이다. `prop="{val}"`처럼 인용된 속성 값은 보간이 아닌 `string`이다. |
| `` `id: ${user.id}` `` | 백틱으로 둘러싼 리터럴 내용은 `string`, 치환 경계 `${`와 `}`는 `string.escape`로 표시한다. 내부 `user`와 `id`는 실제 분류에 따라 `variable`과 `property`를 사용한다. |

### 데이터 직렬화·설정 구문 매핑 기준

| 형식·구문 | 적용할 역할과 경계 |
|---|---|
| **JSON — 키와 값** | 객체 키 `"name"`은 따옴표까지 `property`, 문자열 값 `"name"`은 `string`이다. 키 여부는 객체 문맥으로 판별하며, 값 안의 콜론은 무시한다. 숫자는 `number`, `true`·`false`·`null`은 `constant.builtin`이다. |
| **JSONC — 주석** | JSON과 같은 키·값 규칙을 쓰고, JSONC 문법에서 인식한 `//`와 `/* … */`는 `comment`로 표시한다. `"https://example.com"` 안의 `//`는 문자열이며 순수 JSON에 주석 문법을 추가하지 않는다. |
| **YAML — 앵커·별칭·병합** | `&anchor`, `*alias` 전체는 `attribute`, 병합 키로 식별된 `<<`는 `keyword.control`, 그 뒤 콜론은 `punctuation`이다. 병합은 모든 YAML 1.2 환경에서 보장되지 않으므로 렌더러가 일반 키로 분류하면 `property`를 유지한다. |
| **YAML — 블록 스칼라** | `\|`, `>`, `>-` 등 블록 헤더는 `keyword.declaration`이다. 블록 내용은 들여쓰기 경계까지 `string`이며, 내부의 콜론을 키로 해석하지 않는다. |
| **YAML — 암시 타입과 문서 경계** | YAML 1.2 Core 기준에서 `true`·`false`·`null`은 `constant.builtin`, 숫자는 `number`다. `yes`·`no`·`on`·`off`·`2026-11-23`은 기본적으로 `string`으로 두고, YAML 1.1 등 사용 스키마가 불리언 또는 날짜로 확인할 때만 `constant.builtin`을 적용한다. 문서 시작·종료 마커 `---`·`...`는 `punctuation`이다. |
| **TOML — 테이블 헤더와 점 경로** | `[package]`·`[[bin]]`은 대괄호와 테이블 이름을 함께 `type`(Bold)으로 표시한다. `dependencies.serde.version` 같은 할당 키의 각 구성요소는 `property`, 구성요소를 잇는 점은 `punctuation`이다. 인용된 하나의 키 내부에 있는 점은 경로 구분자가 아니다. |
| **TOML — 날짜·시간과 다중 줄 문자열** | 날짜·시간 리터럴 `1979-05-27T07:32:00Z`는 `constant.builtin`으로 표시한다. 세 작은따옴표로 둘러싼 리터럴 문자열은 `string`을 유지하고, 세 큰따옴표로 둘러싼 기본 문자열은 실제 이스케이프(`\n` 등)에만 `string.escape`를 적용한다. |

매핑 검증에서는 같은 철자가 다른 역할을 갖는 사례를 한 샘플에 둔다. 예를 들어 PowerShell 명령의 `-Path`와 식의 `-eq`, Python 데코레이터와 행렬 곱셈, TSX의 인용 속성과 식 속성, JSON의 `"name": "name"`, YAML 블록 안의 `key: value`, TOML의 날짜 리터럴과 인용된 날짜를 비교한다. 렌더러가 이 경계를 제공하지 않으면 기존 상위 분류를 유지하고 문자열 모양만으로 재분류하지 않는다. 참고: [YAML 버전별 변경 사항](https://yaml.org/spec/1.2.2/ext/changes/) · [TypeScript JSX](https://www.typescriptlang.org/docs/handbook/jsx.html).
