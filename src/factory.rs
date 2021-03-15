

trait Animal {
    fn name(&self) -> &str;

    fn weight(&self) -> i32;
}


struct Duck {
    weight: i32
}

impl Animal for Duck {
    fn name(&self) -> &str {
        "duck"
    }

    fn weight(&self) -> i32 {
        self.weight
    }
}

struct Cat {
    weight: i32
}

impl Animal for Cat {
    fn name(&self) -> &str {
        "cat"
    }
    fn weight(&self) -> i32 {
        self.weight
    }
}

struct AnimalFactory {}

impl AnimalFactory {
    fn get_animal(animal_type: AnimalType) -> Box<dyn Animal> {
        match animal_type {
            AnimalType::DUCK => {
                Box::new(Duck { weight: 10 })
            }
            AnimalType::CAT => {
                Box::new(Cat { weight: 20 })
            }
        }
    }
}

enum AnimalType {
    DUCK,
    CAT,
}

#[test]
fn test() {
    let animal = AnimalFactory::get_animal(AnimalType::DUCK);
    assert_eq!(animal.name(), "duck");

    let animal = AnimalFactory::get_animal(AnimalType::CAT);
    assert_eq!(animal.name(), "cat");
}
