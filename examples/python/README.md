# Python reference example

A minimal, working Python package showing the expected shape for a Python
project built from `libraries-ai-starter`.

## Layout

```
pyproject.toml       # deps, ruff, pytest config
src/example_pkg/      # package code
tests/                # pytest tests
```

## Local development

```bash
cd examples/python
python -m venv .venv && source .venv/bin/activate
pip install -e ".[dev]"
pytest
ruff check .
```

## Promoting this example

See [TEMPLATE_SETUP.md](../../TEMPLATE_SETUP.md) at the repository root.
Roughly:

```bash
cp -r examples/python/* src/
rm -rf examples/typescript examples/rust examples/python
```
