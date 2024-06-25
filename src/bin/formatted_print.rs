use std::fmt;
fn main() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31); // placeholder -> 31 days

    // "{} days" => format string / template

    // Positional arguments can be used. Specifying an integer inside `{}`
    // determines which additional argument will be replaced. Arguments start
    // at 0 immediately after the format string.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    // Alice, this is Bob. Bob, this is Alice

    // As can named arguments.
    println!(
        "{subject} {verb} {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );
    // the quick brown fox jumps over the lazy dog

    // Different formatting can be invoked by specifying the format character
    // after a `:`.
    println!("Base 10:               {}", 69420); // 69420 // i32
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c // ':x' formater

    // You can right-justify text with a specified width. This will
    // output "    1". (Four white spaces and a "1", for a total width of 5.)
    println!("{number:>5}", number = 1); // right arrow >

    // You can pad numbers with extra zeroes,
    println!("{number:0>5}", number = 1); // 00001
                                          // and left-adjust by flipping the sign. This will output "10000".
    println!("{number:0<5}", number = 1); // 10000  // < left arrow

    // You can use named arguments in the format specifier by appending a `$`.
    println!("{0:0>width$}", 1, width = 5);

    // Rust even checks to make sure the correct number of arguments are used.
    println!("My name is {0}, {1} {0}", "Bond", "James");
    // FIXME ^ Add the missing argument: "James"

    // Only types that implement fmt::Display can be formatted with `{}`. User-
    // defined types do not implement fmt::Display by default.

    #[allow(dead_code)] // disable `dead_code` which warn against unused module
    struct Structure(i32);
    impl fmt::Display for Structure {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0) // self.0 tuple first item
        }
    }

    // This will not compile because `Structure` does not implement
    // fmt::Display.
    println!("This struct `{}` print ok", Structure(3));
    // TODO ^ Try uncommenting this line

    // For Rust 1.58 and above, you can directly capture the argument from a
    // surrounding variable. Just like the above, this will output
    // "    1", 4 white spaces and a "1".
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:->width$}");
    println!("{number:0>width$}", number = 33, width = 8);

    let pi = 3.141592;
    println!("Pi is roughly {pi:.4?}"); //  formater :.3
}
