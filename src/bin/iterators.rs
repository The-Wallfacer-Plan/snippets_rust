fn main() {
    let x = vec!["Jill", "Jack", "Jane", "John"];

    let y = x.iter().cloned().take(2).collect::<Vec<_>>();

    let x = &mut (1, 2);
    {
        let y = &mut x.0;
        let z = &mut x.1;

        *y = 0;
        *z = 10;
    }
    println!("{:?}", x);
    *x = (9, 8);
    println!("{:?}", x);
}
