enum Fat {
    Yes,
    No(String),
}

fn main() {
    let foo = 'f';
    assert!(matches!(foo, 'A'..='Z' | 'a'..='z' ));

    let mut fat = Fat::Yes;
    if matches!(fat, Fat::Yes) {
        fat = Fat::No("WRONG!!".to_string());
    }

    match fat {
        Fat::No(s) => println!("{}", s),
        _ => {}
    }

    let point = (0, 0);
    if matches!(point, (x, y) if x == y) {
        println!("x and y are the same!");
    }
}
