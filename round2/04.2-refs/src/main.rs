fn main() {
    // Move-only APIs are inconvenient as hell. References solve that problem by
    // allowing function to 'borrow' values.
    let hello = String::from("Hello");
    let world = String::from("world");

    // By using refrences, ownership is not passed to the greet function:
    // The ampersands create pointers to boxes, which are themselves pointers to
    // the heap values.
    greet(&hello, &world);
    // Which means the values are still available here:
    let _some_other_usage = format!("{}, {}!", hello, world);

    // Dereferencing:
    let mut x: Box<i32> = Box::new(1);
    let _a: i32 = *x; // *x reads the heap value, so a = 1
    *x += 1; // *x on the left-side modifies the heap value,
             //     so x points to the value 2

    let r1: &Box<i32> = &x; // r1 points to x on the stack
    let _b: i32 = **r1; // two dereferences get us to the heap value

    let r2: &i32 = &*x; // r2 points to the heap value directly
    let _c: i32 = *r2; // so only one dereference is needed to read it

    // Rust implicitly adds references and dereferences in certain places, such
    // as when calling methods with the . operator:
    let x: Box<i32> = Box::new(-1);
    let x_abs1 = i32::abs(*x); // explicit dereference
    let x_abs2 = x.abs(); // implicit dereference
    assert_eq!(x_abs1, x_abs2);

    let r: &Box<i32> = &x;
    let r_abs1 = i32::abs(**r); // explicit dereference (twice)
    let r_abs2 = r.abs(); // implicit dereference (twice)
    assert_eq!(r_abs1, r_abs2);

    let s = String::from("Hello");
    let s_len1 = str::len(&s); // explicit reference
    let s_len2 = s.len(); // implicit reference
    assert_eq!(s_len1, s_len2);
}

fn greet(g1: &String, g2: &String) {
    println!("{}, {}!", g1, g2);
}

// Borrow Checker
// --------------
// Data may never be aliased (referenced by a pointer) and mutated at the
// same time.
//
// Variables have 3 kinds of permissions
// - Read: data can be copied
// - Write: data can be mutated in-place
// - Own: data can be moved or dropped.
// By default, a variable has RO permission. The mut keyword grants RWO.

fn _immutable_refs() {
    let mut vec = vec![1, 2, 3]; // RWO
    let num = &vec[2]; // vec: R, num: RO, *num: R
    println!("Third element is {}", *num); // vec: RWO, num: -, *num: -
    vec.push(4);

    // Note that permissions are different between variables and references.
    let x = 0; // RO
    let mut _x_ref = &x; // RWO for the reference, R for *x.
    let y = 1; // RO
               // *x_ref = 2; // This is not fine, because *x_ref is RO
    _x_ref = &y; // This is fine, because the reference itself is mutable
}

fn _mutable_refs() {
    let mut vec = vec![1, 2, 3]; // RWO

    // Mutable reference - can also modify the referenced value.
    let num = &mut vec[2]; // RW

    *num += 1; // Fine, because *num is RW (mutable reference)

    // You can't switch the order of these two lines. vec has no permissions
    // while num exists (until its last use)
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", vec);
}

fn _ascii_capitalize(v: &mut Vec<char>) {
    let c = &v[0]; // v loses WO permissions

    if c.is_ascii_lowercase() {
        let up = c.to_ascii_uppercase();
        // We're done with c, so v gains W permissions again.
        v[0] = up;
    } else {
        // v gains write permissions immediately.
        println!("Already capitalized: {:?}", v);
    }
}
