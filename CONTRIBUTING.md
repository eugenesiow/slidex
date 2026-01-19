# Contributing to slidex

Thanks for helping build slidex.

## Development setup (uv)

```bash
uv venv
uv pip install maturin pytest
uv run maturin develop
```

## Tests

Python tests:

```bash
uv run pytest tests/python
```

Rust tests:

```bash
cargo test --all
```

## Fixture generation (dev-only)

Fixtures are generated via a separate tool subproject that depends on
`python-pptx` but is not required by the library itself.

```bash
cd tools/fixture_gen
python -m venv .venv
source .venv/bin/activate
pip install -e .
python generate_fixtures.py
```

## Style

- Keep changes small and focused.
- Add or update tests when behavior changes.
- Prefer ASCII in source files unless required.

## Pull requests

- Describe the change and rationale.
- Link to relevant issues or docs.
- Ensure CI passes.
