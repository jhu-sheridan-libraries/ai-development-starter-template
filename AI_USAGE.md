# AI usage and provenance

A running record of every AI model and dataset this project actually uses
in its code, pipelines, or generated content — kept separate from
[`CITATIONS.md`](./CITATIONS.md) (which covers *how to cite*) because this
file's purpose is *provenance and license compliance tracking* over the
life of the project.

Update this file whenever you add, remove, or upgrade a model or dataset
dependency.

## Models

| Model | Provider | Version/checkpoint | License | Used for | Data sent to provider? |
| --- | --- | --- | --- | --- | --- |
| _(example)_ Claude Sonnet | Anthropic | claude-sonnet-4 | Commercial API ToS | Draft generation, code review | Yes — prompts/context per request |
| _(example)_ all-MiniLM-L6-v2 | sentence-transformers | v2 | Apache-2.0 | Local embedding generation | No — runs locally |

## Datasets

| Dataset | Source | Version | License | Used for | Contains PII/sensitive data? |
| --- | --- | --- | --- | --- | --- |
| _(example)_ Internal library catalog export | Institutional | 2026-Q1 snapshot | Internal use only | Search/retrieval evaluation | Review required — see `docs/data-governance.md` |

## Notes on API-based models and data transmission

If this project calls a third-party model API (OpenAI, Anthropic, etc.),
document here:

- What data is sent in each request (prompts, retrieved context,
  user-provided documents).
- Whether that data could include PII, PHI, or IRB-governed content, and
  whether the provider's terms permit that (many enterprise/API tiers
  differ from consumer terms on data retention and training use).
- Any data processing agreement or institutional approval covering the
  integration.

See also [`docs/responsible-ai.md`](./docs/responsible-ai.md) and
[`docs/data-governance.md`](./docs/data-governance.md).
