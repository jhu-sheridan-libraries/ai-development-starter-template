# Contributing

Thanks for contributing. This project was created from the
`ai-development-starter-template`; if `TEMPLATE_SETUP.md` still exists at the
repository root, complete it first.

## Getting set up

Pick the section matching whichever example you promoted into `src/`
(delete the others once you've committed to a language):

### Python

```bash
cd src   # or examples/python if not yet promoted
python -m venv .venv && source .venv/bin/activate
pip install -e ".[dev]"
pytest
ruff check .
```

### TypeScript

```bash
cd src   # or examples/typescript if not yet promoted
npm ci
npm test
npm run lint
npm run typecheck
```

### Rust

```bash
cd src   # or examples/rust if not yet promoted
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## Before opening a pull request

- Run the test suite and linter for your language locally.
- If you touched anything under `docs/adr/`, follow the format in
  `docs/adr/README.md`.
- If you added or changed a model/dataset dependency, update
  [`AI_USAGE.md`](./AI_USAGE.md).
- If you touched data handling, review
  [`docs/data-governance.md`](./docs/data-governance.md).
- Fill out the pull request template completely; leave the reviewer
  checklist items unchecked for your reviewer to verify.

## Commit messages

Explain *why*, not just *what* — a commit message that only restates the
diff is less useful than one that explains the reasoning or tradeoff
behind it.

## Code review

At least one CODEOWNERS-designated reviewer must approve before merging.
See [`.github/CODEOWNERS`](./.github/CODEOWNERS).
