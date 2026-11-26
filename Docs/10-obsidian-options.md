# Obsidian 옵션 대장

[적용 가이드](09-obsidian.md) · [문서 앱 공유 ID](11-cross-app-options.md#shared) · [Typora 대응표](08-typora-options.md)

D20의 코드 토큰 매핑과 언어별 읽기 뷰·라이브 프리뷰 검증은 [Obsidian 언어별 코드 구문 강조](24-obsidian-language-highlighting.md)에서 추적한다.

[Obsidian 어댑터](../src/obsidian.rs)가 manifest와 CSS를 생성한다. `[x]`는 생성 여부를 뜻하며 실제 화면·PDF 확인은 [검증 기록](#검증-기록)과 구분한다.

## 공통 문서 기능의 Obsidian 적용

`—`는 해당 뷰의 기능별 화면 검증 기록이 없음을 뜻한다.

| ID · 설정 대상 | 생성 · 적용 범위 | 읽기 | 라이브 |
|---|---|---|---|
| **D01 문서 배경·본문색** | [x] 문서 배경·본문색: 편집 중인 줄만 다른 바탕에 남지 않는가 | — | — |
| **D02 본문 글꼴 우선순위** | [x] 본문 글꼴 순서: EN·KR·JP 혼합 문장과 사용자 지정 글꼴 우선순위 | — | — |
| **D03 본문 크기·줄 높이** | [x] `[prose].use_shared_line_height`로 일반 본문만 공통 배율(현재 `1.65`) 적용. 글자 크기는 앱 설정을 유지한다. | — | — |
| **D04 코드 글꼴·크기·줄 높이** | [x] 고정폭 글꼴 및 `[code].use_shared_line_height`로 여러 줄 코드 블록에 공통 배율(현재 `1.45`) 적용. 글자 크기와 인라인 코드의 줄 높이는 앱 설정을 유지한다. | — | — |
| **D05 본문 최대 폭·좌우 여백** | [x] `[layout]`으로 좌우 각각 `16–32px`, 읽기 편한 줄 길이를 켰을 때만 여백 포함 최대 `860px`. 위아래 여백은 앱 기본값 유지. | — | — |
| **D06 문단 앞뒤 간격** | [x] `[paragraph].gap_px`로 읽기 뷰의 연속된 최상위 일반 문단 사이만 `12px`. 라이브 프리뷰의 빈 줄과 블록 간격은 앱 기본값 유지. | — | 앱 기본값 |
| **D07 문단 정렬·쓰기 방향** | [x] `[paragraph].alignment`로 읽기 뷰의 일반 문단만 `start`(기본) 또는 `justify`. 쓰기 방향·세로쓰기는 앱 기본값 유지. | — | 앱 기본값 |
| **D08 서체 세부 표현** | [x] `[prose_detail]`로 자간·일반 합자·작은 대문자·대소문자 표시를 선택 설정. 기본은 앱에 맡기며, 읽기 뷰의 최상위 일반 문단 중 인라인 코드가 없는 문단에만 적용. | — | 앱 기본값 |
| **D09 제목색** | [x] H1–H6는 기본 `heading`, `[colors]`로 단계별 변경 가능. 인라인 제목은 앱 기본색, 선택 재정의 가능. | — | — |
| **D10 제목 단계별 조판** | [x] `[headings.h1]`–`[headings.h6]`: 크기·굵기는 두 뷰, 제목 위아래 여백과 기본 H1 구분선은 읽기 뷰만. | — | — |
| **D11 제목 자동 번호** | 확정 — 테마에서 자동 번호를 생성하지 않는다. 원문에 직접 쓴 번호와 개요는 그대로 유지한다. | 앱 기본값 | 앱 기본값 |
| **D12 링크색·밑줄** | [x] 기본 `link` 색과 내부·외부 링크 밑줄. `[colors]`에서 일반·호버 색, `[links]`에서 일반·호버 밑줄을 개별 설정. | — | — |
| **D13 키보드 링크 포커스** | [x] `[links].keyboard_focus_outline`으로 읽기 뷰의 내부·외부 링크에 공통 `focus` 색의 2px 윤곽선과 2px 간격 적용. Tab 이동 순서는 그대로다. | — | 앱 기본값 |
| **D14 인용문** | [x] `[blockquote]`로 읽기 뷰의 일반·중첩 인용문에 공통 `quote` 글자·실선 색, 3px 두께, 0.9em 안쪽 여백 적용. 콜아웃과 편집 화면은 제외. | — | 앱 기본값 |
| **D15 인라인 코드색** | [x] 읽기 뷰·라이브 프리뷰의 인라인 코드에 공통 `inline_code` 적용. `[colors]`에서 색 변경 가능, 기존 코드 배경과 최소 4.5:1 대비 검사. | — | — |
| **D16 일반 텍스트 선택색** | [x] 브라우저 기본 선택의 배경은 본문 `prose_selection_background`, 코드 `selection_background`로 분리. 편집기의 별도 선택 레이어는 기존 배색 유지. | — | — |
| **D17 본문 강조 표현** | [x] `[emphasis.light]`·`[emphasis.dark]`로 읽기 뷰·라이브 프리뷰에 Typora와 같은 강조 글자색·기본 형광펜 배경을 적용. 글자 모양과 편집 기호는 유지. | — | — |
| **D18 각주·참조·수평 구분선** | [x] 읽기 뷰의 참조 번호는 `function`, 각주 본문은 90%와 보조 텍스트색, 수평선은 보조 텍스트색 1px와 위아래 24px 여백. 자동 각주 구분선과 이동·복귀 동작은 유지. | — | 기존 표시 유지 |
| **D19 코드 블록 바탕·테두리** | [x] 읽기 뷰·라이브 프리뷰의 독립 바탕색. 읽기 뷰에 보조 텍스트색 1px 테두리와 긴 줄 가로 스크롤. | — | 편집 커서와 줄 배치 유지 |
| **D20 코드 구문색** | [x] 기존 구문색 유지. 숫자·오류는 읽기 뷰와 라이브 프리뷰에서 앱이 해당 토큰을 식별할 때만 적용. 언어별 예외는 언어 표식이 있는 렌더링 코드에 적용. | — | 앱의 토큰 식별에 따름 |
| **D21 표** | [x] 읽기 뷰의 Markdown 표에 `surface` 배경의 굵은 헤더, 셀 여백 8px·12px, 1px 가로선과 표 영역 가로 스크롤. | — | 열 정렬·편집 중 셀 선택·Bases 유지 |
| **D22 순서·비순서 목록** | [x] 읽기 뷰 일반 목록에 단계별 24px 들여쓰기, 연속 항목 사이 4px 추가 간격, 비순서 표식 ● → ○ → ■. | — | 순서 번호·작업 목록·접기·편집 가이드 유지 |
| **D23 작업 목록** | [x] 읽기 뷰의 완료 체크박스에 `accent`, 완료 본문에 보조색과 안전한 범위의 취소선. | — | 하위 미완료 항목·클릭·라이브 프리뷰 유지 |
| **D24 이미지·캡션** | [x] 읽기 뷰의 단독 이미지·임베드는 기본 가운데 정렬, 본문 폭 제한. 명시적 `figcaption`은 8px 간격·90%·보조색. | — | 대체 텍스트·라이브 프리뷰 유지 |
| **D25 수식** | [x] 읽기 뷰 블록 수식에 `surface` 배경, 위아래 12px·좌우 16px 여백, 화면에서 수식 영역 내 가로 스크롤. | — | 인라인 수식·수학 글꼴·라이브 프리뷰 유지 |
| **D26 알림·콜아웃** | [x] 읽기 뷰·라이브 프리뷰의 지정 유형에 `surface` 배경과 유형별 제목·아이콘·강조선 색. 제목색 대비 검사. | — | 다른 유형·중첩·접기 유지 |

## Obsidian 전용

Obsidian 전용 기능의 적용 범위다. 실제 설정 키와 예시는 [obsidian.toml](../themes/lavender-type/editors/obsidian.toml)에 있다.

- [x] **O01 패키지·모드** — manifest 이름과 폴더 일치, 단일 CSS에서 모드 분리. 다크 역할 하나를 바꿔도 라이트 선언은 그대로여야 한다.
- [x] **O02 라이브 프리뷰 편집 상태** — `[live_preview]`로 활성 줄 배경과 입력 커서 색을 각각 설정. Markdown 기호와 접기 노출 방식은 앱 소유. 실제 앱 화면 미검증.
- [x] **O03 소스 모드** — `[source_mode]`로 Markdown 서식 기호의 보조색을 켜거나 끈다. 들여쓰기·선택·커서는 기존 표시 유지. 실제 앱 화면 미검증.
- [x] **O04 내부 링크와 임베드** — `[internal_links]`로 읽기 뷰·라이브 프리뷰의 미생성 링크에 보조색·점선 밑줄, 노트 임베드에 2px 강조선 적용. 정상 링크와 탐색 동작 유지. 실제 앱 화면 미검증.
- [x] **O05 태그·속성(Properties)** — 본문 태그에 `surface`·`accent`, 노트 속성 이름에 보조색, 편집 포커스에 `focus`. 속성 값·태그 칩·YAML·사이드바 유지. 실제 앱 화면 미검증.
- [x] **O06 탐색 사이드바** — `[navigation_sidebar]`로 세 탐색 패널의 호버·현재/선택·키보드 포커스 표시 조정. 정렬·접기·들여쓰기는 앱 소유. 실제 앱 화면 미검증.
- [x] **O07 작업 공간** — `[workspace]`로 활성 탭의 굵은 제목·2px 표시선, 리본·상태 표시줄 배경과 분할 경계색을 각각 설정. 조작과 크기 유지. 실제 앱 화면 미검증.
- [x] **O08 설정·명령 팔레트·팝업** — `[dialogs]`로 데스크톱 설정 창과 공용 팔레트·메뉴의 입력·선택·비활성·키보드 포커스 표시를 각각 조정. 기본 CSS의 라이트·다크 설정 좌우 배경에서 기본 글자 7:1, 보조 글자 4.5:1, 포커스 3:1 기준 충족. 실제 앱 화면 미검증.
- [x] **O09 검색** — `[search]`로 노트 내 일치와 보관함 검색의 일치 글자를 각각 강조. 현재 일치의 별도 표시는 앱이 상태를 제공하는 읽기 뷰에만 적용. 텍스트 선택은 유지. 실제 앱 화면 미검증.
- [x] **O10 그래프 뷰** — `[graph]`로 전체·로컬 그래프의 공용 노드·연결선·레이블·강조 색을 조정. 앱의 공용 색 참조 위치 때문에 그래프별 전환은 불가. 색상 그룹 우선순위·물리·필터 유지. 실제 앱 화면 미검증.
- [x] **O11 Canvas** — `[canvas]`로 기본색 카드·그룹·연결선과 선택 표시를 각각 조정. 프리셋·직접 지정색을 덮어쓰지 않음. 실제 앱 화면 미검증.
- [x] **O12 Bases** — `[bases]`로 표의 열 제목·앱의 선택 범위와 카드의 1px 경계를 각각 조정. Markdown 표(D21)·정렬·필터·편집 유지. 실제 앱 화면 미검증.
- [x] **O13 Mermaid** — `[mermaid]`로 읽기 뷰·라이브 프리뷰의 기본 라벨·도형·선 배색과 화면 가로 넘침을 각각 조정. 앱의 기본 색상 변수만 사용하며 문서 지정색은 덮어쓰지 않음. 실제 앱 화면 미검증.
- [x] **O14 앱 아이콘** — `[icons]`로 데스크톱 리본·노트 상단 버튼의 내장 SVG 상태색을 각각 조정. 기본 크기·선 굵기를 유지하며 선택적 예시 제공. OS 아이콘·아이콘 폰트 제외. 실제 앱 화면 미검증.
- [x] **O15 모바일** — `[mobile]`로 터치 도구막대·서랍 색과 휴대폰 본문 좌우 여백 상한을 조정. 데스크톱·가상 키보드 레이아웃 유지. 실제 모바일 기기 미검증.

## 내보내기와 앱 소유 설정

| ID | 옵션 | 구현 경계 |
|---|---|---|
| OX01 | 인쇄·PDF | `[print]`에서 인쇄 전용 코드·Markdown 표 셀 줄바꿈과 표 머리글 반복을 조정. PDF 모드·용지 설정은 앱 소유이며 실제 출력은 미검증. |
| OX02 | 사용자 외형·편집 설정 | 테마 구현 범위 밖. 기본 모드, 사용자 글꼴·크기, 줄 길이, 속성 표시 방식을 테마가 켜거나 저장하지 않는다. |
| OX03 | CSS 스니펫·플러그인 | 테마 구현 범위 밖. 테마는 스니펫이나 플러그인을 켜거나 끄지 않는다. 기준 검증에서는 끄고, 충돌은 이름·버전·재현 상태를 기록한다. |
| OX04 | Publish·외부 내보내기 | 앱 테마와 별도 배포 대상. 앱 테마에 포함하지 않으며 로컬 적용을 웹 게시 결과로 간주하지 않는다. |

## 앱 테마 변수 매핑

LBRX가 생성하는 변수, 앱이 계산하는 값과 의도적으로 덮지 않는 호환성 항목만 정리한다. 선택자 기반 규칙은 위 기능별 항목을 참고한다. 전체 변수와 Publish 전용 변수는 [Obsidian 공식 CSS 변수 레퍼런스](https://docs.obsidian.md/Reference/CSS+variables/CSS+variables)에서 확인한다.

| CSS 변수 | 영역 | 기능 ID / 원본 | 생성 방식 |
|---|---|---|---|
| `--settings-background`, `--modal-sidebar-background` | 컨트롤 · 설정 창 | O08 · `background`, `surface` | 데스크톱 설정 창에 생성 |
| `--prompt-background`, `--menu-background` | 컨트롤 · 팔레트·메뉴 | O08 · `background`, `surface` | 데스크톱 공용 선택 창·메뉴에 생성 |
| `--bases-table-cell-background-selected` | 편집·문서 · Bases | O12 · `accent` | 표의 선택 범위에 반투명 배경 생성 |
| `--mermaid-text-color`, `--mermaid-surface`, `--mermaid-line-color`, `--mermaid-border-color` | 편집·문서 · Mermaid | O13 · `text`, `surface`, `muted` | 지원 렌더러의 기본색만 설정 |
| `--mobile-sidebar-background` | 창 · 모바일 서랍 | O15 · `surface` | 모바일 서랍에만 생성 |
| `--accent-h` | 기초 · 색상 | `accent` → 색상각 | [x] |
| `--accent-l` | 기초 · 색상 | `accent` → 명도 | [x] |
| `--accent-s` | 기초 · 색상 | `accent` → 채도 | [x] |
| `--background-primary` | 기초 · 색상 | D01 · `background` | [x] |
| `--background-secondary` | 기초 · 색상 | O06 · `surface` | [x] |
| `--bases-cards-shadow-hover` | 편집·문서 · Bases | O12 · `muted` | [x] |
| `--bases-cards-shadow` | 편집·문서 · Bases | O12 · `muted` | [x] |
| `--bases-table-header-background` | 편집·문서 · Bases | O12 · `surface` | [x] |
| `--bases-table-header-color` | 편집·문서 · Bases | O12 · `text` | [x] |
| `--bases-table-header-weight` | 편집·문서 · Bases | O12 · 제목 | [x] |
| `--blockquote-border-color` | 편집·문서 · 인용 | D14 · `quote` | [x] |
| `--blockquote-border-thickness` | 편집·문서 · 인용 | D14 | [x] |
| `--blockquote-color` | 편집·문서 · 인용 | D14 · `quote` | [x] |
| `--bold-color` | 기초 · 서체 | D17 | [x] |
| `--caret-color` | 기초 · 색상 | O02 · `focus` | [x] |
| `--checkbox-color` | 컨트롤 · 체크박스 | D23 | [x] |
| `--checklist-done-color` | 컨트롤 · 체크박스 | D23 | [x] |
| `--checklist-done-decoration` | 컨트롤 · 체크박스 | D23 | [x] |
| `--code-background` | 편집·문서 · 코드 | D19 · `code_block_background` | [x] |
| `--code-comment` | 편집·문서 · 코드 | D20 · `comment` | [x] |
| `--code-function` | 편집·문서 · 코드 | D20 · `function` | [x] |
| `--code-keyword` | 편집·문서 · 코드 | D20 · `keyword` | [x] |
| `--code-normal` | 편집·문서 · 코드 | D19 · `text` | [x] |
| `--code-string` | 편집·문서 · 코드 | D20 · `string` | [x] |
| `--code-white-space` | 편집·문서 · 코드 | D19 | [x] |
| `--color-accent` | 기초 · 색상 | `accent`의 HSL 입력에서 앱이 계산 | 앱 파생값 |
| `--divider-color` | 창 · Divider | O07 | [x] |
| `--embed-border-start` | 편집·문서 · 임베드 | O04 | [x] |
| `--file-line-width` | 편집·문서 · 문서 | D05 | [x] |
| `--file-margins` | 편집·문서 · 문서 | D05 | 변수 미사용 |
| `--font-monospace-theme` | 기초 · 서체 | D04 · `typography.mono.families` | [x] |
| `--font-text-theme` | 기초 · 서체 | D02 · `typography.prose.families` | [x] |
| `--footnote-size` | 편집·문서 · 각주 | D18 | [x] |
| `--graph-line` | 코어 기능 · 그래프 | O10 · `muted` | [x] |
| `--graph-node-attachment` | 코어 기능 · 그래프 | O10 · `warning` | [x] |
| `--graph-node-focused` | 코어 기능 · 그래프 | O10 · `accent` | [x] |
| `--graph-node-tag` | 코어 기능 · 그래프 | O10 · `keyword` | [x] |
| `--graph-node-unresolved` | 코어 기능 · 그래프 | O10 · `function` | [x] |
| `--graph-node` | 코어 기능 · 그래프 | O10 · `muted` | [x] |
| `--graph-text` | 코어 기능 · 그래프 | O10 · `text` | [x] |
| `--h1-color` | 편집·문서 · 제목 | D09 · `heading` | [x] |
| `--h1-size` | 편집·문서 · 제목 | D10 | [x] |
| `--h1-weight` | 편집·문서 · 제목 | D10 | [x] |
| `--h2-color` | 편집·문서 · 제목 | D09 · `heading` | [x] |
| `--h2-size` | 편집·문서 · 제목 | D10 | [x] |
| `--h2-weight` | 편집·문서 · 제목 | D10 | [x] |
| `--h3-color` | 편집·문서 · 제목 | D09 · `heading` | [x] |
| `--h3-size` | 편집·문서 · 제목 | D10 | [x] |
| `--h3-weight` | 편집·문서 · 제목 | D10 | [x] |
| `--h4-color` | 편집·문서 · 제목 | D09 · `heading` | [x] |
| `--h4-size` | 편집·문서 · 제목 | D10 | [x] |
| `--h4-weight` | 편집·문서 · 제목 | D10 | [x] |
| `--h5-color` | 편집·문서 · 제목 | D09 · `heading` | [x] |
| `--h5-size` | 편집·문서 · 제목 | D10 | [x] |
| `--h5-weight` | 편집·문서 · 제목 | D10 | [x] |
| `--h6-color` | 편집·문서 · 제목 | D09 · `heading` | [x] |
| `--h6-size` | 편집·문서 · 제목 | D10 | [x] |
| `--h6-weight` | 편집·문서 · 제목 | D10 | [x] |
| `--hr-color` | 편집·문서 · 수평선 | D18 | [x] |
| `--hr-thickness` | 편집·문서 · 수평선 | D18 | [x] |
| `--icon-color-active` | 기초 · 아이콘 | O14 · `accent` | [x] |
| `--icon-color-focused` | 기초 · 아이콘 | O14 · `focus` | [x] |
| `--icon-color-hover` | 기초 · 아이콘 | O14 · `text` | [x] |
| `--icon-color` | 기초 · 아이콘 | O14 · `muted` | [x] |
| `--icon-size` | 기초 · 아이콘 | O14 · `icons.size_px` | [x] |
| `--icon-stroke` | 기초 · 아이콘 | O14 · `icons.stroke_width` | [x] |
| `--inline-title-color` | 편집·문서 · 노트 제목 | D09 · 인라인 제목 | [x] 선택 |
| `--italic-color` | 기초 · 서체 | D17 | [x] |
| `--line-height-normal` | 기초 · 서체 | D03 · `typography.prose.line_height` | 변수 미사용 |
| `--link-color-hover` | 편집·문서 · 링크 | D12 | [x] 선택 |
| `--link-color` | 편집·문서 · 링크 | D12 · `link` | [x] |
| `--link-decoration-hover` | 편집·문서 · 링크 | D12 | [x] |
| `--link-decoration` | 편집·문서 · 링크 | D12 | [x] |
| `--link-external-color-hover` | 편집·문서 · 링크 | D12 | [x] 선택 |
| `--link-external-color` | 편집·문서 · 링크 | D12 · `link` | [x] |
| `--link-external-decoration-hover` | 편집·문서 · 링크 | D12 | [x] |
| `--link-external-decoration` | 편집·문서 · 링크 | D12 | [x] |
| `--link-unresolved-color` | 편집·문서 · 링크 | O04 | [x] |
| `--link-unresolved-decoration-color` | 편집·문서 · 링크 | O04 | [x] |
| `--link-unresolved-decoration-style` | 편집·문서 · 링크 | O04 | [x] |
| `--link-unresolved-opacity` | 편집·문서 · 링크 | O04 | [x] |
| `--list-bullet-border` | 편집·문서 · 목록 | D22 | [x] |
| `--list-bullet-radius` | 편집·문서 · 목록 | D22 | [x] |
| `--list-indent` | 편집·문서 · 목록 | D22 | [x] |
| `--list-spacing` | 편집·문서 · 목록 | D22 | [x] |
| `--metadata-label-text-color` | 편집·문서 · 속성 | O05 | [x] |
| `--p-spacing` | 기초 · 서체 | D06 | 변수 미사용 |
| `--ribbon-background-collapsed` | 창 · Ribbon | O07 | [x] |
| `--ribbon-background` | 창 · Ribbon | O07 | [x] |
| `--status-bar-background` | 창 · 상태 표시줄 | O07 | [x] |
| `--tab-background-active` | 컨트롤 · 탭 | O07 | [x] |
| `--tab-font-weight` | 컨트롤 · 탭 | O07 | [x] |
| `--table-border-color` | 편집·문서 · 표 | D21 | [x] |
| `--table-border-width` | 편집·문서 · 표 | D21 | [x] |
| `--table-header-background` | 편집·문서 · 표 | D21 | [x] |
| `--table-header-weight` | 편집·문서 · 표 | D21 | [x] |
| `--tag-background` | 편집·문서 · 태그 | O05 | [x] |
| `--tag-color` | 편집·문서 · 태그 | O05 | [x] |
| `--text-highlight-bg` | 기초 · 색상 | D17 | [x] |
| `--text-muted` | 기초 · 색상 | D01 · `muted` | [x] |
| `--text-normal` | 기초 · 색상 | D01 · `text` | [x] |
| `--text-selection` | 기초 · 색상 | D16 · `selection_background` | [x] |

## 검증 기록

2026-11-23 — 최소 테마 화면 확인 요청(읽기 뷰·라이브 프리뷰 × 라이트·다크)에 대해 사용자가 “모두 잘보여”라고 보고했다. 전체 가독성에 대한 확인이며, OS·앱 버전, 항목별 대비 수치와 언어별 토큰 결과는 기록되지 않았다. 위 표의 개별 항목이나 Publish를 이 보고만으로 통과 처리하지 않는다.
