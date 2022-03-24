use std::{path, fs::OpenOptions};

trait FileMetadata {
    fn exists(&self) -> bool;

    fn is_writeable(&self) -> bool;

    fn is_readable(&self) -> bool;
}

impl FileMetadata for path::Path {
    fn is_readable(&self) -> bool {
        let file = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(self);
        file.is_ok()
    }

    fn is_writeable(&self) -> bool {
        if let Ok(attr) = self.metadata() {
            !attr.permissions().readonly()
        } else {
            false
        }

        // Or, shorter:
        // match self.metadata() {
        //     Ok(attr) => !attr.permissions().readonly(),
        //     Err(_) => false,
        // }
    }

    fn exists(&self) -> bool {
        self.exists()
    }
}

fn main() {
    // 
}

#[test]
fn writeable() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    assert!(f.path().is_writeable());

    fs::remove_file(f.path()).unwrap();
}

#[test]
fn read_only() {
    use std::fs;
    use tempfile;

    let f = tempfile::NamedTempFile::new().unwrap();
    let mut perms = fs::metadata(f.path()).unwrap().permissions();
    perms.set_readonly(true);
    fs::set_permissions(f.path(), perms).unwrap();
    assert_eq!(f.path().is_writeable(), false);

    fs::remove_file(f.path()).unwrap();
}
