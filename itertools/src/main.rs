use itertools::Itertools;

fn main() {
    let it = (1..3).interleave(vec![-1, -2]);
    // itertools::assert_equal(it, vec![1, -1, 2, -2]);
    println!("{:?}", it.collect::<Vec<_>>());
}
