// Structs are like tuples, but with named fields.
struct Pet {
    // Note the use of String and not str - this ensures that the struct
    // owns its own data. Structs *can* contain refs, but then you need to use
    // lifetimes.
    name: String,
    age: u8,
    human: String,
    species: Species,
}

enum Species {
    Dog,
    Cat,
    Lizard,
}

fn main() {
    let my_dog = Pet {
        name: String::from("Rusty"),
        age: 8,
        human: String::from("John"),
        species: Species::Dog,
    };

    // The whole thing is mutable or not.
    let mut my_cat = Pet {
        name: String::from("Misty"),
        age: 2,
        human: String::from("John"),
        species: Species::Cat,
    };

    print_pet(&my_dog);

    // The cat had a birthday
    my_cat.age += 1;

    print_pet(&my_cat);

    // The dog has a twin
    let my_other_dog = Pet {
        name: String::from("Dusty"),
        ..my_dog
    };

    print_pet(&my_other_dog);

    let daves_lizard = Pet {
        name: String::from("Lizzy"),
        age: 1,
        human: String::from("Dave"),
        species: Species::Lizard,
    };

    print_pet(&daves_lizard);

    println!("-----------------------------");
    rectangle_example();
    println!("-----------------------------");
    rectangle_method_example();
}

fn print_pet(pet: &Pet) {
    let species = match pet.species {
        Species::Dog => "dog",
        Species::Cat => "cat",
        Species::Lizard => "lizard",
    };

    let human = match pet.human.as_str() {
        "John" => "My".to_owned(),
        name => format!("{}'s", name),
    };

    println!(
        "{} {}'s name is {} and he is {} years old.",
        human, species, pet.name, pet.age
    );
}

// -----------------------------------------------------------------------------

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn rectangle_example() {
    let my_rect = Rectangle {
        width: 300,
        height: 500,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&my_rect)
    );

    // The debug trait allows us to print the struct
    println!("\n{:#?}", my_rect);
    // You can also use the dbg! macro (prints to stderr)
    dbg!(&my_rect);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

// -----------------------------------------------------------------------------

// Method syntax - define associated functions
impl Rectangle {
    // If the first param is a ref to self, you call it on an instance of the
    // struct.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Self is special, means "the type we're defining the method on"
    fn can_hold(&self, other_rect: &Self) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }

    // If there's no ref to self, you call it on the struct itself (class
    // method)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn rectangle_method_example() {
    let my_rect = Rectangle {
        width: 300,
        height: 500,
    };

    let other_rect = Rectangle {
        width: 200,
        height: 400,
    };

    let square = Rectangle::square(100);

    println!(
        "The area of the rectangle is {} square pixels.",
        my_rect.area()
    );

    println!(
        "Can my_rect hold other_rect? {}",
        my_rect.can_hold(&other_rect)
    );
    println!("Can square hold my_rect? {}", square.can_hold(&my_rect));
}
