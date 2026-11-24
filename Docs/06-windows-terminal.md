# Windows Terminal에 LBRX 적용

저장소 루트에서 `cargo run --locked -- build --app windows-terminal`을 실행하면 `dist\lbrx\windows-terminal`에 배색표와 선택 적용용 글꼴 설정 조각이 생성된다.

## 설치·복구

1. Windows Terminal의 **설정 → JSON 파일 열기**에서 현재 `settings.json`을 열고 백업합니다.
2. `schemes.json`에 있는 `LBRX Light`, `LBRX Dark` 객체를 기존 최상위 `schemes` 배열에 추가합니다. 같은 이름이 이미 있으면 해당 객체만 교체합니다. 다른 배색표와 설정은 유지합니다.
3. **설정 → 적용할 프로필 → 모양 → 색 구성표**에서 `LBRX Light` 또는 `LBRX Dark`를 선택합니다. JSON에서 직접 적용하려면 해당 프로필의 `colorScheme`에 이름을 지정합니다.
4. 글꼴도 변경하려면 `profile-font.json`의 `font.face`와 `font.size`만 같은 프로필의 `font`에 병합합니다. 기존 굵기·OpenType 설정은 유지합니다. 글꼴은 설치되어 있어야 하며, 원본의 첫 번째 고정폭 글꼴을 사용합니다. 크기는 CSS px × 0.75로 pt로 환산합니다(기본 14px → 10.5pt). 줄 높이와 글꼴 후보 목록은 이 조각에 넣지 않습니다.

테마를 갱신하려면 다시 생성한 후 같은 이름의 배색표를 교체합니다. 되돌리려면 이전 배색표를 다시 선택하고, 변경한 글꼴 필드를 복원합니다. 필요하면 백업한 설정 파일로 돌아갈 수 있습니다. 프로필에 전경·배경·커서·선택 색상이 직접 지정되어 있으면 배색표보다 우선할 수 있습니다.

## 개발용 Portable 미리보기

[Windows Terminal 1.25.3131.0 x64 ZIP](https://github.com/microsoft/terminal/releases/tag/v1.25.3131.0)을 저장소의 `.dev\windows-terminal-1.25.3131.0-x64`에 풉니다. 별도 설치나 기본 터미널 등록은 필요하지 않습니다. ZIP을 풀면 아래와 같은 구조가 됩니다. `WindowsTerminal.exe` 바로 옆에 빈 `.portable` 파일을 만들면 DEV 설정이 그 폴더의 `settings` 아래에 저장됩니다. 자세한 내용은 [Portable 모드](https://learn.microsoft.com/windows/terminal/distributions#enabling-portable-mode)를 참고합니다.

```text
<저장소 루트>\.dev\windows-terminal-1.25.3131.0-x64\terminal-1.25.3131.0\
    WindowsTerminal.exe
    .portable
    settings\settings.json  (DEV 전용 설정, 미리보기 스크립트가 생성)
```

`.dev`는 Git에 포함되지 않으므로 새로 클론한 환경에서는 ZIP을 다시 풀어야 합니다.

저장소 루트에서 PowerShell 7로 실행합니다.

```powershell
.\scripts\dev-windows-terminal.ps1 -Mode Light
.\scripts\dev-windows-terminal.ps1 -Mode Dark
```

스크립트는 테마를 빌드하고 Portable의 `settings\settings.json`을 `LBRX Light (dev)`와 `LBRX Dark (dev)` 두 프로필로 다시 작성한 뒤, 위에 표시된 `WindowsTerminal.exe`로 선택한 프로필의 새 창을 엽니다. 두 프로필은 같은 Portable 복사본을 사용하며, 평소 사용하는 Windows Terminal 설정은 건드리지 않습니다. `-NoLaunch`를 지정하면 설정까지만 생성합니다. 두 모드에서 한글·영문 글꼴, 커서, 마우스 선택 영역, ANSI 16색 견본을 비교합니다. 기본 글꼴인 `D2Coding ligature`가 없다면 글꼴 대체 동작만 확인할 수 있습니다.

참고: Microsoft Learn의 [배색표](https://learn.microsoft.com/windows/terminal/customize-settings/color-schemes)와 [프로필 모양](https://learn.microsoft.com/windows/terminal/customize-settings/profile-appearance).

글꼴이 설치되지 않았다면 [테마 글꼴 확인과 설치](README.md#테마-글꼴-확인과-설치)를 참고합니다. Portable 미리보기도 현재 OS 사용자의 글꼴을 사용하며, 미리보기 자체는 글꼴을 설치하지 않습니다.
