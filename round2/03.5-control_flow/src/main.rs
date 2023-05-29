fn main() {
    // If statements MUST have a boolean expression. There are no "truthy" or
    // "falsy" values in rust.
    let x = 5;

    if x == 5 {
        println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    }

    // Note that if is an expression, so you can use it in assignments,
    // though all arms must return the same type.
    let y = if x == 5 { 10 } else { 15 };
    println!("y is {}", y);

    // Loops
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");

    // Break & continue apply to the innermost loop by default, but you can add
    // labels to be specific.

    'loop_a: loop {
        println!("loop a");
        loop {
            println!("loop b");
            if 5 == 5 {
                break 'loop_a;
            }
        }
    }

    // While loops
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    // For loops
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is {}", element);
    }

    // Shorter version of the countdown above
    for number in (1..=3).rev() {
        println!("{}!", number);
    }
}
