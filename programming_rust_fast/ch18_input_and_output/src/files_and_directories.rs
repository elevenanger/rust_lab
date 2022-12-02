use std::ffi::OsStr;
use std::io;
use std::path::Path;

#[test]
fn test_path_new() {
    let path = Path::new("Cargo.lock");
    println!("{}", path.display());
}

#[test]
fn test_path_parent() {
    let path = Path::new("/Users/liuanglin/Projects/rust_lab");
    assert_eq!(path.parent(), Some(Path::new("/Users/liuanglin/Projects")))
}

#[test]
fn test_path_file_name() {
    let path = Path::new("/Users/liuanglin/data/test.dat");
    assert_eq!(path.file_name(), Some(OsStr::new("test.dat")));
}

#[test]
fn test_path_is_absolute_and_is_relative() {
    let path = Path::new("Cargo.lock");
    assert!(!path.is_absolute());
    assert!(path.is_relative())
}

#[test]
fn test_path_join() -> io::Result<()> {
    let path = Path::new("/foo");
    assert_eq!(path.join("bar"), Path::new("/foo/bar"));

    let abs_path = std::env::current_dir()?;
    println!("{}", abs_path.display());
    Ok(())
}

#[test]
fn path_components() -> io::Result<()> {
    let abs_path = std::env::current_dir()?;
    let components = abs_path.components();

    println!("{:?}", components);
    Ok(())
}

#[test]
fn path_ancestors() {
    let abs_path = std::env::current_dir().unwrap();
    abs_path.ancestors().for_each(
        |path| println!("{}", path.display())
    )
}

#[test]
fn path_to_string() {
    let abs_path = std::env::current_dir().unwrap();
    println!("{}", abs_path.to_str().unwrap())
}

#[test]
fn path_to_string_lossy() {
    let abs_path = std::env::current_dir().unwrap();
    println!("{}", abs_path.to_string_lossy())
}

/// 使用 read_all 列出目录中所有的内容
fn read_all(path: &Path) {
    for entry_result in path.read_dir().unwrap() {
        let entry = entry_result.unwrap();
        if entry.path().is_dir() {
            read_all(entry.path().as_path())
        } else {
            println!("{}", entry.path().to_string_lossy());
            println!("metadata => {:?}", entry.metadata())
        }
    }
}

#[test]
fn test_read_dir() {
    let path = Path::new("./src");
    read_all(&path)
}
