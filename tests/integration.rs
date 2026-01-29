// tests/integration.rs
use pytest_super_hooks::check_file;
use std::fs::write;
use tempfile::TempDir;

#[test]
fn valid_teardown() {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join("test.py");
    write(
        &file_path,
        r#"
class T:
    def tearDown(self):
        x = 1
        super().tearDown()
"#,
    )
    .unwrap();
    assert!(check_file(&file_path).is_empty());
}

#[test]
fn invalid_teardown() {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join("test.py");
    write(
        &file_path,
        r#"
class T:
    def tearDown(self):
        super().tearDown()
        x = 1
"#,
    )
    .unwrap();
    assert_eq!(check_file(&file_path).len(), 1);
}
