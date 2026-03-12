struct Person{
    name: String,
    age: u8,
}
#[derive(Debug)]
struct Point{
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}

fn calculate_area(rectangle: &Rectangle) -> f64 {
    let width = rectangle.bottom_right.x - rectangle.top_left.x;
    let height = rectangle.top_left.y - rectangle.bottom_right.y;
    width * height
}

fn main(){

    let name = String::from("Alice");
    let age = 30;
    let person = Person { name, age };
    let point_x = Point { x: 1.0, y: 2.0 };
    let point_y = Point { x: 3.0, y: 4.0 };
    let bottom_right = Point { x: 5.0, ..point_y };
    println!("Point X: ({}, {})", point_x.x, point_x.y);
    println!("Point Y: ({}, {})", point_y.x, point_y.y);
    println!("Bottom Right: ({}, {})", bottom_right.x, bottom_right.y);
    let rectangle = Rectangle { top_left: point_x, bottom_right: bottom_right };
    println!("Person: {}", person.name);
    println!("Rectangle: {:?}", rectangle);

    println!("Area of the rectangle: {}", calculate_area(&rectangle));

    println!("Hello, world!");
}