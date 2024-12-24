enum WebEvent {
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click { x: i64, y: i64 },
}

fn inspect(ev: WebEvent) {
    match ev {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        WebEvent::KeyPress(char) => println!("pressed '{}'.", char),
        WebEvent::Paste(str) => println!("pasted \"{}\".", str),
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

enum Num {
    Zero,
    One,
    Two,
}

enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

pub fn run() {
    let page_load = WebEvent::PageLoad;
    let page_unload = WebEvent::PageUnload;
    let key_press = WebEvent::KeyPress('x');
    let paste = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };

    inspect(page_load);
    inspect(page_unload);
    inspect(key_press);
    inspect(paste);
    inspect(click);

    println!("zero is {}", Num::Zero as i32);
    println!("one is {}", Num::One as i32);
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
}
