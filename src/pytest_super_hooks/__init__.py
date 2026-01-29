"""pytest-super-hooks: Fast Rust-based pre-commit hook for setUp/tearDown validation"""

import os
import sys
import subprocess
import platform
from pathlib import Path
import importlib.resources


def get_binary_path() -> Path:
    """Get path to the Rust binary."""
    binary_name = "pytest-super-hooks.exe" if sys.platform == "win32" else "pytest-super-hooks"

    # First, check if we're in the repository and have a compiled binary (development)
    # This takes priority because it's platform-specific
    repo_root = Path(__file__).parent.parent.parent

    local_paths = [
        repo_root / "target" / "release" / binary_name,
        repo_root / "target" / "debug" / binary_name,
    ]

    for path in local_paths:
        if path.exists():
            return path

    # Second, check if binary is in PATH (user has installed via cargo)
    try:
        result = subprocess.run(
            ["which" if sys.platform != "win32" else "where", binary_name],
            capture_output=True,
            text=True,
        )
        if result.returncode == 0:
            return Path(result.stdout.strip())
    except (subprocess.SubprocessError, FileNotFoundError):
        pass

    # Third, try to use bundled binary from package data (only if above failed)
    try:
        # Python 3.9+
        if sys.version_info >= (3, 9):
            files = importlib.resources.files("pytest_super_hooks")
            bin_path = files / "bin" / binary_name
            if bin_path.is_file():
                # Extract to a temporary location if needed
                import tempfile
                temp_dir = Path(tempfile.gettempdir()) / "pytest-super-hooks"
                temp_dir.mkdir(exist_ok=True)
                binary_path = temp_dir / binary_name
                if not binary_path.exists():
                    binary_path.write_bytes(bin_path.read_bytes())
                    binary_path.chmod(0o755)
                return binary_path
    except Exception:
        pass

    # Binary not found
    raise RuntimeError(
        f"pytest-super-hooks binary not found.\n"
        f"In development: cargo build --release\n"
        f"Or install with: cargo install --path .\n"
        f"Or ensure it's available in your PATH."
    )


def main() -> int:
    """Main entry point for the pytest-super-hooks command."""
    try:
        binary_path = get_binary_path()
    except RuntimeError as e:
        print(f"Error: {e}", file=sys.stderr)
        return 1

    # Pass all arguments to the binary
    result = subprocess.run([str(binary_path)] + sys.argv[1:])
    return result.returncode


if __name__ == "__main__":
    sys.exit(main())
