# experiments/

Run outputs, logs, and artifacts from experiments (model training runs,
evaluation sweeps, benchmark comparisons) live here — not committed to git
by default (see the root `.gitignore`), but with a consistent layout so
runs are comparable and reproducible.

## Suggested layout

```
experiments/
└── <experiment-name>/
    └── <run-id>/
        ├── config.yaml     # Full resolved config for this run
        ├── seed.txt         # Random seed(s) used
        ├── metrics.jsonl    # Per-step or per-epoch metrics
        ├── env.txt          # `pip freeze` / `cargo tree` / `npm ls` snapshot
        └── notes.md         # What changed, what you observed
```

Use a timestamp or short hash for `<run-id>` (e.g.
`2026-08-10T14-32-00_a1b2c3d`) so runs sort chronologically and never
collide.

## Reproducibility checklist per run

- Record every random seed used (data shuffling, model init, sampling).
- Capture the exact dependency versions (`pip freeze`, `cargo tree --locked`,
  `npm ls --all`, or your language's equivalent).
- Record hardware/accelerator details if performance-sensitive (CPU/GPU
  model, memory, driver/CUDA version).
- Note the exact model/dataset version or checkpoint used — cross-reference
  [`AI_USAGE.md`](../AI_USAGE.md) and
  [`docs/model-and-data-cards/`](../docs/model-and-data-cards/).

## Tracking at scale

For anything beyond a handful of runs, consider an experiment-tracking tool
(e.g. MLflow, Weights & Biases, or a lab-standard alternative) rather than
relying on this directory structure alone. This layout is meant as a
minimum-viable convention, not a replacement for dedicated tooling once a
project grows.
