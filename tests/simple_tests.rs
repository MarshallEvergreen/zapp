use super_duper_octo_lamp::entry::abinit;
use vfs::{MemoryFS, SeekAndWrite, VfsPath};

#[test]
fn test_simple_setup() {
    // Arrange
    let memfs: VfsPath = MemoryFS::new().into();

    let mut writer: Box<dyn SeekAndWrite + Send> = memfs
        .join("hello_world.py")
        .expect("Failed to create file path")
        .create_file()
        .expect("Failed to create file");

    let python_hello_world: &str = r#"
def hello_world():
    print("Hello World!")
"#;

    writer
        .write_all(python_hello_world.as_bytes())
        .expect("Failed to write to file");

    // Act

    abinit(Some(&memfs));

    // Assert

    let expected_interface: &str = r#"
from .hello_world import hello_world
"#;

    // Read the file back to verify its contents
    let mut reader = memfs
        .join("__init__.py")
        .expect("Failed to create path")
        .open_file()
        .expect("Failed to open file");

    let mut contents = String::new();
    reader
        .read_to_string(&mut contents)
        .expect("Failed to read file");

    assert_eq!(contents, expected_interface);
}
