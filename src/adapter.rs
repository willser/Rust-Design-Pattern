use std::io::Error;

struct DbConnect;

type Result<T> = std::result::Result<T,Error>;

impl DbConnect {
    fn select(&self) -> Option<String> {
        Some(String::from("sql result"))
    }

    fn insert(&self, _sql: &str) -> Result<String> {
        Ok(String::from("insert success"))
    }

    fn update(&self, _sql: &str) -> Result<i32> {
        Ok(1)
    }

    fn delete(&self, _sql: &str) -> Result<i32> {
        Ok(1)
    }
}

struct MyDbConnect {
    db_connect: DbConnect
}

impl MyDbConnect {
    fn new() -> Self {
        MyDbConnect {
            db_connect:DbConnect{}
        }
    }

    fn select(&self) -> Option<String> {
        self.db_connect.select()
    }

    fn insert(&self, sql: &str) -> Result<String> {
        self.db_connect.insert(sql)
    }

    fn update(&self, sql: &str) -> Result<i32> {
        self.db_connect.update(sql)
    }

    fn delete(&self, sql: &str) -> Result<i32> {
        self.db_connect.delete(sql)
    }
}

#[test]
fn test(){
    let connect = MyDbConnect::new();

    if let Some(temp) =  connect.select(){
        assert_eq!(temp,"sql result")
    }

    if let Ok(temp) = connect.insert(""){
        assert_eq!(temp,"insert success")
    }

    if let Ok(temp) = connect.update(""){
        assert_eq!(temp,1)
    }

    if let Ok(temp) = connect.delete(""){
        assert_eq!(temp,1)
    }
}


