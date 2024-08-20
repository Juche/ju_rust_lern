#[derive(Debug)]
struct Rect {
    len: u32,
    width: u32,
}

fn area(rect: &Rect) -> u32 {
    rect.len * rect.width
}

impl Rect {
    fn area(&self) -> u32 {
        self.len * self.width
    }
}

fn main() {
    // let len = 20;
    // let width = 10;

    let rect = Rect { len: 20, width: 10 };

    let rect_area_1 = area(&rect);
    let rect_area_2 = rect.area();

    println!("rect_area_1 = {}", rect_area_1);
    println!("rect_area_2 = {}", rect_area_2);

    println!("rect = {:?}", rect);
    println!("rect = {:#?}", rect);
}

// fn main() {
//     struct User {
//         name: String,
//         age: i32,
//         sex: String,
//     }

//     let mut user = User {
//         name: String::from("Juching"),
//         age: 18,
//         sex: String::from("male"),
//     };

//     user.age = 20;

//     println!(
//         "name = {}, age = {}, sex = {}",
//         user.name, user.age, user.sex
//     );
// }
