# Release Guide

## Publishing a New Release

### Prerequisites

1. Ensure all tests pass locally:
```bash
cargo test
cargo clippy
cargo fmt --check
```

2. Update version in `Cargo.toml`:
```toml
[package]
version = "0.2.0"  # Increment version
```

3. Commit and push the version bump:
```bash
git add Cargo.toml
git commit -m "Bump version to 0.2.0"
git push
```

### Creating a Release

1. Create an annotated git tag:
```bash
git tag -a v0.2.0 -m "Release version 0.2.0"
```

2. Push the tag:
```bash
git push origin v0.2.0
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

## Rollback

If issues are found after releasing, create a new patch release:

1. Fix the issue and commit it
2. Update version in Cargo.toml (e.g., 0.2.0 → 0.2.1)
3. Tag the new release: `git tag -a v0.2.1 -m "Hotfix: ..."`
4. Push: `git push origin v0.2.1`

## Requirements

- GitHub repository must have `CARGO_TOKEN` secret set in Actions settings
  - Get token from https://crates.io/me (Account Settings → API Tokens)
  - Add to repository: Settings → Secrets and Variables → Actions
