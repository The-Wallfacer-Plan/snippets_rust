#[derive(Debug)]
struct Dog {
    walked: bool,
}

fn do_with<F>(dog: &mut Dog, action: F)
where
    F: Fn(&mut Dog),
{
    action(dog);
}

fn walk(dog: &mut Dog) {
    dog.walked = true;
}

fn main() {
    let mut rover = Dog { walked: false };
    // Fn
    do_with(&mut rover, walk);
    // Closure
    do_with(&mut rover, |dog| dog.walked = true);
    println!("{:?}", rover);
}
