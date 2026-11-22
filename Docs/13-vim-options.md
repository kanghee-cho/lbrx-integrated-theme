# Vim 색 구성표 옵션 대장

[적용 가이드](12-vim.md) · [공통 원본](../themes/lbrx.toml) · [Helix 대장](17-helix-options.md)

언어별 토큰이 어느 그룹에 연결되는지와 RGB·cterm별 적용 상태는 [Vim 언어별 코드 구문 강조](23-vim-language-highlighting.md)에서 추적한다.

**범위:** [Vim 9.1.0000](https://github.com/vim/vim/blob/v9.1.0000/runtime/doc/syntax.txt)의 권장 문법 그룹과 `highlight-groups`의 내장 화면 그룹. 그룹명이 추적 키다. 파일 형식·플러그인이 임의로 만드는 이름은 전수 대상이 아니다. GUI 전용 항목은 터미널 적용 대상과 구분한다.

**상태:** `[ ]`는 해당 그룹에 대한 LBRX 출력 없음, `[x]`는 생성 근거 있음이다. 공통 역할은 매핑 제안이지 새 TOML 키가 아니다. **합일 후보**는 Helix 등과 의도를 공유할 수 있지만 동일한 동작은 보장하지 않는다. 다크·RGB만 구현했을 때 그룹 전체를 완료 처리하지 말고 행을 나눈다.

| 독립 필드 | 값·경계 |
|---|---|
| `guifg`, `guibg`, `guisp` | RGB 전경·배경·특수색(예: 밑줄 색). 효과는 터미널 지원에 따름 |
| `ctermfg`, `ctermbg`, `ctermul` | 색 단말용 전경·배경·밑줄 색 번호. 공통 HEX와 별도로 근사/대비를 기록 |
| `gui`, `cterm`, `term` | 굵게·기울임·밑줄 계열·반전·취소선 등 속성. 비색상 단말의 `term`은 색상 필드가 아님 |
| `:highlight link A B` | A가 B의 스타일을 따름. 독립 설정과 구분해 링크 대상을 기록 |
| 호스트/앱 설정 | 글꼴·pt·행 간격은 터미널 담당. 줄 번호·커서줄·맞춤법·diff 켜기는 Vim 옵션이며 색 구성표가 활성화하지 않음 |

**실제 검증:** 미실시. 결과는 `그룹.필드 / Vim·OS·터미널 / 라이트 또는 다크 / RGB 또는 cterm / PASS·FAIL / 근거`로 기록한다. 아래 한 그룹의 전경만 구현했다면 배경·장식은 미구현으로 남긴다.

## 문법 그룹

| 그룹 | 분류 | 원본 또는 합일 대상 | Vim 생성 |
|---|---|---|---|
| `Comment` | 공통 | comment | [ ] |
| `Constant` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `String` | 공통 | string | [ ] |
| `Character` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Number` | 공통 | number | [ ] |
| `Boolean` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Float` | 공통 | number | [ ] |
| `Identifier` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Function` | 공통 | function | [ ] |
| `Statement` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Conditional` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Repeat` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Label` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Operator` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Keyword` | 공통 | keyword | [ ] |
| `Exception` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `PreProc` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Include` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Define` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Macro` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `PreCondit` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Type` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `StorageClass` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Structure` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Typedef` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Special` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `SpecialChar` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Tag` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Delimiter` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `SpecialComment` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Debug` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Underlined` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Ignore` | 합일 후보 | 편집기 구문 강조 | [ ] |
| `Error` | 공통 | error | [ ] |
| `Todo` | 합일 후보 | 편집기 구문 강조 | [ ] |

## 화면 그룹

| 그룹 | 분류 | 원본 또는 합일 대상 | Vim 생성 |
|---|---|---|---|
| `ColorColumn` | Vim 전용 | — | [ ] |
| `Conceal` | Vim 전용 | — | [ ] |
| `Cursor` | 공통 | focus | [ ] |
| `lCursor` | Vim 전용 | — | [ ] |
| `CursorIM` | Vim 전용 | — | [ ] |
| `CursorColumn` | Vim 전용 | — | [ ] |
| `CursorLine` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `Directory` | Vim 전용 | — | [ ] |
| `DiffAdd` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `DiffChange` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `DiffDelete` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `DiffText` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `EndOfBuffer` | Vim 전용 | — | [ ] |
| `ErrorMsg` | 공통 | error | [ ] |
| `VertSplit` | Vim 전용 | — | [ ] |
| `Folded` | Vim 전용 | — | [ ] |
| `FoldColumn` | Vim 전용 | — | [ ] |
| `SignColumn` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `IncSearch` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `LineNr` | 공통 | muted | [ ] |
| `LineNrAbove` | Vim 전용 | — | [ ] |
| `LineNrBelow` | Vim 전용 | — | [ ] |
| `CursorLineNr` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `CursorLineFold` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `CursorLineSign` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `MatchParen` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `MessageWindow` | Vim 전용 | — | [ ] |
| `ModeMsg` | Vim 전용 | — | [ ] |
| `MoreMsg` | Vim 전용 | — | [ ] |
| `NonText` | Vim 전용 | — | [ ] |
| `Normal` | 공통 | text / background | [ ] |
| `Pmenu` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `PmenuSel` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `PmenuKind` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `PmenuKindSel` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `PmenuExtra` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `PmenuExtraSel` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `PmenuSbar` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `PmenuThumb` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `PopupNotification` | Vim 전용 | — | [ ] |
| `Question` | Vim 전용 | — | [ ] |
| `QuickFixLine` | Vim 전용 | — | [ ] |
| `Search` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `CurSearch` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `SpecialKey` | Vim 전용 | — | [ ] |
| `SpellBad` | Vim 전용 | — | [ ] |
| `SpellCap` | Vim 전용 | — | [ ] |
| `SpellLocal` | Vim 전용 | — | [ ] |
| `SpellRare` | Vim 전용 | — | [ ] |
| `StatusLine` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `StatusLineNC` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `StatusLineTerm` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `StatusLineTermNC` | 합일 후보 | 편집기 UI (그룹별 독립) | [ ] |
| `TabLine` | Vim 전용 | — | [ ] |
| `TabLineFill` | Vim 전용 | — | [ ] |
| `TabLineSel` | Vim 전용 | — | [ ] |
| `Terminal` | Vim 전용 | — | [ ] |
| `Title` | Vim 전용 | — | [ ] |
| `Visual` | 공통 | selection_text / selection_background | [ ] |
| `VisualNOS` | Vim 전용 | — | [ ] |
| `WarningMsg` | 공통 | warning | [ ] |
| `WildMenu` | Vim 전용 | — | [ ] |
| `Menu` | GUI 경계 | 터미널 Vim 대상 밖 | — |
| `Scrollbar` | GUI 경계 | 터미널 Vim 대상 밖 | — |
| `Tooltip` | GUI 경계 | 터미널 Vim 대상 밖 | — |
| `User1` | Vim 전용 | — | [ ] |
| `User2` | Vim 전용 | — | [ ] |
| `User3` | Vim 전용 | — | [ ] |
| `User4` | Vim 전용 | — | [ ] |
| `User5` | Vim 전용 | — | [ ] |
| `User6` | Vim 전용 | — | [ ] |
| `User7` | Vim 전용 | — | [ ] |
| `User8` | Vim 전용 | — | [ ] |
| `User9` | Vim 전용 | — | [ ] |
