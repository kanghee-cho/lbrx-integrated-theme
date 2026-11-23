# macOS Terminal.app 프로파일 옵션 대장

[적용 가이드](14-macos-terminal.md) · [Apple 프로파일 설정](https://support.apple.com/guide/terminal/trml107/mac) · [Windows Terminal 대응 출력](../src/windows_terminal.rs)

**범위:** Apple이 설명하는 프로파일의 텍스트·색·커서와 창/탭 외형. UI 항목명이 기준이며, 비공개 plist 키나 직렬화된 색 바이트를 공용 규격으로 간주하지 않는다. `공통`은 기존 원본, `합일 후보`는 두 터미널에서 공유할 수 있는 의도, `Mac 전용`은 프로파일에서만 관리할 값이다. `[ ]`는 Mac 생성 미구현, `수동`은 사용자 설정, `—`는 테마 출력 대상 밖이다. Windows Terminal에 이미 생성되는 값도 Mac에서 검증하기 전에는 체크하지 않는다.

| 설정할 값 | 분류 / 원본 또는 단위 | Windows Terminal 대응 | Mac 생성 |
|---|---|---|---|
| 일반 글자색 | 공통 `terminal_foreground` | `foreground` 생성 | [x] |
| 배경색 | 공통 `terminal_background` | `background` 생성 | [x] |
| 커서색 | 공통 `terminal_cursor` | `cursorColor` 생성 | [x] |
| 선택 배경 | 공통 `selection_background` | `selectionBackground` 생성 | [x] |
| 굵은 글자색 | 일반 글자색과 독립 | 별도 출력 없음 | 선택 생성 |
| 고정폭 글꼴 | 공통 `typography.mono.families`의 첫 패밀리 | `font.face` 조각 생성 | 수동 |
| 글꼴 크기 | 공통 `typography.mono.size_px` → pt 참고값 | `font.size` 조각 생성 | 수동 |
| 행 간격 / 문자 간격 | 합일 후보 / Mac 전용. 별도 수치로 취급 | 출력 없음 | 수동 |

### ANSI 팔레트

기본 색상 재지정과 굵은 글자색은 [macos-terminal.toml](../themes/editors/macos-terminal.toml)에서 지정한다. `[x]`는 파일 생성 검증이며 Mac 앱에서의 가져오기 검증은 아니다.

일반색과 밝은색은 독립 슬롯이다. 아래 각 칸의 체크는 Mac 출력 기준이며 WT 필드는 이미 생성된다. ANSI를 쓰지 않고 프로그램이 직접 지정한 RGB까지 바꾸지는 않는다.

| 일반 슬롯 / WT 필드 | Mac | 밝은 슬롯 / WT 필드 | Mac |
|---|---|---|---|
| 0 검정 `black` | [x] | 8 밝은 검정 `brightBlack` | [x] |
| 1 빨강 `red` | [x] | 9 밝은 빨강 `brightRed` | [x] |
| 2 초록 `green` | [x] | 10 밝은 초록 `brightGreen` | [x] |
| 3 노랑 `yellow` | [x] | 11 밝은 노랑 `brightYellow` | [x] |
| 4 파랑 `blue` | [x] | 12 밝은 파랑 `brightBlue` | [x] |
| 5 마젠타 `purple` | [x] | 13 밝은 마젠타 `brightPurple` | [x] |
| 6 시안 `cyan` | [x] | 14 밝은 시안 `brightCyan` | [x] |
| 7 흰색 `white` | [x] | 15 밝은 흰색 `brightWhite` | [x] |

### 프로파일에서 따로 조절할 값

| 항목 | 독립 옵션 | 경계 / 현재 적용 |
|---|---|---|
| 배경 효과 | 활성 창 불투명도, 활성 창 흐림, 비활성 창 불투명도, 비활성 창 흐림, 배경 이미지 | Mac 전용·수동. 이미지 경로를 배포하지 않음 |
| 글자 렌더링 | 안티앨리어싱, 굵은 글꼴 사용, 점멸 글자 허용 | Mac 전용·수동 |
| ANSI 동작 | ANSI 색 표시, 굵은 글자에 밝은 색 사용 | 합일 후보·수동. 팔레트 슬롯 변경과 별개 |
| 커서 | 모양(블록·밑줄·세로 막대), 점멸 | 합일 후보·수동. 커서색과 별개 |
| 창 | 제목과 표시할 정보, 초기 열·행 수 | Mac 전용·수동. 글자 크기나 셀 높이와 별개 |
| 탭 | 탭 제목과 표시할 프로세스·경로 정보 | Mac 전용·수동 |
| 앱 동작 | 스크롤백, 셸 시작·종료 명령, 키보드 매핑, 인코딩, 벨, 기본·시작 프로파일 | — 기존 동작을 생성물이 변경하지 않음 |

**실제 검증:** 미실시. `항목/슬롯 / macOS·Terminal.app 버전 / 라이트·다크 / 가져오기·새 창·재내보내기 / 결과`로 기록한다. 일부 옵션만 구현하면 해당 행을 나눈다.
