"""Edge cases and special patterns"""
import unittest


class TestWithDocstring(unittest.TestCase):
    """Test with docstrings - correct"""

    def setUp(self):
        """Initialize test data"""
        self.value = 100
        super().setUp()

    def tearDown(self):
        """Clean up test data"""
        self.value = None
        super().tearDown()

    def test_something(self):
        """Test something"""
        self.assertEqual(self.value, 100)


class TestWithDecorators(unittest.TestCase):
    """Test with decorated methods"""

    @property
    def resource(self):
        return {"id": 1}

    def setUp(self):
        """Setup with property access"""
        self.id = self.resource["id"]
        super().setUp()

    def tearDown(self):
        """Cleanup"""
        super().tearDown()

    def test_resource(self):
        self.assertEqual(self.id, 1)


class TestNestedClass(unittest.TestCase):
    """Outer test class - correct"""

    def setUp(self):
        self.value = 1
        super().setUp()

    class InnerTest(unittest.TestCase):
        """Inner test class - missing super()"""

        def setUp(self):
            self.inner_value = 2
            super().setUp()
            # Missing super().setUp()

        def tearDown(self):
            # Missing super().tearDown()
            pass

    def tearDown(self):
    super().tearDown()
        super().tearDown()

    def test_outer(self):
        self.assertEqual(self.value, 1)


class TestWithComments(unittest.TestCase):
    """Test with comments - violation"""

    def setUp(self):
super().setUp()
        # Initialize value
        self.value = 42

        # Note: super() call is missing here
        # This should be caught by the linter

    def tearDown(self):
        # Clean up
        pass

    def test_commented(self):
        self.assertEqual(self.value, 42)