fn main() {
    // Built-in datatypes
    // Rust has scalar types and compound types. Scalar types represent a single
    // value.

    // Integer types
    // {i,u}{8,16,32,64,128,size}
    // Signed/unsigned, number of bits.
    // Size is the architecture of the machine the program is running on.
    // Signing integers costs a bit.
    let _a: u32 = 12;
    let _b: i32 = -12;
    let _c: u128 = 128_000;
    let _d: isize = 0xdeadbeef;

    // Overflow:
    // Development builds will panic, release builds will wrap:
    let _e: u8 = 255;
    let _e = _e + 1;

    // If wrapping is desired, do this:
    let _e = _e.wrapping_add(1);
    // Or you can return a boolean inidcating if there was an overflow
    let (_e, _did_overflow) = _e.overflowing_add(1);

    // To check for overflow, do this:
    let _e = match _e.checked_add(1) {
        Some(v) => v,
        None => panic!("Overflow!"), // Or handle it some other way.
    };

    // To saturate, do this:
    let _e = _e.saturating_add(1);

    // Floats: f32 and f64 (default)
    let _f = 3.14; // f64, double precision
    let _g: f32 = 3.14; // f32, single precision

    // Booleans
    let _h = true;
    let _i: bool = false;

    // Characters (single quotes)
    // 4 bytes, Represent a single 'unicode scalar value'
    let _j = 'a';
    let _k: char = '😀';

    // Compound types: tuples and arrays
    // Tuples are fixed length at compile time, can contain multiple types
    let _l = (1, 2.35, true);
    // You can assign multiple variables at once:
    let (_m, _n, _o) = _l;
    // Access tuple values with a dot:
    let _p = _l.1;

    // A tuple without any values is called the 'unit' type. It represents an
    // empty value or return type. Expressions which don't explicitly return
    // anything implicitly return ().

    // Arrays are fixed length at compile time, must contain the same type.
    // They're allocated on the stack.
    let _q = [1, 2, 3, 4, 5];
    let _r: [i128; 5] = [1, 2, 3, 4, 5];
    // Same notation as js & ruby -
    let _s = _r[0];

    // Initialize an array of a given length with the same value:
    let _t = [0; 5]; // [0, 0, 0, 0, 0]

    // Accessing an array out of bounds will panic at runtime, for both dev and
    // prod builds.
    //
    // Note that the stdlib includes vectors, which act more like arrays in
    // JS/ruby. (variable length). They're allocated on the heap. Will learn
    // them later.
}
