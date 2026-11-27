# Claude Code CLI에 LBRX 적용

`cargo run --locked -- build --app claude-code`로 각 스타일의 `dist\<style>\lbrx\claude-code`에 `lbrx-light.json`, `lbrx-dark.json`, Windows Terminal 배색표 `schemes.json`과 이 안내를 생성한다.

## 실제 CLI 테마 설치

현재 [Claude Code 공식 사용자 정의 테마 문서](https://code.claude.com/docs/en/terminal-config#create-a-custom-theme)의 JSON 형식을 사용한다.

1. `~/.claude/themes/`를 만들고 원하는 스타일의 두 JSON을 복사한다. Windows 경로는 `$HOME\.claude\themes`다. 기존 동명 파일이 있으면 백업 후 교체한다.
2. Claude Code에서 `/theme`를 실행하고 `LBRX Light` 또는 `LBRX Dark`를 선택한다. 저장되는 선택 값은 `custom:lbrx-light` 또는 `custom:lbrx-dark`다.
3. 테마 폴더가 앱 시작 전에 없었다면 최초 복사 후 한 번 재시작한다. 이후 파일 변경은 지원 버전에서 자동 재로드된다.
4. 복구는 `/theme`에서 기본 테마를 선택한 후 추가 파일만 제거하거나 백업을 복원한다.

사용자 정의 테마 항목이 없는 구버전은 앱을 공식 방법으로 업데이트하거나 기본 `/theme` 라이트·다크와 터미널 배색만 사용한다. 비공식 설정 키를 만들거나 Claude 실행 파일을 패치하지 않는다. 생성기는 설치·프로필·Claude 사용자 설정을 수정하지 않는다.

## 터미널 배경과 앱 색의 경계

Claude 테마는 spinner·본문·상태·입력 테두리·메시지 패널 등 공식 색 토큰을 바꾼다. 터미널 자체의 기본 배경·ANSI 팔레트·서체는 바꾸지 않는다. 라이트/다크에 맞춰 같은 스타일의 Windows Terminal `schemes.json`을 적용한다. macOS Terminal·iTerm2 및 Antigravity/VS Code 통합 터미널은 같은 스타일의 해당 앱 출력을 사용한다.

`schemes.json`은 기존 Windows Terminal 출력과 같으며 Windows Terminal 전용 재정의를 그대로 따른다. Claude 전용 토큰 재정의는 터미널 배색에 영향을 주지 않는다. 임의 ANSI 배경·dim·256색·True Color와 앱이 직접 지정한 구문색까지 동일해지는 것은 아니다.

## 전용 TOML

`themes\<style>\editors\claude-code.toml`의 `[colors]`는 공식 토큰을 양쪽 모드에 존재하는 공통 역할에 연결한다. HEX 직접 지정 대신 역할을 재사용한다. 생성되는 `overrides` 목록만 지원하며 알 수 없는 토큰·역할은 생성 단계에서 오류로 알린다.

두 파일은 각각 `base: light`와 `base: dark`를 사용한다. 명시하지 않은 diff 추가/삭제 배경·단어 강조·subagent/rainbow 색과 코드 구문 분류는 기본 preset에 맡긴다. 색 테마는 Claude의 구문 분석을 대체하지 않는다. 토큰 형식과 생성 JSON은 검증하지만 설치된 Claude Code 실제 CLI 화면은 미검증이다.

파일 이름은 스타일 간 같다. 다른 스타일을 설치하면 교체된다. 여러 스타일을 보관하려면 파일명을 `lavender-light.json`처럼 바꾸고 `/theme`에서 선택한다. 실제 선택 slug는 파일명에서 `.json`을 제거한 값이다.
