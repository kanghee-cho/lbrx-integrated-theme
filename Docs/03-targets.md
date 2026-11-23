# 앱별 적용 계약

모드 이름과 내부 식별자는 `LBRX Light` / `LBRX Dark`를 기준으로 유지한다. 아래는 생성기가 제공해야 할 결과와 사용자가 수행할 작업이다. 테마를 바꿀 때 앱의 설정 파일 전체를 덮어쓰지 않는다.

| 대상 | 생성물과 수동 적용 | 서체·간격 경계 |
|---|---|---|
| Typora | 모드별 `lbrx-light.css`, `lbrx-dark.css`를 테마 폴더에 복사하고 재시작 후 테마 메뉴에서 선택. [적용 방법](07-typora.md) | 본문·코드 서체를 분리한다. 편집 UI 전체가 아닌 문서 영역을 기준으로 한다. |
| Obsidian | `manifest.json`과 `theme.css`가 든 `LBRX` 폴더를 보관함의 `.obsidian\themes`에 복사한 뒤 외형에서 활성화. CSS가 `.theme-light`, `.theme-dark`를 구분 | 본문과 고정폭 CSS 변수를 매핑하고 읽기 뷰·라이브 프리뷰를 각각 확인한다. 사용자 서체 설정과 충돌하는 강제 규칙은 피한다. |
| VS Code | 두 색 테마를 선언한 `.vsix`를 **Install from VSIX**로 설치하고 Color Theme에서 선택 | 색 테마는 글꼴을 설정하지 않는다. 편집기 글꼴·크기·줄 높이는 별도 `settings.json` 조각을 사용자가 병합한다. |
| Helix | 두 `.toml`을 Helix 설정 디렉터리의 `themes`에 복사하고 `:theme lbrx_dark` 또는 `:theme lbrx_light`로 확인. 지속 설정은 `config.toml`의 `theme` | 글꼴·크기·간격은 호스트 터미널 소유. 구문 scope와 `ui.*`만 변환한다. |
| Vim | `lbrx_dark.vim`, `lbrx_light.vim`을 사용자 runtimepath의 `colors`에 복사하고 `:colorscheme lbrx_dark` 등으로 선택. 지속 설정은 vimrc | 터미널 Vim을 기준으로 하며 글꼴은 상속한다. `termguicolors`용 RGB와 저색상 터미널용 `cterm` 근사값을 분리한다. |
| Windows Terminal | 두 배색표를 기존 `settings.json`의 `schemes`에 이름 기준으로 병합하고 프로필의 `colorScheme`에서 선택. [적용 방법](06-windows-terminal.md) | 배색표와 프로필의 `font` 설정 조각을 분리한다. 글꼴 크기는 pt로 변환(기준 px × 72/96). 현재 셀 높이는 출력하지 않는다. |
| iTerm2 | [라이트·다크 `.itermcolors` 프리셋](20-iterm2.md)을 기존 프로필에 가져와 선택한다. | 글꼴과 프로필 설정은 수동이다. |
| macOS Terminal.app | **미구현.** [.terminal 프로필 설계](14-macos-terminal.md)를 참고한다. iTerm2 프리셋과 호환되지 않는다. | 글꼴·크기·행 간격은 프로필의 텍스트 설정 범위다. |

**검증 기준**

[iTerm2](20-iterm2.md)는 macOS Terminal.app과 별도다. 프로파일 전체가 아니라 라이트·다크 `.itermcolors`를 생성하며, [프리셋 옵션](21-iterm2-options.md)과 프로파일의 수동 설정을 분리한다. VS Code의 상세 계약은 [적용 가이드](18-vscode.md)와 [옵션 대장](19-vscode-options.md)을 참고한다.

[전체 앱 공통 기준](11-cross-app-options.md)에서 토큰별 적용 범위를 비교한다. Typora와 Obsidian은 [문서 앱 공유 ID](11-cross-app-options.md#shared)를 사용하되 읽기 뷰와 라이브 프리뷰의 검증 결과는 따로 기록한다.

1. `check`로 두 모드의 ANSI 슬롯, 역할과 대비를 검증한다. 선택 빌드에서 미선택 앱의 기존 출력이 그대로인지 확인한다.
2. 형식 검사·실제 앱 설치·실제 화면을 구분해 [검증 버전 표](05-languages-and-dependencies.md)에 기록한다. VSIX 설치 성공이 언어별 강조 표시까지 검증했다는 뜻은 아니다.
3. 원본 수정 → TUI `R` → CLI 재생성을 확인한다. 실제 앱에서는 라이트·다크의 한글 문단, 구문색, 코드 블록, 선택과 폰트 높이를 따로 확인한다.

**배포 경계:** 가져오기·재적용·복구는 앱별 가이드를 따른다. 앱 간 실시간 동기화나 사용자 설정 자동 수정은 하지 않는다.
