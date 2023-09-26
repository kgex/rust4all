// Define a struct to represent a Point in 2D space
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    // Create instances of the Point struct
    let origin = Point { x: 0.0, y: 0.0 };
    let point = Point { x: 3.0, y: 4.0 };

    // Accessing struct fields
    println!("Origin: x={}, y={}", origin.x, origin.y);
    println!("Point: x={}, y={}", point.x, point.y);
}
