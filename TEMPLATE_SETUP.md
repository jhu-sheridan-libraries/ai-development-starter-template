# Template setup checklist

Run through this once, right after creating a repository from
`libraries-ai-starter`. Delete this file (or reduce it to a changelog entry)
once you've completed it.

## 1. Pick a language and promote an example

`examples/` contains three working, tested reference starters:

- `examples/python/` — `pyproject.toml`, ruff, pytest
- `examples/typescript/` — `package.json`, eslint, vitest
- `examples/rust/` — `Cargo.toml`, clippy, `cargo test`

Pick one:

```bash
# Example: adopting the Python starter
cp -r examples/python/* src/
rm -rf examples/typescript examples/rust examples/python
```

Update `src/README.md` (or remove it) once real project code lives there.

## 2. Trim CI to your chosen language

`.github/workflows/ci.yml` has one path-filtered job per language. After
step 1, delete the jobs for the languages you removed.

## 3. Run the Kiro AI Project Bootstrap workflow

This template intentionally does **not** ship `.kiro/steering/`,
`.kiro/agents/`, or `.kiro/settings/mcp.json`. From your new repository:

1. Go to **Actions → New workflow** and find **Kiro AI Project Bootstrap**
   under your organization's starter workflows.
2. Fill in `project_description` (required) and `stack` (now that you've
   picked a language in step 1, set this explicitly rather than letting
   Kiro infer it).
3. Run it. It opens a pull request adding `.kiro/steering/*.md`,
   `.kiro/agents/<slug>-dev.md`, `.kiro/settings/mcp.json`, and installing
   bundled Kiro powers. `README.md`, `AGENTS.md`, `.gitignore`, and
   `docs/adr/0001-record-architecture-decisions.md` already exist in this
   template, so bootstrap will leave them untouched — that's expected.
4. Review the pull request using the checklist in its description before
   merging.

If your organization has not yet relocated `kiro-bootstrap.yml` into the
org's `.github` repository, see that workflow's own runbook
(`kiro-bootstrap.md`) or ask your platform team.

## 4. Fill in academic governance files

- `CITATION.cff` — project title, authors, ORCID/affiliation if available.
- `CITATIONS.md` — how others should cite this work, plus attribution for
  any third-party models, datasets, or code you build on.
- `ACKNOWLEDGMENTS.md` — funding sources, grant numbers, institutional
  acknowledgments.
- `AI_USAGE.md` — every model and dataset actually used, with license and
  provenance.
- `docs/data-governance.md` — confirm whether this project touches
  human-subjects data and note IRB status if applicable.
- `LICENSE` — replace the placeholder with your chosen license (ask your
  library or research computing team if unsure which license JHU projects
  should default to).
- `.zenodo.json` and `ro-crate-metadata.json` — same project facts as
  `CITATION.cff` (title, authors, license), consumed by Zenodo and
  RO-Crate-aware tooling respectively. See
  [`docs/archiving-and-citability.md`](./docs/archiving-and-citability.md)
  for the full Zenodo DOI setup.
- `.github/FUNDING.yml` — uncomment and fill in if this project accepts
  sponsorship/donations; otherwise delete it.
- README badges — update the CI badge's org/repo path, the License badge,
  and the DOI badge (once you have one from Zenodo).

## 5. Update project-identifying files

- `README.md` — replace this template's README with your actual project
  description.
- `AGENTS.md` — update once you know your project's real structure (or let
  the Kiro bootstrap PR refresh it).
- `.github/CODEOWNERS` — set real owners/teams.
- `.env.example` — add real (but empty/placeholder) variable names your
  project needs; never commit real values.

## 6. Remove this file

Once the above is done, delete `TEMPLATE_SETUP.md` or replace it with a
one-line note in your README linking to your own onboarding docs.
