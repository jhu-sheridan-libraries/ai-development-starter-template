# Archiving and citability

This project ships three pieces of machine-readable metadata that work
together to make it citable and archivable. Keep all three in sync when
project details (title, authors, license) change.

| File | Consumed by | Purpose |
| --- | --- | --- |
| [`CITATION.cff`](../CITATION.cff) | GitHub's "Cite this repository" widget, Zotero, other reference managers | Human-facing citation metadata |
| [`.zenodo.json`](../.zenodo.json) | Zenodo, when this repository is connected to it | Overrides/supplements the metadata Zenodo would otherwise infer when minting a DOI for a release |
| [`ro-crate-metadata.json`](../ro-crate-metadata.json) | RO-Crate-aware tooling, some institutional repositories | Structured, machine-readable research object description ([RO-Crate 1.1 spec](https://www.researchobject.org/ro-crate/specification/1.1/)) |

## Getting a DOI via Zenodo

[Zenodo](https://zenodo.org/) mints a permanent DOI for a snapshot of this
repository every time you publish a GitHub release. This is the standard
way to make a research software repository formally citable in a paper.

1. Sign in to Zenodo with your GitHub account.
2. Go to [Zenodo → GitHub settings](https://zenodo.org/account/settings/github/)
   and toggle this repository on. It must be public.
3. Fill in `.zenodo.json` with real project metadata (title, authors,
   ORCID, license, any known grant numbers). If this file is absent,
   Zenodo falls back to inferring metadata from the repository and its
   `CITATION.cff`, but `.zenodo.json` gives you more control (e.g.
   `upload_type`, `communities`, `related_identifiers`).
4. Publish a GitHub release (tag it, e.g. `v0.1.0`). Zenodo automatically
   archives that release and mints a DOI within a few minutes.
5. Copy the DOI badge Zenodo gives you into this repository's README (see
   the badges section) and add the DOI to `CITATION.cff`'s `identifiers`
   field and to `CITATIONS.md`.

Each new release gets its own version-specific DOI, plus Zenodo maintains
a "concept DOI" that always resolves to the latest version -- cite the
concept DOI in ongoing work, and the version-specific DOI when citing an
exact snapshot used in a specific analysis or paper.

## RO-Crate metadata

`ro-crate-metadata.json` is a minimal valid RO-Crate descriptor (per the
spec's required properties: `@type: Dataset`, `@id: ./`, `name`,
`description`, `datePublished`, `license`). Some institutional
repositories, data catalogs, and reproducibility-focused tooling can
ingest this directly. Update it alongside `CITATION.cff` -- same facts,
different consumers. It is not required for GitHub or Zenodo citation to
work; treat it as an additional interoperability surface for tools that
expect it.

## FUNDING.yml vs. ACKNOWLEDGMENTS.md

`.github/FUNDING.yml` powers GitHub's "Sponsor" button -- it's for ongoing
financial support of the project (GitHub Sponsors, Open Collective, a
lab/project donation page), not for citing grant funding that supported
the work. Grant numbers and funder acknowledgments belong in
[`ACKNOWLEDGMENTS.md`](../ACKNOWLEDGMENTS.md) and, where a funder requires
it, in `CITATION.cff`/`.zenodo.json`'s metadata.
