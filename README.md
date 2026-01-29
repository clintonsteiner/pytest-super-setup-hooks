# pytest-super-setup-hooks

A fast, Rust-based pre-commit hook that enforces correct `setUp` / `tearDown` method usage in Python unittest-based test classes.

## What It Does

This tool automatically checks Python test classes to ensure:

- `setUp` and `tearDown` methods **call `super().setUp()` / `super().tearDown()` as the last statement**
- Methods use the correct camelCase naming (`setUp` / `tearDown`, not `setup` / `teardown`)
- Allows you to automatically fix these issues with the `--fix` flag

Built on [Ruff's Python parser](https://github.com/astral-sh/ruff) for fast, reliable parsing.

## Quick Start

### Installation (for standalone usage)

```bash
# Option 1: Install from PyPI (recommended)
pip install pytest-super-hooks

# Option 2: Install from source as Python package
pip install .

# Option 3: Install via cargo
cargo install --path .
```

### Basic Usage (standalone)

```bash
# Check files for issues
pytest-super-hooks test_*.py

# Automatically fix issues
pytest-super-hooks --fix test_*.py
```

### Use as Pre-Commit Hook

Simply add to `.pre-commit-config.yaml`:

**Check mode (fail on violations):**
```yaml
- repo: https://github.com/clintonsteiner/pytest-super-setup-hooks
  rev: v0.11.0
  hooks:
    - id: pytest-super-setup
```

**Or auto-fix mode (automatically fix violations):**
```yaml
- repo: https://github.com/clintonsteiner/pytest-super-setup-hooks
  rev: v0.11.0
  hooks:
    - id: pytest-super-setup-fix
```

That's it! Available hook IDs:
- `pytest-super-setup` - Check mode (report violations, exit with code 1)
- `pytest-super-setup-fix` - Auto-fix mode (automatically correct violations)

How it works:
- Pre-commit automatically installs the Python package with bundled Rust binary
- No cargo or Rust installation required
- Works on macOS, Linux, and Windows

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

## Demo

Try the tool on example test files in the `demo/` directory:

```bash
# Check violations
cargo run --release -- demo/test_violations.py

# Auto-fix violations
cargo run --release -- --fix demo/test_violations.py

# Verify fixes applied
cargo run --release -- demo/test_violations.py
```

The `demo/` directory includes:
- **test_correct.py** - Passing tests (correct usage)
- **test_violations.py** - Multiple violations to demonstrate detection and fixing
- **test_django_style.py** - Django TestCase patterns
- **test_edge_cases.py** - Special cases (decorators, docstrings, nested classes)

See [demo/README.md](demo/README.md) for more details.

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

