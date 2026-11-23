# Helix에 LBRX 적용하기

**라이트·다크 테마 생성 구현.** 색상은 [공통 TOML](../themes/lbrx.toml)에서 직접 수정한다. 생성 범위는 [옵션 대장](17-helix-options.md)에 기록한다.

```powershell
cargo run --locked -- build --app helix
```

## 설치·복구

1. `dist\lbrx\helix`의 `lbrx_light.toml`, `lbrx_dark.toml`을 사용자 Helix 설정 디렉터리의 `themes`에 복사한다. 기본 위치는 Unix `~/.config/helix/themes`(사용자 지정 시 `$XDG_CONFIG_HOME/helix/themes`), Windows `%APPDATA%\helix\themes`다.
2. Helix에서 확장자 없이 `:theme lbrx_dark` 또는 `:theme lbrx_light`로 현재 세션에서 시험한다. 지속 적용하려면 기존 `config.toml`의 **최상위** `theme` 값만 바꾼다. `[editor]` 안에 넣거나 전체 설정을 교체하지 않는다.
3. 갱신하려면 두 테마 파일을 교체하고 다시 선택한다. 복구하려면 `:theme`에 기존 테마 이름을 주고, 영구 설정도 바꿨다면 원래 값으로 복원한다.

## 확인할 차이

### 공통·합일·Helix 전용 옵션 조정

저장소의 `themes/editors/helix.toml`에서 `[styles]`에 옵션 대장(`Docs/17-helix-options.md`)의 `공통`, `합일 후보`, `Helix 전용` 스코프를 지정한다. 모드별 커서·상태줄, 버퍼줄, 가상 문자, 디버거, 진단 등 전용 항목은 설정 파일에 주석 처리된 예시로 제공한다. 원하는 줄 앞의 `#`를 제거하고 `cargo run -- build --app helix`로 두 테마를 다시 생성한다.

```toml
[styles]
"type.builtin" = { fg = "syntax_type", modifiers = ["bold"] }
"diagnostic.warning" = { underline = { style = "line" } }
"ui.cursor.primary.insert" = { bg = "success" }
```

`fg`·`bg`·`underline.color`는 모드별 공통 역할 이름 또는 터미널 기본색 `default`를 받는다. 생략한 필드는 기존 LBRX 스타일을 유지한다. 새 하위 스코프는 LBRX에 정의된 가장 가까운 상위 스타일에서 시작한다. `modifiers = []`는 기존 스타일 목록을 비운다. 기존 `[colors]` 줄 번호 설정은 계속 쓸 수 있으며, 같은 전경을 지정하면 `[styles]`가 우선한다. `[palette]`, `inherits` 및 언어별 재정의는 지원하지 않는다.

테마 설정은 색과 스타일만 바꾼다. `bufferline`, `cursorline`, `cursorcolumn`, 인레이 힌트 등의 표시 여부는 Helix의 `config.toml`에서 별도로 설정한다. 자동완성·호버·진단·디버거 관련 스타일은 해당 기능이 실행될 때 적용된다.

### 앱에서 검증

테마 키는 점이 있으면 `"ui.selection.primary"`처럼 따옴표로 묶는다. `comment.block.documentation`에 직접 지정이 없으면 일치하는 가장 구체적인 상위 구문 스코프를 사용한다. 특정 언어에서 강조가 없다면 색뿐 아니라 Tree-sitter 문법과 쿼리도 확인한다(`hx --health`, `:tree-sitter-scopes`).

라이트·다크 각각에서 같은 코드, Markdown, 기본/추가 선택 영역, 자동완성 메뉴, 경고·오류를 본다. 이 테마는 RGB 색을 사용하므로 True Color 터미널이 필요하다. 터미널이 지원하지만 Helix가 감지하지 못하면 `[editor]`의 `true-color = true`를 확인한다. 글꼴과 행 높이는 호스트 터미널에서 설정한다.  결과에는 `hx --version`, OS, 터미널, 색 지원을 기록한다.

공식 근거: [Helix 25.07 테마 규격](https://github.com/helix-editor/helix/blob/25.07/book/src/themes.md) · [설정 안내](https://docs.helix-editor.com/configuration.html). 문서 기준 버전은 LBRX의 최소 지원 버전을 뜻하지 않는다.
