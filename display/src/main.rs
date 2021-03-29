use std::fmt::{self, Write};

struct Toast(String);

impl fmt::Display for Toast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

enum Shape {
    Circle,
    Diamond,
    Square,
    Star,
    Triangle,
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Shape::Circle => f.write_char('●'),
            Shape::Diamond => f.write_char('◆'),
            Shape::Square => f.write_char('■'),
            Shape::Star => f.write_char('★'),
            Shape::Triangle => f.write_char('▲'),
        }
    }
}

fn main() {
    let t = Toast("Avocado toast".to_string());
    println!("{}", t);

    let shapes: &[Shape; 5] = &[
        Shape::Circle,
        Shape::Diamond,
        Shape::Square,
        Shape::Star,
        Shape::Triangle,
    ];
    shapes.iter().for_each(|s| println!("{}", s));
}
