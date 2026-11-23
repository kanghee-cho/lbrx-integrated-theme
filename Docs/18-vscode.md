# VS Code에 LBRX 적용하기

**라이트·다크 테마 생성 구현.** 색상은 [공통 TOML](../themes/lbrx.toml)에서 직접 수정한다.

```powershell
cargo run --locked -- build --app vscode
```

`dist\lbrx\vscode`에 다음 파일이 생성된다.

| 파일 | 용도 |
|---|---|
| `lbrx-theme.vsix` | `LBRX Light`와 `LBRX Dark`를 함께 등록하는 색 테마 확장. 실행 코드, 글꼴, 아이콘은 포함하지 않는다. |
| `settings-font.json` | 편집기·통합 터미널 글꼴 설정 조각. 편집기 줄 높이는 픽셀 단위로 변환한다. |
| `lbrx-code.css` | HTML 코드 블록용 [동반 CSS](22-vscode-language-highlighting.md). VS Code에 자동 주입되지 않는다. |

1. 기존 색 테마 이름을 기록하고, 명령 팔레트에서 **Extensions: Install from VSIX…**로 VSIX를 설치한다. **Preferences: Color Theme**에서 `LBRX Light` 또는 `LBRX Dark`를 고른다.
2. 글꼴도 바꾸려면 필요한 글꼴을 먼저 설치하고, `settings-font.json`에서 원하는 항목만 기존 사용자 설정에 병합한다. 색 테마를 선택하는 것만으로 글꼴이나 합자 설정은 바뀌지 않는다.
3. 색을 수정했으면 다시 빌드한 VSIX를 설치한다. 원복하려면 기존 테마로 돌아가고 추가한 글꼴 설정만 복원한다.

**검증 순서:** 사용자 색상 재정의가 없는 시험 프로필에서 두 모드를 각각 선택한다. 일반 편집기·사이드바·입력창·선택·키보드 포커스·통합 터미널 ANSI를 비교한다. 같은 소스 파일에 **Developer: Inspect Editor Tokens and Scopes**를 실행하여 TextMate 규칙인지 언어 서비스가 제공한 semantic token인지 기록한다. 사용자 설정의 `workbench.colorCustomizations` 등은 별도 조건으로 본다.

두 테마의 `uiTheme`은 각각 `vs`, `vs-dark`이며 확장 manifest의 요구 버전은 `^1.147.0`이다. 생성된 색과 검증 상태는 [옵션 대장](19-vscode-options.md)에서 관리한다. [공식 색 테마 안내](https://code.visualstudio.com/api/extension-guides/color-theme) · [VSIX 설치](https://code.visualstudio.com/docs/configure/extensions/extension-marketplace#_install-from-a-vsix)
