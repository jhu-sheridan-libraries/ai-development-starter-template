# Model card: <model name>

> Copy this file per model that plays a central role in your project (not
> every incidental API call needs one — use judgment; a one-line entry in
> `AI_USAGE.md` may be enough for peripheral usage).

## Overview

- **Model name / version:**
- **Provider:**
- **License:**
- **Modality:** (text, embedding, vision, audio, multimodal, ...)
- **Where it runs:** (third-party API / self-hosted / local)

## Intended use in this project

What this model is used for here, specifically — not its general
capabilities.

## Out-of-scope / unintended uses

What this model should *not* be used for in this project, and why.

## Data

- **Training data (if known/disclosed by provider):**
- **Input data at inference time in this project:**
- **Does input data include PII/PHI?** See
  [`docs/data-governance.md`](../data-governance.md).

## Known limitations and failure modes

- Hallucination / factual-accuracy limitations relevant to this use case.
- Known biases relevant to this project's domain or user population.
- Performance boundaries (e.g. context length, language coverage, domain
  specificity).

## Evaluation

How you evaluated this model for this specific use (benchmarks used,
manual review process, acceptance criteria).

## Monitoring and review cadence

How/when this model's performance or the model version itself will be
revisited.
