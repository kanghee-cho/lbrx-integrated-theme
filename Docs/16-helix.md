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

테마 키는 점이 있으면 `"ui.selection.primary"`처럼 따옴표로 묶는다. `comment.block.documentation`에 직접 지정이 없으면 일치하는 가장 구체적인 상위 구문 스코프를 사용한다. 특정 언어에서 강조가 없다면 색뿐 아니라 Tree-sitter 문법과 쿼리도 확인한다(`hx --health`, `:tree-sitter-scopes`).

라이트·다크 각각에서 같은 코드, Markdown, 기본/추가 선택 영역, 자동완성 메뉴, 경고·오류를 본다. 이 테마는 RGB 색을 사용하므로 True Color 터미널이 필요하다. 터미널이 지원하지만 Helix가 감지하지 못하면 `[editor]`의 `true-color = true`를 확인한다. 글꼴과 행 높이는 호스트 터미널에서 설정한다. 스타일을 선언한 것과 LSP·인레이 힌트·디버거가 동작하는 것은 별개다. 결과에는 `hx --version`, OS, 터미널, 색 지원을 기록한다.

공식 근거: [Helix 25.07 테마 규격](https://github.com/helix-editor/helix/blob/25.07/book/src/themes.md) · [설정 안내](https://docs.helix-editor.com/configuration.html). 문서 기준 버전은 LBRX의 최소 지원 버전을 뜻하지 않는다.
