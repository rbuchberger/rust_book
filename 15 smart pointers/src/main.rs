use smart_pointers::linked_list;

fn main() {
    linked_list::demo();

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
