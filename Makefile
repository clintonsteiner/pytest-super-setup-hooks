.PHONY: help release-patch release-minor release-major test check fmt clippy clean _check-clean

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

_check-clean:
	@if [ -n "$$(git status --porcelain)" ]; then \
		echo "$(YELLOW)Error: Working directory is not clean$(RESET)"; \
		echo "Commit or stash changes before creating a release"; \
		exit 1; \
	fi
	@echo "✓ Working directory is clean"

release-patch: _check-clean
	@bash -c 'set -e; \
		CURRENT=$$(grep "^version" Cargo.toml | head -1 | sed "s/version = \"\([^\"]*\)\".*/\1/"); \
		MAJOR=$$(echo $$CURRENT | cut -d. -f1); \
		MINOR=$$(echo $$CURRENT | cut -d. -f2); \
		PATCH=$$(echo $$CURRENT | cut -d. -f3); \
		NEW=$$MAJOR.$$MINOR.$$(($$PATCH + 1)); \
		$(MAKE) _release-impl VERSION=$$NEW CURRENT=$$CURRENT'

release-minor: _check-clean
	@bash -c 'set -e; \
		CURRENT=$$(grep "^version" Cargo.toml | head -1 | sed "s/version = \"\([^\"]*\)\".*/\1/"); \
		MAJOR=$$(echo $$CURRENT | cut -d. -f1); \
		MINOR=$$(echo $$CURRENT | cut -d. -f2); \
		NEW=$$MAJOR.$$(($$MINOR + 1)).0; \
		$(MAKE) _release-impl VERSION=$$NEW CURRENT=$$CURRENT'

release-major: _check-clean
	@bash -c 'set -e; \
		CURRENT=$$(grep "^version" Cargo.toml | head -1 | sed "s/version = \"\([^\"]*\)\".*/\1/"); \
		MAJOR=$$(echo $$CURRENT | cut -d. -f1); \
		NEW=$$(($$MAJOR + 1)).0.0; \
		$(MAKE) _release-impl VERSION=$$NEW CURRENT=$$CURRENT'

_release-impl:
	@if [ -z "$(VERSION)" ] || [ -z "$(CURRENT)" ]; then \
		echo "Error: VERSION and CURRENT must be set"; \
		exit 1; \
	fi
	@echo "$(GREEN)Bumping version from $(CURRENT) to $(VERSION)$(RESET)"
	@bash scripts/update-release.sh "$(CURRENT)" "$(VERSION)"
	@git add Cargo.toml CHANGELOG.md
	@git commit -m "Bump version to $(VERSION)"
	@echo "✓ Created version commit"
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
