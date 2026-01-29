.PHONY: help release-patch release-minor release-major test check fmt clippy clean

# Colors for output
RESET := \033[0m
GREEN := \033[32m
YELLOW := \033[33m

help:
	@echo "$(GREEN)pytest-super-hooks Makefile$(RESET)"
	@echo ""
	@echo "Release targets:"
	@echo "  $(YELLOW)release-patch$(RESET)   - Bump patch version (x.y.z -> x.y.z+1)"
	@echo "  $(YELLOW)release-minor$(RESET)   - Bump minor version (x.y.z -> x.y+1.0)"
	@echo "  $(YELLOW)release-major$(RESET)   - Bump major version (x.y.z -> x+1.0.0)"
	@echo ""
	@echo "Development targets:"
	@echo "  $(YELLOW)test$(RESET)            - Run all tests"
	@echo "  $(YELLOW)check$(RESET)           - Run tests, clippy, and fmt checks"
	@echo "  $(YELLOW)fmt$(RESET)             - Format code"
	@echo "  $(YELLOW)clippy$(RESET)          - Run clippy linter"
	@echo "  $(YELLOW)clean$(RESET)           - Remove build artifacts"

# Get current version from Cargo.toml
CURRENT_VERSION := $(shell grep '^version' Cargo.toml | head -1 | sed 's/version = "\([^"]*\)".*/\1/')

# Parse version components
VERSION_MAJOR := $(shell echo $(CURRENT_VERSION) | cut -d. -f1)
VERSION_MINOR := $(shell echo $(CURRENT_VERSION) | cut -d. -f2)
VERSION_PATCH := $(shell echo $(CURRENT_VERSION) | cut -d. -f3)

# Calculate new versions
NEW_PATCH_VERSION := $(VERSION_MAJOR).$(VERSION_MINOR).$$(( $(VERSION_PATCH) + 1 ))
NEW_MINOR_VERSION := $(VERSION_MAJOR).$$(( $(VERSION_MINOR) + 1 )).0
NEW_MAJOR_VERSION := $$(( $(VERSION_MAJOR) + 1 )).0.0

release-patch: _check-clean _release-impl VERSION=$(NEW_PATCH_VERSION)
release-minor: _check-clean _release-impl VERSION=$(NEW_MINOR_VERSION)
release-major: _check-clean _release-impl VERSION=$(NEW_MAJOR_VERSION)

_check-clean:
	@if [ -n "$$(git status --porcelain)" ]; then \
		echo "$(YELLOW)Error: Working directory is not clean$(RESET)"; \
		echo "Commit or stash changes before creating a release"; \
		exit 1; \
	fi
	@echo "✓ Working directory is clean"

_release-impl:
	@if [ -z "$(VERSION)" ]; then \
		echo "Error: VERSION not set"; \
		exit 1; \
	fi
	@echo "$(GREEN)Bumping version from $(CURRENT_VERSION) to $(VERSION)$(RESET)"
	@# Update Cargo.toml
	@sed -i '' 's/^version = "$(CURRENT_VERSION)"/version = "$(VERSION)"/' Cargo.toml
	@echo "✓ Updated Cargo.toml"
	@# Update CHANGELOG.md - add new unreleased section and promote current unreleased
	@sed -i '' \
		'/^## \[Unreleased\]/a\ \
\
## [$(VERSION)] - '$$(date +%Y-%m-%d)'\
' CHANGELOG.md
	@# Update version links at the bottom
	@sed -i '' \
		's|\[Unreleased\]: https://github.com/|\[Unreleased\]: https://github.com/|' CHANGELOG.md
	@sed -i '' \
		"/\[Unreleased\]:/a\ \
[$(VERSION)]: https://github.com/yourusername/pytest-super-setup-hooks/releases/tag/v$(VERSION)" CHANGELOG.md
	@echo "✓ Updated CHANGELOG.md"
	@# Commit version changes
	@git add Cargo.toml CHANGELOG.md
	@git commit -m "Bump version to $(VERSION)"
	@echo "✓ Created version commit"
	@# Create annotated tag
	@git tag -a v$(VERSION) -m "Release version $(VERSION)"
	@echo "✓ Created git tag v$(VERSION)"
	@echo ""
	@echo "$(GREEN)Version bump complete!$(RESET)"
	@echo "To push the release, run:"
	@echo "  git push && git push origin v$(VERSION)"
	@echo ""

test:
	cargo test --all --verbose

check: test clippy fmt
	@echo "$(GREEN)✓ All checks passed$(RESET)"

fmt:
	cargo fmt --check

clippy:
	cargo clippy --all -- -D warnings

clean:
	cargo clean
	rm -f target/release/pytest-super-hooks
	rm -f target/release/pytest-super-hooks.exe
