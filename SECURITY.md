# Security policy

## Reporting a vulnerability

If you discover a security vulnerability in this project, please report it
privately rather than opening a public issue:

- Use GitHub's [private vulnerability reporting](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing/privately-reporting-a-security-vulnerability)
  feature on this repository (Security tab → "Report a vulnerability"), if
  enabled, **or**
- Contact the maintainers listed in
  [`.github/CODEOWNERS`](./.github/CODEOWNERS) directly.

Please include: a description of the issue, steps to reproduce, and the
potential impact. We'll acknowledge receipt and follow up with next steps.

## Secrets handling

- Never commit real credentials, API keys, or tokens. `.env.example`
  should only ever contain variable *names*, never values.
- If a secret is accidentally committed, treat it as compromised
  immediately: rotate/revoke it at the provider, then remove it from git
  history (rewriting history is a separate, disruptive step — coordinate
  with your team before doing it, and note that a rewrite alone doesn't
  undo any exposure that already occurred).
- CI and automation should request the minimum scope/permissions needed
  (see `.github/workflows/ci.yml`'s `permissions:` block for the pattern
  this template follows).

## Sensitive data

If sensitive or PII/PHI data is discovered in this repository (in git
history, an issue, a notebook, or a log), treat it as a security incident:
follow the reporting steps above and see
[`docs/data-governance.md`](./docs/data-governance.md) for institutional
next steps (IRB/data governance notification), since removing the data
from the repo does not by itself remediate an exposure that already
occurred.

## Supported versions

This is a template repository; supported-versions policy is up to the
project created from it. Replace this section with your project's actual
support policy once established.
