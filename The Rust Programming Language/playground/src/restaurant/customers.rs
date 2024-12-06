use super::{back_of_house, front_of_house};

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_wait_list();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
}
