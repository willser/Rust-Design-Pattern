use std::io::{Error, ErrorKind};
// #[macro_use]
use crate::error_contains;

struct File {
    file_type: FileType,
    subdirectory: Vec<File>,
}

#[derive(Copy,Clone,PartialEq,Debug)]
enum FileType {
    FOLDER,
    FILE,
}

impl File {
    fn new(path: &str) -> Self {
        File {
            file_type: FileType::FOLDER,
            subdirectory: vec![],
        }
    }

    fn read_string(&self) -> Result<String, Error> {
        match self.file_type {
            FileType::FILE => {
                Ok(String::from("success"))
            }
            FileType::FOLDER => {
                Err(Error::new(ErrorKind::Other, String::from("error")))
            }
        }
    }

    fn file_type(&self) -> FileType {
        self.file_type
    }
}


#[test]
fn test() {
    let file = File::new("");

    let x = file.file_type();

    assert_eq!(x,FileType::FOLDER);

    let string = file.read_string();

    error_contains!(string,"error")

}