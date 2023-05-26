pub struct Breakfast {
    pub toast: String,
    seasonal_fruit: String,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }

    pub fn serve(&self) -> () {
        println!(
            "Enjoy your {} toast and {}!",
            self.toast, self.seasonal_fruit
        )
    }
}
