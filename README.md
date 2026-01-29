# pytest-super-setup-hooks

A fast pre-commit hook checker written in Rust that enforces correct `setUp` / `tearDown` method usage in Python test classes.

## Overview

This tool checks Python test classes to ensure:
1. `setUp` and `tearDown` methods call `super().setUp()` and `super().tearDown()` as their last statement
2. Method names use correct camelCase (`setUp` / `tearDown`, not `setup` / `teardown`)
3. Automatically fixes issues with the `--fix` flag

Built with [Ruff's Python parser](https://github.com/astral-sh/ruff) for fast, reliable parsing.

## Install

```bash
cargo install --path .
```

## Usage

### Check files
```bash
pytest-super-hooks test_file.py test_dir/**/*.py
```

### Fix issues automatically
```bash
pytest-super-hooks --fix test_file.py
```

### As a pre-commit hook
Add to `.pre-commit-config.yaml`:
```yaml
- repo: local
  hooks:
    - id: pytest-super-setup
      name: pytest setUp/tearDown checker
      entry: pytest-super-hooks
      language: system
      types: [python]
      stages: [commit]
```

## Examples

### Valid code
```python
class TestExample(TestCase):
    def setUp(self):
        self.value = 42
        super().setUp()

    def tearDown(self):
        self.cleanup()
        super().tearDown()
```

### Issues detected
```python
# ❌ Missing super call
class TestExample(TestCase):
    def setUp(self):
        self.value = 42

# ❌ Super call not last
class TestExample(TestCase):
    def setUp(self):
        super().setUp()
        self.value = 42

# ❌ Wrong casing
class TestExample(TestCase):
    def setup(self):
        super().setUp()
```

## Development

Run tests:
```bash
cargo test
```

Build release:
```bash
cargo build --release
```

The tool includes comprehensive test suites covering:
- Real-world Django and unittest patterns
- Edge cases (async, decorators, docstrings, etc.)
- Automatic fixing functionality
- Complex inheritance hierarchies

