# AI Development Starter Template

[![CI](https://github.com/jhu-sheridan-libraries/ai-development-starter-template/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/jhu-sheridan-libraries/ai-development-starter-template/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/github/license/jhu-sheridan-libraries/ai-development-starter-template)](LICENSE)
[![Use this template](https://img.shields.io/badge/GitHub-use%20this%20template-2ea44f?logo=github)](https://github.com/jhu-sheridan-libraries/ai-development-starter-template/generate)

A GitHub template repository for AI projects built by the Johns Hopkins
Sheridan Libraries and academic research computing teams. It gives you a
polyglot starting point (Python, TypeScript, or Rust), a
research-reproducibility layout, and the governance and attribution documents
that enterprise and research-computing work expects — without committing you
to a stack or an architecture.

## Quick start

1. Click **[Use this template](https://github.com/jhu-sheridan-libraries/ai-development-starter-template/generate)** to create a new repository.
2. Work through [TEMPLATE_SETUP.md](TEMPLATE_SETUP.md): pick a language,
   promote that example into `src/`, delete the others, and trim CI to
   match.
3. Run the **Kiro AI Project Bootstrap** starter workflow (**Actions → New
   workflow →** your organization) to scaffold `.kiro/` project context.
4. Fill in the governance files for your project — `CITATION.cff`,
   [AI_USAGE.md](AI_USAGE.md), [docs/data-governance.md](docs/data-governance.md),
   and the rest of the step-4 list in the setup checklist.
5. Replace this README with a description of your own project, and update
   the badges above to point at your repository.

The setup checklist is meant to be deleted once you finish it.

## What this template gives you

- **A language-neutral `src/`** that you promote to your chosen stack,
  backed by working reference implementations under `examples/` (Python,
  TypeScript, Rust) so you can see the intended shape before committing to
  one.
- **A research-reproducibility layout** — `data/`, `notebooks/`,
  `experiments/` — with guidance on data versioning, PII handling, and
  experiment tracking.
- **Academic governance and attribution**: `CITATION.cff`,
  [CITATIONS.md](CITATIONS.md), [ACKNOWLEDGMENTS.md](ACKNOWLEDGMENTS.md),
  model and dataset cards, and an [AI_USAGE.md](AI_USAGE.md) provenance log.
- **Archiving and citability**: `.zenodo.json` and `ro-crate-metadata.json`
  alongside `CITATION.cff`, so your project can archive a GitHub release on
  Zenodo and earn a real DOI. See
  [docs/archiving-and-citability.md](docs/archiving-and-citability.md).
- **AI-specific guidance**: [responsible-AI expectations](docs/responsible-ai.md),
  [prompt-injection awareness](docs/prompt-injection.md), and
  [data-governance, IRB, and PII notes](docs/data-governance.md) for Johns
  Hopkins research contexts.
- **Enterprise scaffolding**: CI with SHA-pinned GitHub Actions and
  least-privilege permissions, issue and pull request templates, CODEOWNERS,
  Dependabot, and a [security policy](SECURITY.md).

### Deliberately no `.kiro/` context

This template ships without `.kiro/steering/`, `.kiro/agents/`, or
`.kiro/settings/mcp.json`. Run the organization's **Kiro AI Project
Bootstrap** starter workflow after creating your repository — it scaffolds
those files, installs the bundled Kiro powers, and opens a pull request for
review. See [TEMPLATE_SETUP.md](TEMPLATE_SETUP.md).

## Repository layout

| Path | Purpose |
| --- | --- |
| `src/` | Your chosen language's application code (an empty placeholder until you promote an example) |
| `examples/{python,typescript,rust}/` | Working reference starters — keep one, delete the rest |
| `data/`, `notebooks/`, `experiments/` | Research reproducibility layout |
| `docs/` | ADRs, responsible AI, prompt injection, data governance, model and dataset cards |
| `.github/` | CI, issue and PR templates, CODEOWNERS, Dependabot, `FUNDING.yml` |
| `CITATION.cff`, `CITATIONS.md`, `ACKNOWLEDGMENTS.md`, `AI_USAGE.md` | Academic attribution, AI and data provenance |
| `.zenodo.json`, `ro-crate-metadata.json` | Archiving and DOI metadata |
| `AGENTS.md` | Conventions for AI coding agents working in this repository |

CI runs one path-filtered job per language, plus `actionlint`, so a change
under `examples/python/` only triggers the Python job.

## Available Kiro workflows

Find these under **Actions → New workflow →** your organization once the
starter workflows are published. Each one needs a repository or organization
secret named `KIRO_API_KEY`.

| Workflow | What it does | Trigger |
| --- | --- | --- |
| **Kiro AI Project Bootstrap** | Scaffolds `.kiro/` context, baseline project docs, and bundled powers in a reviewable pull request | Manual |
| **Kiro** | Implements requested work | A `/kiro <instruction>` comment, a `kiro` label, an assignment, or a manual run |
| **Kiro Docs Sync** | Checks whether source changes need documentation updates | Matching pull requests, or manually against a PR number |
| **Kiro Dependency Audit** | Identifies dependency updates; manual runs accept a version scope, a package filter, and a report-only dry run | Weekly, or on demand |

These workflows give Kiro write access to create branches, update pull
requests, and file issues. Review every Kiro-generated pull request before
merging it.

## Contributing

Contributions to the template itself are welcome. Please read
[CONTRIBUTING.md](CONTRIBUTING.md) and the
[Code of Conduct](CODE_OF_CONDUCT.md) first. To report a security issue,
follow [SECURITY.md](SECURITY.md) rather than opening a public issue.

## License

Released under the [MIT License](LICENSE). The `CITATION.cff`, `.zenodo.json`,
and `ro-crate-metadata.json` files in this repository are placeholders for
projects created from the template, not citation metadata for the template
itself.
