# VS Code 언어별 구문 강조와 동반 CSS

**공통·언어별 토큰 생성 구현.** [언어 설정](../themes/lavender-type/languages/)에서 VS Code 테마와 동반 CSS의 색 역할과 일반·굵게·기울임 스타일을 선택한다. 공통 구문 팔레트와 연결된 TextMate scope, semantic 선택자, 언어별 예외를 출력한다. 작업 화면의 `colors`는 [VS Code 옵션 대장](19-vscode-options.md)에서 별도로 관리한다.

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

다음은 현재 라이트 `function` 역할(`#245E8A`)을 사용한 **출력 예시**다. 생성 시 상수를 복사하지 않고 각 모드의 역할을 해석한다.

```json
{
  "tokenColors": [
    { "scope": "entity.name.function", "settings": { "foreground": "#245E8A" } }
  ],
  "semanticHighlighting": true,
  "semanticTokenColors": { "function": "#245E8A" }
}
```

```html
<pre class="lbrx-code" data-theme="light"><code class="language-typescript"><span class="tok-function">formatTitle</span>()</code></pre>
```

```css
.lbrx-code[data-theme="light"] { --lbrx-code-function: #245E8A; }
.lbrx-code .tok-function { color: var(--lbrx-code-function); }
```

다크도 같은 클래스에 다크 역할값을 쓴다. 언어 전용 예외는 `.lbrx-code .language-typescript …`처럼 코드 영역 안으로 한정한다. 클래스 계약이 다른 렌더러(예: Prism의 `.token.function`)에는 명시적 매핑이 필요하다. 인라인 색을 내보내는 렌더러, Typora·Obsidian의 코드 블록, VS Code 기본 Markdown 미리보기에 이 CSS가 자동 적용된다고 보지 않는다.

## 언어별 지원 대장

언어 TOML은 공통 파일을 제외하면 39개다. 전체 ID와 적용 경로는 빌드할 때 `dist/lavender-type/lbrx/vscode/README.md`에 자동으로 기록된다. 규칙 생성은 문법 설치나 토큰 확인과 별개다.

| 언어 | VS Code 1.147.0 내장 TextMate 확인 범위와 한계 |
|---|---|
| Rust | `println!`은 느낌표를 포함한 하나의 `entity.name.function.macro.rust` 토큰이다. 라이프타임, 어트리뷰트 안의 타입, `fn` 선언도 구별된다. |
| Python | 데코레이터의 `@`와 점은 이름과 같은 역할로 매핑한다. `self`·`cls` 매개변수, 매직 메서드, f-string 경계와 멤버 접근을 구별한다. |
| PowerShell | cmdlet과 `$env:PATH`·`$false` 접두사, 큰따옴표 보간과 작은따옴표 문자열을 구별한다. 시험한 `-Path`·`-Force`는 내장 문법에서 온전한 플래그 토큰으로 식별되지 않았다. |
| TypeScript·TSX | 타입·제네릭, 속성, JSX 기본 태그·사용자 컴포넌트, 템플릿 리터럴의 `${` 경계를 구별한다. |
| JSON·JSONC | 키의 따옴표와 본문에 속성색이 적용되고 문자열 값과 구별된다. JSONC 주석은 주석 scope를 따른다. |
| YAML | 앵커·별칭과 블록 스칼라를 구별한다. 시험한 날짜와 `<<`는 각각 일반 문자열과 키로 분류되므로 별도 색을 강제하지 않는다. |
| Dart | 어노테이션 및 문자열 이스케이프 확인. |
| TOML | 현재 시험한 VS Code에 내장 문법이 없어 `source.toml` 규칙의 토큰 검증에는 별도 문법 확장이 필요하다. |

내장 TextMate 엔진으로 대표 토큰 18개씩 라이트·다크에서 총 36개의 색과 스타일을 확인했다. 의미 강조 제공자, 실제 화면 및 HTML 렌더러 표시는 별도 검증 대상이다.

**완료 기준:** 같은 샘플을 두 출력에서 비교한다. VS Code에서는 **Developer: Inspect Editor Tokens and Scopes**로 실제 scope·semantic 타입·언어 ID를 기록하고, semantic 강조를 끄거나 provider가 없을 때 TextMate 기본색이 남는지 본다. HTML에서는 렌더링된 토큰 클래스와 계산된 전경·배경을 확인한다. 모드별 실제 코드 배경에서 구문색 대비 4.5:1, 언어별 예외가 다른 언어에 영향을 주지 않는지 각각 검증한다. 앱 버전·언어 확장·HTML 렌더러를 명시하며 한쪽만 통과하면 그쪽만 기록한다.

공식 근거: [VS Code Syntax Highlight Guide](https://code.visualstudio.com/api/language-extensions/syntax-highlight-guide) · [Semantic Highlight Guide](https://code.visualstudio.com/api/language-extensions/semantic-highlight-guide)
