# Antigravity CLI (agy)에 LBRX 적용

`cargo run --locked -- build --app agy-cli`로 각 스타일의 `dist\<style>\lbrx\agy-cli`에 `settings-theme.json`, `schemes.json`, `profile-font.json`과 이 안내를 생성한다.

## 적용

1. Windows Terminal에서 `schemes.json`의 LBRX Light/Dark를 기존 최상위 `schemes`에 수동 추가하고 사용할 프로필의 색 구성표를 선택한다. 다른 스타일과 이름이 같으므로 동명 배색표는 백업 후 교체한다.
2. agy에서 `/config` 또는 `/settings`를 열고 **Color Scheme**을 **terminal**로 선택한다. 이 선택은 앱이 저장한다. 또는 설치된 agy 1.5.18의 `--settings` 플래그로 아래처럼 **해당 세션에만** 적용한다.

```powershell
agy --settings .\dist\lavender-type\lbrx\agy-cli\settings-theme.json
```

이 JSON은 `{"colorScheme":"terminal"}`만 포함한다. 권한·sandbox·모델·인증·렌더링 모드는 변경하지 않는다. 명령 예시는 사용자가 직접 실행하며 생성기는 agy 세션을 시작하지 않는다.

## 지원 범위

[공식 설정 스키마](https://antigravity.google/schemas/v1/settings.json)는 `colorScheme`에 terminal 및 기본 preset을 지원한다. 임의 HEX 사용자 정의 테마는 공식 스키마에서 확인되지 않아 가짜 테마 키나 설치 JSON을 만들지 않는다. terminal preset이 상속하는 기본 전경·배경·ANSI 색은 터미널 앱에서 설정한다. 앱이 직접 지정하는 코드·상태·diff 색이 모두 팔레트를 따르는 것은 아니다.

`themes\<style>\editors\agy-cli.toml`에서 `cursorColor`와 `selectionBackground`를 공통 역할로 재정의할 수 있다. agy 출력의 Windows Terminal 배색표에만 적용하며 `windows-terminal.toml` 및 다른 앱의 전용 설정과 분리한다. `profile-font.json`은 선택 적용용 글꼴 조각이다. 전체 사용자 설정 파일을 덮어쓰지 않는다.

macOS Terminal·iTerm2·Antigravity/VS Code 통합 터미널에서는 같은 스타일의 해당 앱 출력과 agy terminal preset을 조합한다. SSH에서는 접속한 로컬 터미널 팔레트를 적용하되 원격 agy의 preset을 별도로 선택한다.

복구는 터미널의 이전 배색표와 agy의 이전 Color Scheme을 선택하거나 세션 전용 `--settings`를 생략하면 된다. 로컬 agy 1.5.18의 버전·도움말 및 공식 스키마는 확인했지만 실제 대화 TUI는 미검증이다.

참고: [agy CLI 설정](https://antigravity.google/docs/cli/using), [디스플레이 설정](https://antigravity.google/docs/settings?tab=cli).
