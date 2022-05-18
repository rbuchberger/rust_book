use smart_pointers::linked_list;
use smart_pointers::my_box;

fn main() {
    linked_list::demo();
    my_box::deref_demo();
    my_box::implicit_deref_demo();
}
