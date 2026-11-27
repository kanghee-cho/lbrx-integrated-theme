# Google Antigravity에 LBRX 적용

`cargo run --locked -- build --app antigravity`로 각 스타일의 `dist\<style>\lbrx\antigravity`에 VSIX, HTML 코드 CSS, 선택 적용용 글꼴 설정과 이 안내를 생성한다.

## 적용과 복구

1. Google Antigravity IDE의 Extensions 화면에서 **Install from VSIX…**를 실행해 원하는 스타일의 `lbrx-theme.vsix`를 선택한다.
2. **Preferences: Color Theme**에서 `LBRX Light` 또는 `LBRX Dark`를 선택한다.
3. 글꼴 변경이 필요하면 `settings-font.json`의 필드만 앱 사용자 설정에 수동 병합한다. 기존 설정을 먼저 백업한다.
4. 복구하려면 이전 색 테마를 선택하고 추가한 글꼴 필드를 복원하거나 LBRX 확장을 제거한다.

Antigravity를 VS Code 확장으로 사용하는 경우에는 동일 스타일의 `vscode` 출력도 사용할 수 있다. 독립 IDE의 호환성을 위해 전용 VSIX는 VS Code API 엔진 `^1.90.0`을 선언한다. 실행 코드는 없으며 표준 색 테마만 등록한다. 호스트가 지원하지 않는 최신 색 ID는 효과가 없을 수 있다. VSIX 구조·색 매핑은 검사하지만 설치된 Antigravity에서의 UI 및 설치 호환성은 미검증이다. 버전 제한 오류가 나면 앱 버전을 확인하며 강제 설치하거나 앱 파일을 패치하지 않는다.

## 전용 색 설정

`themes\<style>\editors\antigravity.toml`의 `[colors]`에 VS Code 형식의 색 ID와 공통 역할을 연결한다. 지원 목록은 [VS Code 색 ID 대장](19-vscode-options.md)과 같다. `surface@25`처럼 `역할@불투명도(0–100)` 문법도 동일하다. `vscode.toml`의 앱 전용 재정의는 이 출력에 전파하지 않으며, 언어별 토큰과 공통 팔레트는 같은 스타일에서 상속한다.

설치 ID와 테마 이름은 기존 LBRX VSIX와 같으므로 스타일은 대안이며 동시 설치 이름을 추가하지 않는다. `lbrx-code.css`는 HTML 토큰 렌더러용이지 Antigravity UI에 삽입하는 CSS가 아니다. 에이전트의 독자적 웹뷰·관리 화면 색은 호스트가 표준 테마 색을 사용하는 범위에서만 적용된다.

참고: [Google Antigravity 시작 안내](https://antigravity.google/docs/getting-started), [VS Code 색 테마](https://code.visualstudio.com/api/extension-guides/color-theme).
