// comprehensive_rust_learning_guide.rs

// This single file aims to guide you through Rust from absolute basics to advanced concepts.
// Read and experiment with each section. Compile and run often!
// Command to compile and run: `rustc comprehensive_rust_learning_guide.rs && ./comprehensive_rust_learning_guide`
// Or, if you create a project with `cargo new rust_journey && cd rust_journey`,
// place this code in `src/main.rs` and run with `cargo run`.

// Necessary imports will be introduced as needed, but some common ones for later examples:
use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Display};
use std::io; // For input/output examples
use std::rc::Rc;
use std::cell::{Cell, RefCell};
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::Duration;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LEVEL 0: ABSOLUTE BASICS - THE "HELLO, WORLD" OF CONCEPTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
fn level_0_basics() {
    println!("\n--- LEVEL 0: ABSOLUTE BASICS ---");

    // --- 0.1. Your First Rust Program: main function and println! ---
    // The `main` function is the entry point of every Rust executable.
    // `println!` is a macro that prints text to the console.
    println!("Hello, Rust World! This is from level_0_basics.");

    // --- 0.2. Comments ---
    // Single-line comment

    /*
     * Multi-line comment
     * Spanning multiple lines.
     */

    // --- 0.3. Variables and Mutability ---
    // Variables are immutable by default. Use `let` to declare them.
    let an_integer = 5; // Type is inferred as i32 (default integer type)
    println!("The immutable integer is: {}", an_integer);

    // an_integer = 10; // This would be a COMPILE-TIME ERROR! Cannot assign twice to an immutable variable.

    // To make a variable mutable, use `mut`.
    let mut a_mutable_integer = 10;
    println!("Initial mutable integer: {}", a_mutable_integer);
    a_mutable_integer = 20;
    println!("Modified mutable integer: {}", a_mutable_integer);

    // --- 0.4. Basic Data Types (Scalar Types) ---
    // Rust is statically typed, but often infers types.

    // Integers:
    // Signed: i8, i16, i32, i64, i128, isize (pointer size)
    // Unsigned: u8, u16, u32, u64, u128, usize (pointer size)
    let small_signed: i8 = -10;
    let large_unsigned: u64 = 1_000_000_000; // `_` can be used as a visual separator
    println!("Small signed: {}, Large unsigned: {}", small_signed, large_unsigned);

    // Floating-Point Numbers:
    // f32 (single-precision), f64 (double-precision, default)
    let pi_approx: f32 = 3.14;
    let precise_gravity: f64 = 9.80665;
    println!("Pi (f32): {}, Gravity (f64): {}", pi_approx, precise_gravity);

    // Booleans:
    // `true` or `false`
    let is_rust_fun: bool = true;
    let is_complex: bool = true; // It can be!
    println!("Is Rust fun? {}, Is it complex? {}", is_rust_fun, is_complex);

    // Characters:
    // `char` represents a single Unicode scalar value (4 bytes).
    // Use single quotes.
    let initial: char = 'R';
    let emoji: char = '🦀'; // Yes, emojis are chars!
    println!("Initial: {}, Emoji: {}", initial, emoji);

    // --- 0.5. Simple Expressions and Numeric Operations ---
    let sum = 5 + 10;
    let difference = 9.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5; // Modulo operator

    println!("Sum: {}", sum);
    println!("Difference: {}", difference);
    println!("Product: {}", product);
    println!("Quotient: {}", quotient);
    println!("Remainder: {}", remainder);

    // --- 0.6. Basic Functions (Definition and Calling) ---
    // Functions are defined with `fn`.
    // We'll define a simple one here and call it.
    fn greet_person(name: &str) { // Takes a string slice `&str` as input
        println!("Hello, {}! Welcome to Rust functions.", name);
    }
    greet_person("Learner"); // Calling the function

    // Functions can return values. The return type is specified after `->`.
    // The last expression in a function is implicitly returned (if no semicolon).
    fn add_two_numbers(x: i32, y: i32) -> i32 {
        x + y // No semicolon means this expression's value is returned
              // `return x + y;` would also work.
    }
    let result_of_addition = add_two_numbers(17, 25);
    println!("17 + 25 = {}", result_of_addition);

    // --- 0.7. Shadowing ---
    // You can declare a new variable with the same name as a previous variable.
    // The new variable "shadows" the previous one.
    let x = 5;
    println!("Outer x = {}", x); // 5
    {
        let x = x * 2; // Inner x shadows outer x
        println!("Inner x = {}", x); // 10
    }
    println!("Outer x is still = {}", x); // 5 (inner x went out of scope)

    let spaces = "   "; // String type
    let spaces = spaces.len(); // Shadowing with a different type (usize)
    println!("Number of spaces: {}", spaces);
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LEVEL 1: FOUNDATIONAL CONTROL FLOW & COMPOUND DATA TYPES
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
fn level_1_foundations() {
    println!("\n--- LEVEL 1: FOUNDATIONAL CONTROL FLOW & COMPOUND DATA TYPES ---");

    // --- 1.1. Conditional `if/else` Expressions ---
    // The condition *must* be a `bool`.
    let number = 7;
    if number < 5 {
        println!("Condition was true: {} < 5", number);
    } else if number == 5 {
        println!("Condition was true: {} == 5", number);
    } else {
        println!("Condition was false: {} is not < 5 and not == 5", number);
    }

    // `if` is an expression, so you can use it in `let` statements.
    // All branches must return the same type.
    let condition = true;
    let value_from_if = if condition { 5 } else { 10 }; // Types must match (both i32 here)
    // let value_from_if_err = if condition { 5 } else { "ten" }; // COMPILE ERROR! Mismatched types.
    println!("Value from if expression: {}", value_from_if);

    // --- 1.2. Loops ---
    // `loop`: Infinite loop, break with `break`.
    println!("`loop` example:");
    let mut counter_loop = 0;
    loop {
        print!("{} ", counter_loop);
        counter_loop += 1;
        if counter_loop >= 5 {
            break; // Exit the loop
        }
    }
    println!(); // Newline

    // `loop` can also return a value from a `break`.
    let mut counter_loop_ret = 0;
    let result_from_loop = loop {
        counter_loop_ret += 1;
        if counter_loop_ret == 10 {
            break counter_loop_ret * 2; // Value to return from the loop
        }
    };
    println!("Result from loop: {}", result_from_loop);

    // `while`: Loop as long as a condition is true.
    println!("`while` loop example:");
    let mut number_while = 3;
    while number_while != 0 {
        print!("{} ", number_while);
        number_while -= 1;
    }
    println!("LIFTOFF!");

    // `for`: Iterate over a collection or a range.
    // This is the most common and idiomatic loop in Rust.
    println!("`for` loop with an array:");
    let an_array_for_loop = [10, 20, 30, 40, 50];
    for element in an_array_for_loop.iter() { // `.iter()` creates an iterator
        print!("{} ", element);
    }
    println!();

    println!("`for` loop with a range:");
    // `1..4` is a range (exclusive end: 1, 2, 3). `1..=4` (inclusive end: 1, 2, 3, 4)
    for number_in_range in 1..5 { // 1, 2, 3, 4
        print!("{} ", number_in_range);
    }
    println!();

    // --- 1.3. Tuples: Fixed-Size Ordered Lists of Different Types ---
    // Tuples group together a fixed number of values with potentially different types.
    let my_tuple: (i32, f64, char) = (500, 6.4, 'Z');

    // Destructuring a tuple:
    let (x_tup, y_tup, z_tup) = my_tuple;
    println!("Destructured tuple: x={}, y={}, z={}", x_tup, y_tup, z_tup);

    // Accessing tuple elements by index (0-based):
    let first_element = my_tuple.0;
    let second_element = my_tuple.1;
    println!("Tuple elements by index: .0={}, .1={}", first_element, second_element);

    // Unit tuple `()`: An empty tuple, often used implicitly (e.g., functions returning nothing).
    let unit_tuple = ();
    println!("Unit tuple: {:?}", unit_tuple); // Needs Debug to print

    // --- 1.4. Arrays: Fixed-Size Lists of the Same Type ---
    // Arrays have a fixed length, defined at compile time. Elements must be of the same type.
    // Data is stored on the stack.
    let numbers_array: [i32; 5] = [1, 2, 3, 4, 5]; // [type; size]
    let months_array = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
                        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]; // Type inferred as [&str; 12]

    println!("First number in array: {}", numbers_array[0]);
    println!("Third month: {}", months_array[2]);

    // Initialize an array with all elements set to the same value:
    let same_value_array = [3; 5]; // [3, 3, 3, 3, 3]
    println!("Same value array: {:?}", same_value_array); // Needs Debug to print

    // Accessing an out-of-bounds array index will cause a runtime panic in debug mode,
    // or potentially undefined behavior in optimized builds if bounds checks are elided (rarely).
    // println!("{}", numbers_array[10]); // This would panic.

    // --- 1.5. Introduction to `String` and `&str` (Briefly) ---
    // We touched on this in Level 0, but it's crucial.
    // `&str` (string slice / string literal):
    //   - A reference to a sequence of UTF-8 encoded bytes.
    //   - Usually immutable and borrowed.
    //   - `let s = "hello";` creates an `&str` pointing to static memory.
    let greeting_str_slice: &str = "Hello there!";

    // `String`:
    //   - An owned, mutable, heap-allocated string type.
    //   - Provided by Rust's standard library.
    let mut greeting_string: String = String::from("Greetings"); // Create from &str
    greeting_string.push_str(", Rustaceans!"); // Append to the String

    println!("String slice: {}", greeting_str_slice);
    println!("Owned String: {}", greeting_string);

    // We will dive deep into ownership with these types in the next level.
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LEVEL 2: OWNERSHIP, BORROWING, SLICES - RUST'S CORE PILLARS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// This is THE most important concept in Rust. It enables memory safety without a garbage collector.

fn level_2_ownership_borrowing() {
    println!("\n--- LEVEL 2: OWNERSHIP, BORROWING, SLICES ---");

    // --- 2.1. Scope and Value Lifetimes (Conceptual Recap) ---
    // - A scope is the region of a program where a variable is valid.
    // - When a variable goes out of scope, Rust automatically calls its `drop` method
    //   if it owns resources (like memory on the heap). This is RAII (Resource Acquisition Is Initialization).
    {
        let s_scoped = String::from("I exist only in this scope."); // s_scoped is valid here.
        println!("Inside scope: {}", s_scoped);
    } // Scope ends, `s_scoped` is dropped, its heap memory is freed.
      // println!("{}", s_scoped); // COMPILE ERROR! s_scoped not found in this scope.

    // --- 2.2. The Stack and The Heap (Conceptual) ---
    // - Stack: Fast memory allocation/deallocation (LIFO). Stores fixed-size data.
    //          Local variables, function call information.
    // - Heap: Slower, more flexible memory allocation. Stores data that can grow or
    //         whose lifetime isn't known at compile time (e.g., `String` content).
    // Rust's ownership system manages heap memory.

    // --- 2.3. Ownership Rules ---
    // 1. Each value in Rust has a variable that’s its *owner*.
    // 2. There can only be *one owner* at a time.
    // 3. When the owner goes out of scope, the value will be *dropped*.

    // Example with `String` (which owns heap data):
    let s1_owner = String::from("hello"); // s1_owner owns the String data "hello" on the heap.

    // The "Move" semantic for heap-owning types:
    // When `s1_owner` is assigned to `s2_new_owner`, ownership of the heap data is *moved*.
    // The pointer, length, and capacity on the stack are copied, but not the heap data itself.
    // `s1_owner` is then invalidated to prevent a "double free" (both trying to free the same memory).
    let s2_new_owner = s1_owner;

    // println!("s1_owner is: {}", s1_owner); // COMPILE ERROR! `s1_owner` value was moved to `s2_new_owner`.
    println!("s2_new_owner is: {}", s2_new_owner); // `s2_new_owner` is now the sole owner.

    // --- 2.4. The `Copy` Trait for Stack-Only Data ---
    // Types whose data is entirely on the stack (like integers, bools, chars, floats,
    // and tuples/arrays containing only `Copy` types) can implement the `Copy` trait.
    // If a type is `Copy`, an assignment creates a bit-for-bit copy, and the original
    // variable remains valid (no "move" occurs).
    let x_copyable: i32 = 5;
    let y_copied = x_copyable; // `x_copyable` is copied to `y_copied`.
    println!("x_copyable = {} (still valid), y_copied = {}", x_copyable, y_copied);

    // Types like `String` do not implement `Copy` because that would mean copying heap data
    // on every assignment, which can be expensive and is often not the desired behavior.

    // --- 2.5. Ownership and Functions (Revisited) ---
    fn takes_ownership(some_string: String) { // `some_string` takes ownership.
        println!("Inside takes_ownership: {}", some_string);
    } // `some_string` is dropped here.

    fn makes_copy(some_integer: i32) { // `some_integer` is a copy.
        println!("Inside makes_copy: {}", some_integer);
    } // `some_integer` (the copy) is dropped here.

    let string_to_give = String::from("give me away");
    takes_ownership(string_to_give);
    // println!("{}", string_to_give); // COMPILE ERROR! `string_to_give` was moved.

    let integer_to_copy = 42;
    makes_copy(integer_to_copy);
    println!("integer_to_copy is still: {}", integer_to_copy); // Still valid.

    // Functions can also return ownership.
    fn gives_ownership() -> String {
        let some_string = String::from("yours now");
        some_string // Returns ownership of `some_string`.
    }
    let received_string = gives_ownership();
    println!("Received ownership of: {}", received_string);

    fn takes_and_gives_back(a_string: String) -> String {
        a_string // Returns ownership of the (possibly modified) string.
    }
    let s_cycle = String::from("cycle me");
    let s_cycled_back = takes_and_gives_back(s_cycle);
    println!("Cycled back: {}", s_cycled_back);

    // --- 2.6. References (`&`) and Borrowing (Immutable) ---
    // What if we want to use a value without taking ownership? We *borrow* it using references.
    // A reference is like a pointer that is guaranteed to point to valid data.
    // Immutable references are created with `&`.
    let s_to_borrow = String::from("borrow me");

    fn calculate_length(s_ref: &String) -> usize { // `s_ref` is an immutable reference to a String.
        // `s_ref` does not own the data. It "borrows" it.
        s_ref.len()
    } // `s_ref` goes out of scope. The data it pointed to (owned by `s_to_borrow`) is NOT dropped.

    let length = calculate_length(&s_to_borrow); // Pass a reference to `s_to_borrow`.
    println!("The length of '{}' is {} (s_to_borrow is still valid).", s_to_borrow, length);

    // You cannot modify something through an immutable reference.
    // fn try_change_immutable(s_ref: &String) {
    //     s_ref.push_str(" world"); // COMPILE ERROR! Cannot borrow `*s_ref` as mutable.
    // }

    // --- 2.7. Mutable References (`&mut`) and Borrowing Rules ---
    // Mutable references allow you to modify the borrowed data. Created with `&mut`.
    // The original variable holding the data must also be `mut`.

    fn change_string(some_string_mut_ref: &mut String) {
        some_string_mut_ref.push_str(", world!");
    }

    let mut s_to_modify = String::from("hello");
    change_string(&mut s_to_modify); // Pass a mutable reference.
    println!("Modified string: {}", s_to_modify);

    // **The Fundamental Borrowing Rules (Enforced at Compile Time):**
    // 1. At any given time, you can have EITHER:
    //    - *One* mutable reference (`&mut T`) to a particular piece of data in a scope.
    //    OR
    //    - *Any number* of immutable references (`&T`) to that data.
    // 2. References must *always* be valid (i.e., not outlive the data they point to -
    //    this is where lifetimes come in, discussed more later).

    // These rules prevent data races at compile time!
    let mut data_for_rules = String::from("rules example");

    // Multiple immutable borrows are fine:
    let r1 = &data_for_rules;
    let r2 = &data_for_rules;
    println!("Immutable r1: {}, r2: {}", r1, r2);
    // r1 and r2 are no longer used after this point, so their borrow "ends" (Non-Lexical Lifetimes - NLL).

    // Now a mutable borrow is fine:
    let r3_mut = &mut data_for_rules;
    r3_mut.push_str(" (changed)");
    println!("Mutable r3_mut: {}", r3_mut);

    // COMPILE ERROR examples (if uncommented):
    // let mut s_err = String::from("error");
    // let r_err1 = &s_err;
    // let r_err_mut = &mut s_err; // ERROR: Cannot have mutable borrow while immutable borrow `r_err1` exists.
    // println!("{}, {}", r_err1, r_err_mut);

    // let mut s_err2 = String::from("error2");
    // let r_err_mut1 = &mut s_err2;
    // let r_err_mut2 = &mut s_err2; // ERROR: Cannot have a second mutable borrow while `r_err_mut1` exists.
    // println!("{}, {}", r_err_mut1, r_err_mut2);


    // --- 2.8. Slices (`&[T]`, `&str`) ---
    // Slices let you reference a contiguous sequence of elements in a collection
    // rather than the whole collection. A slice is a kind of reference, so it doesn't have ownership.
    // String slices (`&str`) are slices of `String` or string literals.
    let my_string_owner = String::from("hello world example");

    // Slicing a String: `&my_string_owner[start_index..end_index]` (end_index is exclusive)
    let hello_slice = &my_string_owner[0..5]; // "hello"
    let world_slice = &my_string_owner[6..11]; // "world"
    println!("Hello slice: '{}', World slice: '{}'", hello_slice, world_slice);

    // Omitting start index means from 0: `&my_string_owner[..5]`
    // Omitting end index means to the end: `&my_string_owner[6..]`
    // Omitting both gets a slice of the entire String: `&my_string_owner[..]`

    // Function that works with string slices:
    fn first_word_slice(s: &str) -> &str { // Takes `&str`, can accept `&String` or `&str` literal.
        let bytes = s.as_bytes(); // Convert to byte array for `b' '` comparison.
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' { // `b' '` is a byte literal for space.
                return &s[0..i]; // Return a slice of the string up to the space.
            }
        }
        &s[..] // If no space, the whole string is one word.
    }

    let word = first_word_slice(&my_string_owner);
    println!("First word of my_string_owner: '{}'", word);

    let my_literal = "another example string"; // `my_literal` is already an `&str`.
    let word_from_literal = first_word_slice(my_literal);
    println!("First word of my_literal: '{}'", word_from_literal);

    // Slices for other collections like arrays: `&[T]`
    let an_array_for_slice = [10, 20, 30, 40, 50];
    let array_slice: &[i32] = &an_array_for_slice[1..3]; // Contains `[20, 30]`
    println!("Array slice: {:?}", array_slice); // Use `{:?}` (Debug trait) for arrays/slices.

    // Slice safety: If you have an immutable slice, you cannot get a mutable borrow
    // of the original data that would invalidate the slice.
    // let mut s_mut_slice_err = String::from("clear me");
    // let slice_ref = &s_mut_slice_err[0..5];
    // s_mut_slice_err.clear(); // COMPILE ERROR! `clear` needs `&mut s_mut_slice_err`,
    //                        // but `slice_ref` holds an immutable borrow.
    // println!("{}", slice_ref);
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LEVEL 3: STRUCTS AND ENUMS - CUSTOM DATA TYPES
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Define your own data structures.

// --- 3.1. Defining Structs ---
// Structs group related data. Three types: Unit, Tuple, Named-field.

// Unit Struct: No fields, useful as markers or for implementing traits on something without data.
#[derive(Debug, Copy, Clone)] // Can be `Copy` as it has no data.
struct MyUnitStruct;

// Tuple Struct: Named tuple, fields accessed by index.
#[derive(Debug, Copy, Clone)] // Can be `Copy` if all fields are `Copy`.
struct Color(u8, u8, u8); // R, G, B

// Named-Field Struct: Most common, fields have names.
#[derive(Debug, Clone)] // Cannot be `Copy` if it contains non-`Copy` types like `String`.
struct User {
    id: u32,
    username: String,
    email: String,
    active: bool,
}

fn level_3_structs_enums() {
    println!("\n--- LEVEL 3: STRUCTS AND ENUMS ---");
    println!("--- 3.1. Defining and Instantiating Structs ---");

    let unit_s = MyUnitStruct;
    println!("Unit struct instance: {:?}", unit_s);

    let red = Color(255, 0, 0);
    println!("Red color (tuple struct): R={}, G={}, B={}", red.0, red.1, red.2);

    let mut user1 = User {
        id: 1,
        username: String::from("alice_in_rustland"),
        email: String::from("alice@example.com"),
        active: true,
    };
    println!("User1: ID={}, Username='{}', Email='{}', Active={}",
             user1.id, user1.username, user1.email, user1.active);

    // Accessing and modifying fields (if struct instance is mutable)
    user1.email = String::from("new_alice_email@example.com");
    println!("User1 (email updated): {}", user1.email);

    // Field init shorthand: if variable names match field names
    let id = 2u32;
    let username = String::from("bob_the_builder");
    let email = String::from("bob@example.org");
    let user2 = User { id, username, email, active: false };
    println!("User2 (shorthand): {:?}", user2);

    // Struct update syntax: create new instance using some fields from another
    let user3 = User {
        id: 3,
        email: String::from("charlie_updated@example.net"),
        ..user2 // Takes `username` and `active` from `user2`.
                // `user2.username` (String) is MOVED here.
                // `user2.active` (bool) is COPIED.
    };
    println!("User3 (update syntax): {:?}", user3);
    // println!("User2 username after move: {}", user2.username); // COMPILE ERROR! `user2.username` was moved.

    // --- 3.2. Methods on Structs (`impl` block) ---
    // Methods are functions associated with a struct type.
    impl User {
        // Associated function (often a constructor, doesn't take `self`)
        fn new(id: u32, username: &str, email: &str) -> User {
            User {
                id,
                username: String::from(username),
                email: String::from(email),
                active: true,
            }
        }

        // Method that takes an immutable reference to self (`&self`)
        fn describe(&self) -> String {
            format!("User (ID: {}): {} <{}> - Active: {}",
                    self.id, self.username, self.email, self.active)
        }

        // Method that takes a mutable reference to self (`&mut self`)
        fn deactivate(&mut self) {
            self.active = false;
        }

        // Method that takes ownership of self (`self`)
        fn into_username(self) -> String {
            self.username // The rest of `self` (id, email, active) is dropped when `self` goes out of scope.
        }
    }

    println!("\n--- 3.2. Methods on Structs ---");
    let user_with_methods = User::new(4, "diana_dev", "diana@example.io");
    println!("{}", user_with_methods.describe());

    let mut user_to_deactivate = User::new(5, "edward_editor", "ed@example.com");
    user_to_deactivate.deactivate();
    println!("{}", user_to_deactivate.describe());

    let username_consumed = user_to_deactivate.into_username(); // `user_to_deactivate` is moved.
    println!("Consumed username: {}", username_consumed);
    // println!("{}", user_to_deactivate.describe()); // COMPILE ERROR! Value moved.

    // --- 3.3. Defining Enums (Enumerations) ---
    // Enums allow a value to be one of a set of possible variants.
    #[derive(Debug)] // For printing
    enum Message {
        Quit,                       // Variant with no data (unit-like)
        Move { x: i32, y: i32 },    // Variant with named fields (struct-like)
        Write(String),              // Variant with a single String (tuple-like)
        ChangeColor(u8, u8, u8),    // Variant with multiple unnamed fields (tuple-like)
    }

    // Enums can also have methods.
    impl Message {
        fn call(&self) { // `&self` here means we are borrowing the Message enum instance
            match self { // `match` is powerful for handling enum variants
                Message::Quit => println!("Quit message called."),
                Message::Move { x, y } => { // Destructure fields from the variant
                    println!("Move message called: moving to x={}, y={}", x, y);
                }
                Message::Write(text) => { // `text` here is an `&String` (borrowed)
                    println!("Write message called: '{}'", text);
                }
                Message::ChangeColor(r, g, b) => {
                    println!("ChangeColor message called: R={}, G={}, B={}", r, g, b);
                }
            }
        }
    }
    println!("\n--- 3.3. Defining and Using Enums ---");
    let m_quit = Message::Quit;
    let m_move = Message::Move { x: 10, y: 20 };
    let m_write = Message::Write(String::from("Hello from enum!"));
    let m_color = Message::ChangeColor(0, 255, 128);

    m_quit.call();
    m_move.call();
    m_write.call();
    m_color.call();

    // --- 3.4. The `Option<T>` Enum: Handling Absence of Value ---
    // Defined in the standard library: `enum Option<T> { Some(T), None }`
    // Used extensively in Rust to handle cases where a value might be missing.
    // This avoids null pointers, a common source of bugs in other languages.
    fn find_user_id(username: &str) -> Option<u32> {
        if username == "alice" {
            Some(1) // Value exists
        } else if username == "bob" {
            Some(2)
        } else {
            None    // Value is absent
        }
    }
    println!("\n--- 3.4. `Option<T>` Enum ---");
    let user_ids = ["alice", "bob", "charlie"];
    for name in user_ids.iter() {
        match find_user_id(name) {
            Some(id) => println!("User '{}' found with ID: {}", name, id),
            None => println!("User '{}' not found.", name),
        }
    }
    // `Option` has many useful methods like `is_some()`, `is_none()`, `unwrap()`, `expect()`, `map()`, etc.
    let alice_id_option = find_user_id("alice");
    if alice_id_option.is_some() {
        println!("Alice ID (unwrapped, use with caution): {}", alice_id_option.unwrap());
    }
    // alice_id_option.unwrap_or(0); // Provides a default if None
    // alice_id_option.expect("User should exist!"); // Panics with message if None

    // --- 3.5. The `Result<T, E>` Enum: Handling Recoverable Errors ---
    // Defined in std: `enum Result<T, E> { Ok(T), Err(E) }`
    // `T` is the type of the value on success, `E` is the type of the error on failure.
    // This is the primary way Rust handles errors that can be recovered from.
    fn parse_number(s: &str) -> Result<i32, String> { // Error type is String here
        match s.parse::<i32>() { // `parse` itself returns a Result
            Ok(num) => Ok(num),
            Err(parse_err) => Err(format!("Failed to parse '{}': {}", s, parse_err)),
        }
    }
    println!("\n--- 3.5. `Result<T, E>` Enum ---");
    let inputs_for_parse = ["42", "100", "not_a_number", "-5"];
    for input_str in inputs_for_parse.iter() {
        match parse_number(input_str) {
            Ok(num) => println!("Parsed '{}' successfully: {}", input_str, num),
            Err(err_msg) => println!("Error parsing '{}': {}", input_str, err_msg),
        }
    }
    // `Result` also has methods like `is_ok()`, `is_err()`, `unwrap()`, `expect()`, `map()`, `unwrap_or_else()`.
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LEVEL 4: GENERICS AND TRAITS - ABSTRACTION AND SHARED BEHAVIOR
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

// --- 4.1. Generics (`<T>`): Abstracting Over Types ---
// Generics allow you to write code that works with different concrete types.
// `T` is a type parameter, a placeholder for a type.

#[derive(Debug)]
struct Point<T, U> { // Generic struct with two type parameters T and U
    x: T,
    y: U,
}

// Generic function
fn get_first<T>(pair: (T, T)) -> T { // Works for any type T
    pair.0
}

// Generic methods in an impl block
impl<T, U> Point<T, U> { // Must declare T and U after impl
    fn x_coord(&self) -> &T { // Returns a reference to the x coordinate
        &self.x
    }
    // We could add more methods here that use T and U.
}

// Trait bounds on generics: specify that T must implement certain traits
// We'll cover traits next, but here's a preview for generic methods.
impl<T: Display + PartialOrd, U: Debug> Point<T, U> {
    // This method only exists if T implements Display and PartialOrd, and U implements Debug.
    fn display_and_debug_if_able(&self) {
        println!("Displayable x: {}, Debuggable y: {:?}", self.x, self.y);
    }
}


fn level_4_generics_traits() {
    println!("\n--- LEVEL 4: GENERICS AND TRAITS ---");
    println!("--- 4.1. Generics ---");

    let integer_point = Point { x: 5, y: 10 }; // T=i32, U=i32
    let float_point = Point { x: 1.0, y: 4.0 }; // T=f64, U=f64
    let mixed_point = Point { x: "Hello", y: '🦀' }; // T=&str, U=char

    println!("Integer point: x = {}, y = {}", integer_point.x_coord(), integer_point.y);
    println!("Float point: x = {}, y = {}", float_point.x_coord(), float_point.y);
    println!("Mixed point: x = {}, y = {}", mixed_point.x_coord(), mixed_point.y);

    // Using the generic display_and_debug_if_able method
    integer_point.display_and_debug_if_able(); // i32 implements Display, PartialOrd, Debug
    // mixed_point.display_and_debug_if_able(); // &str implements Display, PartialOrd, Debug. char implements Debug.

    let pair_of_numbers = (10, 20);
    println!("First of pair_of_numbers: {}", get_first(pair_of_numbers));
    let pair_of_strings = ("Rust", "Rocks");
    println!("First of pair_of_strings: {}", get_first(pair_of_strings));


    // --- 4.2. Traits: Defining Shared Behavior (like Interfaces) ---
    // A trait defines functionality a type can provide.
    // Other types can implement a trait to offer that behavior.
    pub trait Summarizable {
        fn author_summary(&self) -> String; // A method signature this trait requires

        // Traits can have default method implementations
        fn summary(&self) -> String {
            format!("(Read more from {}...)", self.author_summary())
        }
        // Traits can also have associated constants
        const DEFAULT_TAG: &'static str = "article";
    }

    // Implementing a trait for one of our custom types (User from Level 3)
    // We need to define User again or put it in scope for this part to compile if running sections separately.
    // For this consolidated file, User is already defined.
    impl Summarizable for User { // User defined in level_3_structs_enums
        fn author_summary(&self) -> String {
            self.username.clone() // Return a new String
        }
        // We can choose to use the default `summary` or override it.
        // fn summary(&self) -> String {
        //     format!("User profile for {} by {}. Tag: {}", self.username, self.author_summary(), Self::DEFAULT_TAG)
        // }
    }

    // Another type implementing Summarizable
    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }
    impl Summarizable for NewsArticle {
        fn author_summary(&self) -> String {
            format!("{} from {}", self.author, self.location)
        }
        fn summary(&self) -> String {
            format!("{}, by {}. ({})", self.headline, self.author, Self::DEFAULT_TAG)
        }
    }
    println!("\n--- 4.2. Traits ---");
    let article = NewsArticle {
        headline: String::from("Rust Conquers the World!"),
        location: String::from("Global"),
        author: String::from("Rust Evangelist"),
        content: String::from("It was a dark and stormy night... then Rust appeared."),
    };
    let user_for_summary = User::new(10, "summarized_user", "sum@example.com");

    println!("Article Summary: {}", article.summary());
    println!("User Summary: {}", user_for_summary.summary());

    // --- 4.3. Trait Bounds on Generic Functions ---
    // We can use traits to constrain generic types.
    // This function accepts any type `T` that implements `Summarizable`.
    fn notify<T: Summarizable>(item: &T) { // `item` is a reference to some T
        println!("Breaking News! {}", item.summary());
    }
    // Alternative syntax (impl Trait):
    fn notify_impl_trait(item: &impl Summarizable) {
        println!("Breaking News (impl Trait)! {}", item.summary());
    }
    // Multiple trait bounds: `T: Summarizable + Display`
    // `where` clause for complex bounds: `fn func<T, U>(...) where T: Display, U: Clone + Debug`

    notify(&article);
    notify_impl_trait(&user_for_summary);

    // --- 4.4. Returning Types that Implement Traits (`impl Trait`) ---
    // You can specify that a function returns *some* type that implements a trait,
    // without naming the concrete type. Useful for returning closures or iterators.
    fn returns_summarizable(is_user: bool) -> impl Summarizable {
        if is_user {
            User::new(11, "returned_user", "ret@example.com")
        } else {
            // IMPORTANT: If returning `impl Trait`, all possible return paths
            // must return the *same concrete type*.
            // The following would be an error if NewsArticle was a different concrete type:
            // NewsArticle { headline: String::from("Returned Article"), ... }
            // For this example, let's return another User to satisfy this.
            User::new(12, "another_returned_user", "ret2@example.com")
        }
    }
    let some_item = returns_summarizable(true);
    println!("Returned impl Trait item summary: {}", some_item.summary());

    // For truly dynamic return types (different concrete types implementing the same trait),
    // you need Trait Objects (`Box<dyn Trait>`), covered later.

    // --- 4.5. Common Derivable Traits (`#[derive(...)]`) ---
    // Rust can auto-generate implementations for common traits.
    // `Debug`: For `{:?}` formatting.
    // `Clone`: To create deep copies. `Copy` (if all fields are `Copy`) implies `Clone`.
    // `PartialEq`, `Eq`: For equality comparison (`==`, `!=`). `Eq` requires `PartialEq`.
    // `PartialOrd`, `Ord`: For ordering comparison (`<`, `>`, etc.). `Ord` requires `PartialOrd` and `Eq`.
    // `Default`: To create a default instance with `Type::default()`.
    // `Hash`: To use instances as keys in `HashMap` or elements in `HashSet`.

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
    struct SimpleId {
        value: u32,
    }
    println!("\n--- 4.5. Derivable Traits ---");
    let id1 = SimpleId { value: 100 };
    let id2 = SimpleId::default(); // value: 0 (u32 default)
    let id3 = id1.clone(); // Copy, since SimpleId is Copy.

    println!("id1: {:?}, id2: {:?}, id3: {:?}", id1, id2, id3);
    println!("id1 == id3? {}", id1 == id3); // true
    println!("id2 < id1? {}", id2 < id1);   // true

    let mut id_set = HashSet::new(); // Requires Hash and Eq
    id_set.insert(id1);
    id_set.insert(id2);
    println!("ID set: {:?}", id_set);

    // --- 4.6. The `Drop` Trait: Custom Cleanup (RAII) ---
    // Defines code to run when a value goes out of scope.
    // This is how Rust manages resources like files, network connections, memory.
    struct LoudDropper {
        name: String,
    }
    impl LoudDropper {
        fn new(name: &str) -> LoudDropper {
            println!("LoudDropper '{}' is being created.", name);
            LoudDropper { name: name.to_string() }
        }
    }
    impl Drop for LoudDropper {
        fn drop(&mut self) { // `&mut self` because drop might need to modify data (e.g., flush a buffer)
            println!("LoudDropper '{}' is being dropped! Cleaning up...", self.name);
        }
    }
    println!("\n--- 4.6. Drop Trait ---");
    { // Inner scope
        let _d1 = LoudDropper::new("Alpha"); // `_` to suppress unused variable warning
        let _d2 = LoudDropper::new("Beta");
        println!("  Inside scope with LoudDroppers.");
    } // `_d2` then `_d1` are dropped here (LIFO - Last In, First Out order for local variables).
    println!("  Outside scope of LoudDroppers.");

    // You cannot call `drop` manually directly (e.g., `_d1.drop()`).
    // If you need to force a drop early, use `std::mem::drop(value)`.
    let d3 = LoudDropper::new("Gamma");
    println!("  Gamma exists.");
    std::mem::drop(d3); // Force drop of d3
    println!("  Gamma was explicitly dropped early.");
    // d3.name; // COMPILE ERROR! d3 was moved by std::mem::drop.

    // --- 4.7. Operator Overloading (via Traits) ---
    // Many operators can be overloaded by implementing traits from `std::ops`.
    // E.g., `Add` for `+`, `Mul` for `*`, `Index` for `[]`, etc.
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Complex {
        real: f64,
        imag: f64,
    }
    impl Add for Complex { // Overload `+` for `Complex` numbers
        type Output = Self; // The type returned by `+`

        fn add(self, other: Self) -> Self::Output {
            Complex {
                real: self.real + other.real,
                imag: self.imag + other.imag,
            }
        }
    }
    // Similarly, you could implement `Sub`, `Mul`, etc.
    // And `AddAssign` (`+=`), `SubAssign` (`-=`), etc.
    println!("\n--- 4.7. Operator Overloading ---");
    let c1 = Complex { real: 1.0, imag: 2.0 };
    let c2 = Complex { real: 3.0, imag: 4.0 };
    let c_sum = c1 + c2; // Uses our `Add` implementation
    println!("{:?} + {:?} = {:?}", c1, c2, c_sum);
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LEVEL 5: COLLECTIONS AND ITERATORS - MANAGING GROUPS OF DATA
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn level_5_collections_iterators() {
    println!("\n--- LEVEL 5: COLLECTIONS AND ITERATORS ---");

    // --- 5.1. `Vec<T>` (Vectors): Growable Arrays ---
    // Vectors store multiple values of the same type next to each other in memory on the heap.
    // They can grow or shrink in size.
    println!("--- 5.1. Vectors `Vec<T>` ---");
    let mut numbers_vec: Vec<i32> = Vec::new(); // Create an empty vector
    // Or with `vec!` macro:
    let mut names_vec = vec![String::from("Alice"), String::from("Bob")];

    // Adding elements (pushes to the end)
    numbers_vec.push(10);
    numbers_vec.push(20);
    numbers_vec.push(30);
    println!("Numbers vector: {:?}", numbers_vec);

    names_vec.push(String::from("Charlie"));
    println!("Names vector: {:?}", names_vec);

    // Accessing elements (panics if out of bounds)
    let first_number = numbers_vec[0];
    println!("First number: {}", first_number);
    // println!("{}", numbers_vec[10]); // Would panic

    // Safer access with `get()` which returns an `Option<&T>`
    match numbers_vec.get(1) {
        Some(second_num_ref) => println!("Second number (via get): {}", second_num_ref),
        None => println!("No second number."),
    }
    if let Some(name_ref) = names_vec.get(0) {
        println!("First name (via get): {}", name_ref);
    }

    // Iterating over a vector
    println!("Iterating over numbers_vec (immutable borrows):");
    for num_ref in &numbers_vec { // `num_ref` is `&i32`
        print!("{} ", *num_ref + 1); // Dereference `num_ref` to get `i32`
    }
    println!();

    println!("Iterating over names_vec (mutable borrows):");
    for name_mut_ref in &mut names_vec { // `name_mut_ref` is `&mut String`
        name_mut_ref.push_str(" (processed)");
    }
    println!("Processed names_vec: {:?}", names_vec);

    // Ownership with Vec: If Vec stores owned types (like String),
    // and the Vec is dropped, all its elements are also dropped.
    // If you iterate and take ownership (e.g., `for name in names_vec { ... }`),
    // the vector itself is consumed.
    // `names_vec.into_iter()` is often used for this.

    // --- 5.2. `String` (Revisited in Detail) ---
    // `String` is essentially a `Vec<u8>` that is guaranteed to hold valid UTF-8.
    println!("\n--- 5.2. `String` (Revisited) ---");
    let mut s = String::new(); // Empty string
    s.push('H');
    s.push_str("ello");
    s.push_str(" Wörld! 🦀"); // Supports Unicode
    println!("Constructed String: {}", s);

    // Concatenation:
    let s1_concat = String::from("tic");
    let s2_concat = String::from("tac");
    let s3_concat = String::from("toe");
    // `+` operator takes ownership of `s1_concat` and appends `&s2_concat` (a `&str`).
    // let s_combined_err = s1_concat + s2_concat; // `s2_concat` needs to be `&str` for `+`
    let s_combined = s1_concat + &s2_concat + &s3_concat; // s1_concat is moved here.
    println!("Combined string: {}", s_combined);
    // println!("{}", s1_concat); // COMPILE ERROR! s1_concat was moved.

    // `format!` macro: More flexible, doesn't take ownership.
    let f_s1 = "tic";
    let f_s2 = "tac";
    let f_s3 = "toe";
    let formatted_string = format!("{}-{}-{}", f_s1, f_s2, f_s3);
    println!("Formatted string: {}", formatted_string);
    println!("f_s1 still valid: {}", f_s1);

    // Indexing Strings: Rust does not allow direct integer indexing into Strings (`s[0]`)
    // because chars can be multiple bytes in UTF-8.
    // Instead, use slices with byte indices or iterate over chars/graphemes.
    let unicode_string = String::from("नमस्ते"); // Hindi "Namaste"
    // println!("{}", unicode_string[0]); // COMPILE ERROR!
    // Slicing by byte range (must be on char boundaries):
    let slice_unicode = &unicode_string[0..6]; // First two chars (each 3 bytes) "नम"
    println!("Slice of Unicode string: {}", slice_unicode);

    println!("Chars in unicode_string:");
    for c in unicode_string.chars() { // `chars()` iterates over Unicode scalar values.
        print!("'{}' ", c);
    }
    println!();
    println!("Bytes in unicode_string:");
    for b in unicode_string.bytes() { // `bytes()` iterates over raw bytes.
        print!("{} ", b);
    }
    println!();

    // --- 5.3. `HashMap<K, V>`: Key-Value Store ---
    // Stores a mapping of keys of type `K` to values of type `V`.
    // Uses a hashing function to determine how to store keys and values.
    // Keys must implement `Eq` and `Hash` traits.
    println!("\n--- 5.3. `HashMap<K, V>` ---");
    let mut scores: HashMap<String, u32> = HashMap::new();

    // Inserting key-value pairs.
    // `String` keys are moved into the HashMap. `u32` values are copied.
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let red_team_name = String::from("Red");
    scores.insert(red_team_name, 75);
    // println!("{}", red_team_name); // COMPILE ERROR! Moved.

    println!("Scores HashMap: {:?}", scores);

    // Getting a value (returns `Option<&V>`)
    let blue_score: Option<&u32> = scores.get("Blue"); // Key is `&str` due to `Borrow` trait.
    if let Some(score_ref) = blue_score {
        println!("Blue team score: {}", *score_ref);
    }

    // Overwriting a value
    scores.insert(String::from("Blue"), 25); // Overwrites the previous value for "Blue"
    println!("Scores after updating Blue: {:?}", scores);

    // `entry()` API for conditional insertion/update
    // `or_insert()` inserts if key not present, returns mutable ref to value.
    scores.entry(String::from("Green")).or_insert(30);
    let yellow_entry = scores.entry(String::from("Yellow")).or_insert(0);
    *yellow_entry += 5; // Modify existing Yellow score
    println!("Scores after entry API: {:?}", scores);

    // Iterating over a HashMap
    println!("Iterating over scores:");
    for (team_name_ref, score_ref) in &scores { // `&String`, `&u32`
        println!("{}: {}", team_name_ref, score_ref);
    }

    // --- 5.4. Iterators (`Iterator` trait) ---
    // Iterators produce a sequence of values.
    // The `Iterator` trait has a `next()` method that returns `Option<Item>`.
    // Many types implement `IntoIterator` which can be used in `for` loops.
    println!("\n--- 5.4. Iterators ---");
    let v1 = vec![1, 2, 3, 4, 5];

    // `iter()`: creates an iterator that yields immutable references (`&T`).
    let mut v1_iter = v1.iter(); // `v1_iter` is the iterator.
    println!("Manual iteration with `next()`:");
    println!("next(): {:?}", v1_iter.next()); // Some(&1)
    println!("next(): {:?}", v1_iter.next()); // Some(&2)

    // `for` loop uses `into_iter()` implicitly if available, or `iter()`/`iter_mut()`.
    println!("`for` loop with `iter()` (immutable borrows):");
    for val_ref in v1.iter() {
        print!("{} ", *val_ref);
    }
    println!();

    // `iter_mut()`: creates an iterator that yields mutable references (`&mut T`).
    let mut v_mut = vec![10, 20, 30];
    println!("`iter_mut()` (mutable borrows):");
    for val_mut_ref in v_mut.iter_mut() {
        *val_mut_ref *= 2; // Modify in place
    }
    println!("Modified v_mut: {:?}", v_mut);

    // `into_iter()`: creates an iterator that takes ownership of the collection and yields owned values (`T`).
    // The collection is consumed.
    let v_owned = vec![String::from("a"), String::from("b")];
    println!("`into_iter()` (takes ownership):");
    for s_owned in v_owned.into_iter() { // `v_owned` is moved here.
        println!("Owned string from iterator: {}", s_owned);
    }
    // println!("{:?}", v_owned); // COMPILE ERROR! `v_owned` was moved.

    // Iterator adaptors (methods that transform iterators into other iterators):
    // `map()`: Applies a closure to each element.
    // `filter()`: Filters elements based on a closure.
    // `collect()`: Consumes the iterator and gathers results into a collection.
    let numbers_for_adaptors = vec![1, 2, 3, 4, 5, 6];
    let doubled_evens: Vec<i32> = numbers_for_adaptors
        .iter()                 // Iterator yielding &i32
        .filter(|&x| x % 2 == 0) // Filter for even numbers. `|&x|` pattern matches `&i32`.
        .map(|x| x * 2)         // Double them. `x` is `&i32`. `*x * 2` would also work.
        .collect();             // Collect into a new Vec<i32>.
    println!("Doubled evens from adaptors: {:?}", doubled_evens);

    // --- 5.5. Closures: Anonymous Functions ---
    // Closures can capture variables from their environment.
    // Syntax: `|param1, param2, ...| { body }`
    // Types `Fn`, `FnMut`, `FnOnce` (compiler infers, related to how they capture).
    println!("\n--- 5.5. Closures ---");

    let add_one_closure = |x: u32| -> u32 { x + 1 }; // Type annotations optional if inferable.
    println!("Closure add_one_closure(5): {}", add_one_closure(5));

    // Capturing environment variables:
    let factor = 10;
    let multiply_by_factor_closure = |n| n * factor; // Captures `factor` by immutable reference.
    println!("multiply_by_factor_closure(7): {}", multiply_by_factor_closure(7));

    let mut count = 0;
    // This closure needs to mutate `count`, so it's `FnMut`.
    let mut increment_count_closure = || {
        count += 1; // Mutably borrows `count`.
        println!("Count (in closure): {}", count);
    };
    increment_count_closure();
    increment_count_closure();
    println!("Count (outside closure): {}", count);

    // `move` keyword: Forces the closure to take ownership of captured variables.
    let data_to_move = String::from("move me");
    let move_closure = move || { // `data_to_move` is moved into the closure.
        println!("Inside move_closure: {}", data_to_move);
    };
    move_closure();
    // println!("{}", data_to_move); // COMPILE ERROR! `data_to_move` was moved.

    // Closures are often used with iterator adaptors.
    let names_for_closure = vec!["ana", "bob", "chris"];
    let names_with_a: Vec<&str> = names_for_closure
        .into_iter() // Takes ownership of Vec, yields &str (if vec was of &str) or String
        .filter(|name| name.contains('a')) // Closure as predicate
        .collect();
    println!("Names containing 'a': {:?}", names_with_a);
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LEVEL 6: ERROR HANDLING STRATEGIES - ROBUST CODE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn level_6_error_handling() {
    println!("\n--- LEVEL 6: ERROR HANDLING STRATEGIES ---");

    // --- 6.1. Panic: Unrecoverable Errors ---
    // `panic!` macro stops program execution, unwinds stack, prints error message.
    // Use for truly unrecoverable states or bugs.
    // `RUST_BACKTRACE=1 cargo run` can show backtrace.
    fn might_panic(should_panic: bool) {
        if should_panic {
            panic!("This is a deliberate panic from might_panic!");
        } else {
            println!("  might_panic: No panic occurred.");
        }
    }
    println!("--- 6.1. Panic (Unrecoverable Errors) ---");
    might_panic(false);
    // might_panic(true); // Uncomment to see panic in action. Program would stop here.
    println!("  (Panic example is commented out to allow continuation)");

    // --- 6.2. `Result<T, E>` for Recoverable Errors (In-Depth) ---
    // We saw `Result` in Level 3. It's the idiomatic way to handle recoverable errors.
    // `enum Result<T, E> { Ok(T), Err(E) }`
    println!("\n--- 6.2. `Result<T, E>` (In-Depth) ---");

    // Example: Function that might fail to read a file (simulated)
    #[derive(Debug)] // For printing the error
    struct FileReadError {
        path: String,
        reason: String,
    }
    fn read_simulated_file(path: &str) -> Result<String, FileReadError> {
        if path == "secret.txt" {
            Ok(String::from("This is the secret content!"))
        } else if path == "empty.txt" {
            Ok(String::from(""))
        } else {
            Err(FileReadError {
                path: path.to_string(),
                reason: String::from("File not found or access denied (simulated)."),
            })
        }
    }

    let paths_to_try = ["secret.txt", "data.txt", "empty.txt"];
    for path_str in paths_to_try.iter() {
        match read_simulated_file(path_str) {
            Ok(content) => println!("  Successfully read '{}': Content='{}'", path_str, content),
            Err(e) => println!("  Error reading '{}': {:?}", path_str, e),
        }
    }

    // --- 6.3. The `?` Operator: Propagating Errors ---
    // The `?` operator simplifies error propagation.
    // If applied to a `Result<T, E>`:
    // - If `Ok(value)`, it unwraps to `value`.
    // - If `Err(error)`, it *returns* `Err(error)` from the current function.
    //   (The current function's return type must be compatible, i.e., `Result<_, E_compatible>`).
    // Can also be used with `Option<T>` (returns `None` if `None`).
    println!("\n--- 6.3. The `?` Operator ---");

    // Function using `?` to propagate errors from `read_simulated_file`.
    // Its error type must be convertible from `FileReadError` (or be the same).
    fn get_first_line_of_secret() -> Result<String, FileReadError> {
        let content = read_simulated_file("secret.txt")?; // If Err, returns Err from here.
        // If Ok, `content` now holds the String.
        Ok(content.lines().next().unwrap_or("").to_string()) // Get first line, or empty if no lines.
    }

    fn get_first_line_of_nonexistent() -> Result<String, FileReadError> {
        let content = read_simulated_file("nonexistent.txt")?; // This will likely return Err.
        Ok(content.lines().next().unwrap_or("").to_string())
    }

    match get_first_line_of_secret() {
        Ok(line) => println!("  First line of secret: '{}'", line),
        Err(e) => println!("  Error getting secret line: {:?}", e),
    }
    match get_first_line_of_nonexistent() {
        Ok(line) => println!("  First line of nonexistent: '{}'", line),
        Err(e) => println!("  Error (expected) getting nonexistent line: {:?}", e),
    }

    // --- 6.4. Defining Custom Error Types ---
    // For more complex applications, create custom error enums or structs.
    // Good error types often implement `std::error::Error` trait and `std::fmt::Display`.
    #[derive(Debug)]
    enum AppError {
        Io(io::Error), // Wrap standard I/O errors
        Parse(std::num::ParseIntError), // Wrap standard parse errors
        Custom(String), // A custom error message
        SimulatedFile(FileReadError), // Our previous error type
    }

    // Implement `Display` for user-friendly messages.
    impl Display for AppError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                AppError::Io(e) => write!(f, "IO Error: {}", e),
                AppError::Parse(e) => write!(f, "Parse Error: {}", e),
                AppError::Custom(s) => write!(f, "Custom Application Error: {}", s),
                AppError::SimulatedFile(e) => write!(f, "Simulated File Error on '{}': {}", e.path, e.reason),
            }
        }
    }

    // Implement `std::error::Error` for interoperability.
    // This trait requires `Debug` and `Display`. `source()` method is optional.
    impl std::error::Error for AppError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            match self {
                AppError::Io(e) => Some(e),
                AppError::Parse(e) => Some(e),
                AppError::SimulatedFile(_) => None, // Our FileReadError doesn't implement Error yet
                AppError::Custom(_) => None,
            }
        }
    }

    // Allow conversion from other error types using `From` trait (for `?` operator).
    impl From<io::Error> for AppError {
        fn from(err: io::Error) -> AppError { AppError::Io(err) }
    }
    impl From<std::num::ParseIntError> for AppError {
        fn from(err: std::num::ParseIntError) -> AppError { AppError::Parse(err) }
    }
    impl From<FileReadError> for AppError {
        fn from(err: FileReadError) -> AppError { AppError::SimulatedFile(err) }
    }


    println!("\n--- 6.4. Custom Error Types ---");
    fn process_data_with_custom_errors(config_path: &str, value_str: &str) -> Result<i32, AppError> {
        // Simulate reading config (could be a real file read)
        let _config_content = read_simulated_file(config_path)?; // Propagates FileReadError, converted to AppError

        // Parse a value
        let number = value_str.parse::<i32>()?; // Propagates ParseIntError, converted to AppError

        if number < 0 {
            return Err(AppError::Custom(String::from("Value cannot be negative.")));
        }
        Ok(number * 2)
    }

    match process_data_with_custom_errors("secret.txt", "123") {
        Ok(val) => println!("  Processed data successfully: {}", val),
        Err(e) => println!("  Processing error: {} (Debug: {:?})", e, e), // Use Display and Debug
    }
    match process_data_with_custom_errors("config.ini", "abc") { // config.ini will fail read_simulated_file
        Ok(val) => println!("  Processed data successfully: {}", val),
        Err(e) => println!("  Processing error: {} (Debug: {:?})", e, e),
    }
    match process_data_with_custom_errors("secret.txt", "-5") {
        Ok(val) => println!("  Processed data successfully: {}", val),
        Err(e) => println!("  Processing error: {} (Debug: {:?})", e, e),
    }
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LEVEL 7: MODULES, CRATES, AND PROJECT ORGANIZATION - STRUCTURING CODE
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// For this single-file example, we simulate modules. In a real project,
// these would be in separate files.

mod my_library { // `mod` declares a module.
    // Items are private by default. `pub` makes them public.
    pub fn public_function() {
        println!("  Called my_library::public_function()");
        private_function();
    }

    fn private_function() {
        println!("  Called my_library::private_function() (only visible within my_library)");
    }

    pub mod nested_module { // Modules can be nested.
        pub fn nested_public_function() {
            println!("    Called my_library::nested_module::nested_public_function()");
            // To call private_function from parent: super::private_function();
        }
    }

    // `pub(crate)` makes an item public within the current crate but not outside.
    // `pub(super)` makes an item public to the parent module.

    #[derive(Debug)]
    pub struct LibraryItem { // Public struct
        pub public_field: String, // Public field
        private_field: i32,   // Private field (default)
    }
    impl LibraryItem {
        pub fn new(public_data: &str, private_data: i32) -> Self {
            LibraryItem {
                public_field: String::from(public_data),
                private_field: private_data,
            }
        }
        pub fn get_private_data_indirectly(&self) -> i32 {
            // Can access private_field within the module where LibraryItem is defined.
            self.private_field * 2
        }
    }
} // end of my_library module

// `use` brings paths into scope to avoid long qualification.
use my_library::nested_module::nested_public_function; // Specific item
use my_library::LibraryItem as LibItem; // `as` for renaming

fn level_7_modules_crates() {
    println!("\n--- LEVEL 7: MODULES, CRATES, AND PROJECT ORGANIZATION ---");

    // Calling functions from our simulated module:
    println!("--- 7.1. Modules and Visibility ---");
    my_library::public_function();
    // my_library::private_function(); // COMPILE ERROR! private_function is private.

    my_library::nested_module::nested_public_function();
    // Or using the `use` statement:
    nested_public_function();

    let lib_item = LibItem::new("public data", 42);
    println!("  Library item: public_field='{}'", lib_item.public_field);
    // println!("  Library item private: {}", lib_item.private_field); // COMPILE ERROR! private_field is private.
    println!("  Library item indirect private access: {}", lib_item.get_private_data_indirectly());

    println!("\n--- 7.2. Crates (Conceptual) ---");
    // A crate is a compilation unit in Rust.
    // - Binary crate: Produces an executable (like this program if `main.rs`).
    // - Library crate: Produces a library that other programs/libraries can use.
    // This file, if compiled as `main.rs` in a `cargo new` project, is a binary crate.
    // If it were `lib.rs`, it'd be a library crate.
    // `Cargo.toml` manages crate metadata, dependencies, etc.
    // External crates (dependencies) are brought in via `Cargo.toml` and `use extern_crate_name;`.
    println!("  A crate is a package of Rust code (binary or library).");
    println!("  `Cargo` is Rust's build system and package manager.");
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LEVEL 8: ADVANCED LIFETIMES AND BORROWING NUANCES - MASTERING REFERENCES
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Lifetimes ensure references are always valid.

// --- 8.1. Explicit Lifetime Annotations ---
// `'a` is a lifetime parameter. Read as "lifetime a".
// It connects the lifetimes of different references.

// Function whose return value's lifetime depends on the input lifetimes.
// This says: the returned reference (`&'a str`) will live as long as the
// *shorter* of the lifetimes of `x` and `y`.
fn longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// Struct holding a reference needs a lifetime annotation.
#[derive(Debug)]
struct Holder<'a> { // The Holder instance cannot outlive the reference it holds.
    reference_part: &'a str,
}

impl<'a> Holder<'a> {
    // Method returning a reference tied to `self`'s lifetime
    fn get_ref(&self) -> &'a str { // Or simply `&self.reference_part` if elided.
        self.reference_part
    }
    // If a method returned a reference to something created *inside* the method
    // that doesn't live as long as `'a`, it would be an error.
}

fn level_8_advanced_lifetimes() {
    println!("\n--- LEVEL 8: ADVANCED LIFETIMES AND BORROWING NUANCES ---");
    println!("--- 8.1. Explicit Lifetime Annotations ---");

    let string1 = String::from("long string is long");
    let result_lifetime;
    { // Inner scope for string2
        let string2 = String::from("xyz");
        result_lifetime = longest_str(string1.as_str(), string2.as_str());
        // `result_lifetime` is now tied to the shorter lifetime (of `string2`).
        println!("  Longest inside inner scope: {}", result_lifetime);
    } // `string2` is dropped here.
    // println!("  Longest outside inner scope: {}", result_lifetime); // COMPILE ERROR!
                                                                  // `result_lifetime` refers to `string2` data
                                                                  // which no longer exists. Lifetime rules prevent this.
    println!("  (Dangling reference error example commented out)");

    let s_outer = String::from("Outer string for Holder");
    let holder_instance;
    {
        let part_of_s_outer: &str = &s_outer[0..5];
        holder_instance = Holder { reference_part: part_of_s_outer };
        println!("  Holder inside scope: {:?}", holder_instance);
        println!("  Holder's ref: {}", holder_instance.get_ref());
    } // `part_of_s_outer`'s borrow ends, but `s_outer` still lives, so `holder_instance` is fine.
    println!("  Holder outside scope (still valid as s_outer lives): {:?}", holder_instance);


    // --- 8.2. Lifetime Elision Rules ---
    // Rust often infers lifetimes, so you don't always need to write them.
    // Three main rules the compiler uses:
    // 1. Each elided lifetime in an input parameter gets its own distinct lifetime parameter.
    //    `fn foo(x: &T)` becomes `fn foo<'a>(x: &'a T)`
    //    `fn foo(x: &T, y: &U)` becomes `fn foo<'a, 'b>(x: &'a T, y: &'b U)`
    // 2. If there's exactly one input lifetime parameter (elided or explicit),
    //    that lifetime is assigned to all elided output lifetimes.
    //    `fn foo(x: &T) -> &U` becomes `fn foo<'a>(x: &'a T) -> &'a U`
    // 3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`,
    //    the lifetime of `self` is assigned to all elided output lifetimes.
    //    (This makes methods ergonomic).
    // If these rules don't provide a clear lifetime, you must annotate explicitly.
    fn first_elided(s: &str) -> &str { // Elided, equivalent to `fn first_elided<'a>(s: &'a str) -> &'a str`
        s.split_whitespace().next().unwrap_or("")
    }
    println!("\n--- 8.2. Lifetime Elision ---");
    let sentence = "elision makes life easier";
    println!("  First word (elided): '{}'", first_elided(sentence));


    // --- 8.3. The Static Lifetime (`'static`) ---
    // `'static` means the reference can live for the entire duration of the program.
    // String literals (`&str`) have a `'static` lifetime because they are stored in the binary.
    // Can also be used for global constants or data that needs to live forever.
    let static_string: &'static str = "I am static. I live for the whole program.";
    fn print_static_ref(s: &'static str) {
        println!("  Static ref: {}", s);
    }
    println!("\n--- 8.3. Static Lifetime ---");
    print_static_ref(static_string);
    print_static_ref("This literal is also 'static.");

    // Be careful: `'static` can also be a *bound* on a generic type `T: 'static`, meaning
    // `T` itself does not contain any non-`'static` references.

    // --- 8.4. Non-Lexical Lifetimes (NLL) - Conceptual ---
    // NLL means the compiler is smarter about how long a borrow *actually* lasts.
    // A borrow ends when the reference is *last used*, not necessarily at the end
    // of its lexical scope (the curly braces `{}`). This allows more flexible code.
    println!("\n--- 8.4. Non-Lexical Lifetimes (NLL) ---");
    let mut my_vec = vec![1, 2, 3];
    let first_ref = &my_vec[0]; // Immutable borrow starts
    println!("  NLL: First element is {}.", first_ref);
    // `first_ref` is last used here. Its borrow ends.
    // So, we can now get a mutable borrow of `my_vec`.
    my_vec.push(4); // This would have been an error before NLL if `first_ref` was considered live until end of scope.
    println!("  NLL: my_vec after push: {:?}", my_vec);
}

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LEVEL 9: SMART POINTERS AND INTERIOR MUTABILITY - ADVANCED MEMORY MANAGEMENT
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Smart pointers are structs that act like pointers but also have additional metadata and capabilities.

fn level_9_smart_pointers_interior_mutability() {
    println!("\n--- LEVEL 9: SMART POINTERS AND INTERIOR MUTABILITY ---");

    // --- 9.1. `Box<T>`: Simple Heap Allocation ---
    // `Box<T>` allocates data on the heap and stores a pointer to it on the stack.
    // It owns the heap data. When `Box<T>` goes out of scope, it's dropped, and heap data is freed.
    // Useful for:
    // - Storing large data on the heap to keep the stack frame small.
    // - Recursive types where size isn't known at compile time (e.g., cons list).
    // - Transferring ownership of heap data.
    println!("--- 9.1. `Box<T>` ---");
    let b = Box::new(5); // `5` is stored on the heap, `b` is a Box pointer on the stack.
    println!("  Box b contains: {}", *b); // `*b` dereferences the Box to get the value.
                                      // `Box` implements `Deref` and `DerefMut`.
    // `b` goes out of scope here, `5` on heap is deallocated.

    // Recursive type example:
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>), // `Box` allows recursive definition by breaking infinite size.
        Nil,
    }
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("  Recursive list with Box: {:?}", list);

    // --- 9.2. `Deref` and `DerefMut` Traits (How Smart Pointers Work) ---
    // `Deref` trait allows a smart pointer type to be treated like a regular reference.
    // `*my_box` is transformed by the compiler into `*(my_box.deref())`.
    // `DerefMut` is similar for mutable references.
    // `String` and `Vec<T>` are smart pointers (implement `Deref` to `&str` and `&[T]`).
    struct MyBox<T>(T); // Our custom smart pointer
    impl<T> MyBox<T> { fn new(x: T) -> MyBox<T> { MyBox(x) } }
    impl<T> Deref for MyBox<T> {
        type Target = T; // Associated type specifying what `*` dereferences to.
        fn deref(&self) -> &Self::Target { &self.0 }
    }
    impl<T> DerefMut for MyBox<T> {
        fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
    }
    println!("\n--- 9.2. `Deref` and `DerefMut` ---");
    let mut x_mybox = MyBox::new(String::from("my box content"));
    println!("  MyBox content via deref: {}", *x_mybox);
    x_mybox.push_str("... extended!"); // `DerefMut` allows this (String's push_str needs &mut String)
    println!("  MyBox modified content: {}", *x_mybox);


    // --- 9.3. `Rc<T>`: Reference Counting for Single-Threaded Shared Ownership ---
    // `Rc<T>` (Reference Counted) allows multiple parts of your program to share ownership
    // of some data on the heap. It keeps track of the number of references.
    // Data is dropped only when the reference count becomes zero.
    // *Not thread-safe*. For multi-threading, use `Arc<T>`.
    println!("\n--- 9.3. `Rc<T>` (Reference Counting) ---");
    let a_rc = Rc::new(String::from("shared data via Rc"));
    println!("  Initial Rc: '{}', strong_count = {}", a_rc, Rc::strong_count(&a_rc)); // Count is 1

    let b_rc = Rc::clone(&a_rc); // `Rc::clone` increments count, doesn't deep copy data.
    println!("  Cloned Rc (b_rc): '{}', strong_count = {}", b_rc, Rc::strong_count(&a_rc)); // Count is 2

    {
        let c_rc = Rc::clone(&a_rc);
        println!("  Inner scope Rc (c_rc): '{}', strong_count = {}", c_rc, Rc::strong_count(&a_rc)); // Count is 3
    } // `c_rc` goes out of scope, count decrements.
    println!("  After inner scope, strong_count = {}", Rc::strong_count(&a_rc)); // Count is 2

    // Data inside `Rc<T>` is immutable by default. To mutate, combine with `RefCell<T>`.

    // --- 9.4. `RefCell<T>` and `Cell<T>`: Interior Mutability ---
    // Interior mutability allows mutating data even when there are immutable references to it.
    // It moves Rust's borrowing rules from compile-time to *run-time*.
    // If rules are violated at runtime, the program will panic.
    // `Cell<T>`: For `Copy` types. `get()` returns a copy, `set()` replaces value. No runtime checks.
    // `RefCell<T>`: For non-`Copy` types. `borrow()` gives `Ref<T>` (immutable),
    //              `borrow_mut()` gives `RefMut<T>` (mutable). Enforces borrowing rules at runtime.
    println!("\n--- 9.4. Interior Mutability (`RefCell<T>`, `Cell<T>`) ---");

    // `Cell<T>` example
    let cell_val = Cell::new(10); // For Copy types like i32
    let val_ref_immut = &cell_val; // Immutable reference to Cell
    val_ref_immut.set(20); // Can still set value through immutable ref to Cell
    println!("  Cell value after set: {}", cell_val.get());

    // `RefCell<T>` example
    let refcell_string = RefCell::new(String::from("Initial RefCell string"));
    {
        // Get a mutable borrow at runtime
        let mut mut_borrow = refcell_string.borrow_mut(); // Panics if rules violated
        mut_borrow.push_str(" - modified!");
        println!("  RefCell string (mutably borrowed): '{}'", *mut_borrow);
    } // `mut_borrow` (the `RefMut` smart pointer) is dropped, releasing the mutable borrow.

    {
        let immut_borrow = refcell_string.borrow(); // Panics if a mutable borrow is active
        println!("  RefCell string (immutably borrowed): '{}'", *immut_borrow);
    }

    // Example of runtime panic (if uncommented):
    // let mut_borrow1 = refcell_string.borrow_mut();
    // let mut_borrow2 = refcell_string.borrow_mut(); // PANIC! Already mutably borrowed.

    // Combining `Rc<T>` and `RefCell<T>` for mutable shared data:
    let shared_mutable_data = Rc::new(RefCell::new(String::from("shared & mutable")));
    let owner1 = Rc::clone(&shared_mutable_data);
    let owner2 = Rc::clone(&shared_mutable_data);

    owner1.borrow_mut().push_str(" (by owner1)");
    println!("  Shared mutable after owner1: '{}'", owner2.borrow()); // owner2 sees change
    owner2.borrow_mut().clear();
    owner2.borrow_mut().push_str("reset by owner2");
    println!("  Shared mutable after owner2: '{}'", owner1.borrow());


    // --- 9.5. `Weak<T>`: Breaking `Rc` Reference Cycles ---
    // `Rc<T>` can lead to memory leaks if reference cycles are created (A points to B, B points to A).
    // `Weak<T>` is a non-owning reference. It doesn't increase the strong count.
    // To access data, `Weak<T>` must be `upgrade()`d to an `Option<Rc<T>>`.
    // (This is a more advanced topic, brief example).
    use std::rc::Weak;
    println!("\n--- 9.5. `Weak<T>` (Breaking Cycles) ---");

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>, // Weak reference to parent to avoid cycle
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()), // Initially no parent
        children: RefCell::new(vec![]),
    });
    println!("  Leaf strong_count = {}, weak_count = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Make leaf point to branch as parent (using Weak)
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // Create a Weak pointer

    println!("  Branch strong_count = {}, weak_count = {}", Rc::strong_count(&branch), Rc::weak_count(&branch));
    println!("  Leaf strong_count = {}, weak_count = {} (parent is weak)", Rc::strong_count(&leaf), Rc::weak_count(&leaf));

    // To access parent from leaf:
    if let Some(parent_rc) = leaf.parent.borrow().upgrade() { // Tries to get an Rc from Weak
        println!("  Leaf's parent value: {}", parent_rc.value);
    } else {
        println!("  Leaf's parent is no longer accessible.");
    }
    // If `branch` were dropped, `leaf.parent.borrow().upgrade()` would return `None`.
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LEVEL 10: CONCURRENCY - PARALLEL EXECUTION (THE HARD PART)
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// Rust's ownership and type system provide "Fearless Concurrency".

fn level_10_concurrency() {
    println!("\n--- LEVEL 10: CONCURRENCY ---");

    // --- 10.1. Threads (`std::thread::spawn`) ---
    // `spawn` creates a new OS thread. It takes a closure.
    // The closure must take ownership of any captured variables (`move` keyword).
    println!("--- 10.1. Threads ---");
    let handle1 = thread::spawn(|| { // No `move` here, as no captured variables are used.
        for i in 1..=5 {
            println!("  Spawned thread 1 says: {}", i);
            thread::sleep(Duration::from_millis(10));
        }
    });

    let data_for_thread = String::from("hello from main thread");
    let handle2 = thread::spawn(move || { // `move` takes ownership of `data_for_thread`.
        println!("  Spawned thread 2 received: '{}'", data_for_thread);
        // println!("{}", data_for_thread); // `data_for_thread` is still owned by closure here
    });
    // println!("{}", data_for_thread); // COMPILE ERROR! `data_for_thread` was moved.

    // Main thread continues execution.
    for i in 1..=3 {
        println!("Main thread says: {}", i);
        thread::sleep(Duration::from_millis(5));
    }

    // `join()` waits for a thread to finish. Returns a `Result`.
    handle1.join().unwrap(); // unwrap panics on error
    handle2.join().expect("Thread 2 panicked!"); // expect gives custom panic message
    println!("  All spawned threads finished.");

    // --- 10.2. Message Passing (`std::sync::mpsc`) ---
    // mpsc = multiple producer, single consumer. Channels for sending data between threads.
    println!("\n--- 10.2. Message Passing (Channels) ---");
    let (tx, rx) = std::sync::mpsc::channel(); // tx = transmitter, rx = receiver

    let tx_clone = tx.clone(); // Clone transmitter to send from multiple threads if needed.
    thread::spawn(move || {
        let vals = vec![
            String::from("hi from thread A"),
            String::from("more messages"),
            String::from("coming your way"),
        ];
        for val in vals {
            println!("  Thread A sending: '{}'", val);
            tx.send(val).unwrap(); // `send` takes ownership of `val`.
            thread::sleep(Duration::from_millis(10));
        }
        // `tx` is dropped when thread A finishes.
    });

    thread::spawn(move || {
        let val = String::from("message from thread B");
        println!("  Thread B sending: '{}'", val);
        tx_clone.send(val).unwrap();
    });


    // `rx.recv()` blocks until a message is received. Returns `Result`.
    // `rx.try_recv()` is non-blocking.
    // The receiver can also be used as an iterator.
    println!("  Main thread receiving messages:");
    for received_msg in rx { // Loop continues until all `tx` are dropped and channel is empty.
        println!("  Main received: {}", received_msg);
    }
    println!("  Channel closed, all messages received.");

    // --- 10.3. Shared State Concurrency (`Arc<T>`, `Mutex<T>`, `RwLock<T>`) ---
    // Sometimes, threads need to share and modify the same data.
    // `Mutex<T>` (Mutual Exclusion): Allows only one thread at a time to access the data.
    //           `lock()` acquires the mutex, blocking if necessary. Returns `MutexGuard<T>`.
    // `Arc<T>` (Atomically Reference Counted): Like `Rc<T>` but thread-safe.
    //           Used to share ownership of data across threads.
    // `RwLock<T>` (Read-Write Lock): Allows multiple readers OR one writer.
    println!("\n--- 10.3. Shared State (`Arc`, `Mutex`, `RwLock`) ---");

    // `Arc<Mutex<T>>` is a common pattern for shared mutable state.
    let counter_shared = Arc::new(Mutex::new(0)); // Counter shared across threads.
    let mut handles_shared_state = vec![];

    for i in 0..5 { // Spawn 5 threads
        let counter_clone = Arc::clone(&counter_shared); // Clone Arc for each thread
        let handle = thread::spawn(move || {
            // `lock()` acquires the mutex. Blocks if another thread holds the lock.
            // Returns a `Result<MutexGuard<T>, PoisonError>`.
            // `MutexGuard` is a smart pointer that dereferences to `T` and unlocks on drop.
            let mut num_guard = counter_clone.lock().unwrap();
            *num_guard += 1; // Modify the data through the guard.
            println!("  Thread {} incremented counter to: {}", i, *num_guard);
        }); // `num_guard` is dropped here, releasing the lock.
        handles_shared_state.push(handle);
    }

    for handle in handles_shared_state {
        handle.join().unwrap();
    }
    println!("  Final counter value (main thread): {}", *counter_shared.lock().unwrap());

    // `RwLock` example (conceptual, more complex to demo concisely)
    let rw_data = Arc::new(RwLock::new(String::from("Read-Write Data")));
    // Multiple threads could call `rw_data.read().unwrap()` simultaneously.
    // Only one thread could call `rw_data.write().unwrap()` at a time,
    // and it would block readers.


    // --- 10.4. `Send` and `Sync` Traits: Thread Safety Markers ---
    // `Send`: A type `T` is `Send` if it's safe to transfer ownership of `T` to another thread.
    // `Sync`: A type `T` is `Sync` if `&T` (an immutable reference) is `Send` (i.e.,
    //         it's safe to have `&T` accessed from multiple threads simultaneously).
    // Most primitive types, `String`, `Vec<T>` (if `T` is `Send`), `Arc<T>` (if `T` is `Send+Sync`)
    // are `Send` and `Sync`.
    // `Rc<T>` and `RefCell<T>` are *NOT* `Send` or `Sync`. That's why `Arc` and `Mutex`/`RwLock` exist.
    // The compiler checks these traits to prevent data races.
    println!("\n--- 10.4. `Send` and `Sync` Traits (Conceptual) ---");
    println!("  `Send`: Safe to move to another thread.");
    println!("  `Sync`: Safe to share references (`&T`) across threads.");
    println!("  Rust's compiler enforces these for thread safety.");

    // --- 10.5. Basic `async/await` (Introduction to Asynchronous Programming) ---
    // For non-blocking I/O operations, handling many concurrent tasks efficiently
    // without needing many OS threads.
    // Requires an async runtime (e.g., `tokio`, `async-std`).
    // This is a very brief conceptual intro; full async is a large topic.
    println!("\n--- 10.5. `async/await` (Conceptual Intro) ---");
    // `async fn` defines an asynchronous function, which returns a `Future`.
    // `await` pauses execution of the async fn until the `Future` is ready.
    async fn fetch_data_async(url: &str) -> Result<String, String> {
        println!("    Async: Starting to fetch data from {}", url);
        // In a real scenario, this would involve a non-blocking network call.
        // We simulate it with a sleep.
        // `tokio::time::sleep(Duration::from_secs(1)).await;` // If using tokio
        // std::thread::sleep(Duration::from_secs(1)); // This would block the executor thread if not careful
        println!("    Async: Finished fetching data from {}", url);
        Ok(format!("Data from {}", url))
    }

    async fn main_async_logic() {
        println!("  Running async logic...");
        let future1 = fetch_data_async("url1.com");
        let future2 = fetch_data_async("url2.com");

        // `join!` (from futures crate or tokio) can run multiple futures concurrently.
        // let (result1, result2) = futures::join!(future1, future2); // If using futures crate
        // For this simple demo, we'll await them sequentially.
        match future1.await {
            Ok(data) => println!("    Async result 1: {}", data),
            Err(e) => println!("    Async error 1: {}", e),
        }
        match future2.await {
            Ok(data) => println!("    Async result 2: {}", data),
            Err(e) => println!("    Async error 2: {}", e),
        }
    }
    // To actually run `main_async_logic`, you need an async runtime:
    // For example, with tokio:
    // #[tokio::main]
    // async fn main() { level_10_concurrency(); main_async_logic().await; }
    // For this single file demo, we can't easily integrate a full async runtime.
    println!("  (Full async execution requires an async runtime like tokio or async-std)");
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LEVEL 11: ADVANCED TRAITS AND TYPE SYSTEM FEATURES - FINE-GRAINED ABSTRACTION
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn level_11_advanced_traits_types() {
    println!("\n--- LEVEL 11: ADVANCED TRAITS AND TYPE SYSTEM FEATURES ---");

    // --- 11.1. Associated Types in Traits ---
    // Allow traits to have placeholder types that implementing types specify.
    // Useful when a trait needs to refer to some type related to the implementor.
    println!("--- 11.1. Associated Types ---");
    pub trait GenericIterator {
        type Item; // Associated type: implementors will define what `Item` is.
        fn next_item(&mut self) -> Option<Self::Item>;
    }

    struct MyCounter { count: u32, max: u32 }
    impl GenericIterator for MyCounter {
        type Item = u32; // `Item` is `u32` for `MyCounter`.
        fn next_item(&mut self) -> Option<Self::Item> {
            if self.count < self.max {
                self.count += 1;
                Some(self.count - 1)
            } else {
                None
            }
        }
    }
    let mut counter = MyCounter { count: 0, max: 3 };
    while let Some(val) = counter.next_item() {
        print!("  Counter item: {} ", val);
    }
    println!();

    // --- 11.2. Newtype Pattern (Revisited for Type Safety) ---
    // Wrapping an existing type in a new struct to:
    // 1. Implement external traits on it (orphan rule workaround).
    // 2. Add semantic meaning, type safety, and specific methods.
    println!("\n--- 11.2. Newtype Pattern ---");
    #[derive(Debug, PartialEq, Clone, Copy)]
    struct Age(u8); // Newtype for age
    #[derive(Debug, PartialEq, Clone, Copy)]
    struct UserId(u32); // Newtype for user ID

    fn process_age(age: Age) { println!("  Processing age: {:?}", age); }
    fn process_user_id(id: UserId) { println!("  Processing user ID: {:?}", id); }

    let my_age = Age(30);
    let my_id = UserId(12345);
    process_age(my_age);
    process_user_id(my_id);
    // process_age(my_id); // COMPILE ERROR! Expected Age, found UserId. Type safety!

    // --- 11.3. Supertraits: Requiring One Trait for Another ---
    // `trait Foo: Bar` means that to implement `Foo`, a type must also implement `Bar`.
    println!("\n--- 11.3. Supertraits ---");
    trait PersonName { fn get_name(&self) -> String; }
    trait Employee: PersonName { // Employee requires PersonName
        fn employee_id(&self) -> String;
        fn describe_employee(&self) -> String {
            format!("Employee ID: {}, Name: {}", self.employee_id(), self.get_name())
        }
    }
    struct SoftwareEngineer { name: String, id: u32, language: String }
    impl PersonName for SoftwareEngineer {
        fn get_name(&self) -> String { self.name.clone() }
    }
    impl Employee for SoftwareEngineer {
        fn employee_id(&self) -> String { format!("ENG-{}", self.id) }
        // `describe_employee` is inherited or can be overridden.
    }
    let eng = SoftwareEngineer { name: "Ada L.".to_string(), id: 101, language: "Rust".to_string() };
    println!("  Employee description: {}", eng.describe_employee());

    // --- 11.4. Fully Qualified Syntax for Disambiguation ---
    // `<Type as Trait>::method(receiver, args...)`
    // Used when there are multiple methods with the same name, or to call
    // trait methods that are not methods of the type itself (like associated functions in traits).
    println!("\n--- 11.4. Fully Qualified Syntax ---");
    trait Pilot { fn fly(&self); }
    trait Wizard { fn fly(&self); }
    struct Human { name: String }
    impl Pilot for Human { fn fly(&self) { println!("  {} is piloting a plane.", self.name); } }
    impl Wizard for Human { fn fly(&self) { println!("  {} is flying on a broomstick!", self.name); } }
    impl Human { fn fly(&self) { println!("  {} is just flapping their arms.", self.name); } } // Human's own method

    let person = Human { name: "Merlin".to_string() };
    person.fly(); // Calls Human's own fly method by default.
    Pilot::fly(&person);    // Calls Pilot's fly method.
    Wizard::fly(&person);   // Calls Wizard's fly method.
    // <Human as Pilot>::fly(&person); // More explicit version for Pilot.

    // --- 11.5. `PhantomData<T>`: Marker for Unused Type Parameters ---
    // A zero-sized type used to mark that a struct or enum is generic over a type `T`
    // even if `T` isn't used in any of its fields.
    // Useful for type-level programming, enforcing variance, or drop checking in unsafe code.
    println!("\n--- 11.5. `PhantomData<T>` ---");
    #[derive(Debug, PartialEq)]
    struct Length<Unit>(f64, PhantomData<Unit>); // Unit is a marker type
    impl<Unit> Length<Unit> {
        fn new(val: f64) -> Self { Length(val, PhantomData) }
        fn value(&self) -> f64 { self.0 }
    }
    #[derive(Debug, PartialEq, Clone, Copy)] struct Meter;
    #[derive(Debug, PartialEq, Clone, Copy)] struct Kilometer;

    let len_meters = Length::<Meter>::new(1000.0);
    let len_kilometers = Length::<Kilometer>::new(1.0);
    println!("  Length in meters: {:?}, value: {}", len_meters, len_meters.value());
    println!("  Length in kilometers: {:?}, value: {}", len_kilometers, len_kilometers.value());
    // fn add_lengths_meters(l1: Length<Meter>, l2: Length<Meter>) -> Length<Meter> { ... }
    // add_lengths_meters(len_meters, len_kilometers); // COMPILE ERROR! Different types.

    // --- 11.6. Generic Associated Types (GATs) - Very Advanced ---
    // GATs allow associated types within traits to themselves be generic, often over lifetimes.
    // This is a powerful but complex feature, usually encountered in library design.
    // Example (conceptual, not runnable without more setup):
    // trait LendingIterator {
    //     type Item<'a> where Self: 'a; // Item is generic over lifetime 'a
    //     fn next<'a>(&'a mut self) -> Option<Self::Item<'a>>;
    // }
    println!("\n--- 11.6. Generic Associated Types (GATs) ---");
    println!("  GATs allow associated types to be generic (e.g., over lifetimes). Very advanced.");
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// LEVEL 12: UNSAFE RUST AND FFI - BENDING THE RULES (USE WITH EXTREME CAUTION)
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// `unsafe` Rust allows you to bypass some of Rust's safety guarantees.
// It's needed for low-level programming, FFI, or specific performance optimizations.
// You are responsible for upholding memory safety within `unsafe` blocks.

// For FFI example, we need to tell Rust how to link with C functions.
// This usually requires a build script or specific linker flags.
// For this single-file demo, we'll show the Rust side.
#[cfg(feature = "ffi_example")] // Only compile if a feature flag is set (not by default here)
extern "C" {
    fn abs(input: i32) -> i32; // Declare an external C function
}

fn level_12_unsafe_ffi() {
    println!("\n--- LEVEL 12: UNSAFE RUST AND FFI ---");
    println!("  `unsafe` Rust bypasses some safety checks. Use with extreme caution!");

    // --- 12.1. The `unsafe` Keyword and its Five Capabilities ---
    // 1. Dereferencing a raw pointer.
    // 2. Calling an `unsafe` function or method (including FFI).
    // 3. Accessing or modifying a mutable `static` variable.
    // 4. Implementing an `unsafe` trait.
    // 5. Accessing fields of `union`s (not covered in detail here).
    // An `unsafe` block does not turn off the borrow checker or other safety checks.
    // It only allows these five specific operations.

    // --- 12.2. Dereferencing Raw Pointers (`*const T`, `*mut T`) ---
    // Raw pointers can be null, dangle, or point to unaligned data.
    // The compiler doesn't track their validity.
    println!("\n--- 12.2. Raw Pointers ---");
    let mut num = 5;

    // Create raw pointers from references (this part is safe).
    let r1_const_ptr: *const i32 = &num as *const i32; // Immutable raw pointer
    let r2_mut_ptr: *mut i32 = &mut num as *mut i32;   // Mutable raw pointer

    // To dereference raw pointers, you need an `unsafe` block.
    unsafe {
        println!("  Value via *const i32: {}", *r1_const_ptr);
        *r2_mut_ptr = 10; // Modify data via mutable raw pointer
        println!("  Value after modification via *mut i32: {}", *r2_mut_ptr);
        println!("  Original num is now: {}", num);
    }
    // Creating a raw pointer to an arbitrary memory address is also unsafe.
    // let address = 0x012345usize;
    // let r_arbitrary = address as *const i32;
    // unsafe { println!("{}", *r_arbitrary); } // DANGEROUS! Likely segfault.


    // --- 12.3. Calling `unsafe` Functions or Methods (including FFI) ---
    // Functions marked `unsafe fn` signal they have contracts the caller must uphold.
    unsafe fn dangerous_function() {
        println!("    Inside an unsafe function! Caller must ensure safety.");
    }
    println!("\n--- 12.3. Calling `unsafe` Functions ---");
    unsafe {
        dangerous_function(); // Must be called within an unsafe block.
    }

    // FFI (Foreign Function Interface) - Calling C code
    // (This requires linking with the C library where `abs` is defined.
    //  The `#[cfg(feature = "ffi_example")]` means it won't compile unless that feature is enabled,
    //  which we are not doing in this simple single-file setup).
    println!("  FFI Example (conceptual, linking not set up here):");
    #[cfg(feature = "ffi_example_runnable")] // To actually run, you'd need to set this up.
    {
        let x = -3;
        unsafe {
            println!("    Absolute value of {} (from C) is: {}", x, abs(x));
        }
    }
    #[cfg(not(feature = "ffi_example_runnable"))]
    println!("    (FFI `abs` call example is not compiled/run in this demo config)");


    // --- 12.4. Accessing or Modifying a Mutable `static` Variable ---
    // `static` variables have a fixed memory location and live for the program's duration.
    // Mutable `static` variables are inherently unsafe for concurrent access without synchronization.
    static mut COUNTER_STATIC_MUT: u32 = 0; // Mutable static variable
    fn increment_static_counter() {
        unsafe { // Accessing or modifying `static mut` requires unsafe.
            COUNTER_STATIC_MUT += 1;
        }
    }
    println!("\n--- 12.4. Mutable `static` Variables ---");
    increment_static_counter();
    increment_static_counter();
    unsafe {
        println!("  Value of COUNTER_STATIC_MUT: {}", COUNTER_STATIC_MUT); // Should be 2
    }
    // In multi-threaded code, this would be a data race without a Mutex or other sync.

    // --- 12.5. Implementing an `unsafe` Trait ---
    // An `unsafe trait` is a trait where at least one method has some invariant
    // that the compiler can't verify, and implementors must uphold it via `unsafe impl`.
    // `Send` and `Sync` are examples of `unsafe trait`s (though you usually don't impl them manually).
    unsafe trait UnsafeFoo {
        fn do_something_unsafe(&self);
    }
    struct MyUnsafeType;
    unsafe impl UnsafeFoo for MyUnsafeType { // Implementation must also be `unsafe`.
        fn do_something_unsafe(&self) {
            println!("    MyUnsafeType is doing something unsafe!");
        }
    }
    println!("\n--- 12.5. Implementing `unsafe` Traits ---");
    let my_unsafe_obj = MyUnsafeType;
    // Calling the method might or might not require `unsafe` block, depending on method signature.
    // If `do_something_unsafe` itself was `unsafe fn`, then `unsafe` block would be needed.
    my_unsafe_obj.do_something_unsafe(); // In this case, the method isn't `unsafe fn`.

    println!("  `unsafe` provides power but demands responsibility. Minimize its use.");
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// MAIN FUNCTION TO RUN ALL DEMOS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
fn main() {
    println!("========== COMPREHENSIVE RUST LEARNING GUIDE ==========");
    println!("TIP: Read through the code comments for each section.");

    // Level 0: Absolute Basics
    level_0_basics();

    // Level 1: Foundational Control Flow & Compound Data Types
    level_1_foundations();

    // Level 2: Ownership, Borrowing, Slices
    level_2_ownership_borrowing();

    // Level 3: Structs and Enums
    level_3_structs_enums();

    // Level 4: Generics and Traits
    level_4_generics_traits();

    // Level 5: Collections and Iterators
    level_5_collections_iterators();

    // Level 6: Error Handling Strategies
    level_6_error_handling();

    // Level 7: Modules, Crates, and Project Organization
    level_7_modules_crates();

    // Level 8: Advanced Lifetimes and Borrowing Nuances
    level_8_advanced_lifetimes();

    // Level 9: Smart Pointers and Interior Mutability
    level_9_smart_pointers_interior_mutability();

    // Level 10: Concurrency
    level_10_concurrency(); // Note: Async part is conceptual without a runtime.

    // Level 11: Advanced Traits and Type System Features
    level_11_advanced_traits_types();

    // Level 12: Unsafe Rust and FFI
    level_12_unsafe_ffi(); // Note: FFI `abs` call is conceptual without linking.

    println!("\n========== END OF COMPREHENSIVE GUIDE ==========");
    println!("Congratulations on working through this! Keep practicing and building!");
}
