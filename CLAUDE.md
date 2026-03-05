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
    rect.rs            Rect (box 객체, DSL 키워드: box)
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
- Box(Rect) 크기: 내부 크기 (테두리 제외)
- Canvas: 명시적 크기 또는 auto (객체 경계 자동 계산)
- 충돌 제어: collision on (겹침 시 에러) / off (덮어쓰기)
- `width()` (non-CJK mode) 사용: Ambiguous 문자(box-drawing, 화살표)는 1칸, CJK Wide 문자는 2칸
- 렌더링: 2-pass (구조 → 텍스트). 텍스트(`c=`, `lg=`, text 오브젝트)는 구조적 요소 위에 렌더링
- Arrow 앵커: box, text, hline, vline 모두 `id=`로 arrow 대상 가능
- Self-loop: `src_id == dst_id` 시 전용 ㄷ-shape 라우팅
- Adaptive gap: Manhattan 거리 기반 동적 routing gap (2/3/4)

## Guide 예제 관리

- `print_guide()` 예제 DSL 변경 시 OUTPUT 영역을 실제 `cargo run` 결과로 교체 (필수)
- 예제의 다양성 유지: 모든 테두리 스타일, 다양한 arrow 형태, CJK 텍스트, overflow 데모 포함

## 코드 변경 시 문서 업데이트

- DSL 문법, 옵션, 커맨드 변경 시 `print_guide()` 업데이트 검토 (필수)
- CLI 인터페이스, 서브커맨드 변경 시 help 메시지 업데이트 검토 (필수)
