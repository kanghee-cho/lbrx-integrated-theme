# Obsidian에 LBRX 적용하기

**상태: 최소 테마 생성 구현, 전체 가독성 사용자 확인.** 저장소 루트에서 `cargo run --locked -- build`를 실행한다. 세부 매핑과 확인 범위는 [Obsidian 옵션 대장](10-obsidian-options.md#검증-기록)에서 추적한다.

## 패키지와 수동 적용

Typora는 모드별 CSS 두 개를 선택하지만, Obsidian에는 `LBRX` 테마 하나를 설치한다. 한 `theme.css` 안에서 `.theme-light`와 `.theme-dark`를 분리하고, 밝기 모드 선택은 앱에 맡긴다.

| 산출물 | 역할 |
|---|---|
| `dist\lbrx\obsidian\LBRX\manifest.json` | 이름 `LBRX`, 생성기 버전, 제작자, 최소 앱 버전. `1.10.6`은 공식 샘플 manifest의 기준이며 실측한 최소 버전은 아니다. |
| `dist\lbrx\obsidian\LBRX\theme.css` | 공통 토큰을 Obsidian의 색·서체 변수에 매핑한다. 스니펫이나 커뮤니티 플러그인 설치를 전제로 하지 않는다. |

1. **시험용 보관함**에서 먼저 확인한다. 기본 설정 폴더를 쓰는 경우 `<보관함>\.obsidian\themes\LBRX\`에 두 파일이 바로 들어가도록 `LBRX` 폴더를 복사한다. 설정 폴더 이름을 바꿨다면 그 위치를 사용한다.
2. **설정 → 테마/외형(Appearance) → 테마(Themes)**에서 `LBRX`를 선택한다. 목록에 없으면 `LBRX\LBRX`로 이중 복사하지 않았는지, 폴더명과 manifest 이름이 일치하는지 확인하고 앱을 다시 연다.
3. 같은 보관함에서 기본 색상 모드를 라이트와 다크로 바꾼다. 업데이트할 때는 LBRX 패키지 파일만 교체한다. 원복은 기존 테마 또는 기본 테마를 다시 선택하면 된다. 보관함의 설정 JSON, 노트, 스니펫은 덮어쓰지 않는다.

## 무엇을 공유하고 무엇을 분리하는가

| 공통 원본 | Obsidian에서 의도하는 결과 |
|---|---|
| `background`, `surface`, `text`, `muted` | `--background-primary`, `--background-secondary`, `--text-normal`, `--text-muted`에 대응한다. 본문 외에 같은 변수를 쓰는 사이드바·설정 창의 실제 배경과 글자 조합도 검사한다. |
| `link`, `focus`, 선택색 | 내부·외부·미생성 링크와 키보드 포커스는 의미를 구분한다. DOM 텍스트 선택과 편집기가 그리는 선택 영역을 각각 확인한다. |
| `typography.prose`, `typography.mono` | 본문은 LINE Seed EN → KR → JP → `sans-serif`, 코드는 D2Coding ligature → `monospace`. 테마 기본 글꼴로 제공하고 사용자의 본문·고정폭 글꼴과 크기 설정을 강제로 덮지 않는다. UI 아이콘과 수식 글꼴은 대상이 아니다. |

읽기 뷰는 렌더링된 문서를, 라이브 프리뷰는 커서 위치에 따라 Markdown 기호를 보여주는 편집기를 사용한다. **같은 옵션 ID는 같은 디자인 의도이지 같은 CSS 선택자를 뜻하지 않는다.** 먼저 공식 CSS 변수를 쓰고, 부족한 부분만 뷰에 한정한다. Typora의 `#write`나 코드 펜스 선택자를 복사하지 않는다. 소스 모드, 모바일, PDF는 별도 검증 대상이다.

## 첫 적용 확인 순서

1. CSS 스니펫과 커뮤니티 플러그인을 끈 시험용 보관함에서 기존 테마·모드·글꼴 설정을 기록한다. 노트 하나에 `Hello, 한글, 日本語`, 제목, `[[기존 노트]]`, `[[없는 노트]]`, 외부 링크, 인용, 선택 가능한 텍스트, 코드 블록, 긴 표, 중첩 작업 목록, 접이식 콜아웃을 넣는다.
2. 같은 노트를 **읽기 뷰 / 라이브 프리뷰 × 라이트 / 다크** 네 조합으로 비교한다. 라이브 프리뷰에서는 커서를 요소 안팎에 놓고 링크 편집, 표 셀 이동, 체크박스 전환이 그대로 되는지도 본다.
3. 기본 본문 대비 7:1, 읽어야 하는 보조 글자와 선택 글자 4.5:1, 포커스와 인접 배경 3:1을 기준으로 검사한다. 사용자 글꼴을 지정한 경우와 스니펫을 다시 켠 경우는 기본 테마 결과와 별도로 기록한다.
4. [검증 기록](10-obsidian-options.md#검증-기록)에 앱 버전·OS·모드·뷰·옵션 ID를 남긴다. 한 뷰에서 성공해도 다른 뷰를 통과 처리하지 않는다.

공식 자료: [테마 제작·폴더 구조](https://docs.obsidian.md/Themes/App+themes/Build+a+theme) · [CSS 변수 레퍼런스](https://docs.obsidian.md/Reference/CSS+variables/CSS+variables) · [CSS 스니펫](https://help.obsidian.md/snippets) · [편집 및 읽기 뷰](https://help.obsidian.md/edit-and-read)
