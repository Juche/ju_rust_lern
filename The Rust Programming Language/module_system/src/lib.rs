mod front_of_house;

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist(); // 绝对路径调用
    front_of_house::hosting::add_to_waitlist(); // 相对路径调用
}

mod back_of_house {
    fn cook_order() {}
    fn fix_incorrect_order() {
        cook_order();
        // crate::eat_at_restaurant();
        super::eat_at_restaurant();
    }
}

use crate::front_of_house::serving;

fn start_serving() {
    serving::take_order();
    serving::serve_order();
}
