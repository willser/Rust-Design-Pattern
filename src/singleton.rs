use std::fmt::{Formatter, Pointer, Error};
use std::ptr::replace;

#[derive(Debug)]
struct Singleton;

impl Pointer for Singleton{


    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let ptr = self as *const Self;
        Pointer::fmt(&ptr, f)
    }
}

struct SingletonCreator{
    singleton: Option<Singleton>,
}

impl SingletonCreator{
    fn get_singleton(&mut self) -> Option<Singleton> {
        unsafe {
            replace(&mut self.singleton, None)
        }
    }
}

static mut SINGLETON_CREATOR: SingletonCreator = SingletonCreator {
    singleton: Some(Singleton),
};

#[test]
fn test(){

    unsafe {
        let singleton = SINGLETON_CREATOR.get_singleton();

        println!("{:p}",singleton.unwrap());

        let option = SINGLETON_CREATOR.get_singleton();
        if let None = option{
            println!("none");
        }
    }

}
