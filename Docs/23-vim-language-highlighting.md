# Vim 언어별 코드 구문 강조

**상태: 공통 그룹과 일부 언어 전용 그룹 생성 구현.** Vim은 CSS를 읽지 않는다. 문법 파일이 토큰을 **구문 그룹**으로 식별하고 색 구성표가 그룹의 색을 지정한다. [언어 파일](../themes/orange-type/languages/)에 선언된 예외는 어댑터가 매핑한 그룹에만 RGB와 `cterm`으로 출력한다. Neovim Tree-sitter와 플러그인 그룹은 포함하지 않는다.

## 공통 역할 → Vim 하이라이트 그룹

| 원본 역할 | 생성 그룹 | 매핑 범위 |
|---|---|---|
| `comment` | `Comment`, `SpecialComment` | 주석. 문자열 안의 주석 기호까지 칠하지 않는다. |
| `keyword` / `keyword.control` | `Keyword` / `Statement`, `Conditional`, `Repeat`, `Label`, `Exception` | 일반 키워드와 제어 흐름을 분리한다. |
| `keyword.declaration` / `attribute` | `Define`, `StorageClass`, `Structure`, `Typedef` / `PreProc`, `Include`, `PreCondit`, `Special`, `Tag` | 선언과 메타 구문을 분리한다. |
| `string` / `string.escape` | `String`, `Character` / `SpecialChar` | 문자열과 문자 리터럴, 별도로 식별되는 이스케이프. |
| `function` / `function.macro` | `Function`, `Debug`(링크) / `Macro` | 함수명이나 매크로로 식별된 토큰. 선언이 잡혀도 모든 호출까지 식별된다는 뜻은 아니다. |
| `number` | `Number`, `Float` | 정수·실수. 타입 이름이나 숫자가 들어간 식별자는 제외한다. |
| `constant.builtin` | `Constant`, `Boolean` | 세부 분류가 없는 상수에도 공통 상수 역할을 사용한다. |
| `type` / `variable` / `operator` / `punctuation` | `Type` / `Identifier` / `Operator` / `Delimiter` | 문법이 해당 그룹으로 분류한 경우에만 적용된다. |
| `error` | `Error` | 문법 파일이 오류로 표시한 토큰. `ErrorMsg`나 LSP 진단은 별도다. |

예를 들어 Python의 `def format_title():`에서 문법이 함수명을 `pythonFunction`으로 식별하고 `Function`에 연결한다면, 언어 전용 규칙은 필요 없다. 라이트 테마에서 아래처럼 표준 그룹만 지정하면 된다.

```vim
" 라이트 RGB 매핑의 일부
highlight Function guifg=#245E8A
```

실제 생성 시 색은 고정하지 않고 [테마 원본](../themes/orange-type/lbrx.toml)의 모드별 `function`을 해석한다. RGB 경로는 `termguicolors`와 호스트 지원이 필요하다. 비활성일 때 쓸 `ctermfg`는 지원 색 수·호스트 팔레트에 맞춘 근사값으로 별도 검증한다. 편집기 간에 **역할**을 공유하는 것이지, 저색상 터미널에서도 같은 RGB가 보인다는 뜻은 아니다.

## 언어별 지원 대장

[언어 설정 폴더](../themes/orange-type/languages/)에는 공통 `common.toml`을 제외하고 **39개 ID**가 있다. 이는 Vim 내장·플러그인 문법 전체의 수가 아니다. 다음은 저장소 기본 TOML로 빌드한 결과이며, 전체 ID와 적용 경로는 매 빌드의 Vim 설치 안내 `dist/orange-type/lbrx/vim/colors/README.md`에 자동으로 표시된다.

| 적용 경로 | 개수 | 언어 ID |
|---|---:|---|
| 언어 전용 규칙 생성 | 7 | `json`, `jsonc`, `powershell`, `python`, `rust`, `toml`, `yaml` |
| 공통 그룹 사용 | 31 | `bash`, `c`, `common-lisp`, `cpp`, `csharp`, `css`, `csv`, `dart`, `dockerfile`, `go`, `graphql`, `hcl`, `html`, `java`, `javascript`, `julia`, `kotlin`, `log`, `lua`, `markdown`, `mdx`, `php`, `r`, `regex`, `ruby`, `shell`, `sql`, `swift`, `typescript`, `xml`, `zig` |
| 일반 전경색 | 1 | `text` |

전용 규칙은 **해당 언어 TOML에 활성 재정의가 있고 어댑터가 그 토큰을 Vim 그룹에 연결할 때만** RGB·256색으로 생성된다. 예를 들어 `typescript`의 Vim 연결은 숫자와 주석에만 있고, 기본 TOML에서는 두 토큰을 재정의하지 않아 전용 규칙이 나오지 않는다. 공통 그룹은 Vim에 해당 문법이 설치되어 있고 그 문법이 표준 그룹으로 연결할 때 적용된다. 두 경로 모두 실제 화면에서 모든 토큰이 검증됐다는 뜻은 아니다.

## 예외를 추가하기 전에 확인할 것

1. 시험 파일에서 `:set filetype?`와 `:set syntax?`로 문법이 활성화되었는지 확인한다. 비교할 토큰 위에 커서를 둔다.
2. `:echo synIDattr(synID(line('.'), col('.'), 1), 'name')`로 현재 위치의 구문 그룹을, `:echo synIDattr(synIDtrans(synID(line('.'), col('.'), 1)), 'name')`로 링크를 따라간 최종 그룹을 확인한다. 최종 그룹이 `Function`이라면 `:verbose highlight Function`으로 색을 정한 위치도 확인한다.
3. 이미 의도한 그룹에 연결되면 표준 매핑을 재사용한다. 다른 의미로 연결된 경우에만 관측한 **언어 전용 그룹**을 대상으로 예외를 기록한다. 문법이 함수 호출을 일반 식별자와 구분하지 못하면 `Identifier` 전체를 함수색으로 바꾸지 않고 **식별 불가**로 남긴다.
4. 같은 샘플을 라이트·다크 × RGB·cterm에서 확인한다. `Vim 버전 / 문법 파일·플러그인 / 호스트 터미널 / 토큰 → 최종 그룹 / 실제 전경·배경 / 결과`를 기록한다. RGB 구문색은 실제 코드 배경과 4.5:1을 목표로 하고, cterm은 호스트 팔레트 기준 결과를 별도로 적는다.

HTML에 같은 코드를 싣는 경우에는 별도 Vim용 CSS를 만들지 않고 [공유 `.tok-*` 클래스 계약](22-vscode-language-highlighting.md)을 사용한다. HTML 렌더러가 `tok-function`을 붙인 것과 Vim이 `Function`을 식별한 것은 각각 검증한다.

참고: [Vim 적용 가이드](12-vim.md) · [그룹별 옵션 대장](13-vim-options.md) · [Vim 공식 문법·하이라이트 문서](https://github.com/vim/vim/blob/v9.1.0000/runtime/doc/syntax.txt)
