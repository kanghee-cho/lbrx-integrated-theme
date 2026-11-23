# Vim에 LBRX 적용하기

**색 구성표 생성 구현.** 터미널 Vim이 대상이다. Neovim 전용 그룹, GUI 글꼴 및 플러그인별 설정은 포함하지 않는다. 지원 그룹은 [Vim 옵션 대장](13-vim-options.md)에 정리한다.

```text
cargo run --locked -- build --app vim
```

## 설치·복구

1. Vim에서 `:set runtimepath?`로 사용자 설정 경로를 확인한다. 그 아래 `colors` 폴더에 `dist/lbrx/vim/colors/`의 `lbrx_light.vim`, `lbrx_dark.vim`을 복사한다. 일반적인 위치는 Unix `~/.vim/colors`, Windows `~/vimfiles/colors`다. 시스템 `$VIMRUNTIME`에는 복사하지 않는다.
2. `:echo get(g:, 'colors_name', 'default')`로 기존 이름을 기록한다. 현재 세션에서 `:colorscheme lbrx_dark` 또는 `:colorscheme lbrx_light`로 시험한다. 원하는 경우에만 자신의 vimrc에 `colorscheme lbrx_dark`처럼 한 줄을 추가한다.
3. 갱신하려면 두 파일만 교체한 뒤 `:colorscheme`에 이름을 다시 주어 읽힌다. 되돌릴 때는 기록한 테마(없으면 `:colorscheme default`)를 선택하고 추가한 vimrc 줄을 복원한다.

## Vim 전용 옵션

저장소의 `themes/editors/vim.toml`에서 `[highlights]` 아래 원하는 행만 주석 해제하고 다시 빌드한다. 예를 들어 세로 가이드와 맞춤법 오류의 표시를 지정하려면 다음 두 줄을 활성화한다.

```toml
[highlights]
ColorColumn = { bg = "surface" }
SpellBad = { sp = "error", styles = ["undercurl"] }
```

`fg`는 전경, `bg`는 배경, `sp`는 밑줄 등의 특수색이다. 값은 공통 역할 이름이며 라이트·다크에서 각각 해석한다. `"none"`은 해당 색을 해제하고, `styles = []`는 스타일을 해제한다. 생략한 그룹은 기존 동작 그대로이고, 설정한 그룹에는 명시한 필드만 출력한다. Vim에서 기본 링크였던 그룹을 직접 지정하면 링크 대신 독립 그룹이 된다.

테마는 `colorcolumn`, `spell`, `statusline` 같은 Vim 옵션을 켜지 않는다. `User1`–`User9`는 기존 상태줄 포맷에서 `%1*`–`%9*`로 참조해야 표시된다. 밑줄 색·모양과 커서 표시의 지원 범위는 터미널에 따라 달라진다. 기존 `[colors]` 설정은 그대로 쓸 수 있다.

## 색상과 검증 경계

RGB를 지원하는 터미널에서 Vim의 `termguicolors`가 켜져 있으면 `guifg`·`guibg`를 쓴다. 꺼져 있으면 xterm 256색의 16–255번에서 가장 가까운 `ctermfg`·`ctermbg`로 근사한다. 16색 전용 터미널은 지원하지 않으며, 실제 발색은 호스트 팔레트에 따라 달라진다. 테마는 `termguicolors` 설정이나 호스트 글꼴을 바꾸지 않는다.

화면을 검증할 때는 같은 코드·Markdown 파일로 라이트/다크 각각 RGB와 cterm을 비교한다. `:verbose highlight Normal`, `:verbose highlight Comment`로 최종 선언이나 링크를 확인하고, Visual 선택·검색·상태줄을 실제로 표시한다. 그룹 선언이 성공해도 해당 문법 파일이 그 그룹을 쓰는지는 별도로 확인한다.

공식 근거: [Vim 9.1.0000 syntax.txt](https://github.com/vim/vim/blob/v9.1.0000/runtime/doc/syntax.txt), Vim 안의 `:help :colorscheme`, `:help highlight-groups`, `:help termguicolors`. 참조 버전은 지원 검증 버전이 아니다.
