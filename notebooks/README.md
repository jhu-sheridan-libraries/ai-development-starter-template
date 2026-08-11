# notebooks/

Exploratory and analysis notebooks live here.

## Hygiene rules

- **Clear outputs before committing.** Notebook outputs can embed data,
  file paths, or credentials that leaked into a cell's output at some
  point. Use `jupyter nbconvert --clear-output --inplace <notebook>` (or an
  equivalent pre-commit hook) before committing.
- **No secrets or credentials in cells.** Load API keys and tokens from
  environment variables (see `.env.example` at the repository root), never
  hardcode them in a cell.
- **No PII or sensitive data inline.** Read from `data/` (or an external
  store) rather than pasting rows of real data into a cell for
  convenience. See [`docs/data-governance.md`](../docs/data-governance.md).
- **Prefer promoting stable code to `src/`.** A notebook is for exploration;
  once logic is settled, move it into your project's actual package/module
  so it's tested and reusable.
- **Name notebooks with intent**, e.g. `01-data-exploration.ipynb`,
  `02-baseline-model.ipynb`, so the sequence and purpose are clear without
  opening each file.
