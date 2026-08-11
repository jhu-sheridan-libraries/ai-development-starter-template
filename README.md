# libraries-ai-starter

[![CI](https://github.com/jhu-sheridan-libraries/libraries-ai-starter/actions/workflows/ci.yml/badge.svg)](https://github.com/jhu-sheridan-libraries/libraries-ai-starter/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-REPLACE--ME-lightgrey)](./LICENSE)
[![DOI](https://img.shields.io/badge/DOI-pending--first--release-lightgrey)](./docs/archiving-and-citability.md)
[![Cite this repository](https://img.shields.io/badge/cite-CITATION.cff-blue)](./CITATION.cff)

> **Badge setup:** the CI badge assumes this repository lives at
> `jhu-sheridan-libraries/libraries-ai-starter` — update the org/repo path
> once you've created your own repository from this template. Replace the
> License badge once you've picked a license (see `LICENSE`). Replace the
> DOI badge with the real Zenodo badge after your first release (see
> [`docs/archiving-and-citability.md`](./docs/archiving-and-citability.md)).

A GitHub template repository for AI projects built by JHU Sheridan Libraries
and academic research computing teams. It provides a polyglot starting point
(Python, TypeScript, or Rust), a research-reproducibility layout, and the
academic/AI governance documentation expected of enterprise and
research-computing work — without locking you into a stack or an
architecture.

## What this template gives you

- **A language-neutral `src/`** you promote to your chosen stack, backed by
  working reference implementations under `examples/` (Python, TypeScript,
  Rust) so you can see the intended shape before you commit to one.
- **A research-reproducibility layout** (`data/`, `notebooks/`,
  `experiments/`) with guidance on data versioning, PII handling, and
  experiment tracking.
- **Academic governance and attribution** out of the box: `CITATION.cff`,
  `CITATIONS.md`, `ACKNOWLEDGMENTS.md`, model/dataset cards, and an
  `AI_USAGE.md` provenance log.
- **Archiving and citability**: `.zenodo.json` and `ro-crate-metadata.json`
  alongside `CITATION.cff` so a GitHub release can be archived on Zenodo
  with a real DOI, plus a `FUNDING.yml` placeholder for project
  sponsorship. See
  [`docs/archiving-and-citability.md`](./docs/archiving-and-citability.md).
- **AI-specific guidance**: responsible-AI expectations, prompt-injection
  awareness, and data-governance/IRB/PII notes for JHU research contexts.
- **Standard enterprise scaffolding**: CI with SHA-pinned GitHub Actions and
  least-privilege permissions, issue/PR templates, CODEOWNERS, Dependabot,
  and a security policy.
- **Deliberately no `.kiro/` context.** This template intentionally ships
  without `.kiro/steering/`, `.kiro/agents/`, or `.kiro/settings/mcp.json`.
  Run the organization's **Kiro AI Project Bootstrap** starter workflow
  after creating your repository from this template — it will scaffold
  those files (and install bundled Kiro powers) and open a pull request for
  review. See [TEMPLATE_SETUP.md](./TEMPLATE_SETUP.md).

## Getting started

1. Click **Use this template** to create a new repository.
2. Follow [TEMPLATE_SETUP.md](./TEMPLATE_SETUP.md) to pick a language,
   prune the `examples/` you don't need, and promote your choice into
   `src/`.
3. Run the **Kiro AI Project Bootstrap** starter workflow (Actions → New
   workflow → your org) to scaffold `.kiro/` project context.
4. Review and fill in the governance docs at the repository root
   (`CITATION.cff`, `AI_USAGE.md`, `docs/data-governance.md`, etc.) for
   your specific project.
5. Replace this README with a description of your actual project once
   setup is complete.

## Repository layout

| Path | Purpose |
| --- | --- |
| `src/` | Your chosen language's application code (empty placeholder until you promote an example) |
| `examples/{python,typescript,rust}/` | Working reference starters — keep one, delete the rest |
| `data/`, `notebooks/`, `experiments/` | Research reproducibility layout |
| `docs/` | ADRs, responsible-AI, prompt-injection, data-governance, model/dataset cards |
| `.github/` | CI, issue/PR templates, CODEOWNERS, Dependabot, FUNDING.yml |
| `CITATION.cff`, `CITATIONS.md`, `ACKNOWLEDGMENTS.md`, `AI_USAGE.md` | Academic attribution and AI/data provenance |
| `.zenodo.json`, `ro-crate-metadata.json` | Archiving/DOI metadata (see `docs/archiving-and-citability.md`) |

See [TEMPLATE_SETUP.md](./TEMPLATE_SETUP.md) for the full first-run
checklist.
