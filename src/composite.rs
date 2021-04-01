use std::io::{Error, ErrorKind};

struct File {
    file_type: FileType,
    subdirectory: Vec<File>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum FileType {
    FOLDER,
    FILE,
}

impl File {
    fn new(_path: &str) -> Self {
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

    fn get_subdirectory(&self) -> Result<&Vec<File>, Error> {
        match self.file_type {
            FileType::FOLDER => {
                Ok(&self.subdirectory)
            }
            FileType::FILE => {
                Err(Error::new(ErrorKind::Other, String::from("error")))
            }
        }
    }
}


#[test]
fn test() {
    let file = File::new("");

    let x = file.file_type();
    let string = file.read_string();
    let result = file.get_subdirectory();

    assert_eq!(x, FileType::FOLDER);

    error_contains!(string,"error");

    ok!(result,(|v:&Vec<File>| assert_eq!(v.len(),0)));
}