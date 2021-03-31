use std::iter::FromIterator;

trait Criteria<T> {
    fn filter(&self, vec: &Vec<T>) -> Vec<T>;
}

#[derive(Debug)]
struct Node {
    string: String,
    int: i32,
}

impl Node {
    fn new(string: &str, int: i32) -> Self {
        Node { string: String::from(string), int }
    }
}


impl<'a> FromIterator<&'a Node> for Vec<Node> {
    fn from_iter<I: IntoIterator<Item=&'a Node>>(iter: I) -> Vec<Node> {
        let mut map = iter.into_iter().map(|x| -> Node{
            let x: &Node = x;
            Node { string: x.string.clone(), int: x.int }
        });
        let mut vec = Vec::new();
        loop {
            if let Some(option) = map.next(){
                vec.push(option);
            }else{
                break;
            }
        }
        vec
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string && self.int == other.int
    }
}

struct StringCriteria {}

impl StringCriteria {
    fn new() -> Self {
        StringCriteria {}
    }
}

impl Criteria<Node> for StringCriteria {
    fn filter(&self, vec: &Vec<Node>) -> Vec<Node> {
        vec.iter().filter(|x| x.string != "").collect::<Vec<Node>>()
    }
}

struct IntCriteria {}

impl IntCriteria {
    fn new() -> Self {
        IntCriteria {}
    }
}

impl Criteria<Node> for IntCriteria {
    fn filter(&self, vec: &Vec<Node>) -> Vec<Node> {
        vec.iter().filter(|x| x.int != 0).collect::<Vec<Node>>()
    }
}

struct AndCriteria {
    criteria: Box<dyn Criteria<Node>>,
    criteria_other: Box<dyn Criteria<Node>>,
}

impl AndCriteria {
    fn new(criteria: impl Criteria<Node> + 'static, criteria_other: impl Criteria<Node> + 'static) -> Self {
        AndCriteria {
            criteria: Box::new(criteria),
            criteria_other: Box::new(criteria_other),
        }
    }
}

impl Criteria<Node> for AndCriteria {
    fn filter(&self, vec: &Vec<Node>) -> Vec<Node> {
        let vec_1 = self.criteria.filter(vec);
        let vec_2 = self.criteria_other.filter(vec);
        vec_1.iter().filter(|x| vec_2.contains(x)).collect::<Vec<Node>>()
    }
}

#[test]
fn test() {
    let mut vec = Vec::new();

    vec.push(Node::new("q", 1));
    vec.push(Node::new("", 2));
    vec.push(Node::new("w", 0));
    vec.push(Node::new("z", 3));

    let criteria = AndCriteria::new(StringCriteria::new(), IntCriteria::new());
    let vec = criteria.filter(&vec);
    assert_eq!(vec[0], Node::new("q", 1));
    assert_eq!(vec[1], Node::new("z", 3));
}

