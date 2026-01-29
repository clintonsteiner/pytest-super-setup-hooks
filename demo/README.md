# Demo - pytest-super-setup-hooks

This directory contains example test files to demonstrate the tool's functionality.

## Files

- **test_correct.py** - Examples of correct setUp/tearDown usage
  - Proper super() calls at the end of methods
  - Multiple test classes with different patterns

- **test_violations.py** - Examples with violations that the tool catches
  - Missing super() calls
  - Super() calls not at the end
  - Wrong method naming (setup/teardown instead of setUp/tearDown)
  - Mixed violations in one class

- **test_django_style.py** - Django-like test patterns
  - TestCase inheritance patterns
  - Proper and improper super() placement

- **test_edge_cases.py** - Edge cases and special patterns
  - Methods with docstrings
  - Decorated methods
  - Nested test classes
  - Methods with comments

## Try It

### Option 1: Direct binary (from project root)

```bash
# Check for violations
cargo run --release -- demo/

# Auto-fix violations
cargo run --release -- --fix demo/

# Verify fixes
cargo run --release -- demo/
```

### Option 2: Python wrapper with uv (from project root)

```bash
# Sync dependencies and install package
uv sync

# Run tests
uv run pytest tests/test_python_wrapper.py -v

# Check demo files
uv run pytest-super-hooks demo/test_violations.py
```

### Option 3: Pre-commit hook (from demo directory)

No setup needed! Pre-commit automatically downloads and installs the Python package from PyPI:

```bash
# Install pre-commit if needed
pip install pre-commit

# Initialize and run the hook from demo directory
cd demo/
pre-commit install
pre-commit run --all-files
```

The `.pre-commit-config.yaml` uses the hook from GitHub. Pre-commit will:
1. Download the Python package from PyPI
2. Extract the bundled Rust binary
3. Run the hook on Python files

No cargo, Rust, or manual binary installation needed!

## Expected Behavior

1. First run will report violations in:
   - test_violations.py (4 classes, 6 violations total)
   - test_django_style.py (improper super() placement)
   - test_edge_cases.py (nested class issues)

2. Auto-fix will:
   - Move super() calls to end of methods
   - Rename setup/teardown to setUp/tearDown
   - Add missing super() calls

3. Second run will pass without errors (test_correct.py is already passing)
