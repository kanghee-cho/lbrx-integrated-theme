# iTerm2 색 프리셋 옵션 대장

[적용 가이드](20-iterm2.md) · [Terminal.app 비교](15-macos-terminal-options.md) · [Windows Terminal 매핑](../src/windows_terminal.rs)

**범위:** `.itermcolors` 기본 배색과 [프로파일 Colors 화면](https://iterm2.com/documentation-preferences-profiles-colors.html)의 독립 조절 항목. `[x]`는 두 모드에서 생성 및 파싱 검증 완료, `[ ]`는 미구현, `수동`은 프로파일에서 직접 설정할 항목이다. 생성 검증은 macOS에서의 가져오기·화면 검증을 뜻하지 않는다.

| 프리셋 키 | 분류 / 공통 원본 또는 후보 | 다른 터미널과의 관계 | 생성 |
|---|---|---|---|
| `Foreground Color` | 공통 `terminal_foreground` | WT `foreground`, Terminal.app `TextColor` | [x] |
| `Background Color` | 공통 `terminal_background` | WT `background`, Terminal.app `BackgroundColor` | [x] |
| `Cursor Color` | 공통 `terminal_cursor` | WT `cursorColor`, Terminal.app `CursorColor` | [x] |
| `Selection Color` | 공통 `selection_background` | WT `selectionBackground`, Terminal.app `SelectionColor` | [x] |
| `Selected Text Color` | 공통 `selection_text` | WT 현재 미출력. iTerm2에서 사용하려면 프로파일 옵션 확인 | [x] |
| `Cursor Text Color` | iTerm2 전용 | 커서 자체와 커서 위 글자를 따로 지정 | 선택 생성 |
| `Bold Color` | 공통 역할 참조 | Terminal.app에도 별도 색이 있음. 활성화 옵션과 색 값은 별개 | 선택 생성 |
| `Link Color` | 공통 `link` | 터미널 링크 표시용. 쉘이 출력한 임의 글자색이 아님 | 선택 생성 |
| `Cursor Guide Color` | iTerm2 전용 | 색상 값만 출력. 표시 여부와 알파는 앱에서 조정 | 선택 생성 |

선택 생성 항목은 [iterm2.toml](../themes/lavender-type/editors/iterm2.toml)의 `[colors]`에서 활성화한다. 생략하면 기본 21개 키만 생성한다.

| 공통 ANSI 슬롯 → iTerm2 키 | 일반색 생성 | 밝은 슬롯 → iTerm2 키 | 밝은색 생성 |
|---|---|---|---|
| 검정 `Ansi 0 Color` | [x] | `Ansi 8 Color` | [x] |
| 빨강 `Ansi 1 Color` | [x] | `Ansi 9 Color` | [x] |
| 초록 `Ansi 2 Color` | [x] | `Ansi 10 Color` | [x] |
| 노랑 `Ansi 3 Color` | [x] | `Ansi 11 Color` | [x] |
| 파랑 `Ansi 4 Color` | [x] | `Ansi 12 Color` | [x] |
| 마젠타 `Ansi 5 Color` | [x] | `Ansi 13 Color` | [x] |
| 시안 `Ansi 6 Color` | [x] | `Ansi 14 Color` | [x] |
| 흰색 `Ansi 7 Color` | [x] | `Ansi 15 Color` | [x] |

### 프로파일 소유: 프리셋의 색 값과 분리

| 조절 값 | 분류 | 적용 |
|---|---|---|
| 라이트·다크 색 분리, 편집할 모드 선택 | iTerm2 전용 | 수동 |
| Context-aware cursor color, Cursor Boost, Cursor Guide 색·알파 | iTerm2 전용 | 수동 |
| 굵은 글자에 지정색 사용, 밑줄 색 활성화 및 지정색 | 전용 / 합일 후보 | 수동 |
| 선택 글자색 사용 여부 | iTerm2 전용 | 수동 |
| Minimum Contrast, faint text opacity | iTerm2 전용. 원본 HEX와 화면 표시가 달라질 수 있음 | 수동 |
| 마크 표시기에 ANSI 색 사용, 탭 색 사용·지정 | iTerm2 전용 | 수동 |
| 고정폭 글꼴·크기·셀 간격, 대체 글꼴·합자 | 호스트 소유. D2Coding ligature는 선택 안내만 제공 | 수동 |

**검증 상태:** 라이트·다크 XML plist에서 각각 21개 키의 sRGB·불투명 알파·RGB 값과 다크 ANSI 변경의 격리를 테스트한다. macOS에서는 아직 미검증이다. 가져오기·재내보내기와 화면 확인 시 `키 / iTerm2·macOS 버전 / 프로파일·모드 / 결과`를 기록한다.
