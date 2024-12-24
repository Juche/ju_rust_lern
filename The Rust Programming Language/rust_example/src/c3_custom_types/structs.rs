struct Person {
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

fn rect_area(rect: &Rectangle) -> i32 {
    // let Rectangle { p1, p2 } = rect;
    // let Point { x: x1, y: y1 } = p1;
    // let Point { x: x2, y: y2 } = p2;
    // let (Point { x: x1, y: y1 }, Point { x: x2, y: y2 }) = (rect.p1, rect.p2);
    let Rectangle {
        p1: Point { x: x1, y: y1 },
        p2: Point { x: x2, y: y2 },
    } = rect;
    let width = (x1 - x2).abs();
    let height = (y1 - y2).abs();
    let area = width * height;
    area
}

fn square(lt_p: Point, len: i32) -> Rectangle {
    let Point { x, y } = lt_p;
    let rb_p = Point {
        x: x + len,
        y: y + len,
    };

    Rectangle { p1: lt_p, p2: rb_p }
}

pub fn run() {
    let p1 = Point { x: 0, y: 0 };
    let p2 = Point { x: 10, y: 10 };
    let rect = Rectangle { p1, p2 };
    let area = rect_area(&rect);
    println!("rect: {:?}", rect);
    println!("Area of rectangle: {}", area);

    let square_rect = square(Point { x: 20, y: 20 }, 10);
    println!("square_rect: {:?}", square_rect);
}
