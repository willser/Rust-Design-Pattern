struct App {
    path: Vec<String>,

    cors: bool,

}

impl App {
    fn new() -> App {
        App {
            path: vec![],
            cors: false,
        }
    }

    fn cors(mut self, cors: bool) -> Self {
        self.cors = cors;
        self
    }

    fn path(mut self, path: &str) -> Self {
        self.path.push(path.to_string());
        self
    }

    fn path_vec(mut self, path: Vec<String>) -> Self {
        self.path.extend(path);
        self
    }
}

#[test]
fn test() {
    let app = App::new().path("/path0").path_vec(vec!["/path1".to_string()]).cors(true);

    assert_eq!(app.cors,true);
    assert_eq!(app.path[0],"/path0");
    assert_eq!(app.path[1],"/path1");

}