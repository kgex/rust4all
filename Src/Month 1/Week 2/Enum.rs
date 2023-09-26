// Define an enum to represent different types of geometric shapes
enum Shape {
    Circle(f64),     // Circle with a radius
    Rectangle(f64, f64), // Rectangle with width and height
    Square(f64),      // Square with a side length
}

fn main() {
    // Create instances of the Shape enum
    let circle = Shape::Circle(5.0);
    let rectangle = Shape::Rectangle(3.0, 4.0);
    let square = Shape::Square(2.0);

    // Match enum variants to perform different actions
    match circle {
        Shape::Circle(radius) => println!("Circle with radius {}", radius),
        Shape::Rectangle(width, height) => println!("Rectangle with width {} and height {}", width, height),
        Shape::Square(side_length) => println!("Square with side length {}", side_length),
    }
}
