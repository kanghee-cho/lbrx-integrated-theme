# Vim에 LBRX 적용하기

**설계 단계.** 터미널에서 실행하는 Vim이 대상이다. 현재 `build`는 Vim 파일을 생성하지 않는다. Neovim 전용 그룹, GUI 글꼴 및 플러그인별 설정은 포함하지 않는다. 색 구성표가 지정할 수 있는 필드는 [Vim 옵션 대장](13-vim-options.md)에 정리한다.

## 구현 후 설치·복구

1. Vim에서 `:set runtimepath?`로 사용자 설정 경로를 확인한다. 그 아래 `colors` 폴더에 **예정 산출물** `lbrx_light.vim`, `lbrx_dark.vim`만 복사한다. 일반적인 위치는 Unix `~/.vim/colors`, Windows `~/vimfiles/colors`다. 시스템 `$VIMRUNTIME`에는 복사하지 않는다.
2. `:echo get(g:, 'colors_name', 'default')`로 기존 이름을 기록한다. 현재 세션에서 `:colorscheme lbrx_dark` 또는 `:colorscheme lbrx_light`로 시험한다. 원하는 경우에만 자신의 vimrc에 `colorscheme lbrx_dark`처럼 한 줄을 추가한다.
3. 갱신하려면 두 파일만 교체한 뒤 `:colorscheme`에 이름을 다시 주어 읽힌다. 되돌릴 때는 기록한 테마(없으면 `:colorscheme default`)를 선택하고 추가한 vimrc 줄을 복원한다.

## 색상과 검증 경계

RGB를 지원하는 터미널에서 Vim의 `termguicolors`가 켜져 있으면 `guifg`·`guibg`를 쓰고, 꺼져 있으면 `ctermfg`·`ctermbg` 색 번호의 근사 결과를 본다. 색 번호의 실제 색은 호스트 팔레트에 따라 달라진다. 테마가 사용자의 `termguicolors` 옵션이나 호스트 글꼴을 바꾸지는 않는다.

첫 검증에서는 같은 코드·Markdown 파일로 라이트/다크 각각 RGB와 cterm을 비교한다. `:verbose highlight Normal`, `:verbose highlight Comment`로 최종 선언이나 링크를 확인하고, Visual 선택·검색·diff·상태줄을 실제로 표시한다. `:version`, 호스트 터미널, 모드, 색 경로를 옵션 대장에 남긴다. 그룹 선언이 성공해도 해당 문법 파일이 그 그룹을 쓰는지는 별도로 확인한다.

공식 근거: [Vim 9.1.0000 syntax.txt](https://github.com/vim/vim/blob/v9.1.0000/runtime/doc/syntax.txt), Vim 안의 `:help :colorscheme`, `:help highlight-groups`, `:help termguicolors`. 참조 버전은 지원 검증 버전이 아니다.
