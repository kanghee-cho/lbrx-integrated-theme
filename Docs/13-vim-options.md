# Vim 색 구성표 옵션 대장

[적용 가이드](12-vim.md) · [공통 원본](../themes/lbrx.toml) · [Helix 대장](17-helix-options.md)

언어별 토큰이 어느 그룹에 연결되는지와 RGB·cterm별 적용 상태는 [Vim 언어별 코드 구문 강조](23-vim-language-highlighting.md)에서 추적한다.

**범위:** [Vim 9.1.0000](https://github.com/vim/vim/blob/v9.1.0000/runtime/doc/syntax.txt)의 권장 문법 그룹과 `highlight-groups`의 내장 화면 그룹. 그룹명이 추적 키다. 파일 형식·플러그인이 임의로 만드는 이름은 전수 대상이 아니다. GUI 전용 항목은 터미널 적용 대상과 구분한다.

**구현 상태:** `[x]`는 기본 생성, `[x] 선택 생성`은 설정한 항목만 생성, `—`는 Vim 기본 유지 또는 대상 밖을 뜻한다. 실제 화면 검증과는 별개다.

**Vim 전용 40개 그룹:** [vim.toml](../themes/editors/vim.toml)의 `[highlights]`에서 주석을 해제한 그룹만 라이트·다크에 생성한다. 표의 값은 TOML의 권장 예시이며 기본 빌드에서는 비활성이다. 모든 예시를 활성화한 두 테마를 Vim 9.2에서 로딩했고 실제 터미널 화면은 미검증이다.

| 독립 필드 | 값·경계 |
|---|---|
| `guifg`, `guibg`, `guisp` | RGB 전경·배경·특수색(예: 밑줄 색). 효과는 터미널 지원에 따름 |
| `ctermfg`, `ctermbg`, `ctermul` | 색 단말용 전경·배경·밑줄 색 번호. 공통 HEX와 별도로 근사/대비를 기록 |
| `gui`, `cterm`, `term` | 굵게·기울임·밑줄 계열·반전·취소선 등 속성. 비색상 단말의 `term`은 색상 필드가 아님 |
| `:highlight link A B` | A가 B의 스타일을 따름. 독립 설정과 구분해 링크 대상을 기록 |
| 호스트/앱 설정 | 글꼴·pt·행 간격은 터미널 담당. 줄 번호·커서줄·맞춤법·diff 켜기는 Vim 옵션이며 색 구성표가 활성화하지 않음 |

**검증:** Windows의 Vim 9.2에서 사용자 설정 없이 라이트 → 다크 → 라이트를 불러왔다. `Normal`·`Visual`의 RGB 배경, `LineNr`의 RGB 전경, `Visual`의 256색 배경 값과 `termguicolors` 유지 여부를 비대화형으로 확인했다. Rust 샘플에서 매크로와 주석이 각각 `rustMacro`, `rustCommentLine`으로 분류되고 두 모드의 색을 사용하는 것도 확인했다. `Conditional`은 현재 `keyword.control`을 직접 사용한다. 다크 `Normal`의 256색 배경은 앞선 검증에서 확인했다. 실제 터미널 화면은 미검증이다. 단일 역할만 적힌 그룹은 별도 표기가 없는 한 전경색을 지정한다.

추가한 공통 그룹은 Vim 9.2에서 같은 모드 전환으로 검증했다. 완료 메뉴·현재 검색·비활성 상태줄·현재 줄 번호의 RGB와 256색 값, `DiffAdd`·`DiffChange`의 배경 구분과 전경 미지정, `MatchParen`·`Underlined`의 밑줄을 확인했다.

## 문법 그룹

| 그룹 | 분류 | 원본 또는 합일 대상 | Vim 생성 |
|---|---|---|---|
| `Comment` | 공통 | comment | [x] |
| `Constant` | 공통 | `constant.builtin` (세부 분류가 없는 상수의 폴백) | [x] |
| `String` | 공통 | string | [x] |
| `Character` | 공통 | `string` | [x] |
| `Number` | 공통 | number | [x] |
| `Boolean` | 공통 | `constant.builtin` | [x] |
| `Float` | 공통 | `Number`에 링크 | [x] |
| `Identifier` | 공통 | `variable` | [x] |
| `Function` | 공통 | function | [x] |
| `Statement` | 공통 | `keyword.control` | [x] |
| `Conditional` | 공통 | `keyword.control` | [x] |
| `Repeat` | 공통 | `keyword.control` | [x] |
| `Label` | 공통 | `keyword.control` | [x] |
| `Operator` | 공통 | `operator` | [x] |
| `Keyword` | 공통 | keyword | [x] |
| `Exception` | 공통 | `keyword.control` | [x] |
| `PreProc` | 공통 | `attribute` | [x] |
| `Include` | 공통 | `attribute` | [x] |
| `Define` | 공통 | `keyword.declaration` | [x] |
| `Macro` | 공통 | `function.macro` | [x] |
| `PreCondit` | 공통 | `attribute` | [x] |
| `Type` | 공통 | `type` | [x] |
| `StorageClass` | 공통 | `keyword.declaration` | [x] |
| `Structure` | 공통 | `keyword.declaration` | [x] |
| `Typedef` | 공통 | `keyword.declaration` | [x] |
| `Special` | 공통 | `attribute` (별도 분류가 없는 특수 기호) | [x] |
| `SpecialChar` | 공통 | `string.escape` | [x] |
| `Tag` | 공통 | `attribute` | [x] |
| `Delimiter` | 공통 | `punctuation` | [x] |
| `SpecialComment` | 공통 | `comment` | [x] |
| `Debug` | 공통 | `Function`에 링크 | [x] |
| `Underlined` | 공통 | `link` 전경 + 밑줄 | [x] |
| `Ignore` | Vim 기본 유지 | 숨김 목적으로 쓰이므로 가시 토큰으로 매핑하지 않음 | — |
| `Error` | 공통 | error | [x] |
| `Todo` | 공통 | `warning` 전경 + 굵게 | [x] |

## 화면 그룹

| 그룹 | 분류 | 원본 또는 합일 대상 | Vim 생성 |
|---|---|---|---|
| `ColorColumn` | Vim 전용 | `{ bg = "surface" }` | [x] 선택 생성 |
| `Conceal` | Vim 전용 | `{ fg = "muted", bg = "none" }` | [x] 선택 생성 |
| `Cursor` | 공통 | `background` / `focus` (터미널의 커서 표시 지원에 따름) | [x] |
| `lCursor` | Vim 전용 | `{ fg = "background", bg = "focus" }` | [x] 선택 생성 |
| `CursorIM` | Vim 전용 | `{ fg = "background", bg = "focus" }` | [x] 선택 생성 |
| `CursorColumn` | Vim 전용 | `{ bg = "surface" }` | [x] 선택 생성 |
| `CursorLine` | 공통 | 배경 `surface` | [x] |
| `Directory` | Vim 전용 | `{ fg = "focus", styles = ["bold"] }` | [x] 선택 생성 |
| `DiffAdd` | 공통 | 배경 `surface`, 구문 전경 유지 | [x] |
| `DiffChange` | 공통 | 배경 `selection_background`, 구문 전경 유지 | [x] |
| `DiffDelete` | 공통 | `error` / `surface` (삭제 줄 자리 표시) | [x] |
| `DiffText` | 공통 | `background` / `warning` + 굵게 (줄 안의 변경 구간) | [x] |
| `EndOfBuffer` | Vim 전용 | `{ fg = "muted" }` | [x] 선택 생성 |
| `ErrorMsg` | 공통 | error | [x] |
| `VertSplit` | Vim 전용 | `{ fg = "muted", bg = "background" }` | [x] 선택 생성 |
| `Folded` | Vim 전용 | `{ fg = "muted", bg = "surface", styles = ["italic"] }` | [x] 선택 생성 |
| `FoldColumn` | Vim 전용 | `{ fg = "muted", bg = "surface" }` | [x] 선택 생성 |
| `SignColumn` | 공통 | `muted` | [x] |
| `IncSearch` | 공통 | background / focus | [x] |
| `LineNr` | 공통 | muted | [x] |
| `LineNrAbove` | Vim 전용 | `{ fg = "muted" }` | [x] 선택 생성 |
| `LineNrBelow` | Vim 전용 | `{ fg = "muted" }` | [x] 선택 생성 |
| `CursorLineNr` | 공통 | `focus` + 굵게 | [x] |
| `CursorLineFold` | 공통 | `muted` / `surface` | [x] |
| `CursorLineSign` | 공통 | `muted` / `surface` | [x] |
| `MatchParen` | 공통 | `focus` + 굵게·밑줄 | [x] |
| `MessageWindow` | Vim 전용 | `{ fg = "text", bg = "surface" }` | [x] 선택 생성 |
| `ModeMsg` | Vim 전용 | `{ fg = "focus", styles = ["bold"] }` | [x] 선택 생성 |
| `MoreMsg` | Vim 전용 | `{ fg = "focus", styles = ["bold"] }` | [x] 선택 생성 |
| `NonText` | Vim 전용 | `{ fg = "muted" }` | [x] 선택 생성 |
| `Normal` | 공통 | text / background | [x] |
| `Pmenu` | 공통 | `text` / `surface` | [x] |
| `PmenuSel` | 공통 | `selection_text` / `selection_background` + 굵게 | [x] |
| `PmenuKind` | 공통 | `focus` / `surface` | [x] |
| `PmenuKindSel` | 공통 | `selection_text` / `selection_background` + 굵게 | [x] |
| `PmenuExtra` | 공통 | `muted` / `surface` | [x] |
| `PmenuExtraSel` | 공통 | `selection_text` / `selection_background` + 굵게 | [x] |
| `PmenuSbar` | 공통 | 배경 `selection_background` | [x] |
| `PmenuThumb` | 공통 | 배경 `focus` | [x] |
| `PopupNotification` | Vim 전용 | `{ fg = "text", bg = "surface" }` | [x] 선택 생성 |
| `Question` | Vim 전용 | `{ fg = "focus", styles = ["bold"] }` | [x] 선택 생성 |
| `QuickFixLine` | Vim 전용 | `{ fg = "selection_text", bg = "selection_background" }` | [x] 선택 생성 |
| `Search` | 공통 | background / warning | [x] |
| `CurSearch` | 공통 | `background` / `focus` + 굵게 | [x] |
| `SpecialKey` | Vim 전용 | `{ fg = "muted" }` | [x] 선택 생성 |
| `SpellBad` | Vim 전용 | `{ sp = "error", styles = ["undercurl"] }` | [x] 선택 생성 |
| `SpellCap` | Vim 전용 | `{ sp = "warning", styles = ["undercurl"] }` | [x] 선택 생성 |
| `SpellLocal` | Vim 전용 | `{ sp = "link", styles = ["undercurl"] }` | [x] 선택 생성 |
| `SpellRare` | Vim 전용 | `{ sp = "syntax_macro", styles = ["undercurl"] }` | [x] 선택 생성 |
| `StatusLine` | 공통 | text / surface | [x] |
| `StatusLineNC` | 공통 | `muted` / `surface` | [x] |
| `StatusLineTerm` | 공통 | `text` / `surface` | [x] |
| `StatusLineTermNC` | 공통 | `muted` / `surface` | [x] |
| `TabLine` | Vim 전용 | `{ fg = "muted", bg = "surface" }` | [x] 선택 생성 |
| `TabLineFill` | Vim 전용 | `{ fg = "muted", bg = "background" }` | [x] 선택 생성 |
| `TabLineSel` | Vim 전용 | `{ fg = "selection_text", bg = "selection_background", styles = ["bold"] }` | [x] 선택 생성 |
| `Terminal` | Vim 전용 | `{ fg = "text", bg = "background" }` | [x] 선택 생성 |
| `Title` | Vim 전용 | `{ fg = "heading", styles = ["bold"] }` | [x] 선택 생성 |
| `Visual` | 공통 | selection_text / selection_background | [x] |
| `VisualNOS` | Vim 전용 | `{ fg = "selection_text", bg = "selection_background" }` | [x] 선택 생성 |
| `WarningMsg` | 공통 | warning | [x] |
| `WildMenu` | Vim 전용 | `{ fg = "selection_text", bg = "selection_background", styles = ["bold"] }` | [x] 선택 생성 |
| `Menu` | GUI 경계 | 터미널 Vim 대상 밖 | — |
| `Scrollbar` | GUI 경계 | 터미널 Vim 대상 밖 | — |
| `Tooltip` | GUI 경계 | 터미널 Vim 대상 밖 | — |
| `User1` | Vim 전용 | `{ fg = "text", bg = "surface" }` | [x] 선택 생성 |
| `User2` | Vim 전용 | `{ fg = "focus", bg = "surface", styles = ["bold"] }` | [x] 선택 생성 |
| `User3` | Vim 전용 | `{ fg = "muted", bg = "surface" }` | [x] 선택 생성 |
| `User4` | Vim 전용 | `{ fg = "warning", bg = "surface" }` | [x] 선택 생성 |
| `User5` | Vim 전용 | `{ fg = "error", bg = "surface" }` | [x] 선택 생성 |
| `User6` | Vim 전용 | `{ fg = "success", bg = "surface" }` | [x] 선택 생성 |
| `User7` | Vim 전용 | `{ fg = "link", bg = "surface" }` | [x] 선택 생성 |
| `User8` | Vim 전용 | `{ fg = "text", bg = "background" }` | [x] 선택 생성 |
| `User9` | Vim 전용 | `{ fg = "selection_text", bg = "selection_background" }` | [x] 선택 생성 |
