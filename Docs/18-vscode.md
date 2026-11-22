# VS Code에 LBRX 적용하기

**설계 단계.** 예정 산출물은 `LBRX Light`와 `LBRX Dark`를 함께 등록하는 VSIX 하나다. 현재 `build`는 VSIX를 만들지 않는다. [옵션 대장](19-vscode-options.md)은 작업 화면 색, 코드 구문색, 의미 기반 강조, 사용자 설정을 구분한다.

1. 구현 후에는 기존 색 테마 이름을 기록하고, 명령 팔레트에서 **Extensions: Install from VSIX…**로 생성 파일을 설치한다. **Preferences: Color Theme**에서 LBRX 모드를 고른다. 파일 아이콘 테마는 따로 바꿀 필요가 없다.
2. 편집기 글꼴까지 바꾸고 싶다면 별도 설정 조각에서 원하는 항목만 기존 사용자 설정에 병합한다. 테마를 선택하는 것만으로 `editor.fontFamily`나 통합 터미널 글꼴이 바뀌지는 않는다.
3. 업데이트는 새 VSIX를 설치한 뒤 버전과 선택된 테마를 확인한다. 원복은 기존 색 테마를 다시 선택하고, 선택적으로 추가한 글꼴 설정만 복원한다.

**검증 순서:** 사용자 색상 재정의가 없는 시험 프로필에서 두 모드를 각각 선택한다. 일반 편집기·사이드바·입력창·선택·키보드 포커스·통합 터미널 ANSI를 비교한다. 같은 소스 파일에 **Developer: Inspect Editor Tokens and Scopes**를 실행하여 TextMate 규칙인지 언어 서비스가 제공한 semantic token인지 기록한다. 사용자 설정의 `workbench.colorCustomizations` 등은 별도 조건으로 본다.

구현 시 `package.json`의 `contributes.themes`에 두 테마 파일을 등록하고 `uiTheme`은 각각 `vs`, `vs-dark`로 구분한다. `engines.vscode`는 실제 시험한 호환 범위를 근거로 정한다. [공식 색 테마 안내](https://code.visualstudio.com/api/extension-guides/color-theme) · [VSIX 설치](https://code.visualstudio.com/docs/configure/extensions/extension-marketplace#_install-from-a-vsix)
