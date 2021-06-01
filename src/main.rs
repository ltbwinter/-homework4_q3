struct Rectangle {
    width: f64,
    height: f64,
}

struct Triangle {
    base: f64,
    height: f64,
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl HasArea for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}

fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    let rectangle = Rectangle {
        width: 4.0,
        height: 6.0,
    };
    let triangle = Triangle {
        base: 4.0,
        height: 6.0,
    };
    println!("rectangle.area() = {}", rectangle.area());
    println!("triangle.area() = {}", triangle.area());
    println!("area(&rectangle) = {}", area(&rectangle));
    println!("area(&triangle) = {}", area(&triangle));
}
