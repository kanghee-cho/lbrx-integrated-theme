# Obsidian 언어별 코드 구문 강조

공통 코드색은 읽기 뷰와 라이브 프리뷰에 적용한다. [언어 설정](../themes/languages/)의 예외는 언어 표식과 토큰 클래스가 있는 렌더링 코드에만 적용한다. 라이브 프리뷰의 편집 가능한 코드 줄에는 공통색을 사용한다. CSS는 언어를 분석하지 않으며, 언어별 실제 화면 검증은 별도로 기록한다.

## 코드 펜스와 두 가지 뷰

노트에서 여는 펜스 뒤에 언어를 적는다. 다음 블록을 그대로 붙여 넣으면 한 예제로 함수명·문자열·키워드·숫자·주석을 비교할 수 있다.

````markdown
```python
def format_title(name):
    return name + "!" * 2  # 표시용 제목
```
````

Obsidian의 **읽기 뷰와 편집 뷰는 서로 다른 구문 강조 라이브러리**를 사용한다. 같은 `--code-function` 값을 주어도 한쪽만 `format_title`을 함수로 식별할 수 있다. 이런 차이는 색상 오류와 분리해 기록한다. 태그 없는 펜스나 지원하지 않는 언어는 일반 코드색으로 읽을 수 있어야 한다. 인라인 코드(D15), 소스 모드와 Publish는 아래 읽기 뷰·라이브 프리뷰의 결과에 포함하지 않는다.

## 공통 역할 → Obsidian CSS

| 공통 역할 | Obsidian 변수 또는 대응 방식 | 적용 범위 |
|---|---|---|
| `code_block_background`, `text` | 블록에 한정한 `--code-background`, `--code-normal` | 여러 줄 코드 블록 배경과 구문으로 구별되지 않은 텍스트. 인라인 배경은 기존 `background`를 유지한다. |
| `comment` | `--code-comment` | 실제 주석 |
| `keyword` | `--code-keyword` | 제어문·선언 등 키워드 |
| `string` | `--code-string` | 문자열. 보간식은 렌더러가 나눈 부분만 별도 처리 |
| `function` | `--code-function` | 함수로 식별된 이름. 선언·호출을 각각 확인 |
| `number` | 렌더링 코드 `.token.number`, 라이브 프리뷰 편집 줄 `.cm-number` | `--code-value`는 바꾸지 않는다. 불리언과 일반 값은 숫자로 취급하지 않는다. |
| `error` | 렌더링 코드 `.token.error`, 라이브 프리뷰 편집 줄 `.cm-error` | 문법이 내보낸 오류 토큰만 색칠한다. 테마가 오류를 판정하거나 진단 밑줄을 변경하지 않는다. |

`--code-important`, `--code-operator`, `--code-property`, `--code-punctuation`, `--code-tag`, `--code-value`는 [Obsidian 옵션 대장](10-obsidian-options.md)의 독립 항목이다. 공통 역할이 없으면 앱 기본값을 유지한다.

아래는 현재 `function` 색을 사용한 **출력 예시**다. 실제 생성 시에는 HEX를 복제하지 않고 각 모드의 역할을 해석한다.

```css
.theme-light { --code-function: #43617B; }
.theme-dark  { --code-function: #97BBD6; }
```

언어별 예외는 기본 변수로 표현할 수 없고 해당 뷰에서 언어와 토큰을 **모두** 식별할 수 있을 때만 추가한다. 읽기 뷰에서 찾은 선택자를 편집기 전체에 적용하거나, 별도 HTML용 [`.tok-*` 클래스](22-vscode-language-highlighting.md)가 Obsidian에도 있다고 가정하지 않는다.

숫자·오류의 공통 색은 `themes/languages/common.toml`의 `[tokens]`에서 바꾼다. 언어별 파일에서는 같은 키로 덮어쓸 수 있다. 예를 들어 Python의 `[tokens]`에 `number = "function"`을 추가하면 렌더링된 Python 코드 블록에서만 숫자가 `function` 역할색을 사용한다. 라이브 프리뷰의 편집 코드 줄에는 언어별 예외 대신 공통색을 적용한다. 구문 분석기가 전용 클래스를 내보내지 않으면 별도 색이 표시되지 않는다. 생성 CSS 검증과 실제 앱에서의 언어별 토큰 검증은 별개다.

## 언어별 지원 대장

아래 모든 언어는 읽기 뷰용 한정 CSS를 생성할 수 있다. `[ ]`는 해당 예제의 세부 토큰 모두에 대한 매핑 확인 전, `—`는 화면 미검증이다. 뷰별 결과는 `라이트 / 다크` 순서로 기록한다. 비교용 예제는 [VS Code](22-vscode-language-highlighting.md)·[Vim](23-vim-language-highlighting.md)과 같은 의미의 토큰을 사용한다.

| 펜스 언어 | 구별할 토큰 | CSS 생성 | 읽기 뷰 | 라이브 프리뷰 |
|---|---|---|---|---|
| `rust` | `fn` 뒤 선언명, 함수 호출, `println!` 매크로 | [ ] | — / — | — / — |
| `python` | `def` 뒤 선언명, 호출, f-string 안쪽 표현식 | [ ] | — / — | — / — |
| `powershell` | cmdlet, `-Path` 매개변수, 큰따옴표 안의 변수 | [ ] | — / — | — / — |
| `toml` | 테이블명·키와 문자열·숫자 값. 함수 역할은 해당 없음 | [ ] | — / — | — / — |
| `typescript` | 함수 선언·호출과 타입 이름 | [ ] | — / — | — / — |

**검증 방법:** 스니펫·커뮤니티 플러그인을 끈 시험 보관함에서 같은 노트를 두 뷰와 두 모드로 연다. 라이브 프리뷰에서는 커서를 펜스 안팎에 놓고 비교한다. 기대색과 다르면 개발자 도구에서 ① 언어와 토큰이 식별됐는지, ② 해당 요소가 어느 CSS 변수를 소비하는지, ③ 계산된 글자색과 실제 코드 배경의 대비가 4.5:1 이상인지 순서대로 확인한다. `앱 버전 / 언어 / 뷰·모드 / 토큰 / 결과`를 남기며, 한 뷰에서 토큰을 구별하지 못하면 그 뷰만 `식별 불가`로 기록한다.

공식 근거: [Obsidian 코드 CSS 변수와 뷰별 차이](https://github.com/obsidianmd/obsidian-developer-docs/blob/1c26772ac13d6e9870c865936d389fdfe694e742/en/Reference/CSS%20variables/Editor/Code.md)
