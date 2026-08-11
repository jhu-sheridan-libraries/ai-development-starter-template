# AGENTS.md

Cross-tool project brief for AI coding agents (Kiro, Claude Code, Copilot,
etc.) working in this repository. Follows the
[agents.md](https://agents.md) convention.

> This is a template placeholder. Replace it with your actual project
> brief once you've promoted a language example into `src/`. The
> organization's Kiro AI Project Bootstrap workflow will leave this file
> untouched if it already exists (which it does, by shipping with this
> template) — update it yourself rather than relying on that workflow to
> fill it in.

## Project

- **What this is:** _(one or two sentences)_
- **Primary language/stack:** _(set once you've picked one from
  `examples/`)_
- **Entry point(s):** _(main script, API route, CLI command, etc.)_

## Setup

```bash
# Fill in once src/ has real code, e.g.:
# cd src && pip install -e ".[dev]" && pytest
```

## Conventions

- Reproducibility and research-data handling: see
  [`docs/data-governance.md`](./docs/data-governance.md),
  [`data/README.md`](./data/README.md), and
  [`experiments/README.md`](./experiments/README.md).
- AI/model usage and provenance tracking: see
  [`AI_USAGE.md`](./AI_USAGE.md) — keep it updated when adding or changing
  model/dataset dependencies.
- Responsible-AI and prompt-injection expectations: see
  [`docs/responsible-ai.md`](./docs/responsible-ai.md) and
  [`docs/prompt-injection.md`](./docs/prompt-injection.md).
- Architecture decisions: recorded as ADRs in
  [`docs/adr/`](./docs/adr/) (MADR format).

## Off-limits / caution areas

- Do not commit real data, credentials, or PII/PHI — see
  [`data/README.md`](./data/README.md) and `.gitignore`.
- Do not write real secret values into `.env.example` — names only.

## Additional context

Once the Kiro AI Project Bootstrap workflow has run, this repository will
also have `.kiro/steering/*.md` with more detailed product/tech/structure
context — treat those as authoritative once present, and keep this file
in sync with them at a high level.
