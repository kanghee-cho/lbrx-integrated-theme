# Obsidian 옵션 대장

[적용 가이드](09-obsidian.md) · [문서 앱 공유 ID](11-cross-app-options.md#shared) · [Typora 대응표](08-typora-options.md)

D20의 코드 토큰 매핑과 언어별 읽기 뷰·라이브 프리뷰 검증은 [Obsidian 언어별 코드 구문 강조](24-obsidian-language-highlighting.md)에서 추적한다.

[최소 어댑터](../src/obsidian.rs)가 manifest와 CSS를 생성한다. `[x]`는 생성 확인, `[ ]`는 미구현, `—`는 화면 미검증이다. 읽기·라이브 열은 라이트/다크 결과를 각각 기록한다. 변수 색인의 Typora 열은 매핑을 이해하기 위한 비교이며 Typora 구현 근거는 Typora 대장에서 관리한다.

[Obsidian 공식 CSS 변수 레퍼런스의 고정본](https://github.com/obsidianmd/obsidian-developer-docs/tree/1c26772ac13d6e9870c865936d389fdfe694e742/en/Reference/CSS%20variables)에서 표의 변수명과 별도 명시한 호환성 변수를 수록한다. 앱과 Publish는 분리한다. `--font-` 같은 접두사, 임의 CSS 속성·선택자의 무한 조합, 비공개 앱 변수, 커뮤니티 플러그인은 전수 범위가 아니다. 공식 문서 갱신 시 변수명+배포 대상 기준으로 차이를 추가하고 기존 상태를 보존한다. 폐기 예정 변수는 새 테마 옵션으로 권장하지 않는다.

## 공통 문서 기능의 Obsidian 적용

| ID · 설정 대상 | 생성 · 적용 범위 | 읽기 | 라이브 |
|---|---|---|---|
| **D01 문서 배경·본문색** | [x] 문서 배경·본문색: 편집 중인 줄만 다른 바탕에 남지 않는가 | — | — |
| **D02 본문 글꼴 우선순위** | [x] 본문 글꼴 순서: EN·KR·JP 혼합 문장과 사용자 지정 글꼴 우선순위 | — | — |
| **D03 본문 크기·줄 높이** | [ ] 본문 크기·줄 높이: 앱 글자 크기를 바꿔도 줄 겹침이 없는가 | — | — |
| **D04 코드 글꼴·크기·줄 높이** | [x] 고정폭 글꼴. [ ] 나머지 세부 표현: 코드 서체·크기·줄 높이: 인라인 코드와 여러 줄 코드에서 확인 | — | — |
| **D05 본문 최대 폭·좌우 여백** | [ ] 최대 폭·좌우 여백: 앱의 읽기 편한 줄 길이 설정 및 좁은 분할 창 | — | — |
| **D06 문단 앞뒤 간격** | [ ] 문단 앞뒤 간격: 편집기에 빈 줄이 추가된 것처럼 보이지 않는가 | — | — |
| **D07 문단 정렬·쓰기 방향** | [ ] 정렬·쓰기 방향: 혼합 언어 문단과 RTL 커서 이동 | — | — |
| **D08 서체 세부 표현** | [ ] 자간·합자·대소문자 표현: 식별자와 원문을 오해하지 않는가 | — | — |
| **D09 제목색** | [ ] 제목색: H1–H6와 노트의 인라인 제목 구분 | — | — |
| **D10 제목 단계별 조판** | [ ] 제목 단계별 크기·굵기·간격: 접기 표시와 겹치지 않는가 | — | — |
| **D11 제목 자동 번호** | [ ] 제목 자동 번호: 표시 전용 번호가 원문이나 개요에 중복되지 않는가 | — | — |
| **D12 링크색·밑줄** | [x] 링크색. [ ] 나머지 세부 표현: 링크색·밑줄: 내부·외부 링크와 편집 중인 별칭 | — | — |
| **D13 키보드 링크 포커스** | [ ] 링크 포커스: 키보드로 이동 가능한 링크의 포커스 표시 | — | — |
| **D14 인용문** | [ ] 인용문: 중첩 인용의 글자색·경계선·들여쓰기 | — | — |
| **D15 인라인 코드색** | [ ] 인라인 코드: 링크나 표 셀 안에서도 충분한 대비 | — | — |
| **D16 일반 텍스트 선택색** | [x] 브라우저 기본 선택의 배경은 본문 `prose_selection_background`, 코드 `selection_background`로 분리. 편집기의 별도 선택 레이어는 기존 배색 유지. | — | — |
| **D17 본문 강조 표현** | [ ] 굵게·기울임·취소선·하이라이트: 복합 강조와 편집 기호 | — | — |
| **D18 각주·참조·수평 구분선** | [ ] 각주·참조·수평선: 참조 번호와 각주 본문에서 돌아오는 링크 | — | — |
| **D19 코드 블록 바탕·테두리** | [x] 읽기 뷰·라이브 프리뷰의 코드 블록에 독립 바탕색을 생성한다. [ ] 별도 테두리·긴 줄 스크롤·편집 커서. | — | — |
| **D20 코드 구문색** | [x] 주석·키워드·문자열·함수. [ ] 숫자·오류의 언어별 식별 | — | — |
| **D21 표** | [ ] 표: 헤더·행·셀 경계·여백과 편집 중 선택한 셀 | — | — |
| **D22 순서·비순서 목록** | [ ] 순서·비순서 목록: 중첩 표식과 들여쓰기 가이드 | — | — |
| **D23 작업 목록** | [ ] 작업 목록: 완료 표시 아래 미완료 자식의 가독성과 클릭 동작 | — | — |
| **D24 이미지·캡션** | [ ] 이미지·캡션: 지정한 임베드 폭과 좁은 창에서 넘침. 대체 텍스트를 캡션으로 강제하지 않음 | — | — |
| **D25 수식** | [ ] 인라인·블록 수식: 긴 식의 넘침과 수학 전용 글꼴 보존 | — | — |
| **D26 알림·콜아웃** | [ ] 콜아웃: 유형별 아이콘·제목·배경 대비, 중첩·접기 동작 | — | — |

## Obsidian 전용

아래는 Obsidian 전용 옵션 **후보**다. 제안 ID는 아직 TOML 키가 아니다. 공유 옵션과 겹치면 색을 복제하지 않고 같은 원본 역할을 참조한다.

- [x] **O01 패키지·모드** — manifest 이름과 폴더 일치, 단일 CSS에서 모드 분리. 다크 역할 하나를 바꿔도 라이트 선언은 그대로여야 한다.
- [ ] **O02 라이브 프리뷰 편집 상태** — 활성 줄, 커서, Markdown 기호와 접기 표시. 커서 안팎에서 숨김·표시가 원래대로인가.
- [ ] **O03 소스 모드** — Markdown 원문 기호·들여쓰기·선택 대비. 라이브 프리뷰 통과로 대체하지 않는다.
- [ ] **O04 내부 링크와 임베드** — 미생성 링크, 별칭, 블록·제목 참조, `![[노트]]` 경계와 중첩. 탐색 동작은 변경하지 않는다.
- [ ] **O05 태그·속성(Properties)** — 인라인 태그와 속성 칩, 이름·값·편집 필드. YAML 원문 표시 상태는 별도 검사한다.
- [ ] **O06 탐색 사이드바** — 파일 탐색기·개요·백링크의 선택/호버/포커스, 접힌 항목과 들여쓰기. 탐색 정렬은 앱 소유다.
- [ ] **O07 작업 공간** — 리본·탭·분할 경계·상태 표시줄. 활성 탭과 비활성 탭을 색 이외의 단서로도 구분한다.
- [ ] **O08 설정·명령 팔레트·팝업** — 좌측 목록과 우측 내용의 전경/배경을 따로 검사한다. 입력·선택·비활성·키보드 포커스 상태를 포함한다.
- [ ] **O09 검색** — 노트 내 일치, 현재 일치, 보관함 검색 결과의 강조와 텍스트 선택을 구분한다.
- [ ] **O10 그래프 뷰** — 노드·에지·레이블·강조 노드 대비. 로컬/전체 그래프를 나누고 물리·필터 설정은 건드리지 않는다.
- [ ] **O11 Canvas** — 카드·그룹·연결선·선택 경계. 사용자가 지정한 카드색을 덮어쓰지 않는다.
- [ ] **O12 Bases** — 표·카드 뷰의 열 제목, 행 선택, 카드 경계. 노트의 Markdown 표(D21)와 별개로 확인한다.
- [ ] **O13 Mermaid** — 라벨·도형·선 대비와 긴 다이어그램 넘침. 문서에 명시된 배색과 충돌하는지 확인한다.
- [ ] **O14 앱 아이콘** — 내장 SVG의 크기·색·선 굵기 후보. 로컬 아이콘 폰트 보관 여부를 구현으로 계산하지 않는다.
- [ ] **O15 모바일** — 터치 도구막대·서랍·좁은 창·가상 키보드 노출 시 가독성. 데스크톱 결과를 복사하지 않는다.

## 내보내기와 앱 소유 설정

| ID | 옵션 | 구현 경계 |
|---|---|---|
| OX01 | 인쇄·PDF | 미구현. 다크 화면에서 밝은 PDF, 긴 표·코드의 페이지 나눔을 실제 출력으로 확인한다. 용지 설정은 앱 소유다. |
| OX02 | 사용자 외형·편집 설정 | 기본 모드, 사용자 글꼴·크기, 줄 길이, 속성 표시 방식을 테마가 켜거나 저장하지 않는다. |
| OX03 | CSS 스니펫·플러그인 | 기준 검증에서는 끈다. 충돌은 이름·버전·재현 상태를 기록한다. Style Settings 같은 플러그인은 필수 의존성이 아니다. |
| OX04 | Publish·외부 내보내기 | 보관함 앱 테마와 별도 배포 대상. 로컬 테마 적용을 웹 게시 결과로 간주하지 않는다. |

## 공식 변수 색인

서로 다른 변수명 **686개**. 같은 이름도 앱 테마와 Publish의 적용 경로가 다르면 각각 한 행이다. 각 구역에서 변수명을 검색하면 호버·포커스·선택 상태를 독립적으로 추적할 수 있다.

- [공통 원본 매핑](#css-1): 31행
- [Typora 합일 후보](#css-2): 138행
- [Obsidian 전용](#css-3): 467행
- [Publish 별도](#css-4): 60행

<a id="css-1"></a>
### 공통 원본 매핑

| CSS 변수 | 적용 영역 | 연결 ID / 원본 | Typora 대응 선언 | Obsidian 생성 |
|---|---|---|---|---|
| `--background-primary` | 기초 · 색상 | D01 · `background` | 원본 `background` | [x] |
| `--background-secondary` | 기초 · 색상 | O06 · `surface` | 원본 `surface` (T13 다크 좌측만) | [x] |
| `--caret-color` | 기초 · 색상 | T04 · `focus` | 원본 `focus` (코드 커서) | [ ] |
| `--color-accent` | 기초 · 색상 | `accent`의 HSL 입력에서 앱이 계산 | 미구현 (해당 UI 역할) | 앱 파생값 |
| `--text-error` | 기초 · 색상 | 상태·강조 · `error` | 미구현 (해당 UI 역할) | [ ] |
| `--text-muted` | 기초 · 색상 | D01 · `muted` | 원본 `muted` (보조 요소 일부) | [x] |
| `--text-normal` | 기초 · 색상 | D01 · `text` | 원본 `text` | [x] |
| `--text-selection` | 기초 · 색상 | D16 · `selection_background` | 원본 `selection_background` | [x] |
| `--text-success` | 기초 · 색상 | 상태·강조 · `success` | 미구현 (해당 UI 역할) | [ ] |
| `--text-warning` | 기초 · 색상 | 상태·강조 · `warning` | 미구현 (해당 UI 역할) | [ ] |
| `--font-monospace-theme` | 기초 · 서체 | D04 · `typography.mono.families` | 원본 `typography.mono.families` | [x] |
| `--font-text-size` | 기초 · 서체 | D03 · `typography.prose.size_px` | 원본 `typography.prose.size_px` | [ ] |
| `--font-text-theme` | 기초 · 서체 | D02 · `typography.prose.families` | 원본 `typography.prose.families` | [x] |
| `--line-height-normal` | 기초 · 서체 | D03 · `typography.prose.line_height` | 원본 `typography.prose.line_height` | [ ] |
| `--link-color` | 편집·문서 · 링크 | D12 · `link` | 원본 `link` | [x] |
| `--link-external-color` | 편집·문서 · 링크 | D12 · `link` | 원본 `link` | [x] |
| `--blockquote-border-color` | 편집·문서 · 인용 | D14 · `quote` | 원본 `quote` | [ ] |
| `--blockquote-color` | 편집·문서 · 인용 | D14 · `quote` | 원본 `quote` | [ ] |
| `--h1-color` | 편집·문서 · 제목 | D09 · `heading` | 원본 `heading` (모든 단계 동일) | [ ] |
| `--h2-color` | 편집·문서 · 제목 | D09 · `heading` | 원본 `heading` (모든 단계 동일) | [ ] |
| `--h3-color` | 편집·문서 · 제목 | D09 · `heading` | 원본 `heading` (모든 단계 동일) | [ ] |
| `--h4-color` | 편집·문서 · 제목 | D09 · `heading` | 원본 `heading` (모든 단계 동일) | [ ] |
| `--h5-color` | 편집·문서 · 제목 | D09 · `heading` | 원본 `heading` (모든 단계 동일) | [ ] |
| `--h6-color` | 편집·문서 · 제목 | D09 · `heading` | 원본 `heading` (모든 단계 동일) | [ ] |
| `--code-background` | 편집·문서 · 코드 | D19 · `code_block_background` | 코드 블록 안에서만 재정의. 인라인 코드는 기존 `background` 유지. | [x] |
| `--code-comment` | 편집·문서 · 코드 | D20 · `comment` | 원본 `comment` | [x] |
| `--code-function` | 편집·문서 · 코드 | D20 · `function` | 원본 `function` | [x] |
| `--code-keyword` | 편집·문서 · 코드 | D20 · `keyword` | 원본 `keyword` | [x] |
| `--code-normal` | 편집·문서 · 코드 | D19 · `text` | 원본 `text` | [x] |
| `--code-size` | 편집·문서 · 코드 | D04 · `typography.mono.size_px` | 원본 `typography.mono.size_px` | [ ] |
| `--code-string` | 편집·문서 · 코드 | D20 · `string` | 원본 `string` | [x] |

<a id="css-2"></a>
### Typora 합일 후보

| CSS 변수 | 적용 영역 | 연결 ID / 원본 | Typora 대응 선언 | Obsidian 생성 |
|---|---|---|---|---|
| `--text-highlight-bg` | 기초 · 색상 | D17 | 미구현 | [ ] |
| `--bold-color` | 기초 · 서체 | D17 | 미구현 | [ ] |
| `--bold-modifier` | 기초 · 서체 | D17 | 미구현 | [ ] |
| `--bold-weight` | 기초 · 서체 | D17 | 미구현 | [ ] |
| `--italic-color` | 기초 · 서체 | D17 | 미구현 | [ ] |
| `--p-spacing` | 기초 · 서체 | D06 | 미구현 | [ ] |
| `--checkbox-border-color` | 컨트롤 · 체크박스 | D23 | 미구현 | [ ] |
| `--checkbox-border-color-hover` | 컨트롤 · 체크박스 | D23 | 미구현 | [ ] |
| `--checkbox-color` | 컨트롤 · 체크박스 | D23 | 미구현 | [ ] |
| `--checkbox-color-hover` | 컨트롤 · 체크박스 | D23 | 미구현 | [ ] |
| `--checkbox-margin-inline-start` | 컨트롤 · 체크박스 | D23 | 미구현 | [ ] |
| `--checkbox-marker-color` | 컨트롤 · 체크박스 | D23 | 미구현 | [ ] |
| `--checkbox-radius` | 컨트롤 · 체크박스 | D23 | 미구현 | [ ] |
| `--checkbox-size` | 컨트롤 · 체크박스 | D23 | 미구현 | [ ] |
| `--checklist-done-color` | 컨트롤 · 체크박스 | D23 | 미구현 | [ ] |
| `--checklist-done-decoration` | 컨트롤 · 체크박스 | D23 | 미구현 | [ ] |
| `--footnote-size` | 편집·문서 · 각주 | D18 | 미구현 | [ ] |
| `--link-color-hover` | 편집·문서 · 링크 | D12 | 미구현 | [ ] |
| `--link-decoration` | 편집·문서 · 링크 | D12 | 고정 `underline` | [ ] |
| `--link-decoration-hover` | 편집·문서 · 링크 | D12 | 미구현 | [ ] |
| `--link-decoration-thickness` | 편집·문서 · 링크 | D12 | 미구현 | [ ] |
| `--link-external-color-hover` | 편집·문서 · 링크 | D12 | 미구현 | [ ] |
| `--link-external-decoration` | 편집·문서 · 링크 | D12 | 고정 `underline` | [ ] |
| `--link-external-decoration-hover` | 편집·문서 · 링크 | D12 | 미구현 | [ ] |
| `--link-weight` | 편집·문서 · 링크 | D12 | 미구현 | [ ] |
| `--list-bullet-border` | 편집·문서 · 목록 | D22 | 미구현 | [ ] |
| `--list-bullet-end-padding` | 편집·문서 · 목록 | D22 | 미구현 | [ ] |
| `--list-bullet-radius` | 편집·문서 · 목록 | D22 | 미구현 | [ ] |
| `--list-bullet-size` | 편집·문서 · 목록 | D22 | 미구현 | [ ] |
| `--list-bullet-transform` | 편집·문서 · 목록 | D22 | 미구현 | [ ] |
| `--list-indent` | 편집·문서 · 목록 | D22 | 미구현 | [ ] |
| `--list-marker-color` | 편집·문서 · 목록 | D22 | 미구현 | [ ] |
| `--list-marker-color-collapsed` | 편집·문서 · 목록 | D22 | 미구현 | [ ] |
| `--list-marker-color-hover` | 편집·문서 · 목록 | D22 | 미구현 | [ ] |
| `--list-numbered-style` | 편집·문서 · 목록 | D22 | 미구현 | [ ] |
| `--list-spacing` | 편집·문서 · 목록 | D22 | 미구현 | [ ] |
| `--file-line-width` | 편집·문서 · 문서 | D05 | 미구현 | [ ] |
| `--file-margins` | 편집·문서 · 문서 | D05 | 미구현 | [ ] |
| `--hr-color` | 편집·문서 · 수평선 | D18 | 미구현 | [ ] |
| `--hr-thickness` | 편집·문서 · 수평선 | D18 | 미구현 | [ ] |
| `--blockquote-background-color` | 편집·문서 · 인용 | D14 | 미구현 | [ ] |
| `--blockquote-border-thickness` | 편집·문서 · 인용 | D14 | 고정 `3px` | [ ] |
| `--blockquote-font-style` | 편집·문서 · 인용 | D14 | 미구현 | [ ] |
| `--h1-font` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h1-line-height` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h1-size` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h1-style` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h1-variant` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h1-weight` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h2-font` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h2-line-height` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h2-size` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h2-style` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h2-variant` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h2-weight` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h3-font` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h3-line-height` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h3-size` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h3-style` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h3-variant` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h3-weight` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h4-font` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h4-line-height` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h4-size` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h4-style` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h4-variant` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h4-weight` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h5-font` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h5-line-height` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h5-size` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h5-style` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h5-variant` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h5-weight` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h6-font` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h6-line-height` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h6-size` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h6-style` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h6-variant` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--h6-weight` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--heading-spacing` | 편집·문서 · 제목 | D10 | 미구현 | [ ] |
| `--code-important` | 편집·문서 · 코드 | D20 | 미구현 | [ ] |
| `--code-operator` | 편집·문서 · 코드 | D20 | 미구현 | [ ] |
| `--code-property` | 편집·문서 · 코드 | D20 | 미구현 | [ ] |
| `--code-punctuation` | 편집·문서 · 코드 | D20 | 미구현 | [ ] |
| `--code-tag` | 편집·문서 · 코드 | D20 | 미구현 | [ ] |
| `--code-value` | 편집·문서 · 코드 | D20 | 미구현 | [ ] |
| `--code-white-space` | 편집·문서 · 코드 | D19 | 미구현 | [ ] |
| `--callout-blend-mode` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-border-opacity` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-border-width` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-bug` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-color` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-content-background` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-content-padding` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-default` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-error` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-example` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-fail` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-icon` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-important` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-info` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-padding` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-question` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-quote` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-radius` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-success` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-summary` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-tip` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-title-color` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-title-padding` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-title-size` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-title-weight` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-todo` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--callout-warning` | 편집·문서 · 콜아웃 | D26 | 미구현 | [ ] |
| `--table-background` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-border-color` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-border-width` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-cell-vertical-alignment` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-column-alt-background` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-column-first-border-width` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-column-last-border-width` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-column-max-width` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-header-background` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-header-background-hover` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-header-border-color` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-header-border-width` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-header-color` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-header-font` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-header-size` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-header-weight` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-line-height` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-row-alt-background` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-row-alt-background-hover` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-row-background-hover` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-row-last-border-width` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-text-color` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-text-size` | 편집·문서 · 표 | D21 | 미구현 | [ ] |
| `--table-white-space` | 편집·문서 · 표 | D21 | 미구현 | [ ] |

<a id="css-3"></a>
### Obsidian 전용

| CSS 변수 | 적용 영역 | 연결 ID / 원본 | Typora 대응 선언 | Obsidian 생성 |
|---|---|---|---|---|
| `--border-width` | 기초 · Borders | — | 해당 없음 | [ ] |
| `--cursor` | 기초 · Cursor | — | 해당 없음 | [ ] |
| `--cursor-link` | 기초 · Cursor | — | 해당 없음 | [ ] |
| `--radius-l` | 기초 · Radiuses | — | 해당 없음 | [ ] |
| `--radius-m` | 기초 · Radiuses | — | 해당 없음 | [ ] |
| `--radius-s` | 기초 · Radiuses | — | 해당 없음 | [ ] |
| `--radius-xl` | 기초 · Radiuses | — | 해당 없음 | [ ] |
| `--size-2-1` | 기초 · 간격 | — | 해당 없음 | [ ] |
| `--size-2-2` | 기초 · 간격 | — | 해당 없음 | [ ] |
| `--size-2-3` | 기초 · 간격 | — | 해당 없음 | [ ] |
| `--size-4-1` | 기초 · 간격 | — | 해당 없음 | [ ] |
| `--size-4-12` | 기초 · 간격 | — | 해당 없음 | [ ] |
| `--size-4-16` | 기초 · 간격 | — | 해당 없음 | [ ] |
| `--size-4-18` | 기초 · 간격 | — | 해당 없음 | [ ] |
| `--size-4-2` | 기초 · 간격 | — | 해당 없음 | [ ] |
| `--size-4-3` | 기초 · 간격 | — | 해당 없음 | [ ] |
| `--size-4-4` | 기초 · 간격 | — | 해당 없음 | [ ] |
| `--size-4-5` | 기초 · 간격 | — | 해당 없음 | [ ] |
| `--size-4-6` | 기초 · 간격 | — | 해당 없음 | [ ] |
| `--size-4-8` | 기초 · 간격 | — | 해당 없음 | [ ] |
| `--size-4-9` | 기초 · 간격 | — | 해당 없음 | [ ] |
| `--layer-cover` | 기초 · 레이어 | — | 해당 없음 | [ ] |
| `--layer-dragged-item` | 기초 · 레이어 | — | 해당 없음 | [ ] |
| `--layer-menu` | 기초 · 레이어 | — | 해당 없음 | [ ] |
| `--layer-modal` | 기초 · 레이어 | — | 해당 없음 | [ ] |
| `--layer-notice` | 기초 · 레이어 | — | 해당 없음 | [ ] |
| `--layer-popover` | 기초 · 레이어 | — | 해당 없음 | [ ] |
| `--layer-sidedock` | 기초 · 레이어 | — | 해당 없음 | [ ] |
| `--layer-slides` | 기초 · 레이어 | — | 해당 없음 | [ ] |
| `--layer-status-bar` | 기초 · 레이어 | — | 해당 없음 | [ ] |
| `--layer-tooltip` | 기초 · 레이어 | — | 해당 없음 | [ ] |
| `--accent-h` | 기초 · 색상 | `accent` → 색상각 | 해당 없음 | [x] |
| `--accent-l` | 기초 · 색상 | `accent` → 명도 | 해당 없음 | [x] |
| `--accent-s` | 기초 · 색상 | `accent` → 채도 | 해당 없음 | [x] |
| `--background-modifier-active-hover` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--background-modifier-border` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--background-modifier-border-focus` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--background-modifier-border-hover` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--background-modifier-error` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--background-modifier-error-hover` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--background-modifier-error-rgb` | 기초 · 색상 | — · 호환 전용(폐기 예정) | 해당 없음 | [ ] |
| `--background-modifier-form-field` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--background-modifier-hover` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--background-modifier-message` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--background-modifier-success` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--background-modifier-success-rgb` | 기초 · 색상 | — · 호환 전용(폐기 예정) | 해당 없음 | [ ] |
| `--background-primary-alt` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--background-secondary-alt` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-accent-1` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-accent-2` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-accent-hsl` | 기초 · 색상 | — · 호환 전용(폐기 예정) | 해당 없음 | [ ] |
| `--color-base-00` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-base-05` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-base-10` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-base-100` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-base-20` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-base-25` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-base-30` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-base-35` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-base-40` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-base-50` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-base-60` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-base-70` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-blue` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-blue-rgb` | 기초 · 색상 | — · 호환 전용(폐기 예정) | 해당 없음 | [ ] |
| `--color-cyan` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-cyan-rgb` | 기초 · 색상 | — · 호환 전용(폐기 예정) | 해당 없음 | [ ] |
| `--color-green` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-green-rgb` | 기초 · 색상 | — · 호환 전용(폐기 예정) | 해당 없음 | [ ] |
| `--color-orange` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-orange-rgb` | 기초 · 색상 | — · 호환 전용(폐기 예정) | 해당 없음 | [ ] |
| `--color-pink` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-pink-rgb` | 기초 · 색상 | — · 호환 전용(폐기 예정) | 해당 없음 | [ ] |
| `--color-purple` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-purple-rgb` | 기초 · 색상 | — · 호환 전용(폐기 예정) | 해당 없음 | [ ] |
| `--color-red` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-red-rgb` | 기초 · 색상 | — · 호환 전용(폐기 예정) | 해당 없음 | [ ] |
| `--color-yellow` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--color-yellow-rgb` | 기초 · 색상 | — · 호환 전용(폐기 예정) | 해당 없음 | [ ] |
| `--interactive-accent` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--interactive-accent-hover` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--interactive-accent-hsl` | 기초 · 색상 | — · 호환 전용(폐기 예정) | 해당 없음 | [ ] |
| `--interactive-hover` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--interactive-normal` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--mono-0` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--mono-100` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--mono-rgb-0` | 기초 · 색상 | — · 호환 전용(폐기 예정) | 해당 없음 | [ ] |
| `--mono-rgb-100` | 기초 · 색상 | — · 호환 전용(폐기 예정) | 해당 없음 | [ ] |
| `--text-accent` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--text-accent-hover` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--text-faint` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--text-highlight-bg-rgb` | 기초 · 색상 | — · 호환 전용(폐기 예정) | 해당 없음 | [ ] |
| `--text-on-accent` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--text-on-accent-inverted` | 기초 · 색상 | — | 해당 없음 | [ ] |
| `--font-black` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--font-bold` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--font-extrabold` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--font-extralight` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--font-interface-theme` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--font-light` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--font-medium` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--font-normal` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--font-semibold` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--font-small` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--font-smaller` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--font-smallest` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--font-thin` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--font-ui-large` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--font-ui-medium` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--font-ui-small` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--font-ui-smaller` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--font-weight` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--line-height-tight` | 기초 · 서체 | — | 해당 없음 | [ ] |
| `--clickable-icon-radius` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-color` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-color-active` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-color-focused` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-color-hover` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-l` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-l-stroke-width` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-m` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-m-stroke-width` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-opacity` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-opacity-active` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-opacity-hover` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-s` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-s-stroke-width` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-size` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-stroke` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-xl` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-xl-stroke-width` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-xs` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--icon-xs-stroke-width` | 기초 · 아이콘 | — | 해당 없음 | [ ] |
| `--divider-color` | 창 · Divider | — | 해당 없음 | [ ] |
| `--divider-color-hover` | 창 · Divider | — | 해당 없음 | [ ] |
| `--divider-vertical-height` | 창 · Divider | — | 해당 없음 | [ ] |
| `--divider-width` | 창 · Divider | — | 해당 없음 | [ ] |
| `--divider-width-hover` | 창 · Divider | — | 해당 없음 | [ ] |
| `--ribbon-background` | 창 · Ribbon | — | 해당 없음 | [ ] |
| `--ribbon-background-collapsed` | 창 · Ribbon | — | 해당 없음 | [ ] |
| `--ribbon-padding` | 창 · Ribbon | — | 해당 없음 | [ ] |
| `--ribbon-width` | 창 · Ribbon | — | 해당 없음 | [ ] |
| `--scrollbar-active-thumb-bg` | 창 · Scrollbar | — | 해당 없음 | [ ] |
| `--scrollbar-bg` | 창 · Scrollbar | — | 해당 없음 | [ ] |
| `--scrollbar-thumb-bg` | 창 · Scrollbar | — | 해당 없음 | [ ] |
| `--workspace-background-translucent` | 창 · Workspace | — | 해당 없음 | [ ] |
| `--sidebar-markdown-font-size` | 창 · 사이드바 | — | 해당 없음 | [ ] |
| `--sidebar-tab-text-display` | 창 · 사이드바 | — | 해당 없음 | [ ] |
| `--status-bar-background` | 창 · 상태 표시줄 | — | 해당 없음 | [ ] |
| `--status-bar-border-color` | 창 · 상태 표시줄 | — | 해당 없음 | [ ] |
| `--status-bar-border-width` | 창 · 상태 표시줄 | — | 해당 없음 | [ ] |
| `--status-bar-font-size` | 창 · 상태 표시줄 | — | 해당 없음 | [ ] |
| `--status-bar-position` | 창 · 상태 표시줄 | — | 해당 없음 | [ ] |
| `--status-bar-radius` | 창 · 상태 표시줄 | — | 해당 없음 | [ ] |
| `--status-bar-scroll-padding` | 창 · 상태 표시줄 | — | 해당 없음 | [ ] |
| `--status-bar-text-color` | 창 · 상태 표시줄 | — | 해당 없음 | [ ] |
| `--header-height` | 창 · 창 프레임 | — | 해당 없음 | [ ] |
| `--titlebar-background` | 창 · 창 프레임 | — | 해당 없음 | [ ] |
| `--titlebar-background-focused` | 창 · 창 프레임 | — | 해당 없음 | [ ] |
| `--titlebar-border-color` | 창 · 창 프레임 | — | 해당 없음 | [ ] |
| `--titlebar-border-width` | 창 · 창 프레임 | — | 해당 없음 | [ ] |
| `--titlebar-text-color` | 창 · 창 프레임 | — | 해당 없음 | [ ] |
| `--titlebar-text-color-focused` | 창 · 창 프레임 | — | 해당 없음 | [ ] |
| `--titlebar-text-weight` | 창 · 창 프레임 | — | 해당 없음 | [ ] |
| `--swatch-height` | 컨트롤 · Color input | — | 해당 없음 | [ ] |
| `--swatch-radius` | 컨트롤 · Color input | — | 해당 없음 | [ ] |
| `--swatch-shadow` | 컨트롤 · Color input | — | 해당 없음 | [ ] |
| `--swatch-width` | 컨트롤 · Color input | — | 해당 없음 | [ ] |
| `--dialog-max-height` | 컨트롤 · Dialog | — | 해당 없음 | [ ] |
| `--dialog-max-width` | 컨트롤 · Dialog | — | 해당 없음 | [ ] |
| `--dialog-width` | 컨트롤 · Dialog | — | 해당 없음 | [ ] |
| `--drag-ghost-background` | 컨트롤 · Dragging | — | 해당 없음 | [ ] |
| `--drag-ghost-text-color` | 컨트롤 · Dragging | — | 해당 없음 | [ ] |
| `--dropdown-background` | 컨트롤 · Dropdowns | — | 해당 없음 | [ ] |
| `--dropdown-background-blend-mode` | 컨트롤 · Dropdowns | — | 해당 없음 | [ ] |
| `--dropdown-background-hover` | 컨트롤 · Dropdowns | — | 해당 없음 | [ ] |
| `--dropdown-background-position` | 컨트롤 · Dropdowns | — | 해당 없음 | [ ] |
| `--dropdown-background-size` | 컨트롤 · Dropdowns | — | 해당 없음 | [ ] |
| `--dropdown-padding` | 컨트롤 · Dropdowns | — | 해당 없음 | [ ] |
| `--indentation-guide-color` | 컨트롤 · Indentation guides | — | 해당 없음 | [ ] |
| `--indentation-guide-color-active` | 컨트롤 · Indentation guides | — | 해당 없음 | [ ] |
| `--indentation-guide-editing-indent` | 컨트롤 · Indentation guides | — | 해당 없음 | [ ] |
| `--indentation-guide-reading-indent` | 컨트롤 · Indentation guides | — | 해당 없음 | [ ] |
| `--indentation-guide-source-indent` | 컨트롤 · Indentation guides | — | 해당 없음 | [ ] |
| `--indentation-guide-width` | 컨트롤 · Indentation guides | — | 해당 없음 | [ ] |
| `--indentation-guide-width-active` | 컨트롤 · Indentation guides | — | 해당 없음 | [ ] |
| `--pill-background` | 컨트롤 · Multi-select | — | 해당 없음 | [ ] |
| `--pill-background-hover` | 컨트롤 · Multi-select | — | 해당 없음 | [ ] |
| `--pill-border-color` | 컨트롤 · Multi-select | — | 해당 없음 | [ ] |
| `--pill-border-color-hover` | 컨트롤 · Multi-select | — | 해당 없음 | [ ] |
| `--pill-border-width` | 컨트롤 · Multi-select | — | 해당 없음 | [ ] |
| `--pill-color` | 컨트롤 · Multi-select | — | 해당 없음 | [ ] |
| `--pill-color-hover` | 컨트롤 · Multi-select | — | 해당 없음 | [ ] |
| `--pill-color-remove` | 컨트롤 · Multi-select | — | 해당 없음 | [ ] |
| `--pill-color-remove-hover` | 컨트롤 · Multi-select | — | 해당 없음 | [ ] |
| `--pill-decoration` | 컨트롤 · Multi-select | — | 해당 없음 | [ ] |
| `--pill-decoration-hover` | 컨트롤 · Multi-select | — | 해당 없음 | [ ] |
| `--pill-padding-x` | 컨트롤 · Multi-select | — | 해당 없음 | [ ] |
| `--pill-padding-y` | 컨트롤 · Multi-select | — | 해당 없음 | [ ] |
| `--pill-radius` | 컨트롤 · Multi-select | — | 해당 없음 | [ ] |
| `--pill-weight` | 컨트롤 · Multi-select | — | 해당 없음 | [ ] |
| `--slider-thumb-border-color` | 컨트롤 · Slider | — | 해당 없음 | [ ] |
| `--slider-thumb-border-width` | 컨트롤 · Slider | — | 해당 없음 | [ ] |
| `--slider-thumb-height` | 컨트롤 · Slider | — | 해당 없음 | [ ] |
| `--slider-thumb-radius` | 컨트롤 · Slider | — | 해당 없음 | [ ] |
| `--slider-thumb-width` | 컨트롤 · Slider | — | 해당 없음 | [ ] |
| `--slider-thumb-y` | 컨트롤 · Slider | — | 해당 없음 | [ ] |
| `--slider-track-background` | 컨트롤 · Slider | — | 해당 없음 | [ ] |
| `--slider-track-height` | 컨트롤 · Slider | — | 해당 없음 | [ ] |
| `--toggle-border-width` | 컨트롤 · Toggle | — | 해당 없음 | [ ] |
| `--toggle-radius` | 컨트롤 · Toggle | — | 해당 없음 | [ ] |
| `--toggle-s-border-width` | 컨트롤 · Toggle | — | 해당 없음 | [ ] |
| `--toggle-s-thumb-height` | 컨트롤 · Toggle | — | 해당 없음 | [ ] |
| `--toggle-s-thumb-width` | 컨트롤 · Toggle | — | 해당 없음 | [ ] |
| `--toggle-s-width` | 컨트롤 · Toggle | — | 해당 없음 | [ ] |
| `--toggle-thumb-color` | 컨트롤 · Toggle | — | 해당 없음 | [ ] |
| `--toggle-thumb-height` | 컨트롤 · Toggle | — | 해당 없음 | [ ] |
| `--toggle-thumb-radius` | 컨트롤 · Toggle | — | 해당 없음 | [ ] |
| `--toggle-thumb-width` | 컨트롤 · Toggle | — | 해당 없음 | [ ] |
| `--toggle-width` | 컨트롤 · Toggle | — | 해당 없음 | [ ] |
| `--prompt-border-color` | 컨트롤 · 명령 입력창 | — | 해당 없음 | [ ] |
| `--prompt-border-width` | 컨트롤 · 명령 입력창 | — | 해당 없음 | [ ] |
| `--prompt-input-height` | 컨트롤 · 명령 입력창 | — | 해당 없음 | [ ] |
| `--prompt-max-height` | 컨트롤 · 명령 입력창 | — | 해당 없음 | [ ] |
| `--prompt-max-width` | 컨트롤 · 명령 입력창 | — | 해당 없음 | [ ] |
| `--prompt-width` | 컨트롤 · 명령 입력창 | — | 해당 없음 | [ ] |
| `--modal-background` | 컨트롤 · 모달 | — | 해당 없음 | [ ] |
| `--modal-border-color` | 컨트롤 · 모달 | — | 해당 없음 | [ ] |
| `--modal-border-width` | 컨트롤 · 모달 | — | 해당 없음 | [ ] |
| `--modal-community-sidebar-width` | 컨트롤 · 모달 | — | 해당 없음 | [ ] |
| `--modal-height` | 컨트롤 · 모달 | — | 해당 없음 | [ ] |
| `--modal-max-height` | 컨트롤 · 모달 | — | 해당 없음 | [ ] |
| `--modal-max-width` | 컨트롤 · 모달 | — | 해당 없음 | [ ] |
| `--modal-max-width-narrow` | 컨트롤 · 모달 | — | 해당 없음 | [ ] |
| `--modal-radius` | 컨트롤 · 모달 | — | 해당 없음 | [ ] |
| `--modal-width` | 컨트롤 · 모달 | — | 해당 없음 | [ ] |
| `--button-radius` | 컨트롤 · 버튼 | — | 해당 없음 | [ ] |
| `--nav-collapse-icon-color` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-collapse-icon-color-collapsed` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-heading-color` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-heading-color-colapsed-hover` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-heading-color-collapsed` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-heading-color-hover` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-heading-weight` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-heading-weight-hover` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-indentation-guide-color` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-indentation-guide-width` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-item-background-active` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-item-background-hover` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-item-background-selected` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-item-children-margin-start` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-item-children-padding-start` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-item-color` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-item-color-active` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-item-color-highlighted` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-item-color-hover` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-item-color-selected` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-item-padding` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-item-parent-padding` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-item-size` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-item-weight` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-item-weight-active` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-item-weight-hover` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--nav-item-white-space` | 컨트롤 · 탐색 | — | 해당 없음 | [ ] |
| `--tab-background-active` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-container-background` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-curve` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-divider-color` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-font-size` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-font-weight` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-max-width` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-outline-color` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-outline-width` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-radius` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-radius-active` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-stacked-font-size` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-stacked-font-weight` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-stacked-header-width` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-stacked-pane-width` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-stacked-shadow` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-stacked-text-align` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-stacked-text-transform` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-stacked-text-writing-mode` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-text-color` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-text-color-active` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-text-color-focused` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-text-color-focused-active` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-text-color-focused-active-current` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-text-color-focused-highlighted` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--tab-width` | 컨트롤 · 탭 | — | 해당 없음 | [ ] |
| `--input-border-width` | 컨트롤 · 텍스트 입력 | — | 해당 없음 | [ ] |
| `--input-font-weight` | 컨트롤 · 텍스트 입력 | — | 해당 없음 | [ ] |
| `--input-height` | 컨트롤 · 텍스트 입력 | — | 해당 없음 | [ ] |
| `--input-radius` | 컨트롤 · 텍스트 입력 | — | 해당 없음 | [ ] |
| `--popover-font-size` | 컨트롤 · 팝업 | — | 해당 없음 | [ ] |
| `--popover-height` | 컨트롤 · 팝업 | — | 해당 없음 | [ ] |
| `--popover-max-height` | 컨트롤 · 팝업 | — | 해당 없음 | [ ] |
| `--popover-pdf-height` | 컨트롤 · 팝업 | — | 해당 없음 | [ ] |
| `--popover-pdf-width` | 컨트롤 · 팝업 | — | 해당 없음 | [ ] |
| `--popover-width` | 컨트롤 · 팝업 | — | 해당 없음 | [ ] |
| `--canvas-background` | 코어 기능 · Canvas | — | 해당 없음 | [ ] |
| `--canvas-card-label-color` | 코어 기능 · Canvas | — | 해당 없음 | [ ] |
| `--canvas-color-1` | 코어 기능 · Canvas | — | 해당 없음 | [ ] |
| `--canvas-color-2` | 코어 기능 · Canvas | — | 해당 없음 | [ ] |
| `--canvas-color-3` | 코어 기능 · Canvas | — | 해당 없음 | [ ] |
| `--canvas-color-4` | 코어 기능 · Canvas | — | 해당 없음 | [ ] |
| `--canvas-color-5` | 코어 기능 · Canvas | — | 해당 없음 | [ ] |
| `--canvas-color-6` | 코어 기능 · Canvas | — | 해당 없음 | [ ] |
| `--canvas-dot-pattern` | 코어 기능 · Canvas | — | 해당 없음 | [ ] |
| `--sync-avatar-color-1` | 코어 기능 · Sync | — | 해당 없음 | [ ] |
| `--sync-avatar-color-2` | 코어 기능 · Sync | — | 해당 없음 | [ ] |
| `--sync-avatar-color-3` | 코어 기능 · Sync | — | 해당 없음 | [ ] |
| `--sync-avatar-color-4` | 코어 기능 · Sync | — | 해당 없음 | [ ] |
| `--sync-avatar-color-5` | 코어 기능 · Sync | — | 해당 없음 | [ ] |
| `--sync-avatar-color-6` | 코어 기능 · Sync | — | 해당 없음 | [ ] |
| `--sync-avatar-color-7` | 코어 기능 · Sync | — | 해당 없음 | [ ] |
| `--sync-avatar-color-8` | 코어 기능 · Sync | — | 해당 없음 | [ ] |
| `--sync-avatar-color-current-user` | 코어 기능 · Sync | — | 해당 없음 | [ ] |
| `--search-clear-button-color` | 코어 기능 · 검색 | — | 해당 없음 | [ ] |
| `--search-clear-button-size` | 코어 기능 · 검색 | — | 해당 없음 | [ ] |
| `--search-icon-color` | 코어 기능 · 검색 | — | 해당 없음 | [ ] |
| `--search-icon-size` | 코어 기능 · 검색 | — | 해당 없음 | [ ] |
| `--search-result-background` | 코어 기능 · 검색 | — | 해당 없음 | [ ] |
| `--graph-controls-width` | 코어 기능 · 그래프 | — | 해당 없음 | [ ] |
| `--graph-line` | 코어 기능 · 그래프 | — | 해당 없음 | [ ] |
| `--graph-node` | 코어 기능 · 그래프 | — | 해당 없음 | [ ] |
| `--graph-node-attachment` | 코어 기능 · 그래프 | — | 해당 없음 | [ ] |
| `--graph-node-focused` | 코어 기능 · 그래프 | — | 해당 없음 | [ ] |
| `--graph-node-tag` | 코어 기능 · 그래프 | — | 해당 없음 | [ ] |
| `--graph-node-unresolved` | 코어 기능 · 그래프 | — | 해당 없음 | [ ] |
| `--graph-text` | 코어 기능 · 그래프 | — | 해당 없음 | [ ] |
| `--vault-profile-actions-display` | 코어 기능 · 파일 탐색기 | — | 해당 없음 | [ ] |
| `--vault-profile-color` | 코어 기능 · 파일 탐색기 | — | 해당 없음 | [ ] |
| `--vault-profile-color-hover` | 코어 기능 · 파일 탐색기 | — | 해당 없음 | [ ] |
| `--vault-profile-display` | 코어 기능 · 파일 탐색기 | — | 해당 없음 | [ ] |
| `--vault-profile-font-size` | 코어 기능 · 파일 탐색기 | — | 해당 없음 | [ ] |
| `--vault-profile-font-weight` | 코어 기능 · 파일 탐색기 | — | 해당 없음 | [ ] |
| `--bases-cards-background` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-cards-border-width` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-cards-container-background` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-cards-cover-background` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-cards-group-padding` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-cards-line-height` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-cards-scale` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-cards-shadow` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-cards-shadow-hover` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-embed-border-color` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-embed-border-radius` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-embed-border-width` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-filter-menu-width` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-header-border-width` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-header-height` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-header-padding-end` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-header-padding-start` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-border-color` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-cell-background-active` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-cell-background-disabled` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-cell-radius-active` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-cell-shadow-active` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-column-border-width` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-column-max-width` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-column-min-width` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-container-border-radius` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-container-border-width` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-header-background` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-header-background-hover` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-header-color` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-header-icon-display` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-header-sort-mask` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-header-weight` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-row-background-hover` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-row-border-width` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-row-height` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-table-text-size` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-toolbar-badge-display` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--bases-toolbar-label-display` | 편집·문서 · Bases | — | 해당 없음 | [ ] |
| `--embed-block-shadow-hover` | 편집·문서 · Block | — | 해당 없음 | [ ] |
| `--inline-title-color` | 편집·문서 · 노트 제목 | — | 해당 없음 | [ ] |
| `--inline-title-font` | 편집·문서 · 노트 제목 | — | 해당 없음 | [ ] |
| `--inline-title-line-height` | 편집·문서 · 노트 제목 | — | 해당 없음 | [ ] |
| `--inline-title-size` | 편집·문서 · 노트 제목 | — | 해당 없음 | [ ] |
| `--inline-title-style` | 편집·문서 · 노트 제목 | — | 해당 없음 | [ ] |
| `--inline-title-variant` | 편집·문서 · 노트 제목 | — | 해당 없음 | [ ] |
| `--inline-title-weight` | 편집·문서 · 노트 제목 | — | 해당 없음 | [ ] |
| `--link-unresolved-color` | 편집·문서 · 링크 | — | 해당 없음 | [ ] |
| `--link-unresolved-decoration-color` | 편집·문서 · 링크 | — | 해당 없음 | [ ] |
| `--link-unresolved-decoration-style` | 편집·문서 · 링크 | — | 해당 없음 | [ ] |
| `--link-unresolved-filter` | 편집·문서 · 링크 | — | 해당 없음 | [ ] |
| `--link-unresolved-opacity` | 편집·문서 · 링크 | — | 해당 없음 | [ ] |
| `--list-indent-editing` | 편집·문서 · 목록 | — | 해당 없음 | [ ] |
| `--list-indent-source` | 편집·문서 · 목록 | — | 해당 없음 | [ ] |
| `--file-folding-offset` | 편집·문서 · 문서 | — | 해당 없음 | [ ] |
| `--file-header-border` | 편집·문서 · 문서 | — | 해당 없음 | [ ] |
| `--file-header-font-size` | 편집·문서 · 문서 | — | 해당 없음 | [ ] |
| `--file-header-font-weight` | 편집·문서 · 문서 | — | 해당 없음 | [ ] |
| `--file-header-justify` | 편집·문서 · 문서 | — | 해당 없음 | [ ] |
| `--metadata-background` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-border-color` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-border-radius` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-border-width` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-display-editing` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-display-reading` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-divider-color` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-divider-color-focus` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-divider-color-hover` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-divider-width` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-gap` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-input-background` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-input-background-active` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-input-background-hover` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-input-font-size` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-input-height` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-input-text-color` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-label-background-active` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-label-background-hover` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-label-font-size` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-label-font-weight` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-label-text-color` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-label-text-color-hover` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-label-width` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-max-width` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-padding` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-property-background` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-property-background-active` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-property-background-hover` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-property-padding` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-property-radius` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-property-radius-focus` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-property-radius-hover` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-sidebar-input-font-size` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--metadata-sidebar-label-font-size` | 편집·문서 · 속성 | — | 해당 없음 | [ ] |
| `--embed-background` | 편집·문서 · 임베드 | — | 해당 없음 | [ ] |
| `--embed-border-bottom` | 편집·문서 · 임베드 | — | 해당 없음 | [ ] |
| `--embed-border-end` | 편집·문서 · 임베드 | — | 해당 없음 | [ ] |
| `--embed-border-start` | 편집·문서 · 임베드 | — | 해당 없음 | [ ] |
| `--embed-border-top` | 편집·문서 · 임베드 | — | 해당 없음 | [ ] |
| `--embed-canvas-max-height` | 편집·문서 · 임베드 | — | 해당 없음 | [ ] |
| `--embed-font-style` | 편집·문서 · 임베드 | — | 해당 없음 | [ ] |
| `--embed-max-height` | 편집·문서 · 임베드 | — | 해당 없음 | [ ] |
| `--embed-padding` | 편집·문서 · 임베드 | — | 해당 없음 | [ ] |
| `--heading-formatting` | 편집·문서 · 제목 | — | 해당 없음 | [ ] |
| `--tag-background` | 편집·문서 · 태그 | — | 해당 없음 | [ ] |
| `--tag-background-hover` | 편집·문서 · 태그 | — | 해당 없음 | [ ] |
| `--tag-border-color` | 편집·문서 · 태그 | — | 해당 없음 | [ ] |
| `--tag-border-color-hover` | 편집·문서 · 태그 | — | 해당 없음 | [ ] |
| `--tag-border-width` | 편집·문서 · 태그 | — | 해당 없음 | [ ] |
| `--tag-color` | 편집·문서 · 태그 | — | 해당 없음 | [ ] |
| `--tag-color-hover` | 편집·문서 · 태그 | — | 해당 없음 | [ ] |
| `--tag-decoration` | 편집·문서 · 태그 | — | 해당 없음 | [ ] |
| `--tag-decoration-hover` | 편집·문서 · 태그 | — | 해당 없음 | [ ] |
| `--tag-padding-x` | 편집·문서 · 태그 | — | 해당 없음 | [ ] |
| `--tag-padding-y` | 편집·문서 · 태그 | — | 해당 없음 | [ ] |
| `--tag-radius` | 편집·문서 · 태그 | — | 해당 없음 | [ ] |
| `--tag-size` | 편집·문서 · 태그 | — | 해당 없음 | [ ] |
| `--tag-weight` | 편집·문서 · 태그 | — | 해당 없음 | [ ] |
| `--table-add-button-background` | 편집·문서 · 표 | — | 해당 없음 | [ ] |
| `--table-add-button-border-color` | 편집·문서 · 표 | — | 해당 없음 | [ ] |
| `--table-add-button-border-width` | 편집·문서 · 표 | — | 해당 없음 | [ ] |
| `--table-drag-handle-background` | 편집·문서 · 표 | — | 해당 없음 | [ ] |
| `--table-drag-handle-background-active` | 편집·문서 · 표 | — | 해당 없음 | [ ] |
| `--table-drag-handle-color` | 편집·문서 · 표 | — | 해당 없음 | [ ] |
| `--table-drag-handle-color-active` | 편집·문서 · 표 | — | 해당 없음 | [ ] |
| `--table-selection` | 편집·문서 · 표 | — | 해당 없음 | [ ] |
| `--table-selection-blend-mode` | 편집·문서 · 표 | — | 해당 없음 | [ ] |
| `--table-selection-border-color` | 편집·문서 · 표 | — | 해당 없음 | [ ] |
| `--table-selection-border-radius` | 편집·문서 · 표 | — | 해당 없음 | [ ] |
| `--table-selection-border-width` | 편집·문서 · 표 | — | 해당 없음 | [ ] |

<a id="css-4"></a>
### Publish 별도

| CSS 변수 | 적용 영역 | 연결 ID / 원본 | Typora 대응 선언 | Publish 생성 |
|---|---|---|---|---|
| `--component-title-color` | 게시 · Site components | — | 해당 없음 | [ ] |
| `--component-title-font` | 게시 · Site components | — | 해당 없음 | [ ] |
| `--component-title-size` | 게시 · Site components | — | 해당 없음 | [ ] |
| `--component-title-style` | 게시 · Site components | — | 해당 없음 | [ ] |
| `--component-title-transform` | 게시 · Site components | — | 해당 없음 | [ ] |
| `--component-title-variant` | 게시 · Site components | — | 해당 없음 | [ ] |
| `--component-title-weight` | 게시 · Site components | — | 해당 없음 | [ ] |
| `--graph-height` | 게시 · Site components | — | 해당 없음 | [ ] |
| `--outline-heading-color` | 게시 · Site components | — | 해당 없음 | [ ] |
| `--outline-heading-color-active` | 게시 · Site components | — | 해당 없음 | [ ] |
| `--outline-heading-color-hover` | 게시 · Site components | — | 해당 없음 | [ ] |
| `--outline-heading-weight-active` | 게시 · Site components | — | 해당 없음 | [ ] |
| `--font-interface-theme` | 게시 · Site fonts | — | 해당 없음 | [ ] |
| `--font-monospace-theme` | 게시 · Site fonts | — | 해당 없음 | [ ] |
| `--font-text-size` | 게시 · Site fonts | — | 해당 없음 | [ ] |
| `--font-text-theme` | 게시 · Site fonts | — | 해당 없음 | [ ] |
| `--page-title-font` | 게시 · Site fonts | — | 해당 없음 | [ ] |
| `--footer-display` | 게시 · Site footer | — | 해당 없음 | [ ] |
| `--header-height` | 게시 · Site header | — | 해당 없음 | [ ] |
| `--logo-height` | 게시 · Site header | — | 해당 없음 | [ ] |
| `--logo-max-height` | 게시 · Site header | — | 해당 없음 | [ ] |
| `--logo-max-width` | 게시 · Site header | — | 해당 없음 | [ ] |
| `--logo-radius` | 게시 · Site header | — | 해당 없음 | [ ] |
| `--logo-width` | 게시 · Site header | — | 해당 없음 | [ ] |
| `--site-menu-icon-color` | 게시 · Site header | — | 해당 없음 | [ ] |
| `--site-menu-icon-color-hover` | 게시 · Site header | — | 해당 없음 | [ ] |
| `--site-menu-icon-size` | 게시 · Site header | — | 해당 없음 | [ ] |
| `--site-name-color` | 게시 · Site header | — | 해당 없음 | [ ] |
| `--site-name-color-hover` | 게시 · Site header | — | 해당 없음 | [ ] |
| `--site-name-font` | 게시 · Site header | — | 해당 없음 | [ ] |
| `--site-name-size` | 게시 · Site header | — | 해당 없음 | [ ] |
| `--site-name-weight` | 게시 · Site header | — | 해당 없음 | [ ] |
| `--nav-collapse-icon-color` | 게시 · Site navigation | — | 해당 없음 | [ ] |
| `--nav-collapse-icon-color-hover` | 게시 · Site navigation | — | 해당 없음 | [ ] |
| `--nav-item-border-color` | 게시 · Site navigation | — | 해당 없음 | [ ] |
| `--nav-item-border-color-active` | 게시 · Site navigation | — | 해당 없음 | [ ] |
| `--nav-item-border-color-hover` | 게시 · Site navigation | — | 해당 없음 | [ ] |
| `--nav-item-color` | 게시 · Site navigation | — | 해당 없음 | [ ] |
| `--nav-item-color-active` | 게시 · Site navigation | — | 해당 없음 | [ ] |
| `--nav-item-color-hover` | 게시 · Site navigation | — | 해당 없음 | [ ] |
| `--nav-item-weight-active` | 게시 · Site navigation | — | 해당 없음 | [ ] |
| `--nav-parent-item-color` | 게시 · Site navigation | — | 해당 없음 | [ ] |
| `--nav-parent-item-color-active` | 게시 · Site navigation | — | 해당 없음 | [ ] |
| `--nav-parent-item-weight` | 게시 · Site navigation | — | 해당 없음 | [ ] |
| `--page-padding` | 게시 · Site pages | — | 해당 없음 | [ ] |
| `--page-title-color` | 게시 · Site pages | — | 해당 없음 | [ ] |
| `--page-title-line-height` | 게시 · Site pages | — | 해당 없음 | [ ] |
| `--page-title-size` | 게시 · Site pages | — | 해당 없음 | [ ] |
| `--page-title-style` | 게시 · Site pages | — | 해당 없음 | [ ] |
| `--page-title-variant` | 게시 · Site pages | — | 해당 없음 | [ ] |
| `--page-title-weight` | 게시 · Site pages | — | 해당 없음 | [ ] |
| `--page-width` | 게시 · Site pages | — | 해당 없음 | [ ] |
| `--sidebar-left-background` | 게시 · Site sidebars | — | 해당 없음 | [ ] |
| `--sidebar-left-border-color` | 게시 · Site sidebars | — | 해당 없음 | [ ] |
| `--sidebar-left-border-width` | 게시 · Site sidebars | — | 해당 없음 | [ ] |
| `--sidebar-left-width` | 게시 · Site sidebars | — | 해당 없음 | [ ] |
| `--sidebar-right-background` | 게시 · Site sidebars | — | 해당 없음 | [ ] |
| `--sidebar-right-border-color` | 게시 · Site sidebars | — | 해당 없음 | [ ] |
| `--sidebar-right-border-width` | 게시 · Site sidebars | — | 해당 없음 | [ ] |
| `--sidebar-right-width` | 게시 · Site sidebars | — | 해당 없음 | [ ] |

## 검증 기록

2026-11-23 — 최소 테마 화면 확인 요청(읽기 뷰·라이브 프리뷰 × 라이트·다크)에 대해 사용자가 “모두 잘보여”라고 보고했다. 전체 가독성에 대한 확인이며, OS·앱 버전, 항목별 대비 수치와 언어별 토큰 결과는 기록되지 않았다. 위 표의 개별 항목이나 Publish를 이 보고만으로 통과 처리하지 않는다.
