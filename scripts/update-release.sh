#!/bin/bash
set -e

if [ -z "$1" ] || [ -z "$2" ]; then
    echo "Usage: $0 <current-version> <new-version>"
    exit 1
fi

CURRENT="$1"
VERSION="$2"
DATE=$(date +%Y-%m-%d)

# Update Cargo.toml
sed -i '' "s/^version = \"$CURRENT\"/version = \"$VERSION\"/" Cargo.toml
echo "✓ Updated Cargo.toml"

# Create temporary files for CHANGELOG manipulation
TEMP=$(mktemp)

# Process CHANGELOG.md
awk -v v="$VERSION" -v d="$DATE" '
/^## \[Unreleased\]/ && !found {
    print
    print ""
    print "## [" v "] - " d
    found=1
    next
}
{ print }
' CHANGELOG.md > "$TEMP"
mv "$TEMP" CHANGELOG.md

# Add version link after [Unreleased]: line
TEMP=$(mktemp)
awk -v v="$VERSION" '
/^\[Unreleased\]:/ {
    print
    print "[" v "]: https://github.com/yourusername/pytest-super-setup-hooks/releases/tag/v" v
    next
}
{ print }
' CHANGELOG.md > "$TEMP"
mv "$TEMP" CHANGELOG.md

echo "✓ Updated CHANGELOG.md"
