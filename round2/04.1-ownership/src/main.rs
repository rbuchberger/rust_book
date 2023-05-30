fn main() {
    // Ownership is about memory management in order to make undefined behavior
    // impossible. It ensures that memory is managed correctly and that we do
    // not get undefined behavior, with a minimum of runtime cost.

    // Variables the stack
    // ---------------------------------------------------------
    // - Variables live on the stack, in "frames". A frame maps variables to
    //   values in a single scope
    // - It's called the stack because the most recent frame is on top, and it
    //   will be the first one to be removed when the function exits. Think
    //   "call stack."
    // - The stack is faster, and you don't have to worry about ownership.

    let a = 42; // a is on the main stack frame
    let b = plus_one(a); // a is copied to x, which is on the plus_one stack
                         // frame.

    println!("a = {}, b = {}", a, b); // Note this wouldn't work with refs.

    // Boxes and the heap
    // ---------------------------------------------------------
    // - To access data without copying it, we use pointers.
    // - Pointers are values that describe a location in memory.
    // - A common pattern is to allocate memory on the heap, and create a
    //   pointer to it. Heap data can live indefinitely, and is not tied to a
    //   specific frame.
    //
    // Box deallocation principle:
    // If a variable owns a box, when Rust deallocates the variable's frame,
    // then rust deallocates the box's heap memory.

    // c is a pointer to a large array on the heap. c owns the box.
    let c = Box::new([0; 1_000_000]);

    // With this assignment, d points to the same value without copying it.
    // Ownership is 'moved' to d, c is now freed and cannot be used again.
    let _d = c;
    // When d goes out of scope, the box will be deallocated.

    // Cloning to avoid moves:
    let firstname = String::from("Robert");
    // By cloning the string, we create a new box on the heap, and ownership
    // stays with firstname (allowing us to use it later.)
    // Clone is a deep copy.
    let fullname = add_suffix(firstname.clone());

    // This line would not work, because fullname now owns the box
    println!("Hello, {}, originally {}", fullname, firstname);
}

fn plus_one(x: isize) -> isize {
    x + 1
}

fn add_suffix(mut name: String) -> String {
    name.push_str("'); DROP TABLE Students;--");
    name
}
