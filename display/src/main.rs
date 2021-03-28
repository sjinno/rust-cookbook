use std::fmt;

struct Toast(String);

impl fmt::Display for Toast {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

fn main() {
    let t = Toast("Avocado toast".to_string());
    println!("{}", t);
}
