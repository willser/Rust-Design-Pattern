
use std::io::Error;

type Result<T> = std::result::Result<T,Error>;

trait Log {
    fn save_log(&self, log: &str) -> Result<String>;
}

trait DbConnect {
    fn insert(&self, str: &str) -> Result<String>;
}

mod impl_log{

    use super::{DbConnect,Result,Log};

    pub struct EventLog {
        db_connect: Box<dyn DbConnect>
    }

    impl EventLog {
        pub fn new(db_connect: Box<dyn DbConnect>) -> Self {
            EventLog { db_connect }
        }
    }

    impl Log for EventLog {
        fn save_log(&self, log: &str) -> Result<String> {
            self.db_connect.insert(log)
        }
    }

    pub struct IPSLog {
        db_connect: Box<dyn DbConnect>
    }

    impl IPSLog {
        pub fn new(db_connect: Box<dyn DbConnect>) -> Self {
            IPSLog { db_connect }
        }
    }

    impl Log for IPSLog {
        fn save_log(&self, log: &str) -> Result<String> {
            self.db_connect.insert(log)
        }
    }
}

mod impl_db_connect{
    use super::{DbConnect,Result};

    pub struct MysqlConnect{}

    impl MysqlConnect{
        pub fn new() -> Self{
            MysqlConnect{}
        }
    }

    impl DbConnect for MysqlConnect{
        fn insert(&self, _str: &str) -> Result<String> {
            Ok(String::from("insert log to mysql"))
        }
    }

    pub struct PostgreSqlConnect{}
    impl PostgreSqlConnect{
        pub fn new() -> Self{
            PostgreSqlConnect{}
        }
    }

    impl DbConnect for PostgreSqlConnect{
        fn insert(&self, _str: &str) -> Result<String> {
            Ok(String::from("insert log to postgresql"))
        }
    }
}

#[test]
fn test(){
    let mysql_connect = impl_db_connect::MysqlConnect::new();
    let postgresql_connect = impl_db_connect::PostgreSqlConnect::new();

    if let Ok(result) =  impl_log::EventLog::new(Box::new(mysql_connect)).save_log(""){
        assert_eq!(result,"insert log to mysql")
    }else{
        panic!("error")
    }

    if let Ok(result) =  impl_log::IPSLog::new(Box::new(postgresql_connect)).save_log(""){
        assert_eq!(result,"insert log to postgresql")
    }else{
        panic!("error")
    }
}

