# iTerm2 색 프리셋 옵션 대장

[적용 가이드](20-iterm2.md) · [Terminal.app 비교](15-macos-terminal-options.md) · [Windows Terminal 매핑](../src/windows_terminal.rs)

**범위:** `.itermcolors`로 전달할 기본 배색과 [프로파일 Colors 화면](https://iterm2.com/documentation-preferences-profiles-colors.html)의 독립 조절 항목. 전체 iTerm2 환경설정 목록은 아니다. `[ ]`는 iTerm2 생성 미구현, `수동`은 프리셋으로 강제하지 않을 설정이다. 공식 UI에서 바꿀 수 있다는 사실만으로 내보내기 파일에 저장된다고 단정하지 않는다.

| 프리셋 키 | 분류 / 공통 원본 또는 후보 | 다른 터미널과의 관계 | 생성 |
|---|---|---|---|
| `Foreground Color` | 공통 `terminal_foreground` | WT `foreground` 생성, Terminal.app 예정 | [ ] |
| `Background Color` | 공통 `terminal_background` | WT `background` 생성, Terminal.app 예정 | [ ] |
| `Cursor Color` | 공통 `terminal_cursor` | WT `cursorColor` 생성, Terminal.app 예정 | [ ] |
| `Selection Color` | 공통 `selection_background` | WT `selectionBackground` 생성, Terminal.app 예정 | [ ] |
| `Selected Text Color` | 공통 `selection_text` | WT 현재 미출력. iTerm2의 선택 글자색 사용 여부도 확인 | [ ] |
| `Cursor Text Color` | iTerm2 전용 후보 | 커서 자체와 커서 위 글자를 따로 지정 | [ ] |
| `Bold Color` | 합일 후보 | Terminal.app의 굵은 글자색과 비교. 활성화 옵션과 색 값은 별개 | [ ] |
| `Link Color` | 공통 `link` 후보 | 터미널 링크 표시용. 쉘이 출력한 임의 글자색이 아님 | [ ] |

| 공통 ANSI 슬롯 → iTerm2 키 | 일반색 생성 | 밝은 슬롯 → iTerm2 키 | 밝은색 생성 |
|---|---|---|---|
| 검정 `Ansi 0 Color` | [ ] | `Ansi 8 Color` | [ ] |
| 빨강 `Ansi 1 Color` | [ ] | `Ansi 9 Color` | [ ] |
| 초록 `Ansi 2 Color` | [ ] | `Ansi 10 Color` | [ ] |
| 노랑 `Ansi 3 Color` | [ ] | `Ansi 11 Color` | [ ] |
| 파랑 `Ansi 4 Color` | [ ] | `Ansi 12 Color` | [ ] |
| 마젠타 `Ansi 5 Color` | [ ] | `Ansi 13 Color` | [ ] |
| 시안 `Ansi 6 Color` | [ ] | `Ansi 14 Color` | [ ] |
| 흰색 `Ansi 7 Color` | [ ] | `Ansi 15 Color` | [ ] |

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

**실제 검증:** 없음. `키 / iTerm2·macOS 버전 / 프로파일·라이트 또는 다크 / 가져오기·재내보내기 / 화면 결과`로 기록한다. 색상 공간과 성분을 확인하고, 키가 무시되거나 수동 토글이 필요한 경우를 구분한다.
