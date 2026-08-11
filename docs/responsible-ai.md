# Responsible AI guidance

Expectations for any project built from
`ai-development-starter-template` that uses
generative AI, machine learning models, or automated decision-making.

## Before you build

- **State the purpose plainly.** Document what the AI component is for,
  who it affects, and what happens if it's wrong. This belongs in your
  project's README or a design doc, not just in code comments.
- **Consider who is affected.** Library patrons, researchers, students, and
  staff may all be downstream of a tool built here. Think about disparate
  impact, especially for anything touching search/discovery ranking,
  content moderation, or eligibility-like decisions.
- **Prefer the least-capable model that meets the need.** A smaller, local,
  or narrower-scope model is easier to evaluate, cheaper to run, and often
  reduces data-exposure risk relative to a large hosted API.

## While you build

- **Track provenance.** Every model and dataset in use should be recorded
  in [`AI_USAGE.md`](../AI_USAGE.md), including license terms and whether
  data leaves your infrastructure.
- **Treat model output as untrusted input**, not ground truth — validate,
  bound, and sanity-check generated content before it reaches a user or
  feeds another automated step. See
  [`docs/prompt-injection.md`](./prompt-injection.md) for the specific
  case of untrusted text driving agent behavior.
- **Log enough to debug and audit**, without logging more sensitive data
  than necessary. Redact or avoid logging raw PII/PHI (see
  [`docs/data-governance.md`](./data-governance.md)).
- **Evaluate before you ship.** Define what "good" looks like (accuracy,
  relevance, failure modes) and check against it, not just "it seems to
  work in my manual testing."

## Before you publish or deploy

- **Disclose AI involvement** where it materially affects the output a
  person sees or relies on (e.g. AI-generated summaries, AI-assisted
  search results).
- **Document known limitations and failure modes** in your README or a
  dedicated limitations section — hallucination risk, bias in training
  data, degraded performance outside the training distribution, etc.
- **Have a human review path.** Especially for anything influencing
  decisions about people, ensure there's a way to contest, correct, or
  override an automated output.

## Model and dataset cards

Use the templates in
[`docs/model-and-data-cards/`](./model-and-data-cards/) to document any
model or dataset central to your project in more depth than the summary
table in `AI_USAGE.md` allows.
