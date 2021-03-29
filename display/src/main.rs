use std::fmt::{self, Write};

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

struct Toast(String);

impl fmt::Display for Toast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(EnumIter)]
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

    Shape::iter().for_each(|s| println!("{}", s));
}
