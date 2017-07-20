#![allow(dead_code, unused_imports, unused_variables)]

extern crate mylib;

use std::fmt;

use std::thread;

use mylib::test_lib;

fn test_io() {
    // open.rs
    use std::error::Error;
    use std::fs::File;
    use std::io::prelude::*;
    use std::path::Path;

    // Create a path to the desired file
    let path = Path::new("examples/hello.txt");
    let display = path.display();

    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", display, why.description()),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why.description()),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }

    // `file` goes out of scope, and the "hello.txt" file gets closed
}

fn test_thread() {
    static NTHREADS: i32 = 10;

    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    for i in 0..NTHREADS {
        // Spin up another thread
        children.push(thread::spawn(
            move || println!("this is thread number {}", i),
        ));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}

fn test_map() {
    #![allow(dead_code)]

    #[derive(Debug)]
    enum Food {
        Apple,
        Carrot,
        Potato,
    }

    #[derive(Debug)]
    struct Peeled(Food);
    #[derive(Debug)]
    struct Chopped(Food);
    #[derive(Debug)]
    struct Cooked(Food);

    // Peeling food. If there isn't any, then return `None`.
    // Otherwise, return the peeled food.
    fn peel(food: Option<Food>) -> Option<Peeled> {
        match food {
            Some(food) => Some(Peeled(food)),
            None => None,
        }
    }

    // Chopping food. If there isn't any, then return `None`.
    // Otherwise, return the chopped food.
    fn chop(peeled: Option<Peeled>) -> Option<Chopped> {
        match peeled {
            Some(Peeled(food)) => Some(Chopped(food)),
            None => None,
        }
    }

    // Cooking food. Here, we showcase `map()` instead of `match` for case handling.
    fn cook(chopped: Option<Chopped>) -> Option<Cooked> {
        chopped.map(|Chopped(food)| Cooked(food))
    }

    // A function to peel, chop, and cook food all in sequence.
    // We chain multiple uses of `map()` to simplify the code.
    fn process(food: Option<Food>) -> Option<Cooked> {
        food.map(Peeled).map(|Peeled(f)| Chopped(f)).map(
            |Chopped(f)| {
                Cooked(f)
            },
        )
    }

    // Check whether there's food or not before trying to eat it!
    fn eat(food: Option<Cooked>) {
        match food {
            Some(food) => println!("Mmm. I love {:?}", food),
            None => println!("Oh no! It wasn't edible."),
        }
    }

    let apple = Some(Food::Apple);
    let carrot = Some(Food::Carrot);
    let potato = None;

    let cooked_apple = cook(chop(peel(apple)));
    let cooked_carrot = cook(chop(peel(carrot)));
    // Let's try the simpler looking `process()` now.
    let cooked_potato = process(potato);

    eat(cooked_apple);
    eat(cooked_carrot);
    eat(cooked_potato);
}

fn test_macro() {
    macro_rules! create_function {
    // This macro takes an argument of designator `ident` and
    // creates a function named `$func_name`.
    // The `ident` designator is used for variable/function names.
    ($func_name:ident) => (
        fn $func_name() {
            // The `stringify!` macro converts an `ident` into a string.
            println!("You called {:?}()",
                     stringify!($func_name))
        }
    )
}

    // Create functions named `foo` and `bar` with the above macro.
    create_function!(foo);
    create_function!(bar);

    macro_rules! print_result {
    // This macro takes an expression of type `expr` and prints
    // it as a string along with its result.
    // The `expr` designator is used for expressions.
    ($expression:expr) => (
        // `stringify!` will convert the expression *as it is* into a string.
        println!("{:?} = {:?}",
                 stringify!($expression),
                 $expression)
    )
}

    foo();
    bar();

    print_result!(1u32 + 1);

    // Recall that blocks are expressions too!
    print_result!({
        let x = 1u32;

        x * x + 2 * x - 1
    });
}

#[allow(unused_variables)]
fn test_generics() {
    struct A;
    struct SingleGen<T>(T);

    let _char: SingleGen<char> = SingleGen('a');

    let _t = SingleGen(A); // Uses `A` defined at the top.

    ///////////////////////////////////////////////////////////////////////////

    struct Val {
        val: f64,
    }

    struct GenVal<T> {
        gen_val: T,
    }

    // impl of Val
    impl Val {
        fn value(&self) -> &f64 {
            &self.val
        }
    }

    // impl of GenVal for a generic type `T`
    impl<T> GenVal<T> {
        fn value(&self) -> &T {
            &self.gen_val
        }
    }

    let x = Val { val: 3.0 };
    let y = GenVal { gen_val: 3i32 };

    println!("{}, {}", x.value(), y.value());
}

fn test_captures() {
    fn apply<F>(f: F)
    where
        F: FnOnce(),
    {
        // ^ TODO: Try changing this to `Fn` or `FnMut`.
        f();
    }

    // A function which takes a closure and returns an `i32`.
    fn apply_to_3<F>(f: F) -> i32
    where
        F: Fn(i32) -> i32,
    {
        f(3)
    }

    let greeting = "hello";
    use std::mem;
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}", greeting);
        farewell.push_str("!!!");
        println!("The i screamed {}", farewell);

        println!("Now I can sleep. zzzzz");

        mem::drop(farewell);
    };

    apply(diary);

    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}

fn test_collections() {
    #[allow(unused_variables)]
    let movable = Box::new(3);

    let vec1 = vec![1, 2, 3];
    println!("2 in vec1? {}", vec1.iter().any(|&x| x == 2));

    let vec2 = vec![4, 5, 6];
    println!("2 in vec2? {}", vec2.into_iter().any(|x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    // `iter()` for arrays yields `&i32`.
    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    // `into_iter()` for arrays unusually yields `&i32`.
    println!("2 in array2: {}", array2.into_iter().any(|&x| x == 2));
}

fn test_constants() {
    static LANGUAGE: &'static str = "rustc";
    const THRESHOLD: i32 = 10;

    fn is_big(n: i32) -> bool {
        n > THRESHOLD
    }

    let n = 16;
    let lang = LANGUAGE;
    println!("language is {}, lang is also {}", LANGUAGE, lang);
    println!(
        "{} is {}, size={}",
        n,
        if is_big(n) { "big" } else { "small" },
        std::mem::size_of_val(LANGUAGE)
    );
}

fn test_simple() {
    //    clang_struct::struct_clang();
    //    llvm_jit::jit();
    //    llvm_nop::nop();

    println!("hello world");
    let x = 5 + /* 90 + */ 5;
    println!("x={}", x);
    println!("{:b} {:o}", 2, 20);
    println!("{obj}", obj = 20);

    #[derive(Debug)]
    struct Structure(i32);
    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({})", self.0)
        }
    }

    #[derive(Debug)]
    struct Deep(Structure);
    println!("hello {}", Structure(32));

    // Define a structure where the fields are nameable for comparison.
    #[derive(Debug)]
    struct Point2D {
        x: f64,
        y: f64,
    }
    let point_2d = Point2D { x: 3.3, y: 4.4 };
    println!("{:?}", point_2d);

    #[allow(unused_variables)]
    let a_float = 3.0;

    let long_tuple = (
        1u8,
        2u16,
        3u32,
        4u64,
        -1i8,
        -2i16,
        -3i32,
        -4i64,
        0.1f32,
        0.2f64,
        'a',
        true,
    );

    let tuple_e = long_tuple.3;
    println!("3rd of {:?} is {}", long_tuple, tuple_e);

    fn analyze_slice(slice: &[i32]) {
        println!("first element of the slice: {}", slice[0]);
        println!("the slice has {} elements", slice.len());
    }

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    analyze_slice(&xs);
    analyze_slice(&xs[1..4]);

    struct Nil;
    #[allow(unused_variables)]
    let _nil = Nil;
}

pub static mut foo: usize = 5;
pub static mut bar: [u8; 10] = [0; 10];

struct ST {
    i: i32,
    s: String,
}

fn main() {
    test_size_of();
}

fn test_size_of() {
    use std::mem;
    struct Foo<T, U> {
        count: u16,
        data1: T,
        data2: U,
    }

    enum BufCmp {
        Same,
        Diffs(usize, usize),
    }

    println!(
        "size(usize)={}, size(BufCmp)={}",
        mem::size_of::<usize>(),
        mem::size_of::<BufCmp>()
    );
    println!("u16-u32: {}", mem::size_of::<Foo<u16, u32>>());
    println!("u32-u16: {}", mem::size_of::<Foo<u32, u16>>());
    println!(
        "i32\t{}\t{}",
        mem::size_of::<Option<i32>>(),
        mem::size_of::<i32>()
    );
    println!(
        "&str\t{}\t{}",
        mem::size_of::<Option<&str>>(),
        mem::size_of::<&str>()
    );
    println!(
        "String\t{}\t{}",
        mem::size_of::<Option<String>>(),
        mem::size_of::<String>()
    );
    println!(
        "&String\t{}\t{}",
        mem::size_of::<Option<&String>>(),
        mem::size_of::<&String>()
    );
    println!(
        "array1\t{}\t{}",
        mem::size_of::<Option<[u8; 40]>>(),
        mem::size_of::<[u8; 40]>()
    );
    println!(
        "array2\t{}\t{}",
        mem::size_of::<Option<[u64; 40]>>(),
        mem::size_of::<[u64; 40]>()
    );
    println!(
        "array3\t{}\t{}",
        mem::size_of::<Option<[u32; 40]>>(),
        mem::size_of::<[u32; 40]>()
    );
    println!(
        "array4\t{}\t{}",
        mem::size_of::<Option<[usize; 40]>>(),
        mem::size_of::<[usize; 40]>()
    );
    println!(
        "&array1\t{}\t{}",
        mem::size_of::<Option<&[u8; 10]>>(),
        mem::size_of::<&[u8; 10]>()
    );
    println!(
        "&array2\t{}\t{}",
        mem::size_of::<Option<&[u8]>>(),
        mem::size_of::<&[u8]>()
    );
}
