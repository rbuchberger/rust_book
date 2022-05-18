use crate::linked_list::LinkedList::{Cons, Nil};

#[derive(Debug)]
pub enum LinkedList {
    Cons(i32, Box<LinkedList>),
    Nil,
}

impl LinkedList {
    pub fn new(vals: Vec<i32>) -> LinkedList {
        vals.iter()
            .rev()
            .fold(Nil, |list, item| Cons(*item, Box::new(list)))
    }
}

pub fn demo() {
    // let b = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let b = LinkedList::new(vec![1, 2, 3]);

    println!("{:#?}", b);
}
