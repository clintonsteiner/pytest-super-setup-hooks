use std::fs::write;
use tempfile::TempDir;
use pytest_super_hooks::{check_file, fix::fix_file};

/// Test suite for the --fix functionality

fn run_fix(src: &str) -> (Vec<String>, String) {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join("test.py");
    write(&file_path, src).unwrap();

    // Fix the file
    fix_file(&file_path);

    // Check for remaining errors
    let errors = check_file(&file_path);

    // Read the fixed content
    let fixed = std::fs::read_to_string(&file_path).unwrap();

    (errors, fixed)
}

#[test]
fn fix_missing_super_in_setup() {
    let src = r#"class T:
    def setUp(self):
        self.value = 1"#;

    let (errors, fixed) = run_fix(src);

    assert!(errors.is_empty());
    assert!(fixed.contains("super().setUp()"));
    assert!(fixed.contains("self.value = 1"));
}

#[test]
fn fix_missing_super_in_teardown() {
    let src = r#"class T:
    def tearDown(self):
        self.cleanup()"#;

    let (errors, fixed) = run_fix(src);

    assert!(errors.is_empty());
    assert!(fixed.contains("super().tearDown()"));
    assert!(fixed.contains("self.cleanup()"));
}

#[test]
fn fix_wrong_casing_setup() {
    let src = r#"class T:
    def setup(self):
        self.x = 1
        super().setUp()"#;

    let (errors, fixed) = run_fix(src);

    assert!(errors.is_empty());
    assert!(fixed.contains("def setUp(self):"));
}

#[test]
fn fix_wrong_casing_teardown() {
    let src = r#"class T:
    def teardown(self):
        self.cleanup()
        super().tearDown()"#;

    let (errors, fixed) = run_fix(src);

    assert!(errors.is_empty());
    assert!(fixed.contains("def tearDown(self):"));
}

#[test]
fn fix_super_not_last() {
    let src = r#"class T:
    def setUp(self):
        super().setUp()
        self.value = 1"#;

    let (errors, fixed) = run_fix(src);

    assert!(errors.is_empty());
    assert!(fixed.contains("self.value = 1"));
    assert!(fixed.contains("super().setUp()"));
    // The super call should be after the value assignment
    let lines: Vec<&str> = fixed.lines().collect();
    let super_idx = lines.iter().position(|l| l.contains("super().setUp()")).unwrap();
    let value_idx = lines.iter().position(|l| l.contains("self.value = 1")).unwrap();
    assert!(super_idx > value_idx);
}

#[test]
fn fix_multiple_methods() {
    let src = r#"class T:
    def setUp(self):
        self.a = 1

    def tearDown(self):
        self.cleanup()"#;

    let (errors, fixed) = run_fix(src);

    assert!(errors.is_empty());
    assert!(fixed.contains("super().setUp()"));
    assert!(fixed.contains("super().tearDown()"));
}

#[test]
fn fix_preserves_other_code() {
    let src = r#"class TestExample:
    def setUp(self):
        self.db = Database()
        self.logger = Logger()

    def test_something(self):
        """Test method."""
        assert True

    def tearDown(self):
        pass"#;

    let (errors, fixed) = run_fix(src);

    assert!(errors.is_empty());
    assert!(fixed.contains("class TestExample:"));
    assert!(fixed.contains("def test_something(self):"));
    assert!(fixed.contains("\"\"\"Test method.\"\"\""));
    assert!(fixed.contains("assert True"));
    assert!(fixed.contains("self.db = Database()"));
    assert!(fixed.contains("self.logger = Logger()"));
    assert!(fixed.contains("super().setUp()"));
    assert!(fixed.contains("super().tearDown()"));
}

#[test]
fn fix_indentation_preserved() {
    let src = r#"class TestClass:
    def setUp(self):
        x = 1
        y = 2
        z = 3"#;

    let (errors, fixed) = run_fix(src);

    assert!(errors.is_empty());
    // Check that indentation is preserved
    let lines: Vec<&str> = fixed.lines().collect();
    let super_line = lines.iter().find(|l| l.contains("super()")).unwrap();
    assert!(super_line.starts_with("        ")); // 8 spaces like other lines
}

#[test]
fn fix_handles_complex_setup() {
    let src = r#"class ComplexTest:
    def setUp(self):
        self.config = {
            "debug": True,
            "timeout": 30,
        }
        self.items = [1, 2, 3]
        self.result = self.initialize()"#;

    let (errors, fixed) = run_fix(src);

    assert!(errors.is_empty());
    assert!(fixed.contains("super().setUp()"));
    assert!(fixed.contains("\"debug\": True"));
    assert!(fixed.contains("\"timeout\": 30"));
    assert!(fixed.contains("[1, 2, 3]"));
    assert!(fixed.contains("self.initialize()"));
}

#[test]
fn fix_multiple_classes() {
    let src = r#"class TestOne:
    def setUp(self):
        self.one = 1

class TestTwo:
    def setUp(self):
        self.two = 2

    def tearDown(self):
        pass"#;

    let (errors, fixed) = run_fix(src);

    assert!(errors.is_empty());
    // Both should have super calls
    let setup_count = fixed.matches("super().setUp()").count();
    let teardown_count = fixed.matches("super().tearDown()").count();
    assert_eq!(setup_count, 2);
    assert_eq!(teardown_count, 1);
}

#[test]
fn fix_mixed_issues() {
    let src = r#"class MixedTest:
    def setup(self):
        self.prepare()

    def teardown(self):
        super().tearDown()
        self.cleanup()"#;

    let (errors, fixed) = run_fix(src);

    assert!(errors.is_empty());
    // setup should be renamed to setUp
    assert!(fixed.contains("def setUp(self):"));
    assert!(!fixed.contains("def setup(self):"));

    // teardown should be renamed to tearDown
    assert!(fixed.contains("def tearDown(self):"));
    assert!(!fixed.contains("def teardown(self):"));

    // setUp should have super() last
    let lines: Vec<&str> = fixed.lines().collect();
    let setup_idx = lines.iter().position(|l| l.contains("def setUp")).unwrap();
    let setup_end = lines[setup_idx..].iter().position(|l| l.contains("def ")).unwrap_or(lines.len() - setup_idx) + setup_idx;
    let setup_section = &lines[setup_idx..setup_end];
    let last_line = setup_section.last().unwrap();
    assert!(last_line.contains("super().setUp()"));
}

#[test]
fn fix_docstring_preserved() {
    let src = r#"class DocTest:
    def setUp(self):
        """Initialize fixtures."""
        self.fixture = create_fixture()"#;

    let (errors, fixed) = run_fix(src);

    assert!(errors.is_empty());
    assert!(fixed.contains("\"\"\"Initialize fixtures.\"\"\""));
    assert!(fixed.contains("super().setUp()"));
}

#[test]
fn fix_with_comments() {
    let src = r#"class CommentTest:
    def setUp(self):
        # Initialize
        self.x = 1  # important
        # More setup
        self.y = 2"#;

    let (errors, fixed) = run_fix(src);

    assert!(errors.is_empty());
    assert!(fixed.contains("# Initialize"));
    assert!(fixed.contains("# important"));
    assert!(fixed.contains("# More setup"));
    assert!(fixed.contains("super().setUp()"));
}

#[test]
fn fix_multiline_statements() {
    let src = r#"class MultilineTest:
    def setUp(self):
        self.data = {
            "key1": "value1",
            "key2": "value2",
            "key3": "value3",
        }
        self.list = [
            "item1",
            "item2",
            "item3",
        ]"#;

    let (errors, fixed) = run_fix(src);

    assert!(errors.is_empty());
    assert!(fixed.contains("super().setUp()"));
    assert!(fixed.contains("\"key1\": \"value1\""));
    assert!(fixed.contains("\"key3\": \"value3\""));
    assert!(fixed.contains("\"item1\""));
    assert!(fixed.contains("\"item3\""));
}

#[test]
fn fix_empty_method_unchanged() {
    let src = r#"class EmptyTest:
    def setUp(self):
        pass"#;

    let (errors, fixed) = run_fix(src);

    assert!(errors.is_empty());
    assert!(fixed.contains("def setUp(self):"));
    assert!(fixed.contains("pass"));
    // Empty methods don't need super call added
    assert!(!fixed.contains("super().setUp()"));
}

#[test]
fn fix_real_world_django_test() {
    let src = r#"class UserModelTestCase(TestCase):
    def setUp(self):
        """Create test user."""
        self.user = User.objects.create_user(
            username="testuser",
            email="test@example.com",
            password="testpass123",
        )
        self.profile = UserProfile.objects.create(
            user=self.user,
            bio="Test bio",
        )

    def tearDown(self):
        """Clean up."""
        self.user.delete()
        self.profile.delete()"#;

    let (errors, fixed) = run_fix(src);

    assert!(errors.is_empty());
    assert!(fixed.contains("super().setUp()"));
    assert!(fixed.contains("super().tearDown()"));
    assert!(fixed.contains("username=\"testuser\""));
    assert!(fixed.contains("Test bio"));
}

#[test]
fn fix_only_fixes_setup_teardown() {
    let src = r#"class OnlyOthersTest:
    def helper_setup(self):
        self.value = 1

    def setup_data(self):
        self.data = []

    def test_something(self):
        pass"#;

    let (errors, fixed) = run_fix(src);

    assert!(errors.is_empty());
    // These methods should NOT be affected
    assert!(fixed.contains("def helper_setup(self):"));
    assert!(fixed.contains("def setup_data(self):"));
    assert!(fixed.contains("def test_something(self):"));
    // No super() should be added
    assert!(!fixed.contains("super().setUp()"));
}
