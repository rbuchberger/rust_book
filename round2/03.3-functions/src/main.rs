// Main is the entrypoint of most programs
fn main() {
    my_pretty_function();
    let a = add(1, 2);

    println!("1 + 2 = {}", a);
}

// declare functions with the fn keyword
fn my_pretty_function() {
    println!("I'm a pretty function!");
}

// Functions have parameters
// You MUST declare types in function signatures. That way, you rarely need
// types elsewhere.
fn add(x: i32, y: i32) -> i32 {
    // return values are implicit
    x + y
}

// Functions are a series of statements, optionally ending in an expression.
// - Statements are instructions that perform an action, and don't return a
//   value.
// - They end in semicolons.

fn _statement() {
    let _x = 5; // statement
}

// - Expressions evaluate to a resulting value.
// - They don't end in semicolons. Adding one makes it a statement.
fn _expression() -> i32 {
    1 + 2
}

// Curly braces create a new scope, called a block.
fn _block() -> i32 {
    let x = 5;

    {
        let y = 6;
        x + y
    }
}
