mod front_of_house;

pub use crate::front_of_house::hosting;
mod back_of_house;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    let meal = back_of_house::Breakfast::summer("Rye");

    meal.serve();
}
