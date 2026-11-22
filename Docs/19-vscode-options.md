# VS Code 옵션 대장

[적용 가이드](18-vscode.md) · [공통 원본](../themes/lbrx.toml) · [공식 색 ID 목록 고정본](https://github.com/microsoft/vscode-docs/blob/d1e499c3abac3a4d3fb28e18b0e7e9f72c0d17f9/api/references/theme-color.md)

`tokenColors`와 `semanticTokenColors`의 언어별 적용 상태, HTML 코드 블록에 함께 제공할 CSS는 [언어별 구문 강조 설계](22-vscode-language-highlighting.md)에서 추적한다.

`[ ]`는 해당 VS Code 출력 미구현이다. 공통 역할은 매핑 제안이며 다른 앱의 생성 결과를 가져와 완료 처리하지 않는다. 아래 `colors` 색인 범위는 고정본이 명명한 작업 화면 색 ID다. 개별 확장의 추가 ID, 언어별 임의 TextMate scope와 semantic selector 조합은 유한한 전수 목록이 아니다.

| 적용 경로 | 설정 가능한 값 | 공통 / 합일 / 전용 |
|---|---|---|
| `colors` | 작업 화면 요소의 전경·배경·경계·상태 색. 반투명이 필요한 오버레이는 알파를 보존 | 아래 ID별로 추적 |
| `tokenColors[].scope` / `.settings` | TextMate scope별 `foreground`, `fontStyle`(굵게·기울임·밑줄·취소선). 예: `comment`, `string`, `keyword`, `entity.name.function`, `constant.numeric`, `invalid` | 공통 역할 `comment`, `string`, `keyword`, `function`, `number`, `error` → 언어 문법별 합일 후보. 각 매핑 미구현 |
| `semanticHighlighting`, `semanticTokenColors` | 의미 강조 참여 여부와 `타입.수식어:언어` 선택자별 색·굵기·기울임·밑줄. 예: `variable.readonly:typescript` | VS Code 전용 매핑. provider가 반환한 토큰만 적용. [표준 타입·수식어 및 fallback](https://code.visualstudio.com/api/language-extensions/semantic-highlight-guide) |
| 사용자 설정 조각 | `editor.fontFamily`, `editor.fontSize`, `editor.lineHeight`, `editor.fontLigatures`; 통합 터미널은 `terminal.integrated.fontFamily` 등 별도 | 코드 서체 합일 후보. 색 테마에서 강제하지 않음 |
| 아이콘 | 파일 아이콘 / 제품 아이콘 테마 선택 | 별도 확장 기여점. 로컬 아이콘 폰트가 있다고 구현된 것이 아님 |

**검증 기록:** 아직 없음. ID 또는 구문 선택자별로 `VS Code 버전·언어/확장·모드·기대값·실제값·근거`를 기록한다. 생성 열과 실제 화면 검증은 분리한다.

## 공식 작업 화면 색 ID

고정본에서 이름을 명시한 색 ID 971개. 각 행의 체크는 해당 모드의 생성 근거가 있을 때만 갱신한다. 일부 모드만 구현하면 행을 분리한다.

| 색 ID | 공식 분류 | LBRX 분류 / 원본 | 생성 |
|---|---|---|---|
| `contrastActiveBorder` | Contrast colors | VS Code 전용 | [ ] |
| `contrastBorder` | Contrast colors | VS Code 전용 | [ ] |
| `focusBorder` | Base colors | 공통 · focus | [ ] |
| `foreground` | Base colors | VS Code 전용 | [ ] |
| `disabledForeground` | Base colors | VS Code 전용 | [ ] |
| `widget.border` | Base colors | VS Code 전용 | [ ] |
| `widget.shadow` | Base colors | VS Code 전용 | [ ] |
| `selection.background` | Base colors | VS Code 전용 | [ ] |
| `descriptionForeground` | Base colors | VS Code 전용 | [ ] |
| `errorForeground` | Base colors | 공통 · error | [ ] |
| `icon.foreground` | Base colors | VS Code 전용 | [ ] |
| `sash.hoverBorder` | Base colors | VS Code 전용 | [ ] |
| `window.activeBorder` | Window border | VS Code 전용 | [ ] |
| `window.inactiveBorder` | Window border | VS Code 전용 | [ ] |
| `surface.background` | Modern UI colors | VS Code 전용 | [ ] |
| `surface.foreground` | Modern UI colors | VS Code 전용 | [ ] |
| `surface.border` | Modern UI colors | VS Code 전용 | [ ] |
| `editor.border` | Modern UI colors | VS Code 전용 | [ ] |
| `modernPanel.border` | Modern UI colors | VS Code 전용 | [ ] |
| `modernSash.gripForeground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernTab.activeBackground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernTab.activeForeground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernTab.hoverBackground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernTab.hoverForeground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernEditorTab.activeBackground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernEditorTab.activeActionBackground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernEditorTab.activeForeground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernEditorTab.inactiveBackground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernEditorTab.hoverBackground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernEditorTab.hoverActionBackground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernEditorTab.hoverForeground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernEditorTab.activeHoverBackground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernEditorTab.activeHoverActionBackground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernEditorTab.selectedActionBackground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernActivityBar.background` | Modern UI colors | VS Code 전용 | [ ] |
| `modernActivityBar.inactiveBackground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernActivityBarItem.activeBackground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernActivityBarItem.activeForeground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernActivityBarItem.hoverBackground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernActivityBarItem.hoverForeground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernActivityBar.border` | Modern UI colors | VS Code 전용 | [ ] |
| `modernUI.shellBackground` | Modern UI colors | VS Code 전용 | [ ] |
| `modernUI.inactiveShellBackground` | Modern UI colors | VS Code 전용 | [ ] |
| `textBlockQuote.background` | Text colors | VS Code 전용 | [ ] |
| `textBlockQuote.border` | Text colors | VS Code 전용 | [ ] |
| `textCodeBlock.background` | Text colors | VS Code 전용 | [ ] |
| `textLink.activeForeground` | Text colors | VS Code 전용 | [ ] |
| `textLink.foreground` | Text colors | VS Code 전용 | [ ] |
| `textPreformat.foreground` | Text colors | VS Code 전용 | [ ] |
| `textPreformat.background` | Text colors | VS Code 전용 | [ ] |
| `textPreformat.border` | Text colors | VS Code 전용 | [ ] |
| `textSeparator.foreground` | Text colors | VS Code 전용 | [ ] |
| `toolbar.hoverBackground` | Action colors | VS Code 전용 | [ ] |
| `toolbar.hoverOutline` | Action colors | VS Code 전용 | [ ] |
| `toolbar.activeBackground` | Action colors | VS Code 전용 | [ ] |
| `editorActionList.background` | Action colors | VS Code 전용 | [ ] |
| `editorActionList.foreground` | Action colors | VS Code 전용 | [ ] |
| `editorActionList.focusForeground` | Action colors | VS Code 전용 | [ ] |
| `editorActionList.focusBackground` | Action colors | VS Code 전용 | [ ] |
| `button.background` | Button control | VS Code 전용 | [ ] |
| `button.foreground` | Button control | VS Code 전용 | [ ] |
| `button.border` | Button control | VS Code 전용 | [ ] |
| `button.separator` | Button control | VS Code 전용 | [ ] |
| `button.hoverBackground` | Button control | VS Code 전용 | [ ] |
| `button.secondaryForeground` | Button control | VS Code 전용 | [ ] |
| `button.secondaryBackground` | Button control | VS Code 전용 | [ ] |
| `button.secondaryHoverBackground` | Button control | VS Code 전용 | [ ] |
| `button.secondaryBorder` | Button control | VS Code 전용 | [ ] |
| `checkbox.background` | Button control | VS Code 전용 | [ ] |
| `checkbox.foreground` | Button control | VS Code 전용 | [ ] |
| `checkbox.disabled.background` | Button control | VS Code 전용 | [ ] |
| `checkbox.disabled.foreground` | Button control | VS Code 전용 | [ ] |
| `checkbox.border` | Button control | VS Code 전용 | [ ] |
| `checkbox.selectBackground` | Button control | VS Code 전용 | [ ] |
| `checkbox.selectBorder` | Button control | VS Code 전용 | [ ] |
| `radio.activeForeground` | Button control | VS Code 전용 | [ ] |
| `radio.activeBackground` | Button control | VS Code 전용 | [ ] |
| `radio.activeBorder` | Button control | VS Code 전용 | [ ] |
| `radio.inactiveForeground` | Button control | VS Code 전용 | [ ] |
| `radio.inactiveBackground` | Button control | VS Code 전용 | [ ] |
| `radio.inactiveBorder` | Button control | VS Code 전용 | [ ] |
| `radio.inactiveHoverBackground` | Button control | VS Code 전용 | [ ] |
| `dropdown.background` | Dropdown control | VS Code 전용 | [ ] |
| `dropdown.listBackground` | Dropdown control | VS Code 전용 | [ ] |
| `dropdown.border` | Dropdown control | VS Code 전용 | [ ] |
| `dropdown.foreground` | Dropdown control | VS Code 전용 | [ ] |
| `input.background` | Input control | VS Code 전용 | [ ] |
| `input.border` | Input control | VS Code 전용 | [ ] |
| `input.foreground` | Input control | VS Code 전용 | [ ] |
| `input.placeholderForeground` | Input control | VS Code 전용 | [ ] |
| `inputOption.activeBackground` | Input control | VS Code 전용 | [ ] |
| `inputOption.activeBorder` | Input control | VS Code 전용 | [ ] |
| `inputOption.activeForeground` | Input control | VS Code 전용 | [ ] |
| `inputOption.hoverBackground` | Input control | VS Code 전용 | [ ] |
| `inputValidation.errorBackground` | Input control | VS Code 전용 | [ ] |
| `inputValidation.errorForeground` | Input control | VS Code 전용 | [ ] |
| `inputValidation.errorBorder` | Input control | VS Code 전용 | [ ] |
| `inputValidation.infoBackground` | Input control | VS Code 전용 | [ ] |
| `inputValidation.infoForeground` | Input control | VS Code 전용 | [ ] |
| `inputValidation.infoBorder` | Input control | VS Code 전용 | [ ] |
| `inputValidation.warningBackground` | Input control | VS Code 전용 | [ ] |
| `inputValidation.warningForeground` | Input control | VS Code 전용 | [ ] |
| `inputValidation.warningBorder` | Input control | VS Code 전용 | [ ] |
| `scrollbar.background` | Scrollbar control | VS Code 전용 | [ ] |
| `scrollbar.shadow` | Scrollbar control | VS Code 전용 | [ ] |
| `scrollbarSlider.activeBackground` | Scrollbar control | VS Code 전용 | [ ] |
| `scrollbarSlider.background` | Scrollbar control | VS Code 전용 | [ ] |
| `scrollbarSlider.hoverBackground` | Scrollbar control | VS Code 전용 | [ ] |
| `badge.foreground` | Badge | VS Code 전용 | [ ] |
| `badge.background` | Badge | VS Code 전용 | [ ] |
| `progressBar.background` | Progress bar | VS Code 전용 | [ ] |
| `list.activeSelectionBackground` | Lists and trees | VS Code 전용 | [ ] |
| `list.activeSelectionForeground` | Lists and trees | VS Code 전용 | [ ] |
| `list.activeSelectionIconForeground` | Lists and trees | VS Code 전용 | [ ] |
| `list.dropBackground` | Lists and trees | VS Code 전용 | [ ] |
| `list.focusBackground` | Lists and trees | VS Code 전용 | [ ] |
| `list.focusForeground` | Lists and trees | VS Code 전용 | [ ] |
| `list.focusHighlightForeground` | Lists and trees | VS Code 전용 | [ ] |
| `list.focusOutline` | Lists and trees | VS Code 전용 | [ ] |
| `list.focusAndSelectionOutline` | Lists and trees | VS Code 전용 | [ ] |
| `list.highlightForeground` | Lists and trees | VS Code 전용 | [ ] |
| `list.hoverBackground` | Lists and trees | VS Code 전용 | [ ] |
| `list.hoverForeground` | Lists and trees | VS Code 전용 | [ ] |
| `list.inactiveSelectionBackground` | Lists and trees | VS Code 전용 | [ ] |
| `list.inactiveSelectionForeground` | Lists and trees | VS Code 전용 | [ ] |
| `list.inactiveSelectionIconForeground` | Lists and trees | VS Code 전용 | [ ] |
| `list.inactiveFocusBackground` | Lists and trees | VS Code 전용 | [ ] |
| `list.inactiveFocusOutline` | Lists and trees | VS Code 전용 | [ ] |
| `list.invalidItemForeground` | Lists and trees | VS Code 전용 | [ ] |
| `list.errorForeground` | Lists and trees | VS Code 전용 | [ ] |
| `list.warningForeground` | Lists and trees | VS Code 전용 | [ ] |
| `listFilterWidget.background` | Lists and trees | VS Code 전용 | [ ] |
| `listFilterWidget.outline` | Lists and trees | VS Code 전용 | [ ] |
| `listFilterWidget.noMatchesOutline` | Lists and trees | VS Code 전용 | [ ] |
| `listFilterWidget.shadow` | Lists and trees | VS Code 전용 | [ ] |
| `list.filterMatchBackground` | Lists and trees | VS Code 전용 | [ ] |
| `list.filterMatchBorder` | Lists and trees | VS Code 전용 | [ ] |
| `list.deemphasizedForeground` | Lists and trees | VS Code 전용 | [ ] |
| `list.dropBetweenBackground` | Lists and trees | VS Code 전용 | [ ] |
| `tree.indentGuidesStroke` | Lists and trees | VS Code 전용 | [ ] |
| `tree.inactiveIndentGuidesStroke` | Lists and trees | VS Code 전용 | [ ] |
| `tree.tableColumnsBorder` | Lists and trees | VS Code 전용 | [ ] |
| `tree.tableOddRowsBackground` | Lists and trees | VS Code 전용 | [ ] |
| `activityBar.background` | Activity Bar | VS Code 전용 | [ ] |
| `activityBar.dropBorder` | Activity Bar | VS Code 전용 | [ ] |
| `activityBar.foreground` | Activity Bar | VS Code 전용 | [ ] |
| `activityBar.inactiveForeground` | Activity Bar | VS Code 전용 | [ ] |
| `activityBar.border` | Activity Bar | VS Code 전용 | [ ] |
| `activityBarBadge.background` | Activity Bar | VS Code 전용 | [ ] |
| `activityBarBadge.foreground` | Activity Bar | VS Code 전용 | [ ] |
| `activityBar.activeBorder` | Activity Bar | VS Code 전용 | [ ] |
| `activityBar.activeBackground` | Activity Bar | VS Code 전용 | [ ] |
| `activityBar.activeFocusBorder` | Activity Bar | VS Code 전용 | [ ] |
| `activityBarTop.foreground` | Activity Bar | VS Code 전용 | [ ] |
| `activityBarTop.activeBorder` | Activity Bar | VS Code 전용 | [ ] |
| `activityBarTop.inactiveForeground` | Activity Bar | VS Code 전용 | [ ] |
| `activityBarTop.dropBorder` | Activity Bar | VS Code 전용 | [ ] |
| `activityBarTop.background` | Activity Bar | VS Code 전용 | [ ] |
| `activityBarTop.activeBackground` | Activity Bar | VS Code 전용 | [ ] |
| `activityWarningBadge.foreground` | Activity Bar | VS Code 전용 | [ ] |
| `activityWarningBadge.background` | Activity Bar | VS Code 전용 | [ ] |
| `activityErrorBadge.foreground` | Activity Bar | VS Code 전용 | [ ] |
| `activityErrorBadge.background` | Activity Bar | VS Code 전용 | [ ] |
| `profileBadge.background` | Profiles | VS Code 전용 | [ ] |
| `profileBadge.foreground` | Profiles | VS Code 전용 | [ ] |
| `profiles.sashBorder` | Profiles | VS Code 전용 | [ ] |
| `sideBar.background` | Side Bar | VS Code 전용 | [ ] |
| `sideBar.foreground` | Side Bar | VS Code 전용 | [ ] |
| `sideBar.border` | Side Bar | VS Code 전용 | [ ] |
| `sideBar.dropBackground` | Side Bar | VS Code 전용 | [ ] |
| `sideBarTitle.foreground` | Side Bar | VS Code 전용 | [ ] |
| `sideBarSectionHeader.background` | Side Bar | VS Code 전용 | [ ] |
| `sideBarSectionHeader.foreground` | Side Bar | VS Code 전용 | [ ] |
| `sideBarSectionHeader.border` | Side Bar | VS Code 전용 | [ ] |
| `sideBarActivityBarTop.border` | Side Bar | VS Code 전용 | [ ] |
| `sideBarTitle.background` | Side Bar | VS Code 전용 | [ ] |
| `sideBarTitle.border` | Side Bar | VS Code 전용 | [ ] |
| `sideBarStickyScroll.background` | Side Bar | VS Code 전용 | [ ] |
| `sideBarStickyScroll.border` | Side Bar | VS Code 전용 | [ ] |
| `sideBarStickyScroll.shadow` | Side Bar | VS Code 전용 | [ ] |
| `minimap.findMatchHighlight` | Minimap | VS Code 전용 | [ ] |
| `minimap.selectionHighlight` | Minimap | VS Code 전용 | [ ] |
| `minimap.errorHighlight` | Minimap | VS Code 전용 | [ ] |
| `minimap.warningHighlight` | Minimap | VS Code 전용 | [ ] |
| `minimap.background` | Minimap | VS Code 전용 | [ ] |
| `minimap.selectionOccurrenceHighlight` | Minimap | VS Code 전용 | [ ] |
| `minimap.foregroundOpacity` | Minimap | VS Code 전용 | [ ] |
| `minimap.infoHighlight` | Minimap | VS Code 전용 | [ ] |
| `minimap.chatEditHighlight` | Minimap | VS Code 전용 | [ ] |
| `minimapSlider.background` | Minimap | VS Code 전용 | [ ] |
| `minimapSlider.hoverBackground` | Minimap | VS Code 전용 | [ ] |
| `minimapSlider.activeBackground` | Minimap | VS Code 전용 | [ ] |
| `minimapGutter.addedBackground` | Minimap | VS Code 전용 | [ ] |
| `minimapGutter.modifiedBackground` | Minimap | VS Code 전용 | [ ] |
| `minimapGutter.deletedBackground` | Minimap | VS Code 전용 | [ ] |
| `editorMinimap.inlineChatInserted` | Minimap | VS Code 전용 | [ ] |
| `editorGroup.border` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `editorGroup.dropBackground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `editorGroupHeader.noTabsBackground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `editorGroupHeader.tabsBackground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `editorGroupHeader.tabsBorder` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `editorGroupHeader.border` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `editorGroup.emptyBackground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `editorGroup.focusedEmptyBorder` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `editorGroup.dropIntoPromptForeground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `editorGroup.dropIntoPromptBackground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `editorGroup.dropIntoPromptBorder` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.activeBackground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.unfocusedActiveBackground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.activeForeground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.border` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.activeBorder` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.selectedBorderTop` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.selectedBackground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.selectedForeground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.dragAndDropBorder` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.unfocusedActiveBorder` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.activeBorderTop` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.unfocusedActiveBorderTop` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.lastPinnedBorder` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.inactiveBackground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.unfocusedInactiveBackground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.inactiveForeground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.unfocusedActiveForeground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.unfocusedInactiveForeground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.hoverBackground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.unfocusedHoverBackground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.hoverForeground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.unfocusedHoverForeground` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.hoverBorder` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.unfocusedHoverBorder` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.activeModifiedBorder` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.inactiveModifiedBorder` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.unfocusedActiveModifiedBorder` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `tab.unfocusedInactiveModifiedBorder` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `editorPane.background` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `sideBySideEditor.horizontalBorder` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `sideBySideEditor.verticalBorder` | Editor Groups & Tabs | VS Code 전용 | [ ] |
| `editor.background` | Editor colors | 공통 · background | [ ] |
| `editor.foreground` | Editor colors | 공통 · text | [ ] |
| `editorLineNumber.foreground` | Editor colors | 합일 후보 | [ ] |
| `editorLineNumber.activeForeground` | Editor colors | 합일 후보 | [ ] |
| `editorLineNumber.dimmedForeground` | Editor colors | 합일 후보 | [ ] |
| `editorCursor.background` | Editor colors | 합일 후보 | [ ] |
| `editorCursor.foreground` | Editor colors | 합일 후보 | [ ] |
| `editorMultiCursor.primary.foreground` | Editor colors | VS Code 전용 | [ ] |
| `editorMultiCursor.primary.background` | Editor colors | VS Code 전용 | [ ] |
| `editorMultiCursor.secondary.foreground` | Editor colors | VS Code 전용 | [ ] |
| `editorMultiCursor.secondary.background` | Editor colors | VS Code 전용 | [ ] |
| `editor.placeholder.foreground` | Editor colors | VS Code 전용 | [ ] |
| `editor.compositionBorder` | Editor colors | VS Code 전용 | [ ] |
| `editor.selectionBackground` | Editor colors | 공통 · selection_background | [ ] |
| `editor.selectionForeground` | Editor colors | 공통 · selection_text | [ ] |
| `editor.inactiveSelectionBackground` | Editor colors | VS Code 전용 | [ ] |
| `editor.selectionHighlightBackground` | Editor colors | VS Code 전용 | [ ] |
| `editor.selectionHighlightBorder` | Editor colors | VS Code 전용 | [ ] |
| `editor.wordHighlightBackground` | Editor colors | VS Code 전용 | [ ] |
| `editor.wordHighlightBorder` | Editor colors | VS Code 전용 | [ ] |
| `editor.wordHighlightStrongBackground` | Editor colors | VS Code 전용 | [ ] |
| `editor.wordHighlightStrongBorder` | Editor colors | VS Code 전용 | [ ] |
| `editor.wordHighlightTextBackground` | Editor colors | VS Code 전용 | [ ] |
| `editor.wordHighlightTextBorder` | Editor colors | VS Code 전용 | [ ] |
| `editor.findMatchBackground` | Editor colors | VS Code 전용 | [ ] |
| `editor.findMatchForeground` | Editor colors | VS Code 전용 | [ ] |
| `editor.findMatchHighlightForeground` | Editor colors | VS Code 전용 | [ ] |
| `editor.findMatchHighlightBackground` | Editor colors | VS Code 전용 | [ ] |
| `editor.findRangeHighlightBackground` | Editor colors | VS Code 전용 | [ ] |
| `editor.findMatchBorder` | Editor colors | VS Code 전용 | [ ] |
| `editor.findMatchHighlightBorder` | Editor colors | VS Code 전용 | [ ] |
| `editor.findRangeHighlightBorder` | Editor colors | VS Code 전용 | [ ] |
| `search.resultsInfoForeground` | Editor colors | VS Code 전용 | [ ] |
| `searchEditor.findMatchBackground` | Editor colors | VS Code 전용 | [ ] |
| `searchEditor.findMatchBorder` | Editor colors | VS Code 전용 | [ ] |
| `searchEditor.textInputBorder` | Editor colors | VS Code 전용 | [ ] |
| `editor.hoverHighlightBackground` | Editor colors | VS Code 전용 | [ ] |
| `editor.lineHighlightBackground` | Editor colors | VS Code 전용 | [ ] |
| `editor.inactiveLineHighlightBackground` | Editor colors | VS Code 전용 | [ ] |
| `editor.lineHighlightBorder` | Editor colors | VS Code 전용 | [ ] |
| `editorUnicodeHighlight.border` | Editor colors | VS Code 전용 | [ ] |
| `editorUnicodeHighlight.background` | Editor colors | VS Code 전용 | [ ] |
| `editorLink.activeForeground` | Editor colors | 공통 · link | [ ] |
| `editor.rangeHighlightBackground` | Editor colors | VS Code 전용 | [ ] |
| `editor.rangeHighlightBorder` | Editor colors | VS Code 전용 | [ ] |
| `editor.symbolHighlightBackground` | Editor colors | VS Code 전용 | [ ] |
| `editor.symbolHighlightBorder` | Editor colors | VS Code 전용 | [ ] |
| `editorWhitespace.foreground` | Editor colors | VS Code 전용 | [ ] |
| `editorIndentGuide.background` | Editor colors | VS Code 전용 | [ ] |
| `editorIndentGuide.background1` | Editor colors | VS Code 전용 | [ ] |
| `editorIndentGuide.background2` | Editor colors | VS Code 전용 | [ ] |
| `editorIndentGuide.background3` | Editor colors | VS Code 전용 | [ ] |
| `editorIndentGuide.background4` | Editor colors | VS Code 전용 | [ ] |
| `editorIndentGuide.background5` | Editor colors | VS Code 전용 | [ ] |
| `editorIndentGuide.background6` | Editor colors | VS Code 전용 | [ ] |
| `editorIndentGuide.activeBackground` | Editor colors | VS Code 전용 | [ ] |
| `editorIndentGuide.activeBackground1` | Editor colors | VS Code 전용 | [ ] |
| `editorIndentGuide.activeBackground2` | Editor colors | VS Code 전용 | [ ] |
| `editorIndentGuide.activeBackground3` | Editor colors | VS Code 전용 | [ ] |
| `editorIndentGuide.activeBackground4` | Editor colors | VS Code 전용 | [ ] |
| `editorIndentGuide.activeBackground5` | Editor colors | VS Code 전용 | [ ] |
| `editorIndentGuide.activeBackground6` | Editor colors | VS Code 전용 | [ ] |
| `editorInlayHint.background` | Editor colors | VS Code 전용 | [ ] |
| `editorInlayHint.foreground` | Editor colors | VS Code 전용 | [ ] |
| `editorInlayHint.typeForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorInlayHint.typeBackground` | Editor colors | VS Code 전용 | [ ] |
| `editorInlayHint.parameterForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorInlayHint.parameterBackground` | Editor colors | VS Code 전용 | [ ] |
| `editorRuler.foreground` | Editor colors | VS Code 전용 | [ ] |
| `editor.linkedEditingBackground` | Editor colors | VS Code 전용 | [ ] |
| `editorCodeLens.foreground` | Editor colors | VS Code 전용 | [ ] |
| `editorLightBulb.foreground` | Editor colors | VS Code 전용 | [ ] |
| `editorLightBulbAutoFix.foreground` | Editor colors | VS Code 전용 | [ ] |
| `editorLightBulbAi.foreground` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketMatch.background` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketMatch.border` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketMatch.foreground` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketHighlight.foreground1` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketHighlight.foreground2` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketHighlight.foreground3` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketHighlight.foreground4` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketHighlight.foreground5` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketHighlight.foreground6` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketHighlight.unexpectedBracket.foreground` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketPairGuide.activeBackground1` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketPairGuide.activeBackground2` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketPairGuide.activeBackground3` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketPairGuide.activeBackground4` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketPairGuide.activeBackground5` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketPairGuide.activeBackground6` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketPairGuide.background1` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketPairGuide.background2` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketPairGuide.background3` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketPairGuide.background4` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketPairGuide.background5` | Editor colors | VS Code 전용 | [ ] |
| `editorBracketPairGuide.background6` | Editor colors | VS Code 전용 | [ ] |
| `editor.foldBackground` | Editor colors | VS Code 전용 | [ ] |
| `editor.foldPlaceholderForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.background` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.border` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.findMatchForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.rangeHighlightForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.selectionHighlightForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.wordHighlightForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.wordHighlightStrongForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.wordHighlightTextForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.modifiedForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.addedForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.deletedForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.errorForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.warningForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.infoForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.bracketMatchForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.inlineChatInserted` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.inlineChatRemoved` | Editor colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.commentDraftForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorError.foreground` | Editor colors | 합일 후보 | [ ] |
| `editorError.border` | Editor colors | 합일 후보 | [ ] |
| `editorError.background` | Editor colors | 합일 후보 | [ ] |
| `editorWarning.foreground` | Editor colors | 합일 후보 | [ ] |
| `editorWarning.border` | Editor colors | 합일 후보 | [ ] |
| `editorWarning.background` | Editor colors | 합일 후보 | [ ] |
| `editorInfo.foreground` | Editor colors | VS Code 전용 | [ ] |
| `editorInfo.border` | Editor colors | VS Code 전용 | [ ] |
| `editorInfo.background` | Editor colors | VS Code 전용 | [ ] |
| `editorHint.foreground` | Editor colors | VS Code 전용 | [ ] |
| `editorHint.border` | Editor colors | VS Code 전용 | [ ] |
| `problemsErrorIcon.foreground` | Editor colors | VS Code 전용 | [ ] |
| `problemsWarningIcon.foreground` | Editor colors | VS Code 전용 | [ ] |
| `problemsInfoIcon.foreground` | Editor colors | VS Code 전용 | [ ] |
| `editorUnnecessaryCode.border` | Editor colors | VS Code 전용 | [ ] |
| `editorUnnecessaryCode.opacity` | Editor colors | VS Code 전용 | [ ] |
| `editorGutter.background` | Editor colors | VS Code 전용 | [ ] |
| `editorGutter.modifiedBackground` | Editor colors | VS Code 전용 | [ ] |
| `editorGutter.modifiedSecondaryBackground` | Editor colors | VS Code 전용 | [ ] |
| `editorGutter.addedBackground` | Editor colors | VS Code 전용 | [ ] |
| `editorGutter.addedSecondaryBackground` | Editor colors | VS Code 전용 | [ ] |
| `editorGutter.deletedBackground` | Editor colors | VS Code 전용 | [ ] |
| `editorGutter.deletedSecondaryBackground` | Editor colors | VS Code 전용 | [ ] |
| `editorGutter.commentRangeForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorGutter.commentGlyphForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorGutter.commentUnresolvedGlyphForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorGutter.foldingControlForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorGutter.itemGlyphForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorGutter.itemBackground` | Editor colors | VS Code 전용 | [ ] |
| `editorGutter.commentDraftGlyphForeground` | Editor colors | VS Code 전용 | [ ] |
| `editorCommentsWidget.resolvedBorder` | Editor colors | VS Code 전용 | [ ] |
| `editorCommentsWidget.unresolvedBorder` | Editor colors | VS Code 전용 | [ ] |
| `editorCommentsWidget.rangeBackground` | Editor colors | VS Code 전용 | [ ] |
| `editorCommentsWidget.rangeActiveBackground` | Editor colors | VS Code 전용 | [ ] |
| `editorCommentsWidget.replyInputBackground` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.gutterIndicator.primaryBorder` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.gutterIndicator.primaryForeground` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.gutterIndicator.primaryBackground` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.gutterIndicator.secondaryBorder` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.gutterIndicator.secondaryForeground` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.gutterIndicator.secondaryBackground` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.gutterIndicator.successfulBorder` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.gutterIndicator.successfulForeground` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.gutterIndicator.successfulBackground` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.gutterIndicator.background` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.originalBackground` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.modifiedBackground` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.originalChangedLineBackground` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.originalChangedTextBackground` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.modifiedChangedLineBackground` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.modifiedChangedTextBackground` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.originalBorder` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.modifiedBorder` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.tabWillAcceptModifiedBorder` | Editor colors | VS Code 전용 | [ ] |
| `inlineEdit.tabWillAcceptOriginalBorder` | Editor colors | VS Code 전용 | [ ] |
| `diffEditor.insertedTextBackground` | Diff editor colors | 합일 후보 | [ ] |
| `diffEditor.insertedTextBorder` | Diff editor colors | 합일 후보 | [ ] |
| `diffEditor.removedTextBackground` | Diff editor colors | 합일 후보 | [ ] |
| `diffEditor.removedTextBorder` | Diff editor colors | 합일 후보 | [ ] |
| `diffEditor.border` | Diff editor colors | 합일 후보 | [ ] |
| `diffEditor.diagonalFill` | Diff editor colors | 합일 후보 | [ ] |
| `diffEditor.insertedLineBackground` | Diff editor colors | 합일 후보 | [ ] |
| `diffEditor.removedLineBackground` | Diff editor colors | 합일 후보 | [ ] |
| `diffEditorGutter.insertedLineBackground` | Diff editor colors | VS Code 전용 | [ ] |
| `diffEditorGutter.removedLineBackground` | Diff editor colors | VS Code 전용 | [ ] |
| `diffEditorOverview.insertedForeground` | Diff editor colors | VS Code 전용 | [ ] |
| `diffEditorOverview.removedForeground` | Diff editor colors | VS Code 전용 | [ ] |
| `diffEditor.unchangedRegionBackground` | Diff editor colors | 합일 후보 | [ ] |
| `diffEditor.unchangedRegionForeground` | Diff editor colors | 합일 후보 | [ ] |
| `diffEditor.unchangedRegionShadow` | Diff editor colors | 합일 후보 | [ ] |
| `diffEditor.unchangedCodeBackground` | Diff editor colors | 합일 후보 | [ ] |
| `diffEditor.move.border` | Diff editor colors | 합일 후보 | [ ] |
| `diffEditor.moveActive.border` | Diff editor colors | 합일 후보 | [ ] |
| `multiDiffEditor.headerBackground` | Diff editor colors | VS Code 전용 | [ ] |
| `multiDiffEditor.background` | Diff editor colors | VS Code 전용 | [ ] |
| `multiDiffEditor.border` | Diff editor colors | VS Code 전용 | [ ] |
| `chat.requestBorder` | Chat colors | VS Code 전용 | [ ] |
| `chat.requestBackground` | Chat colors | VS Code 전용 | [ ] |
| `chat.slashCommandBackground` | Chat colors | VS Code 전용 | [ ] |
| `chat.slashCommandForeground` | Chat colors | VS Code 전용 | [ ] |
| `chat.avatarBackground` | Chat colors | VS Code 전용 | [ ] |
| `chat.avatarForeground` | Chat colors | VS Code 전용 | [ ] |
| `chat.editedFileForeground` | Chat colors | VS Code 전용 | [ ] |
| `chat.linesAddedForeground` | Chat colors | VS Code 전용 | [ ] |
| `chat.linesRemovedForeground` | Chat colors | VS Code 전용 | [ ] |
| `chat.requestCodeBorder` | Chat colors | VS Code 전용 | [ ] |
| `chat.requestBubbleBackground` | Chat colors | VS Code 전용 | [ ] |
| `chat.requestBubbleHoverBackground` | Chat colors | VS Code 전용 | [ ] |
| `chat.checkpointSeparator` | Chat colors | VS Code 전용 | [ ] |
| `chat.thinkingShimmer` | Chat colors | VS Code 전용 | [ ] |
| `chatManagement.sashBorder` | Chat colors | VS Code 전용 | [ ] |
| `agents.background` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsPanel.background` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsPanel.foreground` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsPanel.border` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsCard.border` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsBottomPanel.border` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsGradient.tintColor` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentFeedbackEditorWidget.background` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentFeedbackEditorWidget.border` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentFeedbackInputWidget.border` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsUpdateButton.downloadingBackground` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsUpdateButton.downloadedBackground` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsChatInput.background` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsChatInput.foreground` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsChatInput.border` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsChatInput.focusBorder` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsChatInput.placeholderForeground` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsNewSessionButton.background` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsNewSessionButton.foreground` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsNewSessionButton.border` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsNewSessionButton.hoverBackground` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsBadge.background` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsBadge.foreground` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsUnreadBadge.background` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsUnreadBadge.foreground` | Agent sessions colors | VS Code 전용 | [ ] |
| `activeSessionView.background` | Agent sessions colors | VS Code 전용 | [ ] |
| `inactiveSessionView.background` | Agent sessions colors | VS Code 전용 | [ ] |
| `activeSessionView.foreground` | Agent sessions colors | VS Code 전용 | [ ] |
| `inactiveSessionView.foreground` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsMobileDiff.addedForeground` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsMobileDiff.modifiedForeground` | Agent sessions colors | VS Code 전용 | [ ] |
| `agentsMobileDiff.deletedForeground` | Agent sessions colors | VS Code 전용 | [ ] |
| `inlineChat.background` | Inline Chat colors | VS Code 전용 | [ ] |
| `inlineChat.foreground` | Inline Chat colors | VS Code 전용 | [ ] |
| `inlineChat.border` | Inline Chat colors | VS Code 전용 | [ ] |
| `inlineChat.shadow` | Inline Chat colors | VS Code 전용 | [ ] |
| `inlineChatInput.border` | Inline Chat colors | VS Code 전용 | [ ] |
| `inlineChatInput.focusBorder` | Inline Chat colors | VS Code 전용 | [ ] |
| `inlineChatInput.placeholderForeground` | Inline Chat colors | VS Code 전용 | [ ] |
| `inlineChatInput.background` | Inline Chat colors | VS Code 전용 | [ ] |
| `inlineChatDiff.inserted` | Inline Chat colors | VS Code 전용 | [ ] |
| `inlineChatDiff.removed` | Inline Chat colors | VS Code 전용 | [ ] |
| `interactive.activeCodeBorder` | Panel Chat colors | VS Code 전용 | [ ] |
| `interactive.inactiveCodeBorder` | Panel Chat colors | VS Code 전용 | [ ] |
| `editorWidget.foreground` | Editor widget colors | VS Code 전용 | [ ] |
| `editorWidget.background` | Editor widget colors | VS Code 전용 | [ ] |
| `editorWidget.border` | Editor widget colors | VS Code 전용 | [ ] |
| `editorWidget.resizeBorder` | Editor widget colors | VS Code 전용 | [ ] |
| `editorSuggestWidget.background` | Editor widget colors | VS Code 전용 | [ ] |
| `editorSuggestWidget.border` | Editor widget colors | VS Code 전용 | [ ] |
| `editorSuggestWidget.foreground` | Editor widget colors | VS Code 전용 | [ ] |
| `editorSuggestWidget.focusHighlightForeground` | Editor widget colors | VS Code 전용 | [ ] |
| `editorSuggestWidget.highlightForeground` | Editor widget colors | VS Code 전용 | [ ] |
| `editorSuggestWidget.selectedBackground` | Editor widget colors | VS Code 전용 | [ ] |
| `editorSuggestWidget.selectedForeground` | Editor widget colors | VS Code 전용 | [ ] |
| `editorSuggestWidget.selectedIconForeground` | Editor widget colors | VS Code 전용 | [ ] |
| `editorSuggestWidgetStatus.foreground` | Editor widget colors | VS Code 전용 | [ ] |
| `editorHoverWidget.foreground` | Editor widget colors | VS Code 전용 | [ ] |
| `editorHoverWidget.background` | Editor widget colors | VS Code 전용 | [ ] |
| `editorHoverWidget.border` | Editor widget colors | VS Code 전용 | [ ] |
| `editorHoverWidget.highlightForeground` | Editor widget colors | VS Code 전용 | [ ] |
| `editorHoverWidget.statusBarBackground` | Editor widget colors | VS Code 전용 | [ ] |
| `editorGhostText.border` | Editor widget colors | VS Code 전용 | [ ] |
| `editorGhostText.background` | Editor widget colors | VS Code 전용 | [ ] |
| `editorGhostText.foreground` | Editor widget colors | VS Code 전용 | [ ] |
| `editorStickyScroll.background` | Editor widget colors | VS Code 전용 | [ ] |
| `editorStickyScroll.border` | Editor widget colors | VS Code 전용 | [ ] |
| `editorStickyScroll.shadow` | Editor widget colors | VS Code 전용 | [ ] |
| `editorStickyScrollGutter.background` | Editor widget colors | VS Code 전용 | [ ] |
| `editorStickyScrollHover.background` | Editor widget colors | VS Code 전용 | [ ] |
| `debugExceptionWidget.background` | Editor widget colors | VS Code 전용 | [ ] |
| `debugExceptionWidget.border` | Editor widget colors | VS Code 전용 | [ ] |
| `editorMarkerNavigation.background` | Editor widget colors | VS Code 전용 | [ ] |
| `editorMarkerNavigationError.background` | Editor widget colors | VS Code 전용 | [ ] |
| `editorMarkerNavigationWarning.background` | Editor widget colors | VS Code 전용 | [ ] |
| `editorMarkerNavigationInfo.background` | Editor widget colors | VS Code 전용 | [ ] |
| `editorMarkerNavigationError.headerBackground` | Editor widget colors | VS Code 전용 | [ ] |
| `editorMarkerNavigationWarning.headerBackground` | Editor widget colors | VS Code 전용 | [ ] |
| `editorMarkerNavigationInfo.headerBackground` | Editor widget colors | VS Code 전용 | [ ] |
| `peekView.border` | Peek view colors | VS Code 전용 | [ ] |
| `peekViewEditor.background` | Peek view colors | VS Code 전용 | [ ] |
| `peekViewEditorGutter.background` | Peek view colors | VS Code 전용 | [ ] |
| `peekViewEditor.matchHighlightBackground` | Peek view colors | VS Code 전용 | [ ] |
| `peekViewEditor.matchHighlightBorder` | Peek view colors | VS Code 전용 | [ ] |
| `peekViewResult.background` | Peek view colors | VS Code 전용 | [ ] |
| `peekViewResult.fileForeground` | Peek view colors | VS Code 전용 | [ ] |
| `peekViewResult.lineForeground` | Peek view colors | VS Code 전용 | [ ] |
| `peekViewResult.matchHighlightBackground` | Peek view colors | VS Code 전용 | [ ] |
| `peekViewResult.selectionBackground` | Peek view colors | VS Code 전용 | [ ] |
| `peekViewResult.selectionForeground` | Peek view colors | VS Code 전용 | [ ] |
| `peekViewTitle.background` | Peek view colors | VS Code 전용 | [ ] |
| `peekViewTitleDescription.foreground` | Peek view colors | VS Code 전용 | [ ] |
| `peekViewTitleLabel.foreground` | Peek view colors | VS Code 전용 | [ ] |
| `peekViewEditorStickyScroll.background` | Peek view colors | VS Code 전용 | [ ] |
| `peekViewEditorStickyScrollGutter.background` | Peek view colors | VS Code 전용 | [ ] |
| `merge.currentHeaderBackground` | Merge conflicts colors | VS Code 전용 | [ ] |
| `merge.currentContentBackground` | Merge conflicts colors | VS Code 전용 | [ ] |
| `merge.incomingHeaderBackground` | Merge conflicts colors | VS Code 전용 | [ ] |
| `merge.incomingContentBackground` | Merge conflicts colors | VS Code 전용 | [ ] |
| `merge.border` | Merge conflicts colors | VS Code 전용 | [ ] |
| `merge.commonContentBackground` | Merge conflicts colors | VS Code 전용 | [ ] |
| `merge.commonHeaderBackground` | Merge conflicts colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.currentContentForeground` | Merge conflicts colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.incomingContentForeground` | Merge conflicts colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.commonContentForeground` | Merge conflicts colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.commentForeground` | Merge conflicts colors | VS Code 전용 | [ ] |
| `editorOverviewRuler.commentUnresolvedForeground` | Merge conflicts colors | VS Code 전용 | [ ] |
| `mergeEditor.change.background` | Merge conflicts colors | VS Code 전용 | [ ] |
| `mergeEditor.change.word.background` | Merge conflicts colors | VS Code 전용 | [ ] |
| `mergeEditor.conflict.unhandledUnfocused.border` | Merge conflicts colors | VS Code 전용 | [ ] |
| `mergeEditor.conflict.unhandledFocused.border` | Merge conflicts colors | VS Code 전용 | [ ] |
| `mergeEditor.conflict.handledUnfocused.border` | Merge conflicts colors | VS Code 전용 | [ ] |
| `mergeEditor.conflict.handledFocused.border` | Merge conflicts colors | VS Code 전용 | [ ] |
| `mergeEditor.conflict.handled.minimapOverViewRuler` | Merge conflicts colors | VS Code 전용 | [ ] |
| `mergeEditor.conflict.unhandled.minimapOverViewRuler` | Merge conflicts colors | VS Code 전용 | [ ] |
| `mergeEditor.conflictingLines.background` | Merge conflicts colors | VS Code 전용 | [ ] |
| `mergeEditor.changeBase.background` | Merge conflicts colors | VS Code 전용 | [ ] |
| `mergeEditor.changeBase.word.background` | Merge conflicts colors | VS Code 전용 | [ ] |
| `mergeEditor.conflict.input1.background` | Merge conflicts colors | VS Code 전용 | [ ] |
| `mergeEditor.conflict.input2.background` | Merge conflicts colors | VS Code 전용 | [ ] |
| `panel.background` | Panel colors | VS Code 전용 | [ ] |
| `panel.border` | Panel colors | VS Code 전용 | [ ] |
| `panel.dropBorder` | Panel colors | VS Code 전용 | [ ] |
| `panelTitle.activeBorder` | Panel colors | VS Code 전용 | [ ] |
| `panelTitle.activeForeground` | Panel colors | VS Code 전용 | [ ] |
| `panelTitle.inactiveForeground` | Panel colors | VS Code 전용 | [ ] |
| `panelTitle.border` | Panel colors | VS Code 전용 | [ ] |
| `panelTitleBadge.background` | Panel colors | VS Code 전용 | [ ] |
| `panelTitleBadge.foreground` | Panel colors | VS Code 전용 | [ ] |
| `panelInput.border` | Panel colors | VS Code 전용 | [ ] |
| `panelSection.border` | Panel colors | VS Code 전용 | [ ] |
| `panelSection.dropBackground` | Panel colors | VS Code 전용 | [ ] |
| `panelSectionHeader.background` | Panel colors | VS Code 전용 | [ ] |
| `panelSectionHeader.foreground` | Panel colors | VS Code 전용 | [ ] |
| `panelStickyScroll.background` | Panel colors | VS Code 전용 | [ ] |
| `panelStickyScroll.border` | Panel colors | VS Code 전용 | [ ] |
| `panelStickyScroll.shadow` | Panel colors | VS Code 전용 | [ ] |
| `panelSectionHeader.border` | Panel colors | VS Code 전용 | [ ] |
| `outputView.background` | Panel colors | VS Code 전용 | [ ] |
| `outputViewStickyScroll.background` | Panel colors | VS Code 전용 | [ ] |
| `statusBar.background` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBar.foreground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBar.border` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBar.debuggingBackground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBar.debuggingForeground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBar.debuggingBorder` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBar.noFolderForeground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBar.noFolderBackground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBar.noFolderBorder` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.activeBackground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.hoverForeground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.hoverBackground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.prominentForeground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.prominentBackground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.prominentHoverForeground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.prominentHoverBackground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.remoteBackground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.remoteForeground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.remoteHoverBackground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.remoteHoverForeground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.errorBackground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.errorForeground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.errorHoverBackground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.errorHoverForeground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.warningBackground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.warningForeground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.warningHoverBackground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.warningHoverForeground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.compactHoverBackground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.focusBorder` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBar.focusBorder` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.offlineBackground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.offlineForeground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.offlineHoverForeground` | Status Bar colors | VS Code 전용 | [ ] |
| `statusBarItem.offlineHoverBackground` | Status Bar colors | VS Code 전용 | [ ] |
| `titleBar.activeBackground` | Title Bar colors | VS Code 전용 | [ ] |
| `titleBar.activeForeground` | Title Bar colors | VS Code 전용 | [ ] |
| `titleBar.inactiveBackground` | Title Bar colors | VS Code 전용 | [ ] |
| `titleBar.inactiveForeground` | Title Bar colors | VS Code 전용 | [ ] |
| `titleBar.border` | Title Bar colors | VS Code 전용 | [ ] |
| `menubar.selectionForeground` | Menu Bar colors | VS Code 전용 | [ ] |
| `menubar.selectionBackground` | Menu Bar colors | VS Code 전용 | [ ] |
| `menubar.selectionBorder` | Menu Bar colors | VS Code 전용 | [ ] |
| `menu.foreground` | Menu Bar colors | VS Code 전용 | [ ] |
| `menu.background` | Menu Bar colors | VS Code 전용 | [ ] |
| `menu.selectionForeground` | Menu Bar colors | VS Code 전용 | [ ] |
| `menu.selectionBackground` | Menu Bar colors | VS Code 전용 | [ ] |
| `menu.selectionBorder` | Menu Bar colors | VS Code 전용 | [ ] |
| `menu.separatorBackground` | Menu Bar colors | VS Code 전용 | [ ] |
| `menu.border` | Menu Bar colors | VS Code 전용 | [ ] |
| `commandCenter.foreground` | Command Center colors | VS Code 전용 | [ ] |
| `commandCenter.activeForeground` | Command Center colors | VS Code 전용 | [ ] |
| `commandCenter.background` | Command Center colors | VS Code 전용 | [ ] |
| `commandCenter.activeBackground` | Command Center colors | VS Code 전용 | [ ] |
| `commandCenter.border` | Command Center colors | VS Code 전용 | [ ] |
| `commandCenter.inactiveForeground` | Command Center colors | VS Code 전용 | [ ] |
| `commandCenter.inactiveBorder` | Command Center colors | VS Code 전용 | [ ] |
| `commandCenter.activeBorder` | Command Center colors | VS Code 전용 | [ ] |
| `commandCenter.debuggingBackground` | Command Center colors | VS Code 전용 | [ ] |
| `notificationCenter.border` | Notification colors | VS Code 전용 | [ ] |
| `notificationCenterHeader.foreground` | Notification colors | VS Code 전용 | [ ] |
| `notificationCenterHeader.background` | Notification colors | VS Code 전용 | [ ] |
| `notificationToast.border` | Notification colors | VS Code 전용 | [ ] |
| `notifications.foreground` | Notification colors | VS Code 전용 | [ ] |
| `notifications.background` | Notification colors | VS Code 전용 | [ ] |
| `notifications.border` | Notification colors | VS Code 전용 | [ ] |
| `notificationLink.foreground` | Notification colors | VS Code 전용 | [ ] |
| `notificationsErrorIcon.foreground` | Notification colors | VS Code 전용 | [ ] |
| `notificationsWarningIcon.foreground` | Notification colors | VS Code 전용 | [ ] |
| `notificationsInfoIcon.foreground` | Notification colors | VS Code 전용 | [ ] |
| `banner.background` | Banner colors | VS Code 전용 | [ ] |
| `banner.foreground` | Banner colors | VS Code 전용 | [ ] |
| `banner.iconForeground` | Banner colors | VS Code 전용 | [ ] |
| `extensionButton.prominentForeground` | Extensions colors | VS Code 전용 | [ ] |
| `extensionButton.prominentBackground` | Extensions colors | VS Code 전용 | [ ] |
| `extensionButton.prominentHoverBackground` | Extensions colors | VS Code 전용 | [ ] |
| `extensionButton.background` | Extensions colors | VS Code 전용 | [ ] |
| `extensionButton.foreground` | Extensions colors | VS Code 전용 | [ ] |
| `extensionButton.hoverBackground` | Extensions colors | VS Code 전용 | [ ] |
| `extensionButton.separator` | Extensions colors | VS Code 전용 | [ ] |
| `extensionButton.border` | Extensions colors | VS Code 전용 | [ ] |
| `extensionBadge.remoteBackground` | Extensions colors | VS Code 전용 | [ ] |
| `extensionBadge.remoteForeground` | Extensions colors | VS Code 전용 | [ ] |
| `extensionIcon.starForeground` | Extensions colors | VS Code 전용 | [ ] |
| `extensionIcon.verifiedForeground` | Extensions colors | VS Code 전용 | [ ] |
| `extensionIcon.preReleaseForeground` | Extensions colors | VS Code 전용 | [ ] |
| `extensionIcon.sponsorForeground` | Extensions colors | VS Code 전용 | [ ] |
| `extensionIcon.privateForeground` | Extensions colors | VS Code 전용 | [ ] |
| `mcpIcon.starForeground` | Extensions colors | VS Code 전용 | [ ] |
| `pickerGroup.border` | Quick picker colors | VS Code 전용 | [ ] |
| `pickerGroup.foreground` | Quick picker colors | VS Code 전용 | [ ] |
| `quickInput.background` | Quick picker colors | VS Code 전용 | [ ] |
| `quickInput.foreground` | Quick picker colors | VS Code 전용 | [ ] |
| `quickInputList.focusBackground` | Quick picker colors | VS Code 전용 | [ ] |
| `quickInputList.focusForeground` | Quick picker colors | VS Code 전용 | [ ] |
| `quickInputList.focusIconForeground` | Quick picker colors | VS Code 전용 | [ ] |
| `quickInputTitle.background` | Quick picker colors | VS Code 전용 | [ ] |
| `keybindingLabel.background` | Keybinding label colors | VS Code 전용 | [ ] |
| `keybindingLabel.foreground` | Keybinding label colors | VS Code 전용 | [ ] |
| `keybindingLabel.border` | Keybinding label colors | VS Code 전용 | [ ] |
| `keybindingLabel.bottomBorder` | Keybinding label colors | VS Code 전용 | [ ] |
| `keybindingTable.headerBackground` | Keyboard shortcut table colors | VS Code 전용 | [ ] |
| `keybindingTable.rowsBackground` | Keyboard shortcut table colors | VS Code 전용 | [ ] |
| `terminal.background` | Integrated Terminal colors | 공통 · terminal_background | [ ] |
| `terminal.border` | Integrated Terminal colors | 합일 후보 | [ ] |
| `terminal.foreground` | Integrated Terminal colors | 공통 · terminal_foreground | [ ] |
| `terminal.ansiBlack` | Integrated Terminal colors | 공통 · ANSI 0 | [ ] |
| `terminal.ansiBlue` | Integrated Terminal colors | 공통 · ANSI 4 | [ ] |
| `terminal.ansiBrightBlack` | Integrated Terminal colors | 공통 · ANSI 8 | [ ] |
| `terminal.ansiBrightBlue` | Integrated Terminal colors | 공통 · ANSI 12 | [ ] |
| `terminal.ansiBrightCyan` | Integrated Terminal colors | 공통 · ANSI 14 | [ ] |
| `terminal.ansiBrightGreen` | Integrated Terminal colors | 공통 · ANSI 10 | [ ] |
| `terminal.ansiBrightMagenta` | Integrated Terminal colors | 공통 · ANSI 13 | [ ] |
| `terminal.ansiBrightRed` | Integrated Terminal colors | 공통 · ANSI 9 | [ ] |
| `terminal.ansiBrightWhite` | Integrated Terminal colors | 공통 · ANSI 15 | [ ] |
| `terminal.ansiBrightYellow` | Integrated Terminal colors | 공통 · ANSI 11 | [ ] |
| `terminal.ansiCyan` | Integrated Terminal colors | 공통 · ANSI 6 | [ ] |
| `terminal.ansiGreen` | Integrated Terminal colors | 공통 · ANSI 2 | [ ] |
| `terminal.ansiMagenta` | Integrated Terminal colors | 공통 · ANSI 5 | [ ] |
| `terminal.ansiRed` | Integrated Terminal colors | 공통 · ANSI 1 | [ ] |
| `terminal.ansiWhite` | Integrated Terminal colors | 공통 · ANSI 7 | [ ] |
| `terminal.ansiYellow` | Integrated Terminal colors | 공통 · ANSI 3 | [ ] |
| `terminal.selectionBackground` | Integrated Terminal colors | 공통 · selection_background | [ ] |
| `terminal.selectionForeground` | Integrated Terminal colors | 공통 · selection_text | [ ] |
| `terminal.inactiveSelectionBackground` | Integrated Terminal colors | 합일 후보 | [ ] |
| `terminal.findMatchBackground` | Integrated Terminal colors | 합일 후보 | [ ] |
| `terminal.findMatchBorder` | Integrated Terminal colors | 합일 후보 | [ ] |
| `terminal.findMatchHighlightBackground` | Integrated Terminal colors | 합일 후보 | [ ] |
| `terminal.findMatchHighlightBorder` | Integrated Terminal colors | 합일 후보 | [ ] |
| `terminal.hoverHighlightBackground` | Integrated Terminal colors | 합일 후보 | [ ] |
| `terminalCursor.background` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalCursor.foreground` | Integrated Terminal colors | 공통 · terminal_cursor | [ ] |
| `terminal.dropBackground` | Integrated Terminal colors | 합일 후보 | [ ] |
| `terminal.tab.activeBorder` | Integrated Terminal colors | 합일 후보 | [ ] |
| `terminalCommandDecoration.defaultBackground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalCommandDecoration.successBackground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalCommandDecoration.errorBackground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalOverviewRuler.cursorForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalOverviewRuler.findMatchForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalStickyScroll.background` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalStickyScroll.border` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalStickyScrollHover.background` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminal.initialHintForeground` | Integrated Terminal colors | 합일 후보 | [ ] |
| `terminalOverviewRuler.border` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalCommandGuide.foreground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.aliasForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.branchForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.commitForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.flagForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.optionForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.optionValueForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.methodForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.argumentForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.inlineSuggestionForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.fileForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.folderForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.pullRequestDoneForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.pullRequestForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.remoteForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.stashForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.symbolText` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.symbolicLinkFileForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.symbolicLinkFolderForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `terminalSymbolIcon.tagForeground` | Integrated Terminal colors | VS Code 전용 | [ ] |
| `debugToolBar.background` | Debug colors | VS Code 전용 | [ ] |
| `debugToolBar.border` | Debug colors | VS Code 전용 | [ ] |
| `editor.stackFrameHighlightBackground` | Debug colors | VS Code 전용 | [ ] |
| `editor.focusedStackFrameHighlightBackground` | Debug colors | VS Code 전용 | [ ] |
| `editor.inlineValuesForeground` | Debug colors | VS Code 전용 | [ ] |
| `editor.inlineValuesBackground` | Debug colors | VS Code 전용 | [ ] |
| `debugView.exceptionLabelForeground` | Debug colors | VS Code 전용 | [ ] |
| `debugView.exceptionLabelBackground` | Debug colors | VS Code 전용 | [ ] |
| `debugView.stateLabelForeground` | Debug colors | VS Code 전용 | [ ] |
| `debugView.stateLabelBackground` | Debug colors | VS Code 전용 | [ ] |
| `debugView.valueChangedHighlight` | Debug colors | VS Code 전용 | [ ] |
| `debugTokenExpression.name` | Debug colors | VS Code 전용 | [ ] |
| `debugTokenExpression.value` | Debug colors | VS Code 전용 | [ ] |
| `debugTokenExpression.string` | Debug colors | VS Code 전용 | [ ] |
| `debugTokenExpression.boolean` | Debug colors | VS Code 전용 | [ ] |
| `debugTokenExpression.number` | Debug colors | VS Code 전용 | [ ] |
| `debugTokenExpression.error` | Debug colors | VS Code 전용 | [ ] |
| `debugTokenExpression.type` | Debug colors | VS Code 전용 | [ ] |
| `testing.runAction` | Testing colors | VS Code 전용 | [ ] |
| `testing.iconErrored` | Testing colors | VS Code 전용 | [ ] |
| `testing.iconFailed` | Testing colors | VS Code 전용 | [ ] |
| `testing.iconPassed` | Testing colors | VS Code 전용 | [ ] |
| `testing.iconQueued` | Testing colors | VS Code 전용 | [ ] |
| `testing.iconUnset` | Testing colors | VS Code 전용 | [ ] |
| `testing.iconSkipped` | Testing colors | VS Code 전용 | [ ] |
| `testing.iconErrored.retired` | Testing colors | VS Code 전용 | [ ] |
| `testing.iconFailed.retired` | Testing colors | VS Code 전용 | [ ] |
| `testing.iconPassed.retired` | Testing colors | VS Code 전용 | [ ] |
| `testing.iconQueued.retired` | Testing colors | VS Code 전용 | [ ] |
| `testing.iconUnset.retired` | Testing colors | VS Code 전용 | [ ] |
| `testing.iconSkipped.retired` | Testing colors | VS Code 전용 | [ ] |
| `testing.peekBorder` | Testing colors | VS Code 전용 | [ ] |
| `testing.peekHeaderBackground` | Testing colors | VS Code 전용 | [ ] |
| `testing.message.error.lineBackground` | Testing colors | VS Code 전용 | [ ] |
| `testing.message.info.decorationForeground` | Testing colors | VS Code 전용 | [ ] |
| `testing.message.info.lineBackground` | Testing colors | VS Code 전용 | [ ] |
| `testing.messagePeekBorder` | Testing colors | VS Code 전용 | [ ] |
| `testing.messagePeekHeaderBackground` | Testing colors | VS Code 전용 | [ ] |
| `testing.coveredBackground` | Testing colors | VS Code 전용 | [ ] |
| `testing.coveredBorder` | Testing colors | VS Code 전용 | [ ] |
| `testing.coveredGutterBackground` | Testing colors | VS Code 전용 | [ ] |
| `testing.uncoveredBranchBackground` | Testing colors | VS Code 전용 | [ ] |
| `testing.uncoveredBackground` | Testing colors | VS Code 전용 | [ ] |
| `testing.uncoveredBorder` | Testing colors | VS Code 전용 | [ ] |
| `testing.uncoveredGutterBackground` | Testing colors | VS Code 전용 | [ ] |
| `testing.coverCountBadgeBackground` | Testing colors | VS Code 전용 | [ ] |
| `testing.coverCountBadgeForeground` | Testing colors | VS Code 전용 | [ ] |
| `testing.message.error.badgeBackground` | Testing colors | VS Code 전용 | [ ] |
| `testing.message.error.badgeBorder` | Testing colors | VS Code 전용 | [ ] |
| `testing.message.error.badgeForeground` | Testing colors | VS Code 전용 | [ ] |
| `welcomePage.background` | Welcome page colors | VS Code 전용 | [ ] |
| `welcomePage.progress.background` | Welcome page colors | VS Code 전용 | [ ] |
| `welcomePage.progress.foreground` | Welcome page colors | VS Code 전용 | [ ] |
| `welcomePage.tileBackground` | Welcome page colors | VS Code 전용 | [ ] |
| `welcomePage.tileHoverBackground` | Welcome page colors | VS Code 전용 | [ ] |
| `welcomePage.tileBorder` | Welcome page colors | VS Code 전용 | [ ] |
| `walkThrough.embeddedEditorBackground` | Welcome page colors | VS Code 전용 | [ ] |
| `walkthrough.stepTitle.foreground` | Welcome page colors | VS Code 전용 | [ ] |
| `gitDecoration.addedResourceForeground` | Git colors | VS Code 전용 | [ ] |
| `gitDecoration.modifiedResourceForeground` | Git colors | VS Code 전용 | [ ] |
| `gitDecoration.deletedResourceForeground` | Git colors | VS Code 전용 | [ ] |
| `gitDecoration.renamedResourceForeground` | Git colors | VS Code 전용 | [ ] |
| `gitDecoration.stageModifiedResourceForeground` | Git colors | VS Code 전용 | [ ] |
| `gitDecoration.stageDeletedResourceForeground` | Git colors | VS Code 전용 | [ ] |
| `gitDecoration.untrackedResourceForeground` | Git colors | VS Code 전용 | [ ] |
| `gitDecoration.ignoredResourceForeground` | Git colors | VS Code 전용 | [ ] |
| `gitDecoration.conflictingResourceForeground` | Git colors | VS Code 전용 | [ ] |
| `gitDecoration.submoduleResourceForeground` | Git colors | VS Code 전용 | [ ] |
| `git.blame.editorDecorationForeground` | Git colors | VS Code 전용 | [ ] |
| `scmGraph.historyItemHoverLabelForeground` | Source Control Graph colors | VS Code 전용 | [ ] |
| `scmGraph.foreground1` | Source Control Graph colors | VS Code 전용 | [ ] |
| `scmGraph.foreground2` | Source Control Graph colors | VS Code 전용 | [ ] |
| `scmGraph.foreground3` | Source Control Graph colors | VS Code 전용 | [ ] |
| `scmGraph.foreground4` | Source Control Graph colors | VS Code 전용 | [ ] |
| `scmGraph.foreground5` | Source Control Graph colors | VS Code 전용 | [ ] |
| `scmGraph.historyItemHoverAdditionsForeground` | Source Control Graph colors | VS Code 전용 | [ ] |
| `scmGraph.historyItemHoverDeletionsForeground` | Source Control Graph colors | VS Code 전용 | [ ] |
| `scmGraph.historyItemRefColor` | Source Control Graph colors | VS Code 전용 | [ ] |
| `scmGraph.historyItemRemoteRefColor` | Source Control Graph colors | VS Code 전용 | [ ] |
| `scmGraph.historyItemBaseRefColor` | Source Control Graph colors | VS Code 전용 | [ ] |
| `scmGraph.historyItemHoverDefaultLabelForeground` | Source Control Graph colors | VS Code 전용 | [ ] |
| `scmGraph.historyItemHoverDefaultLabelBackground` | Source Control Graph colors | VS Code 전용 | [ ] |
| `settings.headerForeground` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.modifiedItemIndicator` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.dropdownBackground` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.dropdownForeground` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.dropdownBorder` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.dropdownListBorder` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.checkboxBackground` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.checkboxForeground` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.checkboxBorder` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.rowHoverBackground` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.textInputBackground` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.textInputForeground` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.textInputBorder` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.numberInputBackground` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.numberInputForeground` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.numberInputBorder` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.focusedRowBackground` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.focusedRowBorder` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.headerBorder` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.sashBorder` | Settings Editor colors | VS Code 전용 | [ ] |
| `settings.settingsHeaderHoverForeground` | Settings Editor colors | VS Code 전용 | [ ] |
| `breadcrumb.foreground` | Breadcrumbs colors | VS Code 전용 | [ ] |
| `breadcrumb.background` | Breadcrumbs colors | VS Code 전용 | [ ] |
| `breadcrumb.focusForeground` | Breadcrumbs colors | VS Code 전용 | [ ] |
| `breadcrumb.activeSelectionForeground` | Breadcrumbs colors | VS Code 전용 | [ ] |
| `breadcrumbPicker.background` | Breadcrumbs colors | VS Code 전용 | [ ] |
| `editor.snippetTabstopHighlightBackground` | Snippets colors | VS Code 전용 | [ ] |
| `editor.snippetTabstopHighlightBorder` | Snippets colors | VS Code 전용 | [ ] |
| `editor.snippetFinalTabstopHighlightBackground` | Snippets colors | VS Code 전용 | [ ] |
| `editor.snippetFinalTabstopHighlightBorder` | Snippets colors | VS Code 전용 | [ ] |
| `symbolIcon.arrayForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.booleanForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.classForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.colorForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.constantForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.constructorForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.enumeratorForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.enumeratorMemberForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.eventForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.fieldForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.fileForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.folderForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.functionForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.interfaceForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.keyForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.keywordForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.methodForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.moduleForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.namespaceForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.nullForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.numberForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.objectForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.operatorForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.packageForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.propertyForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.referenceForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.snippetForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.stringForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.structForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.textForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.typeParameterForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.unitForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `symbolIcon.variableForeground` | Symbol Icons colors | VS Code 전용 | [ ] |
| `debugIcon.breakpointForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugIcon.breakpointDisabledForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugIcon.breakpointUnverifiedForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugIcon.breakpointCurrentStackframeForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugIcon.breakpointStackframeForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugIcon.startForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugIcon.pauseForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugIcon.stopForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugIcon.disconnectForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugIcon.restartForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugIcon.stepOverForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugIcon.stepIntoForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugIcon.stepOutForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugIcon.continueForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugIcon.stepBackForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugConsole.infoForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugConsole.warningForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugConsole.errorForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugConsole.sourceForeground` | Debug Icons colors | VS Code 전용 | [ ] |
| `debugConsoleInputIcon.foreground` | Debug Icons colors | VS Code 전용 | [ ] |
| `notebook.editorBackground` | Notebook colors | VS Code 전용 | [ ] |
| `notebook.cellBorderColor` | Notebook colors | VS Code 전용 | [ ] |
| `notebook.cellHoverBackground` | Notebook colors | VS Code 전용 | [ ] |
| `notebook.cellInsertionIndicator` | Notebook colors | VS Code 전용 | [ ] |
| `notebook.cellStatusBarItemHoverBackground` | Notebook colors | VS Code 전용 | [ ] |
| `notebook.cellToolbarSeparator` | Notebook colors | VS Code 전용 | [ ] |
| `notebook.cellEditorBackground` | Notebook colors | VS Code 전용 | [ ] |
| `notebook.focusedCellBackground` | Notebook colors | VS Code 전용 | [ ] |
| `notebook.focusedCellBorder` | Notebook colors | VS Code 전용 | [ ] |
| `notebook.focusedEditorBorder` | Notebook colors | VS Code 전용 | [ ] |
| `notebook.inactiveFocusedCellBorder` | Notebook colors | VS Code 전용 | [ ] |
| `notebook.inactiveSelectedCellBorder` | Notebook colors | VS Code 전용 | [ ] |
| `notebook.outputContainerBackgroundColor` | Notebook colors | VS Code 전용 | [ ] |
| `notebook.outputContainerBorderColor` | Notebook colors | VS Code 전용 | [ ] |
| `notebook.selectedCellBackground` | Notebook colors | VS Code 전용 | [ ] |
| `notebook.selectedCellBorder` | Notebook colors | VS Code 전용 | [ ] |
| `notebook.symbolHighlightBackground` | Notebook colors | VS Code 전용 | [ ] |
| `notebookScrollbarSlider.activeBackground` | Notebook colors | VS Code 전용 | [ ] |
| `notebookScrollbarSlider.background` | Notebook colors | VS Code 전용 | [ ] |
| `notebookScrollbarSlider.hoverBackground` | Notebook colors | VS Code 전용 | [ ] |
| `notebookStatusErrorIcon.foreground` | Notebook colors | VS Code 전용 | [ ] |
| `notebookStatusRunningIcon.foreground` | Notebook colors | VS Code 전용 | [ ] |
| `notebookStatusSuccessIcon.foreground` | Notebook colors | VS Code 전용 | [ ] |
| `notebookEditorOverviewRuler.runningCellForeground` | Notebook colors | VS Code 전용 | [ ] |
| `charts.foreground` | Chart colors | VS Code 전용 | [ ] |
| `charts.lines` | Chart colors | VS Code 전용 | [ ] |
| `charts.red` | Chart colors | VS Code 전용 | [ ] |
| `charts.blue` | Chart colors | VS Code 전용 | [ ] |
| `charts.yellow` | Chart colors | VS Code 전용 | [ ] |
| `charts.orange` | Chart colors | VS Code 전용 | [ ] |
| `charts.green` | Chart colors | VS Code 전용 | [ ] |
| `charts.purple` | Chart colors | VS Code 전용 | [ ] |
| `chart.line` | Chart colors | VS Code 전용 | [ ] |
| `chart.axis` | Chart colors | VS Code 전용 | [ ] |
| `chart.guide` | Chart colors | VS Code 전용 | [ ] |
| `ports.iconRunningProcessForeground` | Ports colors | VS Code 전용 | [ ] |
| `commentsView.resolvedIcon` | Comments View colors | VS Code 전용 | [ ] |
| `commentsView.unresolvedIcon` | Comments View colors | VS Code 전용 | [ ] |
| `actionBar.toggledBackground` | Action Bar colors | VS Code 전용 | [ ] |
| `simpleFindWidget.sashBorder` | Simple Find Widget colors | VS Code 전용 | [ ] |
| `gauge.background` | Gauge colors | VS Code 전용 | [ ] |
| `gauge.foreground` | Gauge colors | VS Code 전용 | [ ] |
| `gauge.border` | Gauge colors | VS Code 전용 | [ ] |
| `gauge.warningBackground` | Gauge colors | VS Code 전용 | [ ] |
| `gauge.warningForeground` | Gauge colors | VS Code 전용 | [ ] |
| `gauge.errorBackground` | Gauge colors | VS Code 전용 | [ ] |
| `gauge.errorForeground` | Gauge colors | VS Code 전용 | [ ] |
| `markdownAlert.note.foreground` | Markdown | VS Code 전용 | [ ] |
| `markdownAlert.tip.foreground` | Markdown | VS Code 전용 | [ ] |
| `markdownAlert.important.foreground` | Markdown | VS Code 전용 | [ ] |
| `markdownAlert.warning.foreground` | Markdown | VS Code 전용 | [ ] |
| `markdownAlert.caution.foreground` | Markdown | VS Code 전용 | [ ] |
| `agentSessionReadIndicator.foreground` | Agent Session colors | VS Code 전용 | [ ] |
| `agentSessionSelectedBadge.border` | Agent Session colors | VS Code 전용 | [ ] |
| `agentSessionSelectedUnfocusedBadge.border` | Agent Session colors | VS Code 전용 | [ ] |
| `agentStatusIndicator.background` | Agent Session colors | VS Code 전용 | [ ] |
| `aiCustomizationManagement.sashBorder` | Agent Session colors | VS Code 전용 | [ ] |
