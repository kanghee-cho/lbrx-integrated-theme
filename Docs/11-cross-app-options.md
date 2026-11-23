# 전체 앱 공통 옵션과 적용 범위

이 문서는 [테마 원본](../themes/lbrx.toml)의 값을 바꾸면 **어느 앱에서 무엇이 바뀌는지** 찾는 기준이다. 공통은 모든 앱이 지원한다는 뜻이 아니다. 예를 들어 글꼴 하나를 바꿔도 Typora에서는 CSS가 바뀌고, Windows Terminal에서는 선택 적용할 설정 조각이 바뀌며, Vim 안에서는 글꼴을 바꿀 수 없다.

## 앱별 전달 경로

| 대상·상세 문서 | 출력과 현재 생성 여부 | 공통값의 적용 경로 또는 한계 |
|---|---|---|
| [Typora](08-typora-options.md) | 모드별 CSS · 생성 | 문서 색·서체·코드 구문색을 직접 지정. 설정 창 일부는 앱 전용 |
| [Obsidian](10-obsidian-options.md) | 테마 CSS + manifest · 최소 테마 생성 | 문서 역할을 읽기 뷰·라이브 프리뷰에 각각 연결. Publish는 별도 배포 |
| [VS Code](19-vscode-options.md) | VSIX 색 테마 · 생성 구현 | 작업 화면 색 ID·TextMate·semantic token에 연결. 서체는 별도 사용자 설정 |
| [Vim](13-vim-options.md) | Vim 색 구성표 · 생성 | 기본 화면·공통 구문 그룹에 RGB와 256색 `cterm` 근사값을 지정. 글꼴·셀 높이는 호스트 소유 |
| [Helix](17-helix-options.md) | TOML 테마 · 생성 구현 | 구문·UI·진단 스코프에 색과 장식을 지정. 글꼴·셀 높이는 호스트 소유 |
| [Windows Terminal](06-windows-terminal.md) | 배색표 JSON + 글꼴 조각 · 생성 | 전경·배경·커서·선택 배경·ANSI 16색. 글꼴은 첫 패밀리와 pt 크기만 선택 적용 |
| [macOS Terminal.app](15-macos-terminal-options.md) | `.terminal` 프로파일 · 생성 | 프로파일에 터미널 배색을 매핑. 글꼴·간격은 사용자 설정 안내 |
| [iTerm2](21-iterm2-options.md) | `.itermcolors` 프리셋 · 생성 | 기존 프로파일의 배색만 변경. 선택 글자색 사용 여부와 글꼴·간격은 프로파일에서 설정 |

## 공통 옵션을 고르는 기준

| 바꾸고 싶은 것 | 원본 또는 후보 | 함께 볼 대상 / 합치지 않을 대상 |
|---|---|---|
| 기본 배경·글자 | 문서·편집기 `background`, `text` / 터미널 `terminal_background`, `terminal_foreground` | 8개 앱 모두 관련되지만 역할은 분리한다. 같은 팔레트 키를 참조할 때만 그 키의 변경이 함께 전파된다. |
| 선택 영역 | `selection_background`, `selection_text` | 문서 앱·편집기에서는 선택 레이어를, 터미널에서는 프로파일 선택색을 본다. Windows Terminal의 현재 출력에는 선택 글자색이 없다. |
| 포커스·커서 | `focus` / `terminal_cursor` | 링크 윤곽선, 편집기 커서, 터미널 블록 커서는 다른 대상이다. 터미널 커서색 변경으로 링크 포커스가 바뀌어서는 안 된다. |
| 글꼴·크기·줄 높이 | `typography.prose`, `typography.mono` | 문서 앱은 본문과 코드를 분리한다. 터미널·편집기는 주로 고정폭 대상이다. CSS 줄 높이와 터미널 셀 간격은 동일 수치로 옮기지 않는다. |
| 코드 구문색 | `comment`, `string`, `keyword`, `function`, `number`, `error` | Typora·Obsidian·VS Code·Vim·Helix가 각 렌더러에 연결한다. 터미널 에뮬레이터가 셸에서 실행한 프로그램의 구문을 분석하는 것은 아니다. |
| ANSI 16색 | 모드별 `ansi` 슬롯 0–15 | Windows Terminal·Terminal.app·iTerm2 및 VS Code **통합 터미널** 대상. Vim `cterm`이 슬롯을 사용하는 것과 호스트 팔레트를 설정하는 것은 별개다. |
| 문서 조판 | 아래 D01–D26 중 문서 역할 및 합일 후보 | 우선 Typora·Obsidian을 비교한다. 문서 최대 폭을 Vim 줄바꿈이나 터미널 열 수로 바꾸지 않는다. |

이 표는 **지원 경계와 연결 방식**을 기록한다. 개별 옵션의 생성 여부·고정값·모드별 실측은 각 앱 문서에서 관리한다. 값의 의미·단위·범위가 일치할 때만 [공통 옵션으로 승격](04-option-strategy.md)한다.

<a id="shared"></a>

## 문서 앱 하위 범위: 공통 기능 ID

- **D01 문서 배경·본문색**
- **D02 본문 글꼴 우선순위**
- **D03 본문 크기·줄 높이**
- **D04 코드 글꼴·크기·줄 높이**
- **D05 본문 최대 폭·좌우 여백**
- **D06 문단 앞뒤 간격**
- **D07 문단 정렬·쓰기 방향**
- **D08 서체 세부 표현**
- **D09 제목색**
- **D10 제목 단계별 조판**
- **D11 제목 자동 번호**
- **D12 링크색·밑줄**
- **D13 키보드 링크 포커스**
- **D14 인용문**
- **D15 인라인 코드색**
- **D16 일반 텍스트 선택색**
- **D17 본문 강조 표현**
- **D18 각주·참조·수평 구분선**
- **D19 코드 블록 바탕·테두리**
- **D20 코드 구문색**
- **D21 표**
- **D22 순서·비순서 목록**
- **D23 작업 목록**
- **D24 이미지·캡션**
- **D25 수식**
- **D26 알림·콜아웃**

## 선택자·속성 매핑

아래는 두 앱의 [공유 ID](#shared)를 잃지 않기 위한 선택자 단위 보충표다. 속성은 **적용할 값**이며, 두 앱에 같은 선택자를 복사하라는 뜻은 아니다.

| 범위 / ID | 적용할 CSS 값 | Typora 대응 예 |
|---|---|---|
| 공통 D04 | 코드 블록·인라인 코드의 `line-height` | 원본 `typography.mono.line_height` |
| 합일 D07 | 문단 `text-align`, `direction`, `writing-mode` | 미구현 |
| 합일 D08 | 본문 `letter-spacing`, `font-variant-ligatures`, `font-variant-caps`, `text-transform` | 미구현 |
| 합일 D11 | 제목 번호 `counter-reset`, `counter-increment`, `::before`의 `content` | 미구현 |
| 공통 D13 | 링크 `:focus-visible`의 `outline-color` | 원본 `focus` |
| 합일 D13 | 포커스 `outline-width` / `outline-style` / `outline-offset` | 고정 `2px` / `solid` / `2px` |
| 합일 D14 | 인용 `padding-inline-start` | 고정 `0.9em` |
| 공통 D15 | 인라인 코드만의 `color` | 원본 `inline_code` |
| 공통 D16 | 선택 영역의 **글자** `color` (배경 변수와 별개) | 원본 `selection_text` |
| 합일 D17 | 취소선 `text-decoration-color`, `text-decoration-thickness` | 미구현 |
| 합일 D19 | 코드 블록 `border`, `border-radius`, `padding` | 색 `muted`, 나머지 고정 `1px solid`, `0.35em`, `0.7em` |
| 공통 D20 | 숫자·오류 토큰 각각의 `color` | 원본 `number` / `error` |
| 합일 D21 | 표 셀 `padding`, 넓은 표 컨테이너 `overflow-x` | 미구현 |
| 합일 D24 | 이미지 `max-width`, 정렬용 여백, 캡션 `color`·여백 | 미구현 |
| 합일 D25 | 수식 컨테이너 여백·배경·`overflow-x` | 미구현 |
| 합일 T13/T14 ↔ O08 | 설정창 좌측 기본·선택 배경/글자, 우측 내용 배경/글자 | 다크 좌측·라이트 우측만 생성. 공통 UI 전체에 적용하지 않음 |

한 행의 일부만 구현할 때는 속성·상태·뷰별로 행을 쪼개고 완료된 부분만 체크한다. 예: 체크박스 변수를 작업 목록에 매핑하더라도 환경설정 체크박스까지 함께 바꾸는 것은 합일이 아니라 별도 결정이다.
