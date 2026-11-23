# Helix 테마 옵션 대장

[적용 가이드](16-helix.md) · [Vim 옵션 대장](13-vim-options.md) · [공통 원본](../themes/lbrx.toml)

**범위:** [Helix 25.07 공식 테마 규격](https://github.com/helix-editor/helix/blob/25.07/book/src/themes.md)에 열거된 구문·문서·UI·진단 스코프. 언어별 임의 캡처와 커뮤니티 쿼리는 별도다. `공통`은 기존 테마 역할, `합일 후보`는 Vim 등 다른 앱과 같은 의미로 승격할 수 있는 값, `전용`은 Helix에서 따로 매핑할 값이다. `[x]`는 두 모드에서 표시된 필드만 직접 생성, `[ ]`는 미생성이다. 상위 스코프의 상속 효과는 하위 스코프를 독립 구현한 것으로 체크하지 않는다.

| 스코프마다 조절할 필드 | 허용 값·경계 |
|---|---|
| `fg`, `bg` | 각각 전경·배경. HEX, 팔레트 이름 또는 단말 기본색. 문자열 축약은 전경만 지정 |
| `underline.color`, `underline.style` | 밑줄 색과 `line`, `curl`, `dashed`, `dotted`, `double_line`을 독립 추적 |
| `modifiers` | `bold`, `dim`, `italic`, `slow_blink`, `rapid_blink`, `reversed`, `hidden`, `crossed_out`. 구형 `underlined` 대신 `underline.style` 권장 |
| `[palette]`, `inherits` | 이름→색 매핑, 기반 테마 상속. `[palette]`는 최상위 스코프 선언 뒤에 둔다 |
| 호스트·에디터 설정 | 글꼴·크기·셀 높이는 터미널 소유. 커서 모양, 줄 번호 표시, 자동완성·LSP·거터 활성화는 색 테마로 설정하지 않음 |

**검증:** 두 모드의 TOML 생성과 재파싱을 확인했다. Helix에서 적용 및 화면 확인은 미실시다. 실제 검증은 `스코프.필드 / Helix·OS·터미널 / 라이트 또는 다크 / 직접 선언 또는 상속 / 결과`로 기록한다.

## 문법·문서 스코프

| 스코프 | 분류 | 원본 또는 합일 대상 | Helix 생성 |
|---|---|---|---|
| `attribute` | 합일 후보 | 문법·문서 표현 | [ ] |
| `type` | 합일 후보 | 문법·문서 표현 | [ ] |
| `type.builtin` | 합일 후보 | 문법·문서 표현 | [ ] |
| `type.parameter` | 합일 후보 | 문법·문서 표현 | [ ] |
| `type.enum` | 합일 후보 | 문법·문서 표현 | [ ] |
| `type.enum.variant` | 합일 후보 | 문법·문서 표현 | [ ] |
| `constructor` | 합일 후보 | 문법·문서 표현 | [ ] |
| `constant` | 합일 후보 | 문법·문서 표현 | [ ] |
| `constant.builtin` | 합일 후보 | 문법·문서 표현 | [ ] |
| `constant.builtin.boolean` | 합일 후보 | 문법·문서 표현 | [ ] |
| `constant.character` | 합일 후보 | 문법·문서 표현 | [ ] |
| `constant.character.escape` | 합일 후보 | 문법·문서 표현 | [ ] |
| `constant.numeric` | 공통 | `fg`: number | [x] |
| `constant.numeric.integer` | 합일 후보 | 문법·문서 표현 | [ ] |
| `constant.numeric.float` | 합일 후보 | 문법·문서 표현 | [ ] |
| `string` | 공통 | `fg`: string | [x] |
| `string.regexp` | 합일 후보 | 문법·문서 표현 | [ ] |
| `string.special` | 합일 후보 | 문법·문서 표현 | [ ] |
| `string.special.path` | 합일 후보 | 문법·문서 표현 | [ ] |
| `string.special.url` | 합일 후보 | 문법·문서 표현 | [ ] |
| `string.special.symbol` | 합일 후보 | 문법·문서 표현 | [ ] |
| `comment` | 공통 | `fg`: comment | [x] |
| `comment.line` | 합일 후보 | 문법·문서 표현 | [ ] |
| `comment.line.documentation` | 합일 후보 | 문법·문서 표현 | [ ] |
| `comment.block` | 합일 후보 | 문법·문서 표현 | [ ] |
| `comment.block.documentation` | 합일 후보 | 문법·문서 표현 | [ ] |
| `comment.unused` | 합일 후보 | 문법·문서 표현 | [ ] |
| `variable` | 합일 후보 | 문법·문서 표현 | [ ] |
| `variable.builtin` | 합일 후보 | 문법·문서 표현 | [ ] |
| `variable.parameter` | 합일 후보 | 문법·문서 표현 | [ ] |
| `variable.other` | 합일 후보 | 문법·문서 표현 | [ ] |
| `variable.other.member` | 합일 후보 | 문법·문서 표현 | [ ] |
| `variable.other.member.private` | 합일 후보 | 문법·문서 표현 | [ ] |
| `label` | 합일 후보 | 문법·문서 표현 | [ ] |
| `punctuation` | 합일 후보 | 문법·문서 표현 | [ ] |
| `punctuation.delimiter` | 합일 후보 | 문법·문서 표현 | [ ] |
| `punctuation.bracket` | 합일 후보 | 문법·문서 표현 | [ ] |
| `punctuation.special` | 합일 후보 | 문법·문서 표현 | [ ] |
| `keyword` | 공통 | `fg`: keyword | [x] |
| `keyword.control` | 합일 후보 | 문법·문서 표현 | [ ] |
| `keyword.control.conditional` | 합일 후보 | 문법·문서 표현 | [ ] |
| `keyword.control.repeat` | 합일 후보 | 문법·문서 표현 | [ ] |
| `keyword.control.import` | 합일 후보 | 문법·문서 표현 | [ ] |
| `keyword.control.return` | 합일 후보 | 문법·문서 표현 | [ ] |
| `keyword.control.exception` | 합일 후보 | 문법·문서 표현 | [ ] |
| `keyword.operator` | 합일 후보 | 문법·문서 표현 | [ ] |
| `keyword.directive` | 합일 후보 | 문법·문서 표현 | [ ] |
| `keyword.function` | 합일 후보 | 문법·문서 표현 | [ ] |
| `keyword.storage` | 합일 후보 | 문법·문서 표현 | [ ] |
| `keyword.storage.type` | 합일 후보 | 문법·문서 표현 | [ ] |
| `keyword.storage.modifier` | 합일 후보 | 문법·문서 표현 | [ ] |
| `operator` | 합일 후보 | 문법·문서 표현 | [ ] |
| `function` | 공통 | `fg`: function | [x] |
| `function.builtin` | 합일 후보 | 문법·문서 표현 | [ ] |
| `function.method` | 합일 후보 | 문법·문서 표현 | [ ] |
| `function.method.private` | 합일 후보 | 문법·문서 표현 | [ ] |
| `function.macro` | 합일 후보 | 문법·문서 표현 | [ ] |
| `function.special` | 합일 후보 | 문법·문서 표현 | [ ] |
| `tag` | 합일 후보 | 문법·문서 표현 | [ ] |
| `tag.builtin` | 합일 후보 | 문법·문서 표현 | [ ] |
| `namespace` | 합일 후보 | 문법·문서 표현 | [ ] |
| `special` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.heading` | 공통 | `fg`: heading | [x] |
| `markup.heading.marker` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.heading.1` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.heading.2` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.heading.3` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.heading.4` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.heading.5` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.heading.6` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.list` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.list.unnumbered` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.list.numbered` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.list.checked` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.list.unchecked` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.bold` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.italic` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.strikethrough` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.link` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.link.url` | 공통 | `fg`: link | [x] |
| `markup.link.label` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.link.text` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.quote` | 공통 | `fg`: quote | [x] |
| `markup.raw` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.raw.inline` | 공통 | `fg`: inline_code | [x] |
| `markup.raw.block` | 합일 후보 | 문법·문서 표현 | [ ] |
| `diff` | 합일 후보 | 문법·문서 표현 | [ ] |
| `diff.plus` | 합일 후보 | 문법·문서 표현 | [ ] |
| `diff.plus.gutter` | 합일 후보 | 문법·문서 표현 | [ ] |
| `diff.minus` | 합일 후보 | 문법·문서 표현 | [ ] |
| `diff.minus.gutter` | 합일 후보 | 문법·문서 표현 | [ ] |
| `diff.delta` | 합일 후보 | 문법·문서 표현 | [ ] |
| `diff.delta.moved` | 합일 후보 | 문법·문서 표현 | [ ] |
| `diff.delta.conflict` | 합일 후보 | 문법·문서 표현 | [ ] |
| `diff.delta.gutter` | 합일 후보 | 문법·문서 표현 | [ ] |
| `markup.normal` | 합일 후보 | 문법·문서 표현 | [ ] |

## UI·진단 스코프

| 스코프 | 분류 | 원본 또는 합일 대상 | Helix 생성 |
|---|---|---|---|
| `markup.normal.completion` | Helix 전용 | — | [ ] |
| `markup.normal.hover` | Helix 전용 | — | [ ] |
| `markup.heading.completion` | Helix 전용 | — | [ ] |
| `markup.heading.hover` | Helix 전용 | — | [ ] |
| `markup.raw.inline.completion` | Helix 전용 | — | [ ] |
| `markup.raw.inline.hover` | Helix 전용 | — | [ ] |
| `ui.background` | 공통 | `bg`: background | [x] |
| `ui.background.separator` | Helix 전용 | — | [ ] |
| `ui.cursor` | 공통 | `fg`: background / `bg`: focus | [x] |
| `ui.cursor.normal` | Helix 전용 | — | [ ] |
| `ui.cursor.insert` | Helix 전용 | — | [ ] |
| `ui.cursor.select` | Helix 전용 | — | [ ] |
| `ui.cursor.match` | Helix 전용 | — | [ ] |
| `ui.cursor.primary` | 공통 | `fg`: background / `bg`: focus | [x] |
| `ui.cursor.primary.normal` | Helix 전용 | — | [ ] |
| `ui.cursor.primary.insert` | Helix 전용 | — | [ ] |
| `ui.cursor.primary.select` | Helix 전용 | — | [ ] |
| `ui.debug.breakpoint` | Helix 전용 | — | [ ] |
| `ui.debug.active` | Helix 전용 | — | [ ] |
| `ui.gutter` | Helix 전용 | — | [ ] |
| `ui.gutter.selected` | Helix 전용 | — | [ ] |
| `ui.linenr` | 공통 | `fg`: muted | [x] |
| `ui.linenr.selected` | 공통 | `fg`: text | [x] |
| `ui.statusline` | 공통 | `fg`: text / `bg`: surface | [x] |
| `ui.statusline.inactive` | 공통 | `fg`: muted / `bg`: surface | [x] |
| `ui.statusline.normal` | Helix 전용 | — | [ ] |
| `ui.statusline.insert` | Helix 전용 | — | [ ] |
| `ui.statusline.select` | Helix 전용 | — | [ ] |
| `ui.statusline.separator` | Helix 전용 | — | [ ] |
| `ui.bufferline` | Helix 전용 | — | [ ] |
| `ui.bufferline.active` | Helix 전용 | — | [ ] |
| `ui.bufferline.background` | Helix 전용 | — | [ ] |
| `ui.popup` | 공통 | `fg`: text / `bg`: surface | [x] |
| `ui.popup.info` | Helix 전용 | — | [ ] |
| `ui.picker.header` | Helix 전용 | — | [ ] |
| `ui.picker.header.column` | Helix 전용 | — | [ ] |
| `ui.picker.header.column.active` | Helix 전용 | — | [ ] |
| `ui.window` | Helix 전용 | — | [ ] |
| `ui.help` | Helix 전용 | — | [ ] |
| `ui.text` | 공통 | `fg`: text | [x] |
| `ui.text.focus` | Helix 전용 | — | [ ] |
| `ui.text.inactive` | Helix 전용 | — | [ ] |
| `ui.text.info` | Helix 전용 | — | [ ] |
| `ui.text.directory` | Helix 전용 | — | [ ] |
| `ui.virtual.ruler` | Helix 전용 | — | [ ] |
| `ui.virtual.whitespace` | Helix 전용 | — | [ ] |
| `ui.virtual.indent-guide` | Helix 전용 | — | [ ] |
| `ui.virtual.inlay-hint` | Helix 전용 | — | [ ] |
| `ui.virtual.inlay-hint.parameter` | Helix 전용 | — | [ ] |
| `ui.virtual.inlay-hint.type` | Helix 전용 | — | [ ] |
| `ui.virtual.wrap` | Helix 전용 | — | [ ] |
| `ui.virtual.jump-label` | Helix 전용 | — | [ ] |
| `ui.menu` | 공통 | `fg`: text / `bg`: surface | [x] |
| `ui.menu.selected` | 공통 | `fg`: selection_text / `bg`: selection_background | [x] |
| `ui.menu.scroll` | Helix 전용 | — | [ ] |
| `ui.selection` | 공통 | `fg`: selection_text / `bg`: selection_background | [x] |
| `ui.selection.primary` | 공통 | `fg`: selection_text / `bg`: selection_background | [x] |
| `ui.highlight` | Helix 전용 | — | [ ] |
| `ui.highlight.frameline` | Helix 전용 | — | [ ] |
| `ui.cursorline.primary` | Helix 전용 | — | [ ] |
| `ui.cursorline.secondary` | Helix 전용 | — | [ ] |
| `ui.cursorcolumn.primary` | Helix 전용 | — | [ ] |
| `ui.cursorcolumn.secondary` | Helix 전용 | — | [ ] |
| `warning` | 공통 | `fg`: warning | [x] |
| `error` | 공통 | `fg`: error | [x] |
| `info` | Helix 전용 | — | [ ] |
| `hint` | Helix 전용 | — | [ ] |
| `diagnostic` | Helix 전용 | — | [ ] |
| `diagnostic.hint` | Helix 전용 | — | [ ] |
| `diagnostic.info` | Helix 전용 | — | [ ] |
| `diagnostic.warning` | 공통 | `underline.color`: warning / `underline.style`: curl | [x] |
| `diagnostic.error` | 공통 | `underline.color`: error / `underline.style`: curl | [x] |
| `diagnostic.unnecessary` | Helix 전용 | — | [ ] |
| `diagnostic.deprecated` | Helix 전용 | — | [ ] |
| `tabstop` | Helix 전용 | — | [ ] |
