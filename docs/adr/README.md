# Architecture Decision Records

This directory holds Architecture Decision Records (ADRs) in
[MADR](https://adr.github.io/madr/) format, documenting significant
architectural and technical decisions for this project and the reasoning
behind them.

## Adding a new ADR

1. Copy the format of `0001-record-architecture-decisions.md`.
2. Number sequentially (`0002-`, `0003-`, ...).
3. Use a short, descriptive title in the filename
   (`000N-short-decision-title.md`).
4. Fill in context, the decision, and consequences (including tradeoffs
   and what was rejected, not just what was chosen).
5. Reference related ADRs by number if a decision supersedes or builds on
   a prior one.

ADRs are immutable once accepted — if a decision changes, write a new ADR
that supersedes the old one rather than editing history in place.
