"""Django-style test example"""
import unittest


class DjangoTestCase(unittest.TestCase):
    """Base test case similar to Django's"""
    pass


class TestModelOperations(DjangoTestCase):
    """Model operations with setup/teardown"""

    def setUp(self):
        """Set up test fixtures"""
        self.model_data = {
            "id": 1,
            "name": "Test Model",
        }
        super().setUp()

    def tearDown(self):
        """Clean up after test"""
        self.model_data = None
        super().tearDown()

    def test_create_model(self):
        self.assertIn("id", self.model_data)

    def test_update_model(self):
        self.assertEqual(self.model_data["name"], "Test Model")


class TestAPIEndpoints(DjangoTestCase):
    """API endpoint testing with improper super() placement - violation"""

    def setUp(self):
        self.client = None
        self.auth_token = "test-token"
        super().setUp()

    def tearDown(self):
        self.client = None
        super().tearDown()

    def test_get_endpoint(self):
        pass

    def test_post_endpoint(self):
        pass