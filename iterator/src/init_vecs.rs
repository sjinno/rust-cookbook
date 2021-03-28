pub fn init_vecs() {
    let v1 = (0..100).collect::<Vec<_>>();
    println!("v1 = {:?}\n", v1);

    let v2 = (0..100).filter(|x| x % 2 == 0).collect::<Vec<_>>();
    println!("v2 = {:?}\n", v2);

    let v3 = ('a'..='z').collect::<Vec<_>>();
    println!("v3 = {:?}\n", v3);

    let v4 = ('A'..='Z').collect::<Vec<_>>();
    println!("v4 = {:?}\n", v4);

    let v5 = (0..100)
        .filter(|x| !(50..100).contains(x))
        .collect::<Vec<_>>();
    println!("{:?}\n", v5);
}
