#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

fn main() {
    println!("{} days", 31);

    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    println!("{subject} {verb} {object}",
             subject = "the quick brown fox",
             verb = "jumps over",
             object = "the lazy dog");

    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    println!("{number:>width$}", number = 1, width = 6);

    println!("{number:>0width$}", number = 1, width = 6);

    println!("My name is {0}, {1} {0}", "Bond", "Philips");

    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's",
             );

    println!("Now {:?} will print!", Structure(3));

    println!("Now {:?} will print!", Deep(Structure(9)));
}
