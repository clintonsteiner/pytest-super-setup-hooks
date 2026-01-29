# Release Guide

## Quick Start

Use the Makefile to automate the entire release process:

```bash
make release-patch   # Bump x.y.z → x.y.z+1
make release-minor   # Bump x.y.z → x.y+1.0
make release-major   # Bump x.y.z → x+1.0.0
```

Each command will:
1. Update `Cargo.toml` with the new version
2. Update `CHANGELOG.md` with the new version section
3. Create a git commit with the version bump
4. Create an annotated git tag
5. Output instructions to push the release

## Publishing a New Release

### Prerequisites

1. Ensure working directory is clean:
```bash
git status
```

2. Run quality checks:
```bash
make check
```

### Creating a Release

1. Choose your release type and run:
```bash
make release-patch    # For bug fixes (0.1.0 → 0.1.1)
make release-minor    # For new features (0.1.0 → 0.2.0)
make release-major    # For breaking changes (0.1.0 → 1.0.0)
```

2. Follow the printed instructions to push:
```bash
git push && git push origin v0.2.0
```

3. GitHub Actions will automatically:
   - Run all tests on Linux, macOS, and Windows
   - Run clippy and format checks
   - Build binaries for:
     - Linux (x86_64)
     - macOS (x86_64 and ARM64/Apple Silicon)
     - Windows (x86_64)
   - Publish to crates.io
   - Create a GitHub release with binary artifacts

### Monitoring the Release

1. Watch the [Actions tab](../../actions) to see the workflow progress
2. Once complete, check the [Releases page](../../releases) for the new release
3. Verify the binaries are available and the crate is published on [crates.io](https://crates.io/crates/pytest-super-hooks)

### Release Artifacts

The GitHub release will include pre-built binaries for:
- `pytest-super-hooks-linux-x86_64` - Linux (x86_64)
- `pytest-super-hooks-macos-x86_64` - macOS (Intel)
- `pytest-super-hooks-macos-aarch64` - macOS (Apple Silicon)
- `pytest-super-hooks-windows-x86_64.exe` - Windows (x86_64)

Users can download these binaries directly instead of compiling from source.

## Installation from Release

### From GitHub Release
```bash
# Download the appropriate binary for your platform
# Then add it to your PATH

# Or install from crates.io
cargo install pytest-super-hooks
```

## Hotfixes

If issues are found after releasing, create a new patch release:

```bash
# Fix the issue and commit
git add <files>
git commit -m "Fix: ..."

# Create patch release
make release-patch

# Push when ready
git push && git push origin v<new-version>
```

## Manual Release (Advanced)

If you need to release manually without the Makefile:

```bash
# 1. Update version and CHANGELOG manually
vi Cargo.toml
vi CHANGELOG.md

# 2. Commit
git add Cargo.toml CHANGELOG.md
git commit -m "Bump version to X.Y.Z"

# 3. Create and push tag
git tag -a vX.Y.Z -m "Release version X.Y.Z"
git push && git push origin vX.Y.Z
```

## Requirements

- GitHub repository must have `CARGO_TOKEN` secret set in Actions settings
  - Get token from https://crates.io/me (Account Settings → API Tokens)
  - Add to repository: Settings → Secrets and Variables → Actions
