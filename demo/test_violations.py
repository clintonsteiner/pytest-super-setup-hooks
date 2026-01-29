import unittest


class TestMissingSuper(unittest.TestCase):
    """Missing super() call - violation"""

    def setUp(self):
        self.value = 42
        # ERROR: Missing super().setUp()

    def tearDown(self):
        self.cleanup()
        # ERROR: Missing super().tearDown()

    def cleanup(self):
        pass

    def test_example(self):
        self.assertEqual(self.value, 42)


class TestSuperNotAtEnd(unittest.TestCase):
    """Super call not at end - violation"""

    def setUp(self):
        super().setUp()
        self.value = 42  # ERROR: Setup after super() call

    def tearDown(self):
        super().tearDown()
        self.data = None  # ERROR: Cleanup after super() call

    def test_example(self):
        self.assertEqual(self.value, 42)


class TestWrongCasing(unittest.TestCase):
    """Wrong method naming - violation"""

    def setup(self):
        """Should be setUp, not setup - ERROR"""
        self.value = 42
        super().setUp()

    def teardown(self):
        """Should be tearDown, not teardown - ERROR"""
        super().tearDown()

    def test_example(self):
        self.assertEqual(self.value, 42)


class TestMixedIssues(unittest.TestCase):
    """Multiple violations"""

    def setUp(self):
        self.config = {}
        self.setup_config()
        # ERROR: Missing super() call

    def setup_config(self):
        self.config = {"debug": True}

    def tearDown(self):
        super().tearDown()
        self.config.clear()  # ERROR: Cleanup after super()

    def test_config(self):
        self.assertIsNotNone(self.config)
