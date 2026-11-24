# macOS Terminal.app에 LBRX 적용

Apple 기본 터미널용이며 iTerm2나 Windows Terminal 설정 파일이 아니다. 라이트·다크 `.terminal` 프로파일을 생성한다. 실제 Mac에서 가져오기와 표시 검증은 아직 하지 않았다.

```powershell
cargo run --locked -- build --app macos-terminal
```

저장소의 `themes/editors/macos-terminal.toml`에서 원하는 `[colors]` 항목을 주석 해제하고 공통 역할 이름을 지정한다. 기본 네 색과 ANSI 16색은 공통 원본에서 가져오며, `TextBoldColor`는 지정할 때만 생성한다. 글꼴·간격·창·탭은 앱에서 수동 설정한다.

## 설치·복구

1. `dist/lbrx/macos-terminal/`의 `LBRX Light.terminal`, `LBRX Dark.terminal`을 사용한다. 먼저 **터미널 → 설정 → 프로파일**에서 기존 기본 프로파일 이름을 기록한다. 기존 프로파일을 편집할 계획이라면 동작 메뉴에서 **내보내기**로 백업한다.
2. 프로파일 목록의 동작 메뉴 → **가져오기**에서 두 파일을 선택하거나 Finder에서 목록으로 끌어온다. **셸 → 새로운 윈도우**에서 가져온 프로파일로 시험한다. 가져오기와 기본값 변경은 별개다.
3. 마음에 들면 원하는 프로파일을 **기본값**으로 정한다. 시작 시 열리는 창이나 새 탭이 다른 프로파일을 쓰면 **설정 → 일반**의 시작/새 창·탭 설정을 확인한다. 기존 창을 일괄 변경하지 않는다.
4. 갱신은 새 버전을 가져와 비교한 뒤 구버전만 정리한다. 같은 이름을 다시 가져와도 덮어쓰기를 가정하지 않는다. 복구하려면 원래 프로파일을 기본으로 되돌리고 새 창을 연다.

**글꼴:** 프로파일 → **텍스트**에서 설치된 `D2Coding ligature`를 선택한다. 공통 기본 14 CSS px에 대응하는 크기 출발점은 10.5 pt다. Terminal.app의 행·문자 간격과 CSS `line-height`는 같은 단위가 아니므로 실제 혼합 문장 `한글 日本語 0O1l`로 맞춘다.

**승인 기준:** `plutil -lint`는 컨테이너 문법만 확인한다. 실제 Mac에서 두 파일을 가져오고 새 창에서 기본 글자·커서·선택·ANSI 일반/밝은 16색을 확인한다. 예: `printf '\033[31mred\033[0m / \033[91mbright red\033[0m\n'`. 다시 내보내 색상 데이터가 유지되는지도 확인한다. macOS·Terminal.app 버전 및 모드별 결과는 대장에 기록한다.

Apple 공식: [프로파일 개요](https://support.apple.com/guide/terminal/trml107/mac) · [가져오기·내보내기](https://support.apple.com/guide/terminal/trml4299c696/mac) · [텍스트 설정](https://support.apple.com/guide/terminal/trmltxt/mac)
