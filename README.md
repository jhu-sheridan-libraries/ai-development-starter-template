# libraries-ai-starter

[![CI](https://github.com/jhu-sheridan-libraries/libraries-ai-starter/actions/workflows/ci.yml/badge.svg)](https://github.com/jhu-sheridan-libraries/libraries-ai-starter/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](./LICENSE)

> **Badge setup:** the CI badge is configured for
> `jhu-sheridan-libraries/libraries-ai-starter`. Update the org/repo path
> after creating a repository from this template. The License badge reflects
> this template's [MIT License](./LICENSE). This starter template has no DOI
> or citation badge: its citation and Zenodo files are placeholders for
> consumer projects. See
> [`docs/archiving-and-citability.md`](./docs/archiving-and-citability.md) to
> configure those badges after a consumer project's first release.

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
  alongside `CITATION.cff` so a consumer project can archive its own GitHub
  release on Zenodo with a real DOI, plus a `FUNDING.yml` placeholder for
  project sponsorship. See
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

## Available Kiro workflows

After the organization publishes its starter workflows, find them under
**Actions → New workflow → your organization**. Each requires a repository or
organization secret named `KIRO_API_KEY`.

- **Kiro AI Project Bootstrap** — scaffolds `.kiro/` context, baseline project
  documentation, and bundled powers in a reviewable pull request.
- **Kiro** — ask Kiro to implement work from a `/kiro <instruction>` comment,
  a `kiro` label, an assignment, or a manual Actions-tab run.
- **Kiro Docs Sync** — checks whether source-code changes need documentation
  updates; it runs on matching pull requests or manually against a PR number.
- **Kiro Dependency Audit** — runs weekly or on demand to identify dependency
  updates; manual runs support a version-scope choice, a package filter, and a
  report-only dry run.

These workflows grant Kiro write access to create branches, update pull
requests, or file issues as needed. Review every Kiro-generated pull request
before merging it. See [TEMPLATE_SETUP.md](./TEMPLATE_SETUP.md) for bootstrap
setup guidance.

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
