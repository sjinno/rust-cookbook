use ansi_colors::*;

fn main() {
    let mut s1 = ColouredStr::new("Hello, world!");
    s1.cyan();
    s1.bold();
    s1.underline();
    s1.blink();
    println!("{}", s1);
}
