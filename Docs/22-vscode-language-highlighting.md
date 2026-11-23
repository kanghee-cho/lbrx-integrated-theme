# VS Code 언어별 구문 강조와 동반 CSS

**공통·언어별 토큰 생성 구현.** [언어 설정](../themes/languages/)에서 VS Code 테마와 동반 CSS에 쓸 색 역할을 선택한다. 기본 TextMate 전경색 6개와 semantic 선택자 `function`, `number`에 더해, 언어 파일에 명시한 키만 한정 규칙으로 출력한다. 언어별 실제 렌더링은 미검증이다.

| 보는 곳 | 생성 파일 | 토큰을 구별하는 주체 |
|---|---|---|
| VS Code 편집기 | [VSIX의 라이트·다크 색 테마](18-vscode.md)에 `tokenColors`, `semanticTokenColors`를 포함 | TextMate 문법과 언어 서비스. 편집기 자체에는 CSS를 주입하지 않는다. |
| HTML 문서·코드 미리보기 | 선택 배포하는 `lbrx-code.css` 한 파일에 라이트·다크 규칙 포함 | 렌더러가 원문을 토큰으로 나누고 클래스가 붙은 HTML을 만든다. CSS만 로드해서는 원문이 강조되지 않는다. |

## 공통 규칙과 언어별 예외

기본 매핑은 언어에 관계없이 적용한다. 아래 scope는 대표 예시이지 모든 문법이 반환하는 이름이 아니다. 예외는 **같은 역할이 다른 이름으로 보고되거나, 같은 이름이 다른 의미로 쓰일 때**에만 추가한다.

| 역할 | VS Code TextMate 예 | 동반 HTML 클래스 |
|---|---|---|
| `comment` | `comment` | `.tok-comment` |
| `keyword` | `keyword` | `.tok-keyword` |
| `string` | `string` | `.tok-string` |
| `function` | `entity.name.function` | `.tok-function` |
| `number` | `constant.numeric` | `.tok-number` |
| `error` | `invalid.illegal` | `.tok-error` |

예를 들어 TypeScript의 함수 **선언**은 문법이 잡더라도 함수 **호출**은 언어 서비스가 있어야 구분될 수 있다. TypeScript 파일에서 `function` 값을 지정하면 `function:typescript`와 `source.ts entity.name.function`을 출력한다. `invalid.illegal`은 문법 오류 토큰이지 LSP 진단 밑줄을 뜻하지 않는다.

### 같은 함수색을 두 형식으로 제공하는 예

다음은 현재 라이트 `function` 역할(`#43617B`)을 사용한 **출력 예시**다. 생성 시 상수를 복사하지 않고 각 모드의 역할을 해석한다.

```json
{
  "tokenColors": [
    { "scope": "entity.name.function", "settings": { "foreground": "#43617B" } }
  ],
  "semanticHighlighting": true,
  "semanticTokenColors": { "function": "#43617B" }
}
```

```html
<pre class="lbrx-code" data-theme="light"><code class="language-typescript"><span class="tok-function">formatTitle</span>()</code></pre>
```

```css
.lbrx-code[data-theme="light"] { --lbrx-code-function: #43617B; }
.lbrx-code .tok-function { color: var(--lbrx-code-function); }
```

다크도 같은 클래스에 다크 역할값을 쓴다. 언어 전용 예외는 `.lbrx-code .language-typescript …`처럼 코드 영역 안으로 한정한다. 클래스 계약이 다른 렌더러(예: Prism의 `.token.function`)에는 명시적 매핑이 필요하다. 인라인 색을 내보내는 렌더러, Typora·Obsidian의 코드 블록, VS Code 기본 Markdown 미리보기에 이 CSS가 자동 적용된다고 보지 않는다.

## 언어별 지원 대장

기본 5개 언어 모두 명시된 토큰의 편집기·HTML 한정 규칙을 생성할 수 있다. 아래 `[ ]`는 각 예제의 세부 구별(호출·보간·매크로 등)을 아직 개별 확인하지 않았다는 뜻이고, `—`는 실제 검증 기록 없음이다.

| VS Code 언어 ID | 언어별로 구별할 예 | 편집기 JSON | 동반 CSS/HTML 매핑 | 라이트·다크 실측 |
|---|---|---|---|---|
| `rust` | `fn` 뒤 선언명, 함수 호출, `println!` 매크로를 구분 | [ ] | [ ] | — |
| `python` | `def` 뒤 선언명, 호출, f-string 안쪽 표현식 | [ ] | [ ] | — |
| `powershell` | cmdlet 이름, `-Path` 매개변수, 큰따옴표 안의 변수 | [ ] | [ ] | — |
| `toml` | 테이블 이름·키와 문자열·숫자 값. 함수가 없는 언어에 함수 규칙을 만들지 않음 | [ ] | [ ] | — |
| `typescript` | 함수 선언·호출과 타입 이름. 같은 규칙이 JavaScript까지 번지지 않는지 확인 | [ ] | [ ] | — |

**완료 기준:** 같은 샘플을 두 출력에서 비교한다. VS Code에서는 **Developer: Inspect Editor Tokens and Scopes**로 실제 scope·semantic 타입·언어 ID를 기록하고, semantic 강조를 끄거나 provider가 없을 때 TextMate 기본색이 남는지 본다. HTML에서는 렌더링된 토큰 클래스와 계산된 전경·배경을 확인한다. 모드별 실제 코드 배경에서 구문색 대비 4.5:1, 언어별 예외가 다른 언어에 영향을 주지 않는지 각각 검증한다. 앱 버전·언어 확장·HTML 렌더러를 명시하며 한쪽만 통과하면 그쪽만 기록한다.

공식 근거: [VS Code Syntax Highlight Guide](https://code.visualstudio.com/api/language-extensions/syntax-highlight-guide) · [Semantic Highlight Guide](https://code.visualstudio.com/api/language-extensions/semantic-highlight-guide)
