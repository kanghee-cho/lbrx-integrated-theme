# LBRX Studio 테마 출력과 적용 계약

React/Tauri 기반 Studio의 CSS 변수 계약을 사용한다. 생성기는 Studio 저장소, 앱 설정, 설치된 글꼴을 수정하지 않는다.

## 생성과 적용

```powershell
cargo run --locked -- check
cargo run --locked -- build --app lbrx-studio
```

출력은 `dist\orange-type\lbrx\lbrx-studio\theme.css`와 이 설치 안내다. 입력은 [공통 원본](../themes/orange-type/lbrx.toml), [Studio 전용 설정](../themes/orange-type/editors/lbrx-studio.toml), 공통·언어별 `languages` 설정이다. 생성본의 변수 이름은 Studio의 `src\app\theme.css`와 호환된다.

Studio 쪽에서 기존 `src\app\theme.css`를 백업한 뒤 생성된 CSS로 교체하고 앱 진입점에서 한 번만 import한다. 기존 하드코딩 색상 CSS와 다크 미디어 규칙이 남아 있으면 함께 검토한다. 이 저장소의 빌드가 해당 파일을 자동 교체하지는 않는다. 생성본을 편집하지 말고 TOML을 수정해 다시 생성한다.

## 결정된 경계

| 항목 | 계약 |
|---|---|
| 입력 형식 | `themes\orange-type\editors\lbrx-studio.toml`의 `[colors]`. 값은 공통 `roles` 이름. 알 수 없는 키·역할은 오류 |
| 출력 형식 | 두 모드와 시스템 선택을 담은 단일 `theme.css` + `README.md`. JSON·자동 설치·설정 저장은 제외 |
| 테마 선택 | `<html data-theme="light">`는 라이트, `dark`는 다크를 고정. 속성이 없거나 `system`이면 OS `prefers-color-scheme`을 따름. Studio가 선택값을 적용·저장 |
| 앱 UI | 기존 변수 11종과 `--focus`, `--link` 생성. 컨트롤 모양·레이아웃은 앱 CSS 책임 |
| 테두리 | `--border`의 역할색에 HEX 알파 `40`(64/255)을 추가. 포커스 표시로 사용하지 않고 불투명 `--focus` 사용 |
| 서체 | `--font-prose`, `--font-mono`와 각각 `-size`(px), `-line-height`(배수). 앱 UI가 변수를 소비해야 적용됨. 코드 영역은 생성 CSS가 고정폭 변수를 사용 |
| 선택·포커스·링크 | 변수만 제공. 앱 CSS에서 `::selection`, `:focus-visible`, 링크에 적용. 임의 재매핑의 대비는 실제 화면에서 별도 확인 |
| 구문 강조 | 아래 HTML 토큰 계약. 문법 엔진·Monaco/CodeMirror 테마 API·토큰 분류기는 생성하지 않음 |
| 패널·사용자 설정 | 패널 크기·배치·표시 여부·테마 선택값은 Studio 소유. TOML에서 변경하지 않음 |

UI CSS의 예:

```css
:root {
  font-family: var(--font-prose);
  font-size: var(--font-prose-size);
  line-height: var(--font-prose-line-height);
  color: var(--text);
  background: var(--background);
}
::selection { background: var(--selection-background); color: var(--selection-text); }
:focus-visible { outline: 3px solid var(--focus); outline-offset: 3px; }
a { color: var(--link); }
```

## 공통·언어별 구문 토큰

기존 VS Code 동반 HTML CSS와 같은 클래스 계약을 사용한다. `.lbrx-code`는 코드 영역, `.language-<ID>`는 해당 영역 또는 내부 언어 컨테이너, `.tok-<토큰>`은 렌더러가 분류한 텍스트다. 토큰 키의 점은 하이픈으로 바꾼다. 언어 ID는 TOML 파일 이름 그대로 사용한다(`typescript`, `python`, `common-lisp` 등). `ts`, `py` 같은 별칭은 렌더러에서 정규화한다.

```html
<pre class="lbrx-code language-rust"><code><span class="tok-keyword-declaration">fn</span> <span class="tok-function">main</span>() {}</code></pre>
```

`languages/common.toml`의 공통 색·스타일과 언어별 재정의를 모두 생성한다. 색 우선순위는 **개별 언어 → 공통 → 기본 역할**, 스타일은 개별 언어 → 공통 → normal이다. 일반 코드 배경은 공통 `background`이며 문서용 `code_block_background`와 분리한다. 토큰 목록은 `comment`, `keyword`, `string`, `function`, `number`, `error`, `keyword.control`, `keyword.declaration`, `string.escape`, `constant.builtin`, `function.macro`, `type`, `variable`, `variable.parameter`, `property`, `operator`, `attribute`, `punctuation`이다.

CSS만 불러와서는 원문이 구문 강조되지 않는다. Studio 렌더러가 텍스트를 안전하게 이스케이프하고 토큰 클래스를 부여해야 한다. 중첩 언어 컨테이너는 부모에서 상속한 재정의와 섞일 수 있으므로 독립 `.lbrx-code` 영역을 사용한다. 앱의 현재/향후 코드 편집기에서 다른 API를 사용할 경우 별도 어댑터 계약을 결정해야 한다.

## 검증과 남은 앱 측 결정

생성 테스트는 전용 매핑 격리, 두 모드, 서체 전달, 언어별 색·스타일, 선택 빌드와 오류 보존을 확인한다. 실제 Studio 화면 검증을 대신하지 않는다.

2026-11-25 Windows 통합 브라우저에서 생성 CSS의 계산된 스타일을 확인했다. OS 라이트/다크 × 루트 속성 없음/system/light/dark의 8개 조합에서 배경과 코드 전경을 확인했고, Common Lisp 함수의 언어별 bold, 매개변수 italic, 코드 14px·줄 높이 20.3px를 확인했다. 기존 VS Code 동반 CSS는 공통 렌더러 추출 후에도 바이트 단위로 동일하다. Studio 앱 및 세 OS의 실제 WebView 화면은 미검증이다.

Studio에서는 (1) CSS import 위치와 기존 색상 규칙 제거, (2) 테마 선택값의 루트 속성 연결, (3) 변수의 실제 UI 소비, (4) 구문 분류기·렌더러 선택을 구현해야 한다. Windows WebView2, macOS WKWebView, Linux WebKitGTK에서 시스템/라이트/다크, OS 밝기 변경, 한글·일본어·코드 글꼴, 선택·포커스·오류 상태를 확인한다. 글꼴 파일은 출력에 포함하지 않는다.
