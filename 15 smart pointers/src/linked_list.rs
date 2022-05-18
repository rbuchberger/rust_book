use crate::linked_list::LinkedList::{Nil, Pair};
use std::rc::Rc;

#[derive(Debug)]
pub enum LinkedList {
    Pair(i32, Rc<LinkedList>),
    Nil,
}

impl LinkedList {
    pub fn new(vals: Vec<i32>) -> LinkedList {
        vals.iter()
            .rev()
            .fold(Nil, |list, item| Pair(*item, Rc::new(list)))
    }
}

pub fn recursion_demo() {
    // let b = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    let b = LinkedList::new(vec![1, 2, 3]);

    println!("{:#?}", b);
}

pub fn rc_demo() {
    let a = Rc::new(LinkedList::new(vec![5, 10]));
    println!("Count after creating a = {}", Rc::strong_count(&a));

    let _b = Pair(3, Rc::clone(&a));
    println!("Count after creating b = {}", Rc::strong_count(&a));

    {
        let _c = Pair(4, Rc::clone(&a));
        println!("Count after creating c = {}", Rc::strong_count(&a));
    }

    println!("Count after c is dropped = {}", Rc::strong_count(&a));
}
