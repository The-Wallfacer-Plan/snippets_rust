trait Insect {
    fn category(&self) -> String;
}

trait Fly {
    fn fly(&self);
    // fn cc(&self) -> String {
    //     self.category()
    // }
}

impl Insect for Fly {
    fn category(&self) -> String {
        "Fly".to_owned()
    }
}

struct Bee;

impl Fly for Bee {
    fn fly(&self) {
        println!("wooo...");
    }
}

fn main() {
    let bee = Bee;
    bee.fly();
}
