# pytest-super-setup-hooks

A fast, Rust-based pre-commit hook that enforces correct `setUp` / `tearDown` method usage in Python unittest-based test classes.

## What It Does

This tool automatically checks Python test classes to ensure:

- `setUp` and `tearDown` methods **call `super().setUp()` / `super().tearDown()` as the last statement**
- Methods use the correct camelCase naming (`setUp` / `tearDown`, not `setup` / `teardown`)
- Allows you to automatically fix these issues with the `--fix` flag

Built on [Ruff's Python parser](https://github.com/astral-sh/ruff) for fast, reliable parsing.

## Quick Start

### Installation

```bash
cargo install --path .
```

### Basic Usage

```bash
# Check files for issues
pytest-super-hooks test_*.py

# Automatically fix issues
pytest-super-hooks --fix test_*.py
```

### Use as Pre-Commit Hook

Add to `.pre-commit-config.yaml`:
```yaml
- repo: https://github.com/yourusername/pytest-super-setup-hooks
  rev: v1.0.0  # Use the latest version tag
  hooks:
    - id: pytest-super-setup
      name: Check setUp/tearDown methods
      entry: pytest-super-hooks
      language: system
      types: [python]
      stages: [commit]
```

Or use as a local hook:
```yaml
- repo: local
  hooks:
    - id: pytest-super-setup
      name: Check setUp/tearDown methods
      entry: pytest-super-hooks
      language: system
      types: [python]
      stages: [commit]
```

## Examples

### Valid Code

```python
class TestExample(unittest.TestCase):
    def setUp(self):
        self.value = 42
        super().setUp()  # Last statement!

    def tearDown(self):
        self.cleanup()
        super().tearDown()  # Last statement!
```

### Issues Detected

```python
# Missing super() call
class TestExample(unittest.TestCase):
    def setUp(self):
        self.value = 42
        # ERROR: super().setUp() must be the last line

# Super call not at end
class TestExample(unittest.TestCase):
    def setUp(self):
        super().setUp()
        self.value = 42  # ERROR: super().setUp() must be the last line

# Wrong method naming
class TestExample(unittest.TestCase):
    def setup(self):  # ERROR: use correct casing: setUp / tearDown
        super().setUp()
```

## Features

- **Fast**: Rust-based parsing is much faster than Python checkers
- **Comprehensive**: Handles Django TestCase, unittest.TestCase, and custom base classes
- **Auto-fix**: Can automatically rename methods and add super() calls
- **Smart**: Skips pass-only methods, handles decorators, docstrings, async methods
- **Recursive**: Checks methods in nested classes

## Development

### Run Tests

```bash
cargo test
```

All 70 tests included covering:
- Real-world Django and unittest patterns
- Edge cases (async, decorators, docstrings, etc.)
- Automatic fixing functionality
- Complex inheritance hierarchies

### Build Release

```bash
cargo build --release
```

Binary will be in `target/release/pytest-super-hooks`

## How It Works

1. Parses Python files using Ruff's parser
2. Recursively walks through all classes in the module
3. For each `setUp` / `tearDown` method:
   - Checks if the method name uses correct casing
   - Verifies the last statement is a `super().setUp()` / `super().tearDown()` call
   - Reports any errors with file name and line number

With `--fix`, it also:
- Renames incorrectly-cased methods
- Adds the super() call as the last statement if missing
- Preserves all other code and formatting

