use std::cmp::Ordering;
use std::ops::Add;
use std::ops::Mul;

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
    z: T,
}

impl<T> Point<T>
where
    T: Add<Output = T> + Mul<Output = T> + Copy + Ord,
{
    fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }

    fn compare_x_and_y(&self) {
        match self.x.cmp(&self.y) {
            Ordering::Greater => println!("x is greater than y."),
            Ordering::Equal => println!("x and y are equal."),
            Ordering::Less => println!("x is less than y."),
        }
    }

    fn compare_xz_and_y(&self) {
        let xz = self.x * self.z;
        match xz.cmp(&self.y) {
            Ordering::Greater => println!("xz is greater than y."),
            Ordering::Equal => println!("xz and y are equal."),
            Ordering::Less => println!("xz is less than y."),
        }
    }

    fn x_plus_y_plus_z(&self) -> T {
        self.x + self.y + self.z
    }
}

fn main() {
    let mut p1 = Point::new(6, 8, 2);
    println!("{:#?}", p1);
    p1.compare_x_and_y();

    p1.x = 4;
    p1.compare_xz_and_y();

    println!("x + y + z = {}", p1.x_plus_y_plus_z());
}
