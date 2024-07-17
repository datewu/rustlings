fn main() {
    let an_integer = 1u32; // u32
    let a_boolean = true; // boolean
    let unit = (); // empty unit

    // copy `an_integer` into `copied_integer`
    let copied_integer = an_integer; // copy 1

    println!("An integer: {:?}", copied_integer); // debug formate
    println!("A boolean: {:?}", a_boolean); // debug fromater
    println!("Meet the unit value: {:?}", unit); // ()

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    let _noisy_unused_variable = 2u32;
    // FIXME ^ Prefix with an underscore to suppress the warning
    // Please note that warnings may not be shown in a browser
    mutability();
    scope_();
    shadowing_();
    declare_();
    freeeing();
}
fn mutability() {
    let _immutable_binding = 1;
    let mut mutable_binding = 1; // mutable variable

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    //
    // you can use "@:" execute latest command in vim/neovim

    // Error! Cannot assign a new value to an immutable variable
    //    _immutable_binding += 1;
}

fn scope_() {
    // This binding lives in the main function
    let long_lived_binding = 1;

    // This is a block, and has a smaller scope than the main function
    {
        // This binding only exists in this block
        let short_lived_binding = 2;

        println!("inner short: {}", short_lived_binding);
    }
    // End of the block

    // Error! `short_lived_binding` doesn't exist in this scope
    // println!("outer short: {}", short_lived_binding);
    // FIXME ^ Comment out this line

    println!("outer long: {}", long_lived_binding);
}

fn shadowing_() {
    let shadowed_binding = 1; // type: i32

    // block
    {
        println!("before being shadowed: {}", shadowed_binding); // i

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";

        println!("shadowed in inner block: {}", shadowed_binding);
    }
    println!("outside inner block: {}", shadowed_binding); // 1

    // This binding *shadows* the previous binding
    let shadowed_binding = false;
    println!("shadowed in outer block: {}", shadowed_binding);
}

fn declare_() {
    // Declare a variable binding
    let a_binding; // cannot  read, but can be set/binding

    {
        let x = 2;

        // Initialize the binding
        a_binding = x * x; // 4
    }

    println!("a binding: {}", a_binding);

    let another_binding; // declare but not initialize

    // Error! Use of uninitialized binding
    //    println!("another binding: {}", another_binding);
    // FIXME ^ Comment out this line

    another_binding = 1;

    println!("another binding: {}", another_binding);
}

fn freeeing() {
    let mut _mutable_integer = 7i32; // i32 pass by copied value;

    // block
    {
        // Shadowing by immutable `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // Error! `_mutable_integer` is frozen in this scope
        //        _mutable_integer = 50;
        // FIXME ^ Comment out this line

        println!("mutable binding: {}", _mutable_integer);
        // `_mutable_integer` goes out of scope
    }
    dota(_mutable_integer);

    lol(_mutable_integer);

    // Ok! `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
}
fn lol(a: i32) {
    println!("in function lol, param: {}", a);
}

fn dota(mut a: i32) {
    a += 999;
    println!("in function lol, param: {}", a);
}
