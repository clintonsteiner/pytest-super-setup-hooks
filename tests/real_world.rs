use pytest_super_hooks::check_file;
use std::fs::write;
use tempfile::TempDir;

/// Test suite with real-world Python test class examples

fn run(src: &str) -> Vec<String> {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join("test.py");
    write(&file_path, src).unwrap();
    check_file(&file_path)
}

#[test]
fn django_style_test_setup_with_database() {
    let src = r#"
class UserTestCase(TestCase):
    def setUp(self):
        self.user = User.objects.create(username="test")
        self.client = Client()
        super().setUp()
"#;
    assert!(run(src).is_empty());
}

#[test]
fn django_style_test_setup_missing_super() {
    let src = r#"
class UserTestCase(TestCase):
    def setUp(self):
        self.user = User.objects.create(username="test")
        self.client = Client()
"#;
    let errors = run(src);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("super().setUp()"));
}

#[test]
fn fixture_based_setup_wrong_casing() {
    let src = r#"
class APITests(TestCase):
    def setup(self):
        self.api = APIClient()
        super().setUp()
"#;
    let errors = run(src);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("correct casing"));
}

#[test]
fn multiple_setup_methods_in_class_hierarchy() {
    let src = r#"
class BaseTest(TestCase):
    def setUp(self):
        self.base_setup = True
        super().setUp()

class DerivedTest(BaseTest):
    def setUp(self):
        self.derived_setup = True
        super().setUp()
"#;
    assert!(run(src).is_empty());
}

#[test]
fn setup_with_try_except() {
    let src = r#"
class DatabaseTest(TestCase):
    def setUp(self):
        try:
            self.db = connect_to_database()
        except Exception:
            self.db = None
        super().setUp()
"#;
    assert!(run(src).is_empty());
}

#[test]
fn setup_with_complex_initialization() {
    let src = r#"
class ConfigTest(TestCase):
    def setUp(self):
        self.config = {
            "debug": True,
            "timeout": 30,
        }
        self.logger = logging.getLogger(__name__)
        self.fixtures = self._load_fixtures()
        super().setUp()

    def _load_fixtures(self):
        return []
"#;
    assert!(run(src).is_empty());
}

#[test]
fn teardown_with_cleanup() {
    let src = r#"
class TemporaryFileTest(TestCase):
    def setUp(self):
        self.temp_dir = mkdtemp()
        super().setUp()

    def tearDown(self):
        rmtree(self.temp_dir)
        super().tearDown()
"#;
    assert!(run(src).is_empty());
}

#[test]
fn teardown_super_call_not_last() {
    let src = r#"
class TemporaryFileTest(TestCase):
    def tearDown(self):
        super().tearDown()
        rmtree(self.temp_dir)
"#;
    let errors = run(src);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("must be the last line"));
}

#[test]
fn mixed_valid_and_invalid_methods() {
    let src = r#"
class MixedTest(TestCase):
    def setUp(self):
        self.value = 42
        super().setUp()

    def tearDown(self):
        self.cleanup()
        super().tearDown()

    def test_something(self):
        pass

    def setup(self):
        pass
"#;
    let errors = run(src);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("correct casing"));
}

#[test]
fn async_setup_method() {
    let src = r#"
class AsyncTest(IsolatedAsyncioTestCase):
    async def asyncSetUp(self):
        self.client = AsyncClient()
        await super().asyncSetUp()
"#;
    // Note: async setup methods are allowed to have different names
    assert!(run(src).is_empty());
}

#[test]
fn setup_with_mocking() {
    let src = r#"
class MockTest(TestCase):
    def setUp(self):
        self.patcher = patch("module.function")
        self.mock_func = self.patcher.start()
        super().setUp()

    def tearDown(self):
        self.patcher.stop()
        super().tearDown()
"#;
    assert!(run(src).is_empty());
}

#[test]
fn nested_class_with_setup() {
    let src = r#"
class OuterTest(TestCase):
    def setUp(self):
        self.outer = True
        super().setUp()

    class InnerTest(TestCase):
        def setUp(self):
            self.inner = True
            super().setUp()
"#;
    assert!(run(src).is_empty());
}

#[test]
fn setup_with_multiple_errors() {
    let src = r#"
class MultiErrorTest(TestCase):
    def setup(self):
        self.value = 1

    def teardown(self):
        self.cleanup()
"#;
    let errors = run(src);
    assert_eq!(errors.len(), 2);
    assert!(errors.iter().any(|e| e.contains("correct casing")));
    assert!(errors.iter().all(|e| e.contains("correct casing")));
}

#[test]
fn setup_with_super_in_middle() {
    let src = r#"
class MiddleTest(TestCase):
    def setUp(self):
        self.value = 1
        super().setUp()
        self.value = 2
"#;
    let errors = run(src);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("must be the last line"));
}

#[test]
fn empty_setup_method() {
    let src = r#"
class EmptyTest(TestCase):
    def setUp(self):
        pass
"#;
    // Empty methods are allowed to not have super call
    assert!(run(src).is_empty());
}

#[test]
fn setup_with_only_super() {
    let src = r#"
class OnlySuperTest(TestCase):
    def setUp(self):
        super().setUp()
"#;
    assert!(run(src).is_empty());
}

#[test]
fn setup_with_return_before_super() {
    let src = r#"
class EarlyReturnTest(TestCase):
    def setUp(self):
        if self.skip_setup:
            return
        super().setUp()
"#;
    assert!(run(src).is_empty());
}

#[test]
fn setup_with_docstring() {
    let src = r#"
class DocstringTest(TestCase):
    def setUp(self):
        """Initialize test fixtures."""
        self.value = 42
        super().setUp()
"#;
    assert!(run(src).is_empty());
}

#[test]
fn teardown_with_docstring_and_super() {
    let src = r#"
class DocstringTeardownTest(TestCase):
    def tearDown(self):
        """Clean up test fixtures."""
        self.cleanup()
        super().tearDown()
"#;
    assert!(run(src).is_empty());
}

#[test]
fn setup_wrong_target_method() {
    let src = r#"
class WrongTargetTest(TestCase):
    def setUp(self):
        super().tearDown()
"#;
    let errors = run(src);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("must be the last line"));
}

#[test]
fn multiple_classes_all_valid() {
    let src = r#"
class TestOne(TestCase):
    def setUp(self):
        self.one = 1
        super().setUp()

    def tearDown(self):
        super().tearDown()

class TestTwo(TestCase):
    def setUp(self):
        self.two = 2
        super().setUp()

    def tearDown(self):
        super().tearDown()
"#;
    assert!(run(src).is_empty());
}

#[test]
fn multiple_classes_one_invalid() {
    let src = r#"
class TestOne(TestCase):
    def setUp(self):
        self.one = 1
        super().setUp()

class TestTwo(TestCase):
    def setUp(self):
        self.two = 2
"#;
    let errors = run(src);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("must be the last line"));
}

#[test]
fn decorator_on_setup() {
    let src = r#"
class DecoratorTest(TestCase):
    @skip("TODO: Implement")
    def setUp(self):
        super().setUp()
"#;
    assert!(run(src).is_empty());
}

#[test]
fn setup_with_conditional_logic() {
    let src = r#"
class ConditionalTest(TestCase):
    def setUp(self):
        if os.getenv("USE_REAL_DB"):
            self.db = RealDatabase()
        else:
            self.db = MockDatabase()
        super().setUp()
"#;
    assert!(run(src).is_empty());
}

#[test]
fn real_django_test_class() {
    let src = r#"
class UserCreateTestCase(TestCase):
    """Test user creation functionality."""

    def setUp(self):
        """Set up test fixtures."""
        self.user_data = {
            "username": "testuser",
            "email": "test@example.com",
            "password": "testpass123",
        }
        self.factory = RequestFactory()
        super().setUp()

    def tearDown(self):
        """Clean up after tests."""
        User.objects.all().delete()
        super().tearDown()

    def test_user_creation(self):
        """Test that a user can be created."""
        user = User.objects.create(**self.user_data)
        self.assertEqual(user.username, "testuser")
"#;
    assert!(run(src).is_empty());
}

#[test]
fn real_django_test_with_wrong_casing() {
    let src = r#"
class UserCreateTestCase(TestCase):
    """Test user creation functionality."""

    def setup(self):
        """Set up test fixtures."""
        self.user_data = {
            "username": "testuser",
            "email": "test@example.com",
        }
        super().setUp()

    def tearDown(self):
        """Clean up after tests."""
        User.objects.all().delete()
        super().tearDown()
"#;
    let errors = run(src);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("correct casing"));
}

#[test]
fn real_unittest_with_missing_super() {
    let src = r#"
class CalculatorTestCase(unittest.TestCase):
    """Test calculator functionality."""

    def setUp(self):
        """Initialize calculator."""
        self.calc = Calculator()
        self.test_values = [1, 2, 3, 4, 5]

    def test_add(self):
        """Test addition."""
        result = self.calc.add(2, 3)
        self.assertEqual(result, 5)
"#;
    let errors = run(src);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("super().setUp()"));
}

#[test]
fn real_pytest_unittest_adapter() {
    let src = r#"
class TestDataProcessing(unittest.TestCase):
    """Test data processing with pytest."""

    def setUp(self):
        """Load test data."""
        self.data = load_fixture("data.json")
        self.processor = DataProcessor()
        super().setUp()

    def tearDown(self):
        """Cleanup."""
        self.processor.cleanup()
        super().tearDown()

    def test_process_valid_data(self):
        """Test processing valid data."""
        result = self.processor.process(self.data)
        self.assertIsNotNone(result)
"#;
    assert!(run(src).is_empty());
}

#[test]
fn inheritance_chain() {
    let src = r#"
class BaseTestCase(TestCase):
    def setUp(self):
        self.base = True
        super().setUp()

class MiddleTestCase(BaseTestCase):
    def setUp(self):
        self.middle = True
        super().setUp()

class FinalTestCase(MiddleTestCase):
    def setUp(self):
        self.final = True
        super().setUp()
"#;
    assert!(run(src).is_empty());
}

#[test]
fn inheritance_chain_with_error() {
    let src = r#"
class BaseTestCase(TestCase):
    def setUp(self):
        self.base = True
        super().setUp()

class MiddleTestCase(BaseTestCase):
    def setUp(self):
        self.middle = True

class FinalTestCase(MiddleTestCase):
    def setUp(self):
        self.final = True
        super().setUp()
"#;
    let errors = run(src);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("must be the last line"));
}

#[test]
fn setup_with_resource_allocation() {
    let src = r#"
class ResourceTest(TestCase):
    def setUp(self):
        """Allocate resources."""
        self.lock = threading.Lock()
        self.queue = queue.Queue()
        self.thread = threading.Thread(target=self._worker)
        self.thread.start()
        super().setUp()

    def tearDown(self):
        """Release resources."""
        self.thread.join()
        super().tearDown()

    def _worker(self):
        """Worker thread."""
        pass
"#;
    assert!(run(src).is_empty());
}
