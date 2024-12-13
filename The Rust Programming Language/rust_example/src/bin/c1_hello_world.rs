use rust_example::c1_hello_world::{comment, format};

fn main() {
    let person = comment::Person::new("Juching");
    person.hello();

    format::demo::run();
}
