use std::ops::Deref;

pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    pub fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub fn deref_demo() {
    let x = 5;
    let y = &x;
    let z = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    assert_eq!(5, *z);
}

// Because MyBox implements Deref, the compiler can automatically convert
// &MyBox<String> -> &String -> &str, through deref coercion.
pub fn implicit_deref_demo() {
    let m = MyBox::new(String::from("Adam"));

    hello(&m);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}
