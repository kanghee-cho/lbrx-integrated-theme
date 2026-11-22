# Vim 언어별 코드 구문 강조

**상태: 설계안.** 터미널 Vim의 라이트·다크 색 구성표가 [VS Code 및 HTML](22-vscode-language-highlighting.md)과 동일한 구문 역할을 사용하도록 한다. Vim은 CSS를 읽지 않는다. 언어의 문법 파일이 토큰을 **구문 그룹**으로 식별하고, 색 구성표는 그 그룹이 연결된 **하이라이트 그룹**에 색을 지정한다. Neovim Tree-sitter 캡처와 플러그인 전용 그룹은 기본 범위 밖이다.

## 공통 역할 → Vim 하이라이트 그룹

| 원본 역할 | 기본 매핑 후보 | 매핑 범위 |
|---|---|---|
| `comment` | `Comment` | 주석. 문자열 안의 주석 기호까지 칠하지 않는다. |
| `keyword` | `Keyword`, `Conditional`, `Repeat` | 키워드·조건·반복. 모든 `Statement` 하위 그룹을 키워드로 간주하지 않는다. |
| `string` | `String` | 문자열 본문. 보간식은 문법이 식별한 안쪽 그룹을 따른다. |
| `function` | `Function` | 함수명으로 식별된 토큰. 선언이 잡혀도 모든 호출까지 식별된다는 뜻은 아니다. |
| `number` | `Number`, `Float` | 정수·실수. 타입 이름이나 숫자가 들어간 식별자는 제외한다. |
| `error` | `Error` | 문법 파일이 오류로 표시한 토큰. `ErrorMsg`나 LSP 진단은 별도다. |

예를 들어 Python의 `def format_title():`에서 문법이 함수명을 `pythonFunction`으로 식별하고 `Function`에 연결한다면, 언어 전용 규칙은 필요 없다. 라이트 테마에서 아래처럼 표준 그룹만 지정하면 된다.

```vim
" 라이트 RGB 경로의 출력 예시. 현재 생성되는 파일은 아니다.
highlight Function guifg=#43617B
```

실제 생성 시 색은 고정하지 않고 [테마 원본](../themes/lbrx.toml)의 모드별 `function`을 해석한다. RGB 경로는 `termguicolors`와 호스트 지원이 필요하다. 비활성일 때 쓸 `ctermfg`는 지원 색 수·호스트 팔레트에 맞춘 근사값으로 별도 검증한다. 편집기 간에 **역할**을 공유하는 것이지, 저색상 터미널에서도 같은 RGB가 보인다는 뜻은 아니다.

## 언어별 지원 대장

`filetype`은 Vim 이름을 사용한다. `[ ]`는 해당 경로에 대한 LBRX 출력 미구현, `—`는 화면 미검증이다. VS Code 문서와 같은 샘플을 쓰되 토큰 경계까지 같다고 가정하지 않는다.

| Vim `filetype` | 비교할 토큰 | RGB 생성 | cterm 생성 | 라이트 / 다크 실측 |
|---|---|---|---|---|
| `rust` | `fn` 뒤 선언명, 함수 호출, `println!` 매크로 | [ ] | [ ] | — / — |
| `python` | `def` 뒤 선언명, 호출, f-string 안쪽 표현식 | [ ] | [ ] | — / — |
| `ps1` | PowerShell cmdlet, `-Path` 매개변수, 큰따옴표 안의 변수 | [ ] | [ ] | — / — |
| `toml` | 테이블명·키와 문자열·숫자 값. 함수 역할은 해당 없음 | [ ] | [ ] | — / — |
| `typescript` | 함수 선언·호출과 타입 이름. JavaScript 버퍼에 예외가 번지지 않는지 비교 | [ ] | [ ] | — / — |

## 예외를 추가하기 전에 확인할 것

1. 시험 파일에서 `:set filetype?`와 `:set syntax?`로 문법이 활성화되었는지 확인한다. 비교할 토큰 위에 커서를 둔다.
2. `:echo synIDattr(synID(line('.'), col('.'), 1), 'name')`로 현재 위치의 구문 그룹을, `:echo synIDattr(synIDtrans(synID(line('.'), col('.'), 1)), 'name')`로 링크를 따라간 최종 그룹을 확인한다. 최종 그룹이 `Function`이라면 `:verbose highlight Function`으로 색을 정한 위치도 확인한다.
3. 이미 의도한 그룹에 연결되면 표준 매핑을 재사용한다. 다른 의미로 연결된 경우에만 관측한 **언어 전용 그룹**을 대상으로 예외를 기록한다. 문법이 함수 호출을 일반 식별자와 구분하지 못하면 `Identifier` 전체를 함수색으로 바꾸지 않고 **식별 불가**로 남긴다.
4. 같은 샘플을 라이트·다크 × RGB·cterm에서 확인한다. `Vim 버전 / 문법 파일·플러그인 / 호스트 터미널 / 토큰 → 최종 그룹 / 실제 전경·배경 / 결과`를 기록한다. RGB 구문색은 실제 코드 배경과 4.5:1을 목표로 하고, cterm은 호스트 팔레트 기준 결과를 별도로 적는다.

HTML에 같은 코드를 싣는 경우에는 별도 Vim용 CSS를 만들지 않고 [공유 `.tok-*` 클래스 계약](22-vscode-language-highlighting.md)을 사용한다. HTML 렌더러가 `tok-function`을 붙인 것과 Vim이 `Function`을 식별한 것은 각각 검증한다.

참고: [Vim 적용 가이드](12-vim.md) · [그룹별 옵션 대장](13-vim-options.md) · [Vim 공식 문법·하이라이트 문서](https://github.com/vim/vim/blob/v9.1.0000/runtime/doc/syntax.txt)
