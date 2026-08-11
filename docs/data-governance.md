# Data governance for research computing projects

This project may handle data subject to institutional, legal, or
funding-agency requirements. This document is a starting checklist, not a
substitute for consulting your IRB, data governance office, or research
computing security contact.

## IRB and human-subjects data

- **Determine applicability early.** If this project uses, analyzes, or
  generates data about identifiable people (including via a model trained
  on or querying such data), determine whether it falls under IRB review
  before collecting or using it.
- **Record IRB status.** If applicable, note the protocol number, approval
  status, and any conditions (e.g. data retention limits, permitted
  secondary uses) here or in a linked internal doc.
- **AI-specific IRB considerations.** Sending human-subjects data to a
  third-party model API is a data disclosure — confirm this is within the
  bounds of your protocol and any data use agreement before doing it. See
  [`AI_USAGE.md`](../AI_USAGE.md) for tracking exactly what data reaches
  which provider.

_(Fill in below once known)_

| Item | Status |
| --- | --- |
| IRB protocol number | _TBD_ |
| Approval status | _TBD_ |
| Data retention limit | _TBD_ |
| Permitted secondary uses | _TBD_ |

## Data classification

Classify data before deciding where it can live (this repository, a
shared drive, institutional storage, a third-party API):

- **Public** — safe to commit to a public or internal repository.
- **Internal/restricted** — institutional data not meant for public
  release, but not individually identifying.
- **Sensitive/PII/PHI** — identifiable or health-related data; typically
  requires IRB oversight, restricted storage, and explicit review before
  any AI/third-party processing.

When in doubt, treat data as more sensitive than less, and ask your
library's or research computing group's data governance contact.

## PII handling in practice

- Never commit real PII/PHI to this repository, in code, notebooks,
  fixtures, logs, or commit messages. See
  [`data/README.md`](../data/README.md) and
  [`notebooks/README.md`](../notebooks/README.md).
- Use synthetic or de-identified data for tests, examples, and
  documentation.
- If PII must be processed at runtime, minimize what's logged, cached, or
  sent to external services, and document the flow in
  [`AI_USAGE.md`](../AI_USAGE.md).
- Review [`docs/responsible-ai.md`](./responsible-ai.md) for the broader
  set of expectations around AI components handling data about people.

## Reporting a data-handling concern

If you discover sensitive data was committed, logged, or transmitted
inappropriately, treat it like a security incident: see
[`SECURITY.md`](../SECURITY.md) for reporting steps, and additionally
notify your institutional data governance or IRB contact as appropriate,
since git history rewrites alone do not fully remediate an exposure once
data has left your control.
