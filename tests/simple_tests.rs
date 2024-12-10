use std::default;

use vfs::{MemoryFS, PhysicalFS, SeekAndWrite, VfsPath};

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
}

fn abinit(fs: Option<&VfsPath>) {
    let default_fs: VfsPath = PhysicalFS::new("/").into();
    let _fs = fs.unwrap_or(&default_fs);
}
