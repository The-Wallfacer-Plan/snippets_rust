struct NewType<T>(Vec<T>);

// You can create a new struct which will contain a reference to your set of data.
struct IterNewType<'a, T: 'a> {
    inner: &'a NewType<T>,
    // And there is a position used to know where you are in your iteration.
    pos: usize,
}

// Now you can just implement the `Iterator` trait on your `IterNewType` struct.
impl<'a, T> Iterator for IterNewType<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.inner.0.len() {
            // Obviously, there isn't any more data to read so let's stop here.
            None
        } else {
            // We increment the position of our iterator.
            self.pos += 1;
            // We return the current value pointed by our iterator.
            self.inner.0.get(self.pos - 1)
        }
    }
}

impl<T> NewType<T> {
    fn iter<'a>(&'a self) -> IterNewType<'a, T> {
        IterNewType {
            inner: self,
            pos: 0,
        }
    }
}

pub fn main() {
    for x in NewType(vec![1, 3, 5, 8]).iter() {
        println!("=> {}", x);
    }
}
