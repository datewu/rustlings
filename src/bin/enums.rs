#![allow(dead_code)]
// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
#[derive(Debug)]
enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    use WebEvent::*;
    match event {
        //WebEvent::PageLoad => println!("page loaded"),
        PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        KeyPress(c) => println!("pressed '{}'.", c),
        Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
        // fallback/default
        e => println!("default/fallback event: {e:?}"),
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
    use_lol();
    c_lol();
    list_lol();
    constants_lol();
}

// An attribute to hide warnings for unused code.

#[derive(Debug, PartialEq)]
enum Stage {
    Beginner,
    Advanced,
}

enum Role {
    Student,
    Teacher,
}

fn use_lol() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::Stage::{Advanced, Beginner};
    // Automatically `use` each name inside `Role`.
    use crate::Role::*;

    // stack vs heap
    // copy vs move trait

    // Equivalent to `Stage::Beginner`.
    let stage = Beginner;
    let stage1 = Beginner;
    let p = &stage;
    let p3 = &stage;
    let p1 = &stage1;

    println!("i think you're differnt enum::Beginer: {p:p}, {p3:p}, {p1:p}");
    println!("i think you're differnt enum::Beginer: {p:p}, {p3:p}, {p1:p}");
    assert_eq!(stage, stage1);
    assert_eq!(p, p1);
    let a = 4;
    let b = 4;
    let p1 = &a;
    let p2 = &b;
    println!("i think you're differnt interge: {p1:p}, {p2:p}");
    // Equivalent to `Role::Student`.
    let role = Student;

    match stage {
        // Note the lack of scoping because of the explicit `use` above.
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match role {
        // Note again the lack of scoping.
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }
}

// enum with implicit discriminator (starts at 0)
enum Number {
    Zero,
    One,
    Two,
}

// enum with explicit discriminator

enum Color {
    VV,
    Red = 0xff0000,
    //    Green = 0x00ff00,
    Green,
    Blue = 0x0000ff,
    Purple,
}

fn c_lol() {
    // `enums` can be cast as integers.
    println!("zero is {}", Number::Zero as i32);
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    println!("one is {}", Color::VV as i32);
    assert_eq!(Color::VV as i32, 0);
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);
    println!("green are #{:06x}", Color::Green as i32);
    println!("purple are #{:06}", Color::Purple as i32);
}

use List::*;

enum List {
    // Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>), // Box smart pointer
    // Nil: A node that signifies the end of the linked list
    Nil,
}

// Methods can be attached to an enum
impl List {
    // Create an empty list
    fn new() -> List {
        // `Nil` has type `List`
        Nil
    }

    // Consume a list, and return the same list with a new element at its front
    fn prepend(self, elem: u32) -> List {
        // `Cons` also has type List
        Cons(elem, Box::new(self))
    }

    // &self => self: &List syntax surger
    // Return the length of the list
    fn len(&self) -> u32 {
        // `self` has to be matched, because the behavior of this method
        // depends on the variant of `self`
        // `self` has type `&List`, and `*self` has type `List`, matching on a
        // concrete type `T` is preferred over a match on a reference `&T`
        // after Rust 2018 you can use self here and tail (with no ref) below as well,
        // rust will infer &s and ref tail.
        // See https://doc.rust-lang.org/edition-guide/rust-2018/ownership-and-lifetimes/default-match-bindings.html
        match *self {
            // Can't take ownership of the tail, because `self` is borrowed;
            // instead take a reference to the tail
            // pattern mathc ref
            Cons(_, ref tail) => 1 + tail.len(),
            // Base Case: An empty list has zero length
            Nil => 0,
        }
    }

    // Return representation of the list as a (heap allocated) string
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` is similar to `print!`, but returns a heap
                // allocated string instead of printing to the console
                format!("{}, {}", head, tail.stringify())
            }
            Nil => {
                format!("Nil")
            }
        }
    }
}

fn list_lol() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}

// Globals are declared outside all other scopes.
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool {
    // Access constant in some function
    n > THRESHOLD
}

fn constants_lol() {
    let n = 16;

    // Access constant in the main thread
    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n) { "big" } else { "small" });
    // if else => expresion not statement
    // let variable =  expression ;

    // Error! Cannot modify a `const`.
    // THRESHOLD = 5;
    // FIXME ^ Comment out this line
}
