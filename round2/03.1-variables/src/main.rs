fn main() {
    let x = 12;
    println!("x is {}", x);
    let x = 7;
    println!("x is {}", x);

    {
        let x = x * 2;
        println!("x is {} in the inner scope", x);
    }

    println!("x is {} in the outer scope", x);
}

// Output of cargo run:
//
//     Finished dev [unoptimized + debuginfo] target(s) in 0.00s
//      Running `target/debug/variables`
// x is 12
// x is 7
// x is 14 in the inner scope
// x is 7 in the outer scope
