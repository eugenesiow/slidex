# slidex

Rust-backed Python library for reading, modifying, and generating PowerPoint
(`.pptx`) files.

## Status

Early scaffolding. APIs and behavior are not stable yet.

## Requirements

- Python 3.9+
- Rust toolchain (stable)
- `uv` for Python dependency management

## Quickstart (uv)

```bash
uv venv
uv pip install maturin pytest
uv run maturin develop
uv run python -c "import slidex; print(slidex)"
```

## Usage

Basic read/modify/write flow:

```python
from slidex import Presentation

pres = Presentation.open("deck.pptx")
pres.replace_text("{{quarter}}", "Q1 2026")

slide = pres.slides[0]
shape = slide.shapes[0]
text = shape.as_text()
text.text = "Hello from slidex"

pres.save("updated.pptx")
```

## Development

Build the extension in editable mode:

```bash
uv run maturin develop
```

Run Python tests:

```bash
uv run pytest tests/python
```

Run Rust tests:

```bash
cargo test --all
```

## Layout

- `src/` Rust core + PyO3 bridge
- `python/slidex/` Python package and typing stubs
- `docs/` design and architecture docs
- `tests/python/` Python tests

## Docs

- `docs/DESIGN.md`
- `docs/ARCHITECTURE.md`
- `docs/API.md`
