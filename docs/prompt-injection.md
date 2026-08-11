# Prompt injection and untrusted-input guidance

If your project feeds any external or user-controlled text into an LLM
prompt — search results, documents, web pages, uploaded files, chat
messages from someone other than the operator — that text can contain
instructions crafted to hijack the model's behavior. This is prompt
injection, and it is a design concern, not just an input-validation bug.

## Core principle: separate instructions from data

Whatever prompt-construction approach you use, make the boundary between
"things the system was told to do" and "content the system is processing"
explicit and mechanical, not just implied by phrasing.

- Wrap untrusted content in clear delimiters (e.g. a marked block) and
  instruct the model that content between those markers is data to
  process, not instructions to follow.
- Never string-interpolate untrusted content directly into a system
  prompt or the instructional part of a user prompt without such a
  boundary.
- Where your tooling supports it (structured message roles, system vs.
  user vs. tool content), use that structure rather than flattening
  everything into one string.

## Bound what a compromised prompt can actually do

Assume some injected content will eventually get past your prompt
boundary — a determined adversary or an unusual document will find an
edge case. The mitigation that actually matters is **limiting the blast
radius**, not just the prompt wording:

- **Least-privilege tool/function access.** If an agent has tools (file
  access, code execution, network requests, database writes), grant only
  what the specific task needs. An agent that only needs to read and
  summarize a document should not also have write or shell access.
- **No secrets in the model's reachable context** beyond what's strictly
  required, and mask/redact anything that is.
- **A verification gate on generated output**, especially for anything
  that results in a side effect (a file write, a pull request, a database
  change). Check the actual result against an allow-list or invariant
  rather than trusting that the model followed instructions.
- **Human review before anything irreversible.** Treat AI-driven actions
  with real-world effects (merges, deletions, external API calls with
  side effects) as requiring a review step, not full autonomy, until
  you've built enough confidence and guardrails to justify otherwise.

This repository's own ecosystem has a concrete example of this pattern:
the organization's Kiro AI Project Bootstrap starter workflow treats a
user-supplied project description as untrusted data (never
shell-interpolated, explicitly marked as data-not-instructions in the
prompt), and backs that up with an additive-only verification gate so a
compromised or manipulated generation run can't do more than create files
inside a fixed manifest. Apply the same layered thinking here: prompt
hygiene as a first line of defense, and a mechanical check on the actual
result as the real backstop.

## Testing for it

If prompt-construction code is part of your project (not just ad hoc
interactive use), write a test that injects shell metacharacters,
delimiter-breaking text, and "ignore previous instructions"-style content
into every place untrusted text flows into a prompt, and asserts it
appears verbatim as data rather than being executed or breaking the
intended instruction/data boundary.
