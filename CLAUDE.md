# unicode-diagram

Unicode box-drawing 다이어그램 렌더링 CLI. 바이너리 이름: `unid`

## 프로젝트 구조

```
src/
  main.rs              CLI 진입점 (clap 디스패치)
  lib.rs               pub mod 선언
  cli.rs               clap derive 구조체 (Parser, Subcommand)
  error.rs             UnidError (thiserror)
  width.rs             unicode-width 래퍼 유틸
  canvas/
    mod.rs
    cell.rs            Cell { ch, display_width }
    grid.rs            Canvas (2D 셀 그리드, display-column 좌표계)
  object/
    mod.rs             DrawObject 열거형
    rect.rs            Rect (박스)
    text.rs            Text
    line.rs            HLine, VLine
    arrow.rs           Arrow
  renderer/
    mod.rs
    engine.rs          Canvas에 DrawObject를 그리는 엔진
  dsl/
    mod.rs
    command.rs         DslCommand 열거형
    parser.rs          DSL 텍스트 파서
tests/
  integration.rs       CLI 통합 테스트
```

## 개발

- Language: Rust (edition 2024)
- Task Runner: mise
- Build: `mise run build`
- Test: `mise run test`
- Lint: `mise run lint`
- Format: `mise run fmt`

## 핵심 설계

- 좌표계: display-column 기반 `(col, row)`, CJK 문자는 2열 차지
- Rect 크기: 내부 크기 (테두리 제외)
- Canvas: 명시적 크기 또는 auto (객체 경계 자동 계산)
- 충돌 제어: collision on (겹침 시 에러) / off (덮어쓰기)
- `width()` (non-CJK mode) 사용: Ambiguous 문자(box-drawing, 화살표)는 1칸, CJK Wide 문자는 2칸
