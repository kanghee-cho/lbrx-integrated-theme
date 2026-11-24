# Helix에 LBRX 적용

라이트·다크 테마와 편집기·언어 설정을 생성한다. 색상은 저장소의 `themes/lbrx.toml`에서 수정한다. 테마 스코프는 저장소의 `Docs/17-helix-options.md`에서 확인한다.

```powershell
cargo run --locked -- build --app helix
```

## 설치·복구

1. `dist\lbrx\helix`의 `lbrx_light.toml`, `lbrx_dark.toml`을 사용자 Helix 설정 디렉터리의 `themes`에 복사한다. 기본 위치는 Unix `~/.config/helix/themes`(사용자 지정 시 `$XDG_CONFIG_HOME/helix/themes`), Windows `%APPDATA%\helix\themes`다.
2. Helix에서 확장자 없이 `:theme lbrx_dark` 또는 `:theme lbrx_light`로 현재 세션에서 시험한다. 지속 적용하려면 기존 `config.toml`의 **최상위** `theme` 값만 바꾼다. `[editor]` 안에 넣거나 전체 설정을 교체하지 않는다.
3. 갱신하려면 두 테마 파일을 교체하고 다시 선택한다. 복구하려면 `:theme`에 기존 테마 이름을 주고, 영구 설정도 바꿨다면 원래 값으로 복원한다.

## 편집기·언어 설정 내보내기

저장소의 `themes/editors/helix.toml`에서 원하는 예시의 표 제목과 값의 주석을 해제하거나, 다음 규칙대로 Helix 설정을 추가한 뒤 다시 빌드한다.

| 원본 | 출력 (`dist\lbrx\helix`) | 예시 |
|---|---|---|
| `[config]` 아래 | `config.toml` | `theme`, `[config.editor]`, `[config.keys.normal]` |
| `[languages]` 아래 | `languages.toml` | `[[languages.language]]`, `[languages.language-server.rust-analyzer.config]` |

출력에서는 바깥의 `config.` 또는 `languages.` 접두어를 제거한다. `config.toml`에는 명시한 값만 내보낸다. 시작 테마·줄 번호·현재 줄 강조는 앱 기본값을 유지하기로 했으므로 기본 원본에서는 세 키가 모두 주석 상태다.

`config.toml` 예시에는 스크롤 여백·눈금자·커서 모양·자동 저장·줄바꿈·공백 표시·상태줄·LSP·키맵 설정이 포함되어 있다. Helix 25.07에서 `color-modes`는 상태줄의 모드 색을 바꾸며, 자동 저장은 `[editor.auto-save]`의 `focus-lost`와 `[editor.auto-save.after-delay]`로 설정한다.

`languages.toml`은 `themes/languages`에 등록한 언어 중 Helix 25.07에 해당 기본값이 있는 언어의 **들여쓰기와 줄·블록 주석 기호**를 자동으로 포함한다. `csharp`는 `c-sharp`, `shell`과 `bash`는 하나의 `bash` 항목으로 대응한다. `mdx`·`text`처럼 대응하는 Helix 이름이 없는 항목과 `csv`·`log`·`regex`처럼 이 범위에 내보낼 값이 없는 항목은 자동 생성하지 않는다. 구문색, 파일 확장자, 언어 서버, 포매터는 자동으로 복사하지 않는다.

`[[languages.language]]`에서 Helix 이름을 `name`으로 지정하면 같은 언어의 자동값을 덮어쓴다. `indent`는 지정한 하위 키만 바꾸고 나머지는 유지한다. 자동 생성 대상이 아닌 언어도 수동으로 추가할 수 있다. TOML 문법은 빌드에서 확인하지만 버전별 키·값은 Helix에서 확인한다. 생성기는 언어 서버나 포매터를 설치하거나 실행하지 않는다. 비밀값은 저장소나 배포 파일에 넣지 않는다.

두 설정 파일은 `themes` 폴더가 아니라 그 **상위 Helix 설정 디렉터리**에 둔다. 기존 파일이 있으면 백업하고 필요한 항목만 병합한다. 생성기가 사용자 설정에 직접 복사하거나 기존 설정을 덮어쓰지는 않는다.

## 확인할 차이

### 공통·합일·Helix 전용 옵션 조정

저장소의 `themes/editors/helix.toml`에서 `[styles]`에 옵션 대장의 스코프를 지정한다. 모드별 커서·상태줄, 버퍼줄, 가상 문자, 디버거, 진단 등 전용 항목은 주석 처리된 예시로 제공한다. 원하는 줄 앞의 `#`를 제거하고 `cargo run --locked -- build --app helix`로 두 테마를 다시 생성한다.

```toml
[styles]
"type.builtin" = { fg = "syntax_type", modifiers = ["bold"] }
"diagnostic.warning" = { underline = { style = "line" } }
"ui.cursor.primary.insert" = { bg = "success" }
```

`fg`·`bg`·`underline.color`는 모드별 공통 역할 이름 또는 터미널 기본색 `default`를 받는다. 생략한 필드는 기존 LBRX 스타일을 유지한다. 새 하위 스코프는 LBRX에 정의된 가장 가까운 상위 스타일에서 시작한다. `modifiers = []`는 기존 스타일 목록을 비운다. 기존 `[colors]` 줄 번호 설정은 계속 쓸 수 있으며, 같은 전경을 지정하면 `[styles]`가 우선한다. 테마 파일에서 `[palette]`, `inherits` 및 언어별 색상 재정의는 지원하지 않는다.

테마 설정은 색과 스타일만 바꾼다. `bufferline`, `cursorline`, `cursorcolumn`, 인레이 힌트 등의 표시 여부는 Helix의 `config.toml`에서 별도로 설정한다. 자동완성·호버·진단·디버거 관련 스타일은 해당 기능이 실행될 때 적용된다.

### 앱에서 검증

테마 키는 점이 있으면 `"ui.selection.primary"`처럼 따옴표로 묶는다. `comment.block.documentation`에 직접 지정이 없으면 일치하는 가장 구체적인 상위 구문 스코프를 사용한다. 특정 언어에서 강조가 없다면 색뿐 아니라 Tree-sitter 문법과 쿼리도 확인한다(`hx --health`, `:tree-sitter-scopes`).

라이트·다크 각각에서 같은 코드, Markdown, 기본/추가 선택 영역, 자동완성 메뉴, 경고·오류를 본다. 이 테마는 RGB 색을 사용하므로 True Color 터미널이 필요하다. 터미널이 지원하지만 Helix가 감지하지 못하면 `[editor]`의 `true-color = true`를 확인한다. 글꼴과 행 높이는 호스트 터미널에서 설정한다.  결과에는 `hx --version`, OS, 터미널, 색 지원을 기록한다.

공식 근거: [Helix 25.07 테마 규격](https://github.com/helix-editor/helix/blob/25.07/book/src/themes.md) · [편집기 설정](https://docs.helix-editor.com/configuration.html) · [언어 설정](https://docs.helix-editor.com/languages.html). 문서 기준 버전은 LBRX의 최소 지원 버전을 뜻하지 않는다.
