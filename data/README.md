# data/

## No sensitive or PII data in git

This directory (and this repository generally) is **not** an appropriate
place for raw research data, especially anything containing personally
identifiable information (PII), protected health information (PHI), or
other data covered by an IRB protocol. Git history is effectively
permanent and repositories may be shared more widely than intended (forks,
mirrors, template reuse).

Before adding anything under `data/`:

- Confirm the data's classification and any IRB/data-use-agreement
  constraints. See [`docs/data-governance.md`](../docs/data-governance.md).
- If the data is safe to version, prefer small, de-identified, or synthetic
  fixtures for tests and examples — commit those directly.
- If the data is large or sensitive, do **not** commit it. Use a
  data-versioning tool that keeps large/sensitive content out of git
  history:
  - [DVC](https://dvc.org/) — tracks data alongside code with pointers in
    git, content stored in cloud/remote storage.
  - [git-lfs](https://git-lfs.com/) — large file storage for git, still
    subject to the same sensitivity concerns as regular git content unless
    the LFS remote itself is access-controlled appropriately.
  - An institutional storage system (e.g. a JHU-approved research data
    store) referenced by path/URL in code or a manifest file, rather than
    committed at all.

## Layout convention

```
data/
├── raw/          # Original, unmodified source data (or pointers to it)
├── interim/      # Intermediate processing outputs
└── processed/    # Final, analysis-ready datasets
```

Create these subdirectories as needed; they are not scaffolded by default
since an empty template shouldn't presuppose your pipeline shape.

`.gitignore` at the repository root ignores everything under `data/` except
this README and `.gitkeep`, so you won't accidentally commit data by
default — you have to opt in per file or adjust `.gitignore`.
