# Typora에 LBRX 적용

저장소 루트에서 `cargo run --locked -- build`를 실행하면 `dist\lbrx\typora`에 `lbrx-light.css`와 `lbrx-dark.css`가 생성됩니다. 생성기는 Typora 설정을 바꾸지 않습니다.

1. Typora의 **환경설정 → 모양 → 테마 폴더 열기**에서 테마 폴더를 엽니다.
2. 두 CSS 파일을 복사하고 Typora를 다시 시작합니다. **테마** 메뉴에서 **Lbrx Light** 또는 **Lbrx Dark**를 선택합니다.
3. 업데이트할 때는 다시 생성한 같은 이름의 파일만 교체합니다. 이전 테마를 다시 선택하면 LBRX를 적용하기 전 상태로 돌아갈 수 있습니다. 기존 `base.user.css`나 테마별 `.user.css`는 생성된 스타일보다 우선할 수 있습니다.

본문은 LINE Seed EN → KR → JP → 시스템 sans-serif, 코드는 D2Coding ligature → 시스템 monospace 순서로 표시됩니다. 글꼴이 설치되지 않았으면 대체 글꼴이 사용됩니다. 테마는 글꼴을 번들하거나 설치하지 않습니다.

두 모드에서 영어·한글·일본어가 섞인 문장(`Hello, 한글, 日本語の文章`), 제목, 링크, 인용, 선택 영역, 인라인 코드와 코드 블록을 확인합니다. **Lbrx Light**에서는 **환경설정 오른쪽 내용**의 제목·설명·입력 필드가 밝은 배경에서 잘 읽히는지 확인합니다. **Lbrx Dark**에서는 **환경설정 왼쪽 메뉴**의 기본·선택·마우스 오버 상태를 확인합니다. 문서 탐색 사이드바, 기타 앱 메뉴, Markdown 소스 모드는 이 수정의 범위에 포함되지 않습니다.

참고: [Typora 테마 설치](https://support.typora.io/About-Themes/) · [코드 블록 스타일](https://support.typora.io/Code-Block-Styles/)
