# Typora에 LBRX 적용

저장소 루트에서 `cargo run --locked -- build --app typora`를 실행하면 `dist\orange-type\lbrx\typora`에 `lbrx-light.css`와 `lbrx-dark.css`가 생성된다.

## 설치·복구

1. Typora의 **환경설정 → 모양 → 테마 폴더 열기**에서 테마 폴더를 엽니다.
2. 두 CSS 파일을 복사하고 Typora를 다시 시작합니다. **테마** 메뉴에서 **Lbrx Light** 또는 **Lbrx Dark**를 선택합니다.
3. 업데이트할 때는 다시 생성한 같은 이름의 파일만 교체합니다. 이전 테마를 다시 선택하면 LBRX를 적용하기 전 상태로 돌아갈 수 있습니다. 기존 `base.user.css`나 테마별 `.user.css`는 생성된 스타일보다 우선할 수 있습니다.

본문은 LINE Seed EN → KR → JP → 시스템 sans-serif, 코드는 D2Coding ligature → 시스템 monospace 순서로 표시됩니다. 글꼴이 설치되지 않았으면 대체 글꼴이 사용됩니다. 테마는 글꼴을 번들하거나 설치하지 않습니다.

두 모드에서 영어·한글·일본어가 섞인 문장(`Hello, 한글, 日本語の文章`), 제목, 링크, 인용, 선택 영역, 인라인 코드와 코드 블록을 확인합니다. **Lbrx Light**에서는 **환경설정 오른쪽 내용**의 제목·설명·입력 필드가 밝은 배경에서 잘 읽히는지 확인합니다. **Lbrx Dark**에서는 **환경설정 왼쪽 메뉴**의 기본·선택·마우스 오버 상태를 확인합니다. 이 환경설정 대비 규칙은 문서 탐색 사이드바나 기타 앱 메뉴에는 적용하지 않습니다.

Markdown 소스 모드의 스타일을 바꾸려면 `themes/orange-type/editors/typora.toml`의 `[source_mode]` 또는 `[source_mode.colors]`에서 원하는 예시를 주석 해제한 뒤 다시 빌드합니다. 두 테마에서 소스 모드의 글꼴·구문·드래그 선택을 확인하고, 소스 모드를 끈 뒤 본문과 코드 블록에 변경이 없는지 확인합니다.

문서 안의 `[TOC]`를 꾸미려면 `themes/orange-type/editors/typora.toml`의 `[toc]` 또는 `[toc.indent_px]`에서 원하는 예시를 주석 해제한 뒤 다시 빌드합니다. 목차의 표시 깊이와 번호는 본문 제목이나 사이드바 개요를 변경하지 않습니다.

탐색 사이드바의 바탕색, 개요 들여쓰기·번호, 개요 및 파일 트리의 상태 색은 `themes/orange-type/editors/typora.toml`의 `[sidebar]`와 하위 표에서 변경할 수 있습니다. 두 테마에서 개요의 현재 위치와 파일 트리의 열린 파일·선택 상태를 각각 확인합니다. 파일 목록 보기와 검색 결과에는 행별 색 설정이 적용되지 않습니다.

`[editing_aids.yaml]`, `[editing_aids.search]`, `[editing_aids.link]`에서 YAML 머리말, 문서 안에서 찾은 검색 결과, 편집 중 펼쳐진 인라인 링크의 색을 변경할 수 있습니다. 주석 해제 후 두 테마를 다시 빌드하고, 검색 결과 강조가 일반 드래그 선택과 별도로 적용되는지 확인합니다.

다이어그램 옵션은 `themes/orange-type/editors/typora.toml`의 `[diagrams.mermaid]`, `[diagrams.sequence]`, `[diagrams.flow]`에서 각각 지정합니다. 다이어그램 활성화는 Typora의 설정이며, LBRX는 문서 안의 Mermaid `init` 설정이나 `flow` 구문을 바꾸지 않습니다. 미리보기 정렬은 편집 화면용입니다. [Typora 공식 문서](https://support.typora.io/Diagram-Options/)에 따르면 편집 화면과 다른 테마로 HTML·PDF·이미지를 내보낼 때 일부 Mermaid 옵션이 반영되지 않을 수 있습니다.

## 내보내기

- **인쇄·기본 PDF:** `themes/orange-type/editors/typora.toml`의 `[print]`에서 사용할 예시를 주석 해제한 뒤 다시 빌드합니다. 종이색과 본문·제목 및 링크 색은 인쇄에만 적용됩니다. 다크 테마로 편집하더라도 **환경설정 → 내보내기 → PDF**에서 **Lbrx Light**를 명시적으로 선택하는 것을 권장합니다. 특히 Windows/Linux의 Typora는 PDF와 인쇄에 밝은 테마만 지원합니다.
- **페이지 나눔:** 제목과 다음 블록, 명시적인 이미지 `figure`와 `figcaption`을 함께 배치하도록 각각 지정할 수 있습니다. 한 페이지보다 큰 요소에는 보장되지 않으며 표와 코드 블록의 분할은 강제하지 않습니다. 두 페이지 이상인 문서로 제목, 캡션이 있는 그림, 긴 표·코드를 확인합니다. PDF 프로필에서 **최상위 제목 사이 페이지 나누기**를 켠 경우 앱의 강제 나눔이 우선할 수 있습니다.
- **HTML·이미지:** HTML 프로필에서 내보내기용 테마와 사이드바 개요 포함 여부를, 이미지 프로필에서 테마·폭·글자 크기를 직접 지정합니다. **HTML(스타일 없음)**에는 LBRX CSS가 포함되지 않습니다.
- **PDF 용지·메타데이터:** PDF 프로필에서 용지 크기, 여백, 페이지 머리말·꼬리말을 설정합니다. 문서 제목·저자 등은 YAML 머리말을 사용합니다. 문서별 YAML로 PDF 프로필 설정을 덮어쓰려면 해당 프로필에서 YAML 설정 읽기를 켭니다.

참고: [Typora 내보내기](https://support.typora.io/Export/) · [YAML 머리말](https://support.typora.io/YAML/) · [Typora 테마 설치](https://support.typora.io/About-Themes/) · [코드 블록 스타일](https://support.typora.io/Code-Block-Styles/)
