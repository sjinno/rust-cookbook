use std::fmt;

use colored::Colorize;
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
            Shape::Circle => write!(f, "{}", "●".red()),
            Shape::Diamond => write!(f, "{}", "◆".green()),
            Shape::Square => write!(f, "{}", "■".cyan()),
            Shape::Star => write!(f, "{}", "★".yellow().blink()),
            Shape::Triangle => write!(f, "{}", "▲".magenta()),
        }
    }
}

fn main() {
    let t = Toast("Avocado toast".to_string());
    println!("{}", t);

    Shape::iter().for_each(|s| println!("{}", s));
}
