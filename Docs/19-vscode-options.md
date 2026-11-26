# VS Code 옵션 대장

[적용 가이드](18-vscode.md) · [공통 원본](../themes/orange-type/lbrx.toml) · [공식 색 ID 목록 고정본](https://github.com/microsoft/vscode-docs/blob/d1e499c3abac3a4d3fb28e18b0e7e9f72c0d17f9/api/references/theme-color.md)

`tokenColors`와 `semanticTokenColors`의 언어별 적용 상태 및 HTML 코드 블록용 동반 CSS는 [언어별 구문 강조](22-vscode-language-highlighting.md)에서 추적한다.

`[x]`는 두 모드에서 기본 생성, `[x] 선택 생성`은 [vscode.toml](../themes/orange-type/editors/vscode.toml)의 `[colors]`에 설정할 때 두 모드에 생성한다는 뜻이다. 체크는 실제 화면 확인을 뜻하지 않는다. 생성기가 아래 공식 색인에 있는 ID만 받으므로 오타나 색인에 없는 확장 전용 ID는 오류로 표시된다.

| 적용 경로 | 설정 가능한 값 | 공통 / 합일 / 전용 |
|---|---|---|
| `colors` | 작업 화면 요소의 전경·배경·경계·상태 색. 반투명이 필요한 오버레이는 알파를 보존 | 아래 ID별로 추적 |
| `tokenColors[].scope` / `.settings` | TextMate scope별 `foreground`, `fontStyle` | [x] 공통 토큰과 연결된 언어별 규칙에 일반·굵게·기울임을 출력한다. |
| `semanticHighlighting`, `semanticTokenColors` | 의미 강조 참여 여부와 선택자별 색·서체 | [x] 함수·타입·매크로·매개변수 등 공통 선택자와 언어별 재정의. provider가 반환한 토큰만 적용. [표준 타입·수식어 및 fallback](https://code.visualstudio.com/api/language-extensions/semantic-highlight-guide) |
| 사용자 설정 조각 | `editor.fontFamily`, `editor.fontSize`, `editor.lineHeight`, `terminal.integrated.fontFamily`, `terminal.integrated.fontSize` | [x] 선택 병합용 설정 조각 생성. 합자 설정은 수동이며 색 테마에서 강제하지 않음 |
| 아이콘 | 파일 아이콘 / 제품 아이콘 테마 선택 | 별도 확장 기여점. 로컬 아이콘 폰트가 있다고 구현된 것이 아님 |

**검증 기록:** 기본 빌드에 두 모드 각각 색 ID 35개를 유지한다. 그 외 색은 설정한 경우에만 생성한다. `vscode.toml`의 주석 예시는 권장 조합이며 대장의 모든 ID를 설정할 수 있다. 검색과 diff 등에는 코드 가독성을 위한 기본 알파를 제공하며 `역할@불투명도`(0~100 정수)로 덮어쓸 수 있다. 기존 VSIX는 Windows VS Code 1.147.0 임시 프로필에서 설치와 확장 등록을 확인했다. 새 옵션의 실제 화면은 미검증이다.

## 공식 작업 화면 색 ID

고정본에서 이름을 명시한 색 ID 971개. 각 행의 체크는 해당 모드의 생성 근거가 있을 때만 갱신한다. 일부 모드만 구현하면 행을 분리한다.

| 색 ID | 공식 분류 | LBRX 분류 / 원본 | 생성 |
|---|---|---|---|
| `contrastActiveBorder` | Contrast colors | VS Code 전용 | [x] 선택 생성 |
| `contrastBorder` | Contrast colors | VS Code 전용 | [x] 선택 생성 |
| `focusBorder` | Base colors | 공통 · focus | [x] |
| `foreground` | Base colors | 공통 · text | [x] |
| `disabledForeground` | Base colors | VS Code 전용 | [x] 선택 생성 |
| `widget.border` | Base colors | VS Code 전용 | [x] 선택 생성 |
| `widget.shadow` | Base colors | VS Code 전용 | [x] 선택 생성 |
| `selection.background` | Base colors | VS Code 전용 | [x] 선택 생성 |
| `descriptionForeground` | Base colors | VS Code 전용 | [x] 선택 생성 |
| `errorForeground` | Base colors | 공통 · error | [x] |
| `icon.foreground` | Base colors | VS Code 전용 | [x] 선택 생성 |
| `sash.hoverBorder` | Base colors | VS Code 전용 | [x] 선택 생성 |
| `window.activeBorder` | Window border | VS Code 전용 | [x] 선택 생성 |
| `window.inactiveBorder` | Window border | VS Code 전용 | [x] 선택 생성 |
| `surface.background` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `surface.foreground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `surface.border` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `editor.border` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernPanel.border` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernSash.gripForeground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernTab.activeBackground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernTab.activeForeground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernTab.hoverBackground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernTab.hoverForeground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernEditorTab.activeBackground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernEditorTab.activeActionBackground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernEditorTab.activeForeground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernEditorTab.inactiveBackground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernEditorTab.hoverBackground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernEditorTab.hoverActionBackground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernEditorTab.hoverForeground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernEditorTab.activeHoverBackground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernEditorTab.activeHoverActionBackground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernEditorTab.selectedActionBackground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernActivityBar.background` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernActivityBar.inactiveBackground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernActivityBarItem.activeBackground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernActivityBarItem.activeForeground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernActivityBarItem.hoverBackground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernActivityBarItem.hoverForeground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernActivityBar.border` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernUI.shellBackground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `modernUI.inactiveShellBackground` | Modern UI colors | VS Code 전용 | [x] 선택 생성 |
| `textBlockQuote.background` | Text colors | VS Code 전용 | [x] 선택 생성 |
| `textBlockQuote.border` | Text colors | VS Code 전용 | [x] 선택 생성 |
| `textCodeBlock.background` | Text colors | VS Code 전용 | [x] 선택 생성 |
| `textLink.activeForeground` | Text colors | VS Code 전용 | [x] 선택 생성 |
| `textLink.foreground` | Text colors | 공통 · link | [x] |
| `textPreformat.foreground` | Text colors | VS Code 전용 | [x] 선택 생성 |
| `textPreformat.background` | Text colors | VS Code 전용 | [x] 선택 생성 |
| `textPreformat.border` | Text colors | VS Code 전용 | [x] 선택 생성 |
| `textSeparator.foreground` | Text colors | VS Code 전용 | [x] 선택 생성 |
| `toolbar.hoverBackground` | Action colors | VS Code 전용 | [x] 선택 생성 |
| `toolbar.hoverOutline` | Action colors | VS Code 전용 | [x] 선택 생성 |
| `toolbar.activeBackground` | Action colors | VS Code 전용 | [x] 선택 생성 |
| `editorActionList.background` | Action colors | VS Code 전용 | [x] 선택 생성 |
| `editorActionList.foreground` | Action colors | VS Code 전용 | [x] 선택 생성 |
| `editorActionList.focusForeground` | Action colors | VS Code 전용 | [x] 선택 생성 |
| `editorActionList.focusBackground` | Action colors | VS Code 전용 | [x] 선택 생성 |
| `button.background` | Button control | VS Code 전용 | [x] 선택 생성 |
| `button.foreground` | Button control | VS Code 전용 | [x] 선택 생성 |
| `button.border` | Button control | VS Code 전용 | [x] 선택 생성 |
| `button.separator` | Button control | VS Code 전용 | [x] 선택 생성 |
| `button.hoverBackground` | Button control | VS Code 전용 | [x] 선택 생성 |
| `button.secondaryForeground` | Button control | VS Code 전용 | [x] 선택 생성 |
| `button.secondaryBackground` | Button control | VS Code 전용 | [x] 선택 생성 |
| `button.secondaryHoverBackground` | Button control | VS Code 전용 | [x] 선택 생성 |
| `button.secondaryBorder` | Button control | VS Code 전용 | [x] 선택 생성 |
| `checkbox.background` | Button control | VS Code 전용 | [x] 선택 생성 |
| `checkbox.foreground` | Button control | VS Code 전용 | [x] 선택 생성 |
| `checkbox.disabled.background` | Button control | VS Code 전용 | [x] 선택 생성 |
| `checkbox.disabled.foreground` | Button control | VS Code 전용 | [x] 선택 생성 |
| `checkbox.border` | Button control | VS Code 전용 | [x] 선택 생성 |
| `checkbox.selectBackground` | Button control | VS Code 전용 | [x] 선택 생성 |
| `checkbox.selectBorder` | Button control | VS Code 전용 | [x] 선택 생성 |
| `radio.activeForeground` | Button control | VS Code 전용 | [x] 선택 생성 |
| `radio.activeBackground` | Button control | VS Code 전용 | [x] 선택 생성 |
| `radio.activeBorder` | Button control | VS Code 전용 | [x] 선택 생성 |
| `radio.inactiveForeground` | Button control | VS Code 전용 | [x] 선택 생성 |
| `radio.inactiveBackground` | Button control | VS Code 전용 | [x] 선택 생성 |
| `radio.inactiveBorder` | Button control | VS Code 전용 | [x] 선택 생성 |
| `radio.inactiveHoverBackground` | Button control | VS Code 전용 | [x] 선택 생성 |
| `dropdown.background` | Dropdown control | VS Code 전용 | [x] 선택 생성 |
| `dropdown.listBackground` | Dropdown control | VS Code 전용 | [x] 선택 생성 |
| `dropdown.border` | Dropdown control | VS Code 전용 | [x] 선택 생성 |
| `dropdown.foreground` | Dropdown control | VS Code 전용 | [x] 선택 생성 |
| `input.background` | Input control | 공통 · background | [x] |
| `input.border` | Input control | VS Code 전용 | [x] 선택 생성 |
| `input.foreground` | Input control | 공통 · text | [x] |
| `input.placeholderForeground` | Input control | VS Code 전용 | [x] 선택 생성 |
| `inputOption.activeBackground` | Input control | VS Code 전용 | [x] 선택 생성 |
| `inputOption.activeBorder` | Input control | VS Code 전용 | [x] 선택 생성 |
| `inputOption.activeForeground` | Input control | VS Code 전용 | [x] 선택 생성 |
| `inputOption.hoverBackground` | Input control | VS Code 전용 | [x] 선택 생성 |
| `inputValidation.errorBackground` | Input control | VS Code 전용 | [x] 선택 생성 |
| `inputValidation.errorForeground` | Input control | VS Code 전용 | [x] 선택 생성 |
| `inputValidation.errorBorder` | Input control | VS Code 전용 | [x] 선택 생성 |
| `inputValidation.infoBackground` | Input control | VS Code 전용 | [x] 선택 생성 |
| `inputValidation.infoForeground` | Input control | VS Code 전용 | [x] 선택 생성 |
| `inputValidation.infoBorder` | Input control | VS Code 전용 | [x] 선택 생성 |
| `inputValidation.warningBackground` | Input control | VS Code 전용 | [x] 선택 생성 |
| `inputValidation.warningForeground` | Input control | VS Code 전용 | [x] 선택 생성 |
| `inputValidation.warningBorder` | Input control | VS Code 전용 | [x] 선택 생성 |
| `scrollbar.background` | Scrollbar control | VS Code 전용 | [x] 선택 생성 |
| `scrollbar.shadow` | Scrollbar control | VS Code 전용 | [x] 선택 생성 |
| `scrollbarSlider.activeBackground` | Scrollbar control | VS Code 전용 | [x] 선택 생성 |
| `scrollbarSlider.background` | Scrollbar control | VS Code 전용 | [x] 선택 생성 |
| `scrollbarSlider.hoverBackground` | Scrollbar control | VS Code 전용 | [x] 선택 생성 |
| `badge.foreground` | Badge | VS Code 전용 | [x] 선택 생성 |
| `badge.background` | Badge | VS Code 전용 | [x] 선택 생성 |
| `progressBar.background` | Progress bar | VS Code 전용 | [x] 선택 생성 |
| `list.activeSelectionBackground` | Lists and trees | `selection_background` 권장 | [x] 선택 생성 |
| `list.activeSelectionForeground` | Lists and trees | `selection_text` 권장 | [x] 선택 생성 |
| `list.activeSelectionIconForeground` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.dropBackground` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.focusBackground` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.focusForeground` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.focusHighlightForeground` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.focusOutline` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.focusAndSelectionOutline` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.highlightForeground` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.hoverBackground` | Lists and trees | `selection_background` 권장 | [x] 선택 생성 |
| `list.hoverForeground` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.inactiveSelectionBackground` | Lists and trees | `surface` 권장 | [x] 선택 생성 |
| `list.inactiveSelectionForeground` | Lists and trees | `text` 권장 | [x] 선택 생성 |
| `list.inactiveSelectionIconForeground` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.inactiveFocusBackground` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.inactiveFocusOutline` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.invalidItemForeground` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.errorForeground` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.warningForeground` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `listFilterWidget.background` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `listFilterWidget.outline` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `listFilterWidget.noMatchesOutline` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `listFilterWidget.shadow` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.filterMatchBackground` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.filterMatchBorder` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.deemphasizedForeground` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `list.dropBetweenBackground` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `tree.indentGuidesStroke` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `tree.inactiveIndentGuidesStroke` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `tree.tableColumnsBorder` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `tree.tableOddRowsBackground` | Lists and trees | VS Code 전용 | [x] 선택 생성 |
| `activityBar.background` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityBar.dropBorder` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityBar.foreground` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityBar.inactiveForeground` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityBar.border` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityBarBadge.background` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityBarBadge.foreground` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityBar.activeBorder` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityBar.activeBackground` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityBar.activeFocusBorder` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityBarTop.foreground` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityBarTop.activeBorder` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityBarTop.inactiveForeground` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityBarTop.dropBorder` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityBarTop.background` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityBarTop.activeBackground` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityWarningBadge.foreground` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityWarningBadge.background` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityErrorBadge.foreground` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `activityErrorBadge.background` | Activity Bar | VS Code 전용 | [x] 선택 생성 |
| `profileBadge.background` | Profiles | VS Code 전용 | [x] 선택 생성 |
| `profileBadge.foreground` | Profiles | VS Code 전용 | [x] 선택 생성 |
| `profiles.sashBorder` | Profiles | VS Code 전용 | [x] 선택 생성 |
| `sideBar.background` | Side Bar | 공통 · surface | [x] |
| `sideBar.foreground` | Side Bar | 공통 · text | [x] |
| `sideBar.border` | Side Bar | VS Code 전용 | [x] 선택 생성 |
| `sideBar.dropBackground` | Side Bar | VS Code 전용 | [x] 선택 생성 |
| `sideBarTitle.foreground` | Side Bar | VS Code 전용 | [x] 선택 생성 |
| `sideBarSectionHeader.background` | Side Bar | `surface` 권장 | [x] 선택 생성 |
| `sideBarSectionHeader.foreground` | Side Bar | `muted` 권장 | [x] 선택 생성 |
| `sideBarSectionHeader.border` | Side Bar | VS Code 전용 | [x] 선택 생성 |
| `sideBarActivityBarTop.border` | Side Bar | VS Code 전용 | [x] 선택 생성 |
| `sideBarTitle.background` | Side Bar | VS Code 전용 | [x] 선택 생성 |
| `sideBarTitle.border` | Side Bar | VS Code 전용 | [x] 선택 생성 |
| `sideBarStickyScroll.background` | Side Bar | VS Code 전용 | [x] 선택 생성 |
| `sideBarStickyScroll.border` | Side Bar | VS Code 전용 | [x] 선택 생성 |
| `sideBarStickyScroll.shadow` | Side Bar | VS Code 전용 | [x] 선택 생성 |
| `minimap.findMatchHighlight` | Minimap | VS Code 전용 | [x] 선택 생성 |
| `minimap.selectionHighlight` | Minimap | VS Code 전용 | [x] 선택 생성 |
| `minimap.errorHighlight` | Minimap | VS Code 전용 | [x] 선택 생성 |
| `minimap.warningHighlight` | Minimap | VS Code 전용 | [x] 선택 생성 |
| `minimap.background` | Minimap | VS Code 전용 | [x] 선택 생성 |
| `minimap.selectionOccurrenceHighlight` | Minimap | VS Code 전용 | [x] 선택 생성 |
| `minimap.foregroundOpacity` | Minimap | VS Code 전용 | [x] 선택 생성 |
| `minimap.infoHighlight` | Minimap | VS Code 전용 | [x] 선택 생성 |
| `minimap.chatEditHighlight` | Minimap | VS Code 전용 | [x] 선택 생성 |
| `minimapSlider.background` | Minimap | VS Code 전용 | [x] 선택 생성 |
| `minimapSlider.hoverBackground` | Minimap | VS Code 전용 | [x] 선택 생성 |
| `minimapSlider.activeBackground` | Minimap | VS Code 전용 | [x] 선택 생성 |
| `minimapGutter.addedBackground` | Minimap | VS Code 전용 | [x] 선택 생성 |
| `minimapGutter.modifiedBackground` | Minimap | VS Code 전용 | [x] 선택 생성 |
| `minimapGutter.deletedBackground` | Minimap | VS Code 전용 | [x] 선택 생성 |
| `editorMinimap.inlineChatInserted` | Minimap | VS Code 전용 | [x] 선택 생성 |
| `editorGroup.border` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `editorGroup.dropBackground` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `editorGroupHeader.noTabsBackground` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `editorGroupHeader.tabsBackground` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `editorGroupHeader.tabsBorder` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `editorGroupHeader.border` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `editorGroup.emptyBackground` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `editorGroup.focusedEmptyBorder` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `editorGroup.dropIntoPromptForeground` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `editorGroup.dropIntoPromptBackground` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `editorGroup.dropIntoPromptBorder` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.activeBackground` | Editor Groups & Tabs | `background` 권장 | [x] 선택 생성 |
| `tab.unfocusedActiveBackground` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.activeForeground` | Editor Groups & Tabs | `text` 권장 | [x] 선택 생성 |
| `tab.border` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.activeBorder` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.selectedBorderTop` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.selectedBackground` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.selectedForeground` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.dragAndDropBorder` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.unfocusedActiveBorder` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.activeBorderTop` | Editor Groups & Tabs | `focus` 권장 | [x] 선택 생성 |
| `tab.unfocusedActiveBorderTop` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.lastPinnedBorder` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.inactiveBackground` | Editor Groups & Tabs | `surface` 권장 | [x] 선택 생성 |
| `tab.unfocusedInactiveBackground` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.inactiveForeground` | Editor Groups & Tabs | `muted` 권장 | [x] 선택 생성 |
| `tab.unfocusedActiveForeground` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.unfocusedInactiveForeground` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.hoverBackground` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.unfocusedHoverBackground` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.hoverForeground` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.unfocusedHoverForeground` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.hoverBorder` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.unfocusedHoverBorder` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.activeModifiedBorder` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.inactiveModifiedBorder` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.unfocusedActiveModifiedBorder` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `tab.unfocusedInactiveModifiedBorder` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `editorPane.background` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `sideBySideEditor.horizontalBorder` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `sideBySideEditor.verticalBorder` | Editor Groups & Tabs | VS Code 전용 | [x] 선택 생성 |
| `editor.background` | Editor colors | 공통 · background | [x] |
| `editor.foreground` | Editor colors | 공통 · text | [x] |
| `editorLineNumber.foreground` | Editor colors | 공통 · muted | [x] |
| `editorLineNumber.activeForeground` | Editor colors | 공통 역할 `focus` 권장 | [x] 선택 생성 |
| `editorLineNumber.dimmedForeground` | Editor colors | 공통 역할 `muted` 권장 | [x] 선택 생성 |
| `editorCursor.background` | Editor colors | 공통 역할 `background` 권장 | [x] 선택 생성 |
| `editorCursor.foreground` | Editor colors | 공통 · focus | [x] |
| `editorMultiCursor.primary.foreground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorMultiCursor.primary.background` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorMultiCursor.secondary.foreground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorMultiCursor.secondary.background` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.placeholder.foreground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.compositionBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.selectionBackground` | Editor colors | 공통 · selection_background | [x] |
| `editor.selectionForeground` | Editor colors | 공통 · selection_text | [x] |
| `editor.inactiveSelectionBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.selectionHighlightBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.selectionHighlightBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.wordHighlightBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.wordHighlightBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.wordHighlightStrongBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.wordHighlightStrongBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.wordHighlightTextBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.wordHighlightTextBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.findMatchBackground` | Editor colors | `warning` 권장 | [x] 선택 생성 |
| `editor.findMatchForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.findMatchHighlightForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.findMatchHighlightBackground` | Editor colors | `focus` 권장 | [x] 선택 생성 |
| `editor.findRangeHighlightBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.findMatchBorder` | Editor colors | `warning` 권장 | [x] 선택 생성 |
| `editor.findMatchHighlightBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.findRangeHighlightBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `search.resultsInfoForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `searchEditor.findMatchBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `searchEditor.findMatchBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `searchEditor.textInputBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.hoverHighlightBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.lineHighlightBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.inactiveLineHighlightBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.lineHighlightBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorUnicodeHighlight.border` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorUnicodeHighlight.background` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorLink.activeForeground` | Editor colors | 공통 · link | [x] 선택 생성 |
| `editor.rangeHighlightBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.rangeHighlightBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.symbolHighlightBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.symbolHighlightBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorWhitespace.foreground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorIndentGuide.background` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorIndentGuide.background1` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorIndentGuide.background2` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorIndentGuide.background3` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorIndentGuide.background4` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorIndentGuide.background5` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorIndentGuide.background6` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorIndentGuide.activeBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorIndentGuide.activeBackground1` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorIndentGuide.activeBackground2` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorIndentGuide.activeBackground3` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorIndentGuide.activeBackground4` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorIndentGuide.activeBackground5` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorIndentGuide.activeBackground6` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorInlayHint.background` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorInlayHint.foreground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorInlayHint.typeForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorInlayHint.typeBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorInlayHint.parameterForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorInlayHint.parameterBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorRuler.foreground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.linkedEditingBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorCodeLens.foreground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorLightBulb.foreground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorLightBulbAutoFix.foreground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorLightBulbAi.foreground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketMatch.background` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketMatch.border` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketMatch.foreground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketHighlight.foreground1` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketHighlight.foreground2` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketHighlight.foreground3` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketHighlight.foreground4` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketHighlight.foreground5` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketHighlight.foreground6` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketHighlight.unexpectedBracket.foreground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketPairGuide.activeBackground1` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketPairGuide.activeBackground2` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketPairGuide.activeBackground3` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketPairGuide.activeBackground4` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketPairGuide.activeBackground5` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketPairGuide.activeBackground6` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketPairGuide.background1` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketPairGuide.background2` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketPairGuide.background3` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketPairGuide.background4` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketPairGuide.background5` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorBracketPairGuide.background6` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.foldBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editor.foldPlaceholderForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.background` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.border` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.findMatchForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.rangeHighlightForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.selectionHighlightForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.wordHighlightForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.wordHighlightStrongForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.wordHighlightTextForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.modifiedForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.addedForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.deletedForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.errorForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.warningForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.infoForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.bracketMatchForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.inlineChatInserted` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.inlineChatRemoved` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.commentDraftForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorError.foreground` | Editor colors | 공통 역할 `error` 권장 | [x] 선택 생성 |
| `editorError.border` | Editor colors | 공통 역할 `error` 권장 | [x] 선택 생성 |
| `editorError.background` | Editor colors | 공통 역할 `error` 권장 | [x] 선택 생성 |
| `editorWarning.foreground` | Editor colors | 공통 역할 `warning` 권장 | [x] 선택 생성 |
| `editorWarning.border` | Editor colors | 공통 역할 `warning` 권장 | [x] 선택 생성 |
| `editorWarning.background` | Editor colors | 공통 역할 `warning` 권장 | [x] 선택 생성 |
| `editorInfo.foreground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorInfo.border` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorInfo.background` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorHint.foreground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorHint.border` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `problemsErrorIcon.foreground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `problemsWarningIcon.foreground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `problemsInfoIcon.foreground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorUnnecessaryCode.border` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorUnnecessaryCode.opacity` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorGutter.background` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorGutter.modifiedBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorGutter.modifiedSecondaryBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorGutter.addedBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorGutter.addedSecondaryBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorGutter.deletedBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorGutter.deletedSecondaryBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorGutter.commentRangeForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorGutter.commentGlyphForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorGutter.commentUnresolvedGlyphForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorGutter.foldingControlForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorGutter.itemGlyphForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorGutter.itemBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorGutter.commentDraftGlyphForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorCommentsWidget.resolvedBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorCommentsWidget.unresolvedBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorCommentsWidget.rangeBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorCommentsWidget.rangeActiveBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `editorCommentsWidget.replyInputBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.gutterIndicator.primaryBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.gutterIndicator.primaryForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.gutterIndicator.primaryBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.gutterIndicator.secondaryBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.gutterIndicator.secondaryForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.gutterIndicator.secondaryBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.gutterIndicator.successfulBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.gutterIndicator.successfulForeground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.gutterIndicator.successfulBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.gutterIndicator.background` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.originalBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.modifiedBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.originalChangedLineBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.originalChangedTextBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.modifiedChangedLineBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.modifiedChangedTextBackground` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.originalBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.modifiedBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.tabWillAcceptModifiedBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `inlineEdit.tabWillAcceptOriginalBorder` | Editor colors | VS Code 전용 | [x] 선택 생성 |
| `diffEditor.insertedTextBackground` | Diff editor colors | `success` 권장 | [x] 선택 생성 |
| `diffEditor.insertedTextBorder` | Diff editor colors | 공통 역할 `success` 권장 | [x] 선택 생성 |
| `diffEditor.removedTextBackground` | Diff editor colors | `error` 권장 | [x] 선택 생성 |
| `diffEditor.removedTextBorder` | Diff editor colors | 공통 역할 `error` 권장 | [x] 선택 생성 |
| `diffEditor.border` | Diff editor colors | 공통 역할 `muted` 권장 | [x] 선택 생성 |
| `diffEditor.diagonalFill` | Diff editor colors | 공통 역할 `muted` 권장 | [x] 선택 생성 |
| `diffEditor.insertedLineBackground` | Diff editor colors | `success` 권장 | [x] 선택 생성 |
| `diffEditor.removedLineBackground` | Diff editor colors | `error` 권장 | [x] 선택 생성 |
| `diffEditorGutter.insertedLineBackground` | Diff editor colors | VS Code 전용 | [x] 선택 생성 |
| `diffEditorGutter.removedLineBackground` | Diff editor colors | VS Code 전용 | [x] 선택 생성 |
| `diffEditorOverview.insertedForeground` | Diff editor colors | VS Code 전용 | [x] 선택 생성 |
| `diffEditorOverview.removedForeground` | Diff editor colors | VS Code 전용 | [x] 선택 생성 |
| `diffEditor.unchangedRegionBackground` | Diff editor colors | 공통 역할 `surface` 권장 | [x] 선택 생성 |
| `diffEditor.unchangedRegionForeground` | Diff editor colors | 공통 역할 `muted` 권장 | [x] 선택 생성 |
| `diffEditor.unchangedRegionShadow` | Diff editor colors | 공통 역할 `muted` 권장 | [x] 선택 생성 |
| `diffEditor.unchangedCodeBackground` | Diff editor colors | 공통 역할 `muted` 권장 | [x] 선택 생성 |
| `diffEditor.move.border` | Diff editor colors | 공통 역할 `muted` 권장 | [x] 선택 생성 |
| `diffEditor.moveActive.border` | Diff editor colors | 공통 역할 `focus` 권장 | [x] 선택 생성 |
| `multiDiffEditor.headerBackground` | Diff editor colors | VS Code 전용 | [x] 선택 생성 |
| `multiDiffEditor.background` | Diff editor colors | VS Code 전용 | [x] 선택 생성 |
| `multiDiffEditor.border` | Diff editor colors | VS Code 전용 | [x] 선택 생성 |
| `chat.requestBorder` | Chat colors | VS Code 전용 | [x] 선택 생성 |
| `chat.requestBackground` | Chat colors | VS Code 전용 | [x] 선택 생성 |
| `chat.slashCommandBackground` | Chat colors | VS Code 전용 | [x] 선택 생성 |
| `chat.slashCommandForeground` | Chat colors | VS Code 전용 | [x] 선택 생성 |
| `chat.avatarBackground` | Chat colors | VS Code 전용 | [x] 선택 생성 |
| `chat.avatarForeground` | Chat colors | VS Code 전용 | [x] 선택 생성 |
| `chat.editedFileForeground` | Chat colors | VS Code 전용 | [x] 선택 생성 |
| `chat.linesAddedForeground` | Chat colors | VS Code 전용 | [x] 선택 생성 |
| `chat.linesRemovedForeground` | Chat colors | VS Code 전용 | [x] 선택 생성 |
| `chat.requestCodeBorder` | Chat colors | VS Code 전용 | [x] 선택 생성 |
| `chat.requestBubbleBackground` | Chat colors | VS Code 전용 | [x] 선택 생성 |
| `chat.requestBubbleHoverBackground` | Chat colors | VS Code 전용 | [x] 선택 생성 |
| `chat.checkpointSeparator` | Chat colors | VS Code 전용 | [x] 선택 생성 |
| `chat.thinkingShimmer` | Chat colors | VS Code 전용 | [x] 선택 생성 |
| `chatManagement.sashBorder` | Chat colors | VS Code 전용 | [x] 선택 생성 |
| `agents.background` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsPanel.background` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsPanel.foreground` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsPanel.border` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsCard.border` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsBottomPanel.border` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsGradient.tintColor` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentFeedbackEditorWidget.background` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentFeedbackEditorWidget.border` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentFeedbackInputWidget.border` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsUpdateButton.downloadingBackground` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsUpdateButton.downloadedBackground` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsChatInput.background` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsChatInput.foreground` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsChatInput.border` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsChatInput.focusBorder` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsChatInput.placeholderForeground` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsNewSessionButton.background` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsNewSessionButton.foreground` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsNewSessionButton.border` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsNewSessionButton.hoverBackground` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsBadge.background` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsBadge.foreground` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsUnreadBadge.background` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsUnreadBadge.foreground` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `activeSessionView.background` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `inactiveSessionView.background` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `activeSessionView.foreground` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `inactiveSessionView.foreground` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsMobileDiff.addedForeground` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsMobileDiff.modifiedForeground` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `agentsMobileDiff.deletedForeground` | Agent sessions colors | VS Code 전용 | [x] 선택 생성 |
| `inlineChat.background` | Inline Chat colors | VS Code 전용 | [x] 선택 생성 |
| `inlineChat.foreground` | Inline Chat colors | VS Code 전용 | [x] 선택 생성 |
| `inlineChat.border` | Inline Chat colors | VS Code 전용 | [x] 선택 생성 |
| `inlineChat.shadow` | Inline Chat colors | VS Code 전용 | [x] 선택 생성 |
| `inlineChatInput.border` | Inline Chat colors | VS Code 전용 | [x] 선택 생성 |
| `inlineChatInput.focusBorder` | Inline Chat colors | VS Code 전용 | [x] 선택 생성 |
| `inlineChatInput.placeholderForeground` | Inline Chat colors | VS Code 전용 | [x] 선택 생성 |
| `inlineChatInput.background` | Inline Chat colors | VS Code 전용 | [x] 선택 생성 |
| `inlineChatDiff.inserted` | Inline Chat colors | VS Code 전용 | [x] 선택 생성 |
| `inlineChatDiff.removed` | Inline Chat colors | VS Code 전용 | [x] 선택 생성 |
| `interactive.activeCodeBorder` | Panel Chat colors | VS Code 전용 | [x] 선택 생성 |
| `interactive.inactiveCodeBorder` | Panel Chat colors | VS Code 전용 | [x] 선택 생성 |
| `editorWidget.foreground` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorWidget.background` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorWidget.border` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorWidget.resizeBorder` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorSuggestWidget.background` | Editor widget colors | `surface` 권장 | [x] 선택 생성 |
| `editorSuggestWidget.border` | Editor widget colors | `focus` 권장 | [x] 선택 생성 |
| `editorSuggestWidget.foreground` | Editor widget colors | `text` 권장 | [x] 선택 생성 |
| `editorSuggestWidget.focusHighlightForeground` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorSuggestWidget.highlightForeground` | Editor widget colors | `focus` 권장 | [x] 선택 생성 |
| `editorSuggestWidget.selectedBackground` | Editor widget colors | `selection_background` 권장 | [x] 선택 생성 |
| `editorSuggestWidget.selectedForeground` | Editor widget colors | `selection_text` 권장 | [x] 선택 생성 |
| `editorSuggestWidget.selectedIconForeground` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorSuggestWidgetStatus.foreground` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorHoverWidget.foreground` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorHoverWidget.background` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorHoverWidget.border` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorHoverWidget.highlightForeground` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorHoverWidget.statusBarBackground` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorGhostText.border` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorGhostText.background` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorGhostText.foreground` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorStickyScroll.background` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorStickyScroll.border` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorStickyScroll.shadow` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorStickyScrollGutter.background` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorStickyScrollHover.background` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `debugExceptionWidget.background` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `debugExceptionWidget.border` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorMarkerNavigation.background` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorMarkerNavigationError.background` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorMarkerNavigationWarning.background` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorMarkerNavigationInfo.background` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorMarkerNavigationError.headerBackground` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorMarkerNavigationWarning.headerBackground` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `editorMarkerNavigationInfo.headerBackground` | Editor widget colors | VS Code 전용 | [x] 선택 생성 |
| `peekView.border` | Peek view colors | VS Code 전용 | [x] 선택 생성 |
| `peekViewEditor.background` | Peek view colors | VS Code 전용 | [x] 선택 생성 |
| `peekViewEditorGutter.background` | Peek view colors | VS Code 전용 | [x] 선택 생성 |
| `peekViewEditor.matchHighlightBackground` | Peek view colors | VS Code 전용 | [x] 선택 생성 |
| `peekViewEditor.matchHighlightBorder` | Peek view colors | VS Code 전용 | [x] 선택 생성 |
| `peekViewResult.background` | Peek view colors | VS Code 전용 | [x] 선택 생성 |
| `peekViewResult.fileForeground` | Peek view colors | VS Code 전용 | [x] 선택 생성 |
| `peekViewResult.lineForeground` | Peek view colors | VS Code 전용 | [x] 선택 생성 |
| `peekViewResult.matchHighlightBackground` | Peek view colors | VS Code 전용 | [x] 선택 생성 |
| `peekViewResult.selectionBackground` | Peek view colors | VS Code 전용 | [x] 선택 생성 |
| `peekViewResult.selectionForeground` | Peek view colors | VS Code 전용 | [x] 선택 생성 |
| `peekViewTitle.background` | Peek view colors | VS Code 전용 | [x] 선택 생성 |
| `peekViewTitleDescription.foreground` | Peek view colors | VS Code 전용 | [x] 선택 생성 |
| `peekViewTitleLabel.foreground` | Peek view colors | VS Code 전용 | [x] 선택 생성 |
| `peekViewEditorStickyScroll.background` | Peek view colors | VS Code 전용 | [x] 선택 생성 |
| `peekViewEditorStickyScrollGutter.background` | Peek view colors | VS Code 전용 | [x] 선택 생성 |
| `merge.currentHeaderBackground` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `merge.currentContentBackground` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `merge.incomingHeaderBackground` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `merge.incomingContentBackground` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `merge.border` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `merge.commonContentBackground` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `merge.commonHeaderBackground` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.currentContentForeground` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.incomingContentForeground` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.commonContentForeground` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.commentForeground` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `editorOverviewRuler.commentUnresolvedForeground` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `mergeEditor.change.background` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `mergeEditor.change.word.background` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `mergeEditor.conflict.unhandledUnfocused.border` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `mergeEditor.conflict.unhandledFocused.border` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `mergeEditor.conflict.handledUnfocused.border` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `mergeEditor.conflict.handledFocused.border` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `mergeEditor.conflict.handled.minimapOverViewRuler` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `mergeEditor.conflict.unhandled.minimapOverViewRuler` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `mergeEditor.conflictingLines.background` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `mergeEditor.changeBase.background` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `mergeEditor.changeBase.word.background` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `mergeEditor.conflict.input1.background` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `mergeEditor.conflict.input2.background` | Merge conflicts colors | VS Code 전용 | [x] 선택 생성 |
| `panel.background` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `panel.border` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `panel.dropBorder` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `panelTitle.activeBorder` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `panelTitle.activeForeground` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `panelTitle.inactiveForeground` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `panelTitle.border` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `panelTitleBadge.background` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `panelTitleBadge.foreground` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `panelInput.border` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `panelSection.border` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `panelSection.dropBackground` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `panelSectionHeader.background` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `panelSectionHeader.foreground` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `panelStickyScroll.background` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `panelStickyScroll.border` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `panelStickyScroll.shadow` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `panelSectionHeader.border` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `outputView.background` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `outputViewStickyScroll.background` | Panel colors | VS Code 전용 | [x] 선택 생성 |
| `statusBar.background` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBar.foreground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBar.border` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBar.debuggingBackground` | Status Bar colors | `warning` 권장 | [x] 선택 생성 |
| `statusBar.debuggingForeground` | Status Bar colors | `background` 권장 | [x] 선택 생성 |
| `statusBar.debuggingBorder` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBar.noFolderForeground` | Status Bar colors | `text` 권장 | [x] 선택 생성 |
| `statusBar.noFolderBackground` | Status Bar colors | `surface` 권장 | [x] 선택 생성 |
| `statusBar.noFolderBorder` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.activeBackground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.hoverForeground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.hoverBackground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.prominentForeground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.prominentBackground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.prominentHoverForeground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.prominentHoverBackground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.remoteBackground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.remoteForeground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.remoteHoverBackground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.remoteHoverForeground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.errorBackground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.errorForeground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.errorHoverBackground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.errorHoverForeground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.warningBackground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.warningForeground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.warningHoverBackground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.warningHoverForeground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.compactHoverBackground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.focusBorder` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBar.focusBorder` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.offlineBackground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.offlineForeground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.offlineHoverForeground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `statusBarItem.offlineHoverBackground` | Status Bar colors | VS Code 전용 | [x] 선택 생성 |
| `titleBar.activeBackground` | Title Bar colors | VS Code 전용 | [x] 선택 생성 |
| `titleBar.activeForeground` | Title Bar colors | VS Code 전용 | [x] 선택 생성 |
| `titleBar.inactiveBackground` | Title Bar colors | VS Code 전용 | [x] 선택 생성 |
| `titleBar.inactiveForeground` | Title Bar colors | VS Code 전용 | [x] 선택 생성 |
| `titleBar.border` | Title Bar colors | VS Code 전용 | [x] 선택 생성 |
| `menubar.selectionForeground` | Menu Bar colors | VS Code 전용 | [x] 선택 생성 |
| `menubar.selectionBackground` | Menu Bar colors | VS Code 전용 | [x] 선택 생성 |
| `menubar.selectionBorder` | Menu Bar colors | VS Code 전용 | [x] 선택 생성 |
| `menu.foreground` | Menu Bar colors | VS Code 전용 | [x] 선택 생성 |
| `menu.background` | Menu Bar colors | VS Code 전용 | [x] 선택 생성 |
| `menu.selectionForeground` | Menu Bar colors | VS Code 전용 | [x] 선택 생성 |
| `menu.selectionBackground` | Menu Bar colors | VS Code 전용 | [x] 선택 생성 |
| `menu.selectionBorder` | Menu Bar colors | VS Code 전용 | [x] 선택 생성 |
| `menu.separatorBackground` | Menu Bar colors | VS Code 전용 | [x] 선택 생성 |
| `menu.border` | Menu Bar colors | VS Code 전용 | [x] 선택 생성 |
| `commandCenter.foreground` | Command Center colors | VS Code 전용 | [x] 선택 생성 |
| `commandCenter.activeForeground` | Command Center colors | VS Code 전용 | [x] 선택 생성 |
| `commandCenter.background` | Command Center colors | VS Code 전용 | [x] 선택 생성 |
| `commandCenter.activeBackground` | Command Center colors | VS Code 전용 | [x] 선택 생성 |
| `commandCenter.border` | Command Center colors | VS Code 전용 | [x] 선택 생성 |
| `commandCenter.inactiveForeground` | Command Center colors | VS Code 전용 | [x] 선택 생성 |
| `commandCenter.inactiveBorder` | Command Center colors | VS Code 전용 | [x] 선택 생성 |
| `commandCenter.activeBorder` | Command Center colors | VS Code 전용 | [x] 선택 생성 |
| `commandCenter.debuggingBackground` | Command Center colors | VS Code 전용 | [x] 선택 생성 |
| `notificationCenter.border` | Notification colors | VS Code 전용 | [x] 선택 생성 |
| `notificationCenterHeader.foreground` | Notification colors | VS Code 전용 | [x] 선택 생성 |
| `notificationCenterHeader.background` | Notification colors | VS Code 전용 | [x] 선택 생성 |
| `notificationToast.border` | Notification colors | VS Code 전용 | [x] 선택 생성 |
| `notifications.foreground` | Notification colors | VS Code 전용 | [x] 선택 생성 |
| `notifications.background` | Notification colors | VS Code 전용 | [x] 선택 생성 |
| `notifications.border` | Notification colors | VS Code 전용 | [x] 선택 생성 |
| `notificationLink.foreground` | Notification colors | VS Code 전용 | [x] 선택 생성 |
| `notificationsErrorIcon.foreground` | Notification colors | VS Code 전용 | [x] 선택 생성 |
| `notificationsWarningIcon.foreground` | Notification colors | VS Code 전용 | [x] 선택 생성 |
| `notificationsInfoIcon.foreground` | Notification colors | VS Code 전용 | [x] 선택 생성 |
| `banner.background` | Banner colors | VS Code 전용 | [x] 선택 생성 |
| `banner.foreground` | Banner colors | VS Code 전용 | [x] 선택 생성 |
| `banner.iconForeground` | Banner colors | VS Code 전용 | [x] 선택 생성 |
| `extensionButton.prominentForeground` | Extensions colors | VS Code 전용 | [x] 선택 생성 |
| `extensionButton.prominentBackground` | Extensions colors | VS Code 전용 | [x] 선택 생성 |
| `extensionButton.prominentHoverBackground` | Extensions colors | VS Code 전용 | [x] 선택 생성 |
| `extensionButton.background` | Extensions colors | VS Code 전용 | [x] 선택 생성 |
| `extensionButton.foreground` | Extensions colors | VS Code 전용 | [x] 선택 생성 |
| `extensionButton.hoverBackground` | Extensions colors | VS Code 전용 | [x] 선택 생성 |
| `extensionButton.separator` | Extensions colors | VS Code 전용 | [x] 선택 생성 |
| `extensionButton.border` | Extensions colors | VS Code 전용 | [x] 선택 생성 |
| `extensionBadge.remoteBackground` | Extensions colors | VS Code 전용 | [x] 선택 생성 |
| `extensionBadge.remoteForeground` | Extensions colors | VS Code 전용 | [x] 선택 생성 |
| `extensionIcon.starForeground` | Extensions colors | VS Code 전용 | [x] 선택 생성 |
| `extensionIcon.verifiedForeground` | Extensions colors | VS Code 전용 | [x] 선택 생성 |
| `extensionIcon.preReleaseForeground` | Extensions colors | VS Code 전용 | [x] 선택 생성 |
| `extensionIcon.sponsorForeground` | Extensions colors | VS Code 전용 | [x] 선택 생성 |
| `extensionIcon.privateForeground` | Extensions colors | VS Code 전용 | [x] 선택 생성 |
| `mcpIcon.starForeground` | Extensions colors | VS Code 전용 | [x] 선택 생성 |
| `pickerGroup.border` | Quick picker colors | VS Code 전용 | [x] 선택 생성 |
| `pickerGroup.foreground` | Quick picker colors | VS Code 전용 | [x] 선택 생성 |
| `quickInput.background` | Quick picker colors | VS Code 전용 | [x] 선택 생성 |
| `quickInput.foreground` | Quick picker colors | VS Code 전용 | [x] 선택 생성 |
| `quickInputList.focusBackground` | Quick picker colors | VS Code 전용 | [x] 선택 생성 |
| `quickInputList.focusForeground` | Quick picker colors | VS Code 전용 | [x] 선택 생성 |
| `quickInputList.focusIconForeground` | Quick picker colors | VS Code 전용 | [x] 선택 생성 |
| `quickInputTitle.background` | Quick picker colors | VS Code 전용 | [x] 선택 생성 |
| `keybindingLabel.background` | Keybinding label colors | VS Code 전용 | [x] 선택 생성 |
| `keybindingLabel.foreground` | Keybinding label colors | VS Code 전용 | [x] 선택 생성 |
| `keybindingLabel.border` | Keybinding label colors | VS Code 전용 | [x] 선택 생성 |
| `keybindingLabel.bottomBorder` | Keybinding label colors | VS Code 전용 | [x] 선택 생성 |
| `keybindingTable.headerBackground` | Keyboard shortcut table colors | VS Code 전용 | [x] 선택 생성 |
| `keybindingTable.rowsBackground` | Keyboard shortcut table colors | VS Code 전용 | [x] 선택 생성 |
| `terminal.background` | Integrated Terminal colors | 공통 · terminal_background | [x] |
| `terminal.border` | Integrated Terminal colors | 공통 역할 `muted` 권장 | [x] 선택 생성 |
| `terminal.foreground` | Integrated Terminal colors | 공통 · terminal_foreground | [x] |
| `terminal.ansiBlack` | Integrated Terminal colors | 공통 · ANSI 0 | [x] |
| `terminal.ansiBlue` | Integrated Terminal colors | 공통 · ANSI 4 | [x] |
| `terminal.ansiBrightBlack` | Integrated Terminal colors | 공통 · ANSI 8 | [x] |
| `terminal.ansiBrightBlue` | Integrated Terminal colors | 공통 · ANSI 12 | [x] |
| `terminal.ansiBrightCyan` | Integrated Terminal colors | 공통 · ANSI 14 | [x] |
| `terminal.ansiBrightGreen` | Integrated Terminal colors | 공통 · ANSI 10 | [x] |
| `terminal.ansiBrightMagenta` | Integrated Terminal colors | 공통 · ANSI 13 | [x] |
| `terminal.ansiBrightRed` | Integrated Terminal colors | 공통 · ANSI 9 | [x] |
| `terminal.ansiBrightWhite` | Integrated Terminal colors | 공통 · ANSI 15 | [x] |
| `terminal.ansiBrightYellow` | Integrated Terminal colors | 공통 · ANSI 11 | [x] |
| `terminal.ansiCyan` | Integrated Terminal colors | 공통 · ANSI 6 | [x] |
| `terminal.ansiGreen` | Integrated Terminal colors | 공통 · ANSI 2 | [x] |
| `terminal.ansiMagenta` | Integrated Terminal colors | 공통 · ANSI 5 | [x] |
| `terminal.ansiRed` | Integrated Terminal colors | 공통 · ANSI 1 | [x] |
| `terminal.ansiWhite` | Integrated Terminal colors | 공통 · ANSI 7 | [x] |
| `terminal.ansiYellow` | Integrated Terminal colors | 공통 · ANSI 3 | [x] |
| `terminal.selectionBackground` | Integrated Terminal colors | 공통 · selection_background | [x] |
| `terminal.selectionForeground` | Integrated Terminal colors | 공통 · selection_text | [x] |
| `terminal.inactiveSelectionBackground` | Integrated Terminal colors | 공통 역할 `selection_background` 권장 | [x] 선택 생성 |
| `terminal.findMatchBackground` | Integrated Terminal colors | 공통 역할 `warning` 권장 | [x] 선택 생성 |
| `terminal.findMatchBorder` | Integrated Terminal colors | 공통 역할 `warning` 권장 | [x] 선택 생성 |
| `terminal.findMatchHighlightBackground` | Integrated Terminal colors | 공통 역할 `focus` 권장 | [x] 선택 생성 |
| `terminal.findMatchHighlightBorder` | Integrated Terminal colors | 공통 역할 `focus` 권장 | [x] 선택 생성 |
| `terminal.hoverHighlightBackground` | Integrated Terminal colors | 공통 역할 `focus` 권장 | [x] 선택 생성 |
| `terminalCursor.background` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalCursor.foreground` | Integrated Terminal colors | 공통 · terminal_cursor | [x] |
| `terminal.dropBackground` | Integrated Terminal colors | 공통 역할 `focus` 권장 | [x] 선택 생성 |
| `terminal.tab.activeBorder` | Integrated Terminal colors | 공통 역할 `focus` 권장 | [x] 선택 생성 |
| `terminalCommandDecoration.defaultBackground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalCommandDecoration.successBackground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalCommandDecoration.errorBackground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalOverviewRuler.cursorForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalOverviewRuler.findMatchForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalStickyScroll.background` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalStickyScroll.border` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalStickyScrollHover.background` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminal.initialHintForeground` | Integrated Terminal colors | 공통 역할 `muted` 권장 | [x] 선택 생성 |
| `terminalOverviewRuler.border` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalCommandGuide.foreground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.aliasForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.branchForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.commitForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.flagForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.optionForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.optionValueForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.methodForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.argumentForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.inlineSuggestionForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.fileForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.folderForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.pullRequestDoneForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.pullRequestForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.remoteForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.stashForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.symbolText` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.symbolicLinkFileForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.symbolicLinkFolderForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `terminalSymbolIcon.tagForeground` | Integrated Terminal colors | VS Code 전용 | [x] 선택 생성 |
| `debugToolBar.background` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `debugToolBar.border` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `editor.stackFrameHighlightBackground` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `editor.focusedStackFrameHighlightBackground` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `editor.inlineValuesForeground` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `editor.inlineValuesBackground` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `debugView.exceptionLabelForeground` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `debugView.exceptionLabelBackground` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `debugView.stateLabelForeground` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `debugView.stateLabelBackground` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `debugView.valueChangedHighlight` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `debugTokenExpression.name` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `debugTokenExpression.value` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `debugTokenExpression.string` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `debugTokenExpression.boolean` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `debugTokenExpression.number` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `debugTokenExpression.error` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `debugTokenExpression.type` | Debug colors | VS Code 전용 | [x] 선택 생성 |
| `testing.runAction` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.iconErrored` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.iconFailed` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.iconPassed` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.iconQueued` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.iconUnset` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.iconSkipped` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.iconErrored.retired` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.iconFailed.retired` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.iconPassed.retired` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.iconQueued.retired` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.iconUnset.retired` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.iconSkipped.retired` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.peekBorder` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.peekHeaderBackground` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.message.error.lineBackground` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.message.info.decorationForeground` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.message.info.lineBackground` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.messagePeekBorder` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.messagePeekHeaderBackground` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.coveredBackground` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.coveredBorder` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.coveredGutterBackground` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.uncoveredBranchBackground` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.uncoveredBackground` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.uncoveredBorder` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.uncoveredGutterBackground` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.coverCountBadgeBackground` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.coverCountBadgeForeground` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.message.error.badgeBackground` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.message.error.badgeBorder` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `testing.message.error.badgeForeground` | Testing colors | VS Code 전용 | [x] 선택 생성 |
| `welcomePage.background` | Welcome page colors | VS Code 전용 | [x] 선택 생성 |
| `welcomePage.progress.background` | Welcome page colors | VS Code 전용 | [x] 선택 생성 |
| `welcomePage.progress.foreground` | Welcome page colors | VS Code 전용 | [x] 선택 생성 |
| `welcomePage.tileBackground` | Welcome page colors | VS Code 전용 | [x] 선택 생성 |
| `welcomePage.tileHoverBackground` | Welcome page colors | VS Code 전용 | [x] 선택 생성 |
| `welcomePage.tileBorder` | Welcome page colors | VS Code 전용 | [x] 선택 생성 |
| `walkThrough.embeddedEditorBackground` | Welcome page colors | VS Code 전용 | [x] 선택 생성 |
| `walkthrough.stepTitle.foreground` | Welcome page colors | VS Code 전용 | [x] 선택 생성 |
| `gitDecoration.addedResourceForeground` | Git colors | VS Code 전용 | [x] 선택 생성 |
| `gitDecoration.modifiedResourceForeground` | Git colors | VS Code 전용 | [x] 선택 생성 |
| `gitDecoration.deletedResourceForeground` | Git colors | VS Code 전용 | [x] 선택 생성 |
| `gitDecoration.renamedResourceForeground` | Git colors | VS Code 전용 | [x] 선택 생성 |
| `gitDecoration.stageModifiedResourceForeground` | Git colors | VS Code 전용 | [x] 선택 생성 |
| `gitDecoration.stageDeletedResourceForeground` | Git colors | VS Code 전용 | [x] 선택 생성 |
| `gitDecoration.untrackedResourceForeground` | Git colors | VS Code 전용 | [x] 선택 생성 |
| `gitDecoration.ignoredResourceForeground` | Git colors | VS Code 전용 | [x] 선택 생성 |
| `gitDecoration.conflictingResourceForeground` | Git colors | VS Code 전용 | [x] 선택 생성 |
| `gitDecoration.submoduleResourceForeground` | Git colors | VS Code 전용 | [x] 선택 생성 |
| `git.blame.editorDecorationForeground` | Git colors | VS Code 전용 | [x] 선택 생성 |
| `scmGraph.historyItemHoverLabelForeground` | Source Control Graph colors | VS Code 전용 | [x] 선택 생성 |
| `scmGraph.foreground1` | Source Control Graph colors | VS Code 전용 | [x] 선택 생성 |
| `scmGraph.foreground2` | Source Control Graph colors | VS Code 전용 | [x] 선택 생성 |
| `scmGraph.foreground3` | Source Control Graph colors | VS Code 전용 | [x] 선택 생성 |
| `scmGraph.foreground4` | Source Control Graph colors | VS Code 전용 | [x] 선택 생성 |
| `scmGraph.foreground5` | Source Control Graph colors | VS Code 전용 | [x] 선택 생성 |
| `scmGraph.historyItemHoverAdditionsForeground` | Source Control Graph colors | VS Code 전용 | [x] 선택 생성 |
| `scmGraph.historyItemHoverDeletionsForeground` | Source Control Graph colors | VS Code 전용 | [x] 선택 생성 |
| `scmGraph.historyItemRefColor` | Source Control Graph colors | VS Code 전용 | [x] 선택 생성 |
| `scmGraph.historyItemRemoteRefColor` | Source Control Graph colors | VS Code 전용 | [x] 선택 생성 |
| `scmGraph.historyItemBaseRefColor` | Source Control Graph colors | VS Code 전용 | [x] 선택 생성 |
| `scmGraph.historyItemHoverDefaultLabelForeground` | Source Control Graph colors | VS Code 전용 | [x] 선택 생성 |
| `scmGraph.historyItemHoverDefaultLabelBackground` | Source Control Graph colors | VS Code 전용 | [x] 선택 생성 |
| `settings.headerForeground` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.modifiedItemIndicator` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.dropdownBackground` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.dropdownForeground` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.dropdownBorder` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.dropdownListBorder` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.checkboxBackground` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.checkboxForeground` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.checkboxBorder` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.rowHoverBackground` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.textInputBackground` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.textInputForeground` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.textInputBorder` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.numberInputBackground` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.numberInputForeground` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.numberInputBorder` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.focusedRowBackground` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.focusedRowBorder` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.headerBorder` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.sashBorder` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `settings.settingsHeaderHoverForeground` | Settings Editor colors | VS Code 전용 | [x] 선택 생성 |
| `breadcrumb.foreground` | Breadcrumbs colors | VS Code 전용 | [x] 선택 생성 |
| `breadcrumb.background` | Breadcrumbs colors | VS Code 전용 | [x] 선택 생성 |
| `breadcrumb.focusForeground` | Breadcrumbs colors | VS Code 전용 | [x] 선택 생성 |
| `breadcrumb.activeSelectionForeground` | Breadcrumbs colors | VS Code 전용 | [x] 선택 생성 |
| `breadcrumbPicker.background` | Breadcrumbs colors | VS Code 전용 | [x] 선택 생성 |
| `editor.snippetTabstopHighlightBackground` | Snippets colors | VS Code 전용 | [x] 선택 생성 |
| `editor.snippetTabstopHighlightBorder` | Snippets colors | VS Code 전용 | [x] 선택 생성 |
| `editor.snippetFinalTabstopHighlightBackground` | Snippets colors | VS Code 전용 | [x] 선택 생성 |
| `editor.snippetFinalTabstopHighlightBorder` | Snippets colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.arrayForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.booleanForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.classForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.colorForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.constantForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.constructorForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.enumeratorForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.enumeratorMemberForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.eventForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.fieldForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.fileForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.folderForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.functionForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.interfaceForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.keyForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.keywordForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.methodForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.moduleForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.namespaceForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.nullForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.numberForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.objectForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.operatorForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.packageForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.propertyForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.referenceForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.snippetForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.stringForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.structForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.textForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.typeParameterForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.unitForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `symbolIcon.variableForeground` | Symbol Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugIcon.breakpointForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugIcon.breakpointDisabledForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugIcon.breakpointUnverifiedForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugIcon.breakpointCurrentStackframeForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugIcon.breakpointStackframeForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugIcon.startForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugIcon.pauseForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugIcon.stopForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugIcon.disconnectForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugIcon.restartForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugIcon.stepOverForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugIcon.stepIntoForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugIcon.stepOutForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugIcon.continueForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugIcon.stepBackForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugConsole.infoForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugConsole.warningForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugConsole.errorForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugConsole.sourceForeground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `debugConsoleInputIcon.foreground` | Debug Icons colors | VS Code 전용 | [x] 선택 생성 |
| `notebook.editorBackground` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebook.cellBorderColor` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebook.cellHoverBackground` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebook.cellInsertionIndicator` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebook.cellStatusBarItemHoverBackground` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebook.cellToolbarSeparator` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebook.cellEditorBackground` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebook.focusedCellBackground` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebook.focusedCellBorder` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebook.focusedEditorBorder` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebook.inactiveFocusedCellBorder` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebook.inactiveSelectedCellBorder` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebook.outputContainerBackgroundColor` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebook.outputContainerBorderColor` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebook.selectedCellBackground` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebook.selectedCellBorder` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebook.symbolHighlightBackground` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebookScrollbarSlider.activeBackground` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebookScrollbarSlider.background` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebookScrollbarSlider.hoverBackground` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebookStatusErrorIcon.foreground` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebookStatusRunningIcon.foreground` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebookStatusSuccessIcon.foreground` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `notebookEditorOverviewRuler.runningCellForeground` | Notebook colors | VS Code 전용 | [x] 선택 생성 |
| `charts.foreground` | Chart colors | VS Code 전용 | [x] 선택 생성 |
| `charts.lines` | Chart colors | VS Code 전용 | [x] 선택 생성 |
| `charts.red` | Chart colors | VS Code 전용 | [x] 선택 생성 |
| `charts.blue` | Chart colors | VS Code 전용 | [x] 선택 생성 |
| `charts.yellow` | Chart colors | VS Code 전용 | [x] 선택 생성 |
| `charts.orange` | Chart colors | VS Code 전용 | [x] 선택 생성 |
| `charts.green` | Chart colors | VS Code 전용 | [x] 선택 생성 |
| `charts.purple` | Chart colors | VS Code 전용 | [x] 선택 생성 |
| `chart.line` | Chart colors | VS Code 전용 | [x] 선택 생성 |
| `chart.axis` | Chart colors | VS Code 전용 | [x] 선택 생성 |
| `chart.guide` | Chart colors | VS Code 전용 | [x] 선택 생성 |
| `ports.iconRunningProcessForeground` | Ports colors | VS Code 전용 | [x] 선택 생성 |
| `commentsView.resolvedIcon` | Comments View colors | VS Code 전용 | [x] 선택 생성 |
| `commentsView.unresolvedIcon` | Comments View colors | VS Code 전용 | [x] 선택 생성 |
| `actionBar.toggledBackground` | Action Bar colors | VS Code 전용 | [x] 선택 생성 |
| `simpleFindWidget.sashBorder` | Simple Find Widget colors | VS Code 전용 | [x] 선택 생성 |
| `gauge.background` | Gauge colors | VS Code 전용 | [x] 선택 생성 |
| `gauge.foreground` | Gauge colors | VS Code 전용 | [x] 선택 생성 |
| `gauge.border` | Gauge colors | VS Code 전용 | [x] 선택 생성 |
| `gauge.warningBackground` | Gauge colors | VS Code 전용 | [x] 선택 생성 |
| `gauge.warningForeground` | Gauge colors | VS Code 전용 | [x] 선택 생성 |
| `gauge.errorBackground` | Gauge colors | VS Code 전용 | [x] 선택 생성 |
| `gauge.errorForeground` | Gauge colors | VS Code 전용 | [x] 선택 생성 |
| `markdownAlert.note.foreground` | Markdown | VS Code 전용 | [x] 선택 생성 |
| `markdownAlert.tip.foreground` | Markdown | VS Code 전용 | [x] 선택 생성 |
| `markdownAlert.important.foreground` | Markdown | VS Code 전용 | [x] 선택 생성 |
| `markdownAlert.warning.foreground` | Markdown | VS Code 전용 | [x] 선택 생성 |
| `markdownAlert.caution.foreground` | Markdown | VS Code 전용 | [x] 선택 생성 |
| `agentSessionReadIndicator.foreground` | Agent Session colors | VS Code 전용 | [x] 선택 생성 |
| `agentSessionSelectedBadge.border` | Agent Session colors | VS Code 전용 | [x] 선택 생성 |
| `agentSessionSelectedUnfocusedBadge.border` | Agent Session colors | VS Code 전용 | [x] 선택 생성 |
| `agentStatusIndicator.background` | Agent Session colors | VS Code 전용 | [x] 선택 생성 |
| `aiCustomizationManagement.sashBorder` | Agent Session colors | VS Code 전용 | [x] 선택 생성 |
