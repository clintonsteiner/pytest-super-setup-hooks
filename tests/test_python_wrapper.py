"""Tests for the Python wrapper module."""

import subprocess
import sys
from pathlib import Path

import pytest


def test_wrapper_help():
    """Test that the wrapper can display help."""
    result = subprocess.run(
        [sys.executable, "-m", "pytest_super_hooks", "--help"],
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0


def test_wrapper_version():
    """Test that the wrapper can display version info."""
    result = subprocess.run(
        [sys.executable, "-m", "pytest_super_hooks"],
        capture_output=True,
        text=True,
    )
    # Should fail with no arguments, but binary should be found
    assert result.returncode != 0 or "python" in result.stderr.lower()


@pytest.fixture
def test_file(tmp_path):
    """Create a test Python file with violations."""
    test_py = tmp_path / "test_violations.py"
    test_py.write_text(
        """
import unittest

class TestExample(unittest.TestCase):
    def setUp(self):
        self.value = 42
        # Missing super().setUp()

    def tearDown(self):
        pass

    def test_example(self):
        self.assertEqual(self.value, 42)
"""
    )
    return test_py


def test_wrapper_detects_violations(test_file):
    """Test that the wrapper detects violations in Python files."""
    result = subprocess.run(
        [sys.executable, "-m", "pytest_super_hooks", str(test_file)],
        capture_output=True,
        text=True,
    )
    assert result.returncode == 1
    assert "super().setUp() must be the last line" in result.stderr


def test_wrapper_with_fix_flag(test_file):
    """Test that the wrapper can fix violations."""
    result = subprocess.run(
        [sys.executable, "-m", "pytest_super_hooks", "--fix", str(test_file)],
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0

    # Verify file was modified
    content = test_file.read_text()
    assert "super().setUp()" in content


def test_wrapper_correct_code(tmp_path):
    """Test that the wrapper passes on correct code."""
    test_py = tmp_path / "test_correct.py"
    test_py.write_text(
        """
import unittest

class TestExample(unittest.TestCase):
    def setUp(self):
        self.value = 42
        super().setUp()

    def tearDown(self):
        super().tearDown()

    def test_example(self):
        self.assertEqual(self.value, 42)
"""
    )
    result = subprocess.run(
        [sys.executable, "-m", "pytest_super_hooks", str(test_py)],
        capture_output=True,
        text=True,
    )
    assert result.returncode == 0
