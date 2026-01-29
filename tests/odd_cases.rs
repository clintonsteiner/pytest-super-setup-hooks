// tests/weird_cases.rs
use pytest_super_hooks::check_file;
use std::fs::write;
use tempfile::TempDir;

fn run(src: &str) -> Vec<String> {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join("test.py");
    write(&file_path, src).unwrap();
    check_file(&file_path)
}

#[test]
fn empty_class() {
    assert!(run("class T:\n    pass\n").is_empty());
}

#[test]
fn setup_only_valid() {
    assert!(run("class T:\n    def setUp(self):\n        super().setUp()\n").is_empty());
}

#[test]
fn teardown_only_valid() {
    assert!(run("class T:\n    def tearDown(self):\n        super().tearDown()\n").is_empty());
}

#[test]
fn setup_missing_super() {
    assert_eq!(
        run("class T:\n    def setUp(self):\n        x=1\n").len(),
        1
    );
}

#[test]
fn teardown_missing_super() {
    assert_eq!(
        run("class T:\n    def tearDown(self):\n        x=1\n").len(),
        1
    );
}

#[test]
fn setup_super_not_last() {
    assert_eq!(
        run("class T:\n    def setUp(self):\n        super().setUp()\n        x=1\n").len(),
        1
    );
}

#[test]
fn teardown_super_not_last() {
    assert_eq!(
        run("class T:\n    def tearDown(self):\n        super().tearDown()\n        x=1\n").len(),
        1
    );
}

#[test]
fn bad_setup_casing() {
    assert_eq!(
        run("class T:\n    def setup(self):\n        pass\n").len(),
        1
    );
}

#[test]
fn bad_teardown_casing() {
    assert_eq!(
        run("class T:\n    def teardown(self):\n        pass\n").len(),
        1
    );
}

#[test]
fn mixed_good_and_bad() {
    assert_eq!(
        run(
            "class T:\n    def setUp(self):\n        super().setUp()\n    def teardown(self):\n        super().tearDown()\n"
        )
        .len(),
        1
    );
}

#[test]
fn nested_function_ignored() {
    assert!(run("class T:\n\
         def setUp(self):\n\
             def inner(): pass\n\
             super().setUp()\n")
    .is_empty());
}

#[test]
fn async_setup() {
    assert!(run("class T:\n\
         async def setUp(self):\n\
             super().setUp()\n")
    .is_empty());
}

#[test]
fn decorator_on_teardown() {
    assert!(run("class T:\n\
         @foo\n\
         def tearDown(self):\n\
             super().tearDown()\n")
    .is_empty());
}

#[test]
fn wrong_super_target() {
    assert_eq!(
        run("class T:\n    def tearDown(self):\n        super().setUp()\n").len(),
        1
    );
}

#[test]
fn attribute_after_super_call() {
    assert_eq!(
        run("class T:\n    def tearDown(self):\n        super().tearDown()\n        self.x = 1\n")
            .len(),
        1
    );
}

#[test]
fn multiple_classes() {
    assert_eq!(
        run(
            "class A:\n    def tearDown(self):\n        super().tearDown()\n    class B:\n        def setup(self): pass\n"
        )
        .len(),
        1
    );
}

#[test]
fn unrelated_function_named_teardown() {
    assert_eq!(run("def teardown():\n    pass\n").len(), 1);
}

#[test]
fn docstring_before_super() {
    assert!(run("class T:\n\
         def tearDown(self):\n\
             \"doc\"\n\
             super().tearDown()\n")
    .is_empty());
}

#[test]
fn return_after_super() {
    assert_eq!(
        run("class T:\n    def setUp(self):\n        super().setUp()\n        return\n").len(),
        1
    );
}

#[test]
fn comments_only_after_super() {
    assert!(run("class T:\n\
         def tearDown(self):\n\
             super().tearDown()\n\
             # comment\n")
    .is_empty());
}
