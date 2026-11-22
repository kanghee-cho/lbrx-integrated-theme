# iTerm2에 LBRX 적용하기

**설계 단계.** Apple 기본 [Terminal.app](14-macos-terminal.md)의 `.terminal`은 프로파일이고, iTerm2의 `.itermcolors`는 **기존 프로파일에 적용하는 색 프리셋**이다. 서로 가져올 수 있는 파일로 취급하지 않는다. 현재 `build`는 어느 쪽도 생성하지 않는다.

## 구현 후 설치·복구

1. 예정 산출물은 `LBRX Light.itermcolors`와 `LBRX Dark.itermcolors`다. iTerm2 **Settings → Profiles**에서 시험할 프로파일을 복제하거나, 해당 프로파일의 **Colors → Color Presets… → Export…**로 기존 배색을 보관한다.
2. 같은 메뉴의 **Import…**에서 두 파일을 가져온다. 프리셋 이름은 파일명에서 온다. 가져오기만 하지 말고 **Color Presets…**에서 원하는 LBRX 이름을 선택한다.
3. 프로파일이 라이트·다크 색상 분리를 지원하고 사용하려면 **Use separate colors for light and dark mode**를 켠 뒤 **Editing**에서 모드를 고르고 각각 대응하는 프리셋을 적용한다. 분리하지 않으면 해당 프로파일에 한 세트만 적용된다.
4. 새 세션에서 배색을 확인한다. 글꼴은 **Profiles → Text**에서 따로 선택한다. 기존 프로파일·기본 셸·키 바인딩은 교체하지 않는다. 되돌릴 때는 백업 프리셋을 선택하거나 원래 프로파일로 새 세션을 연다.

**검증:** [옵션 대장](21-iterm2-options.md)의 일반·밝은 ANSI 16색을 따로 출력한다. 기본 텍스트, 선택된 글자/배경, 블록 커서 위 글자도 비교한다. Minimum Contrast와 자동 커서색은 표시 결과를 바꿀 수 있으므로 그 상태를 함께 기록한다. plist 파싱 성공만으로 승인하지 말고 Mac에서 두 프리셋의 가져오기 → 선택 → 새 세션 → 재내보내기를 확인한다.

공식 근거: [프로파일 색·프리셋](https://iterm2.com/documentation-preferences-profiles-colors.html) · [프로파일 글꼴](https://iterm2.com/documentation-preferences-profiles-text.html)
