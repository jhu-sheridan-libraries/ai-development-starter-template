# Archiving and citability

This template includes three pieces of machine-readable metadata for
**consumer projects created from it**. The template repository itself is not
intended for Zenodo archiving or DOI citation. Before publishing a consumer
project, replace every placeholder and keep these records in sync whenever its
title, authors, license, or release metadata changes.

| File | Consumed by | Purpose |
| --- | --- | --- |
| [`CITATION.cff`](../CITATION.cff) | GitHub's "Cite this repository" widget, Zotero, other reference managers | Human-facing citation metadata for the consumer project |
| [`.zenodo.json`](../.zenodo.json) | Zenodo, when the consumer repository is connected to it | Overrides/supplements metadata Zenodo infers when minting a DOI for a consumer-project release |
| [`ro-crate-metadata.json`](../ro-crate-metadata.json) | RO-Crate-aware tooling, some institutional repositories | Structured, machine-readable consumer-project research object description ([RO-Crate 1.1 spec](https://www.researchobject.org/ro-crate/specification/1.1/)) |

## Releasing this template

Use the **Release template** workflow (Actions → Release template → Run
workflow) to publish a source-only GitHub release that versions this starter
template. It creates the tag and release together at the selected
default-branch commit; it does not publish the independent Python, TypeScript,
or Rust examples.

1. Merge the template changes intended for the release to the default branch.
2. Wait for the `CI` workflow to pass for that commit, then run **Release
   template** from the default branch. Supply a SemVer tag such as `v0.1.0`,
   set **prerelease** only when the tag has a prerelease suffix (for example,
   `v0.1.0-rc.1`), and type `RELEASE` in the confirmation field.
3. The workflow verifies the branch, confirmation, SemVer/prerelease match,
   successful CI run, and absence of an existing tag or release. It then uses
   GitHub-generated release notes to publish the release and create the tag at
   that exact commit.

The template-release workflow deliberately does not inspect or publish
`CITATION.cff`, `.zenodo.json`, or `ro-crate-metadata.json`, and it never
invokes Zenodo. Those files are placeholders for consumer projects, which
choose independently whether to enable DOI archiving.

## Getting a DOI via Zenodo (consumer projects)

If your public project was created from this template and should be formally
citable, [Zenodo](https://zenodo.org/) can mint a permanent DOI for each
published GitHub release. This is a common way to preserve an exact software
snapshot used in a paper or analysis.

1. Replace this template's placeholder fields in `CITATION.cff`,
   `.zenodo.json`, and `ro-crate-metadata.json` with your project's real
   metadata before its first release.
2. Sign in to Zenodo with your GitHub account.
3. Go to [Zenodo → GitHub settings](https://zenodo.org/account/settings/github/)
   and toggle the consumer repository on. It must be public.
4. Fill in `.zenodo.json` with project metadata (title, authors, ORCID,
   license, and any known grant numbers). If the file is absent, Zenodo falls
   back to inferring metadata from the repository and its `CITATION.cff`, but
   `.zenodo.json` gives you more control (for example `upload_type`,
   `communities`, and `related_identifiers`).
5. Publish a GitHub release (tag it, for example `v0.1.0`). Zenodo archives
   that release and mints a DOI within a few minutes.
6. Copy the DOI badge Zenodo gives you into the consumer project's README and
   add the DOI to `CITATION.cff`'s `identifiers` field and to `CITATIONS.md`.

Each release gets its own version-specific DOI, plus Zenodo maintains a
"concept DOI" that always resolves to the latest version. Cite the concept
DOI in ongoing work, and the version-specific DOI when citing an exact
snapshot used in a specific analysis or paper.

## RO-Crate metadata

`ro-crate-metadata.json` is a minimal valid RO-Crate descriptor (per the
spec's required properties: `@type: Dataset`, `@id: ./`, `name`,
`description`, `datePublished`, `license`). Consumer projects should update it
alongside `CITATION.cff`: same facts, different consumers. It is not required
for GitHub or Zenodo citation to work; treat it as an additional
interoperability surface for tools that expect it.

## FUNDING.yml vs. ACKNOWLEDGMENTS.md

`.github/FUNDING.yml` powers GitHub's "Sponsor" button — it is for ongoing
financial support of a consumer project (GitHub Sponsors, Open Collective, a
lab/project donation page), not for citing grant funding that supported the
work. Grant numbers and funder acknowledgments belong in
[`ACKNOWLEDGMENTS.md`](../ACKNOWLEDGMENTS.md) and, where a funder requires it,
in `CITATION.cff`/`.zenodo.json` metadata.
