# TypeScript reference example

A minimal, working TypeScript package showing the expected shape for a
TypeScript project built from `libraries-ai-starter`.

## Layout

```
package.json      # deps, scripts
tsconfig.json      # strict TS config
src/               # source
test/              # vitest tests
```

## Local development

```bash
cd examples/typescript
npm ci
npm test
npm run lint
npm run typecheck
```

## Promoting this example

See [TEMPLATE_SETUP.md](../../TEMPLATE_SETUP.md) at the repository root.
Roughly:

```bash
cp -r examples/typescript/* src/
rm -rf examples/python examples/rust examples/typescript
```
