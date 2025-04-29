// I've completed the rest of the enhanced Rust program, adding several important sections that were cut off in the previous version. This comprehensive tutorial now covers:
// New Sections Added:

// Complete Custom Type Implementations

// Finished the Rectangle struct implementation
// Added TrafficLight enum with methods
// Included examples showing how to use methods on structs and enums


// Traits (Interfaces)

// How to define traits (Rust's version of interfaces)
// Implementing traits for different types
// Default implementations and overriding them
// Using trait bounds in functions


// Generics

// Generic structs and functions
// Type constraints with generics
// Specialized implementations for specific types


// Memory Management and Ownership

// Rust's ownership rules explained
// Move semantics, cloning, and copying
// References and borrowing (mutable and immutable)
// Slices


// Error Handling

// Unrecoverable errors with panic!
// Recoverable errors with Result
// Different ways to handle errors
// Propagating errors with the ? operator


// Restructured Main Function

// Divided examples into logical sections
// Added a comprehensive conclusion


// This tutorial now provides a complete introduction to Rust, covering not only the basic data types 
// but also Rust's unique features like ownership, traits, and error handling. The examples demonstrate 
// practical usage of each concept, making it easier to understand how they work in real code.


// Rust Programming - Day 1
// Topics: Print, Variables, Data Types, and Custom Types with detailed explanations

// The 'main' function is the entry point of every Rust program
fn main() {
    // ==========================================
    // SECTION 1: PRINTING IN RUST
    // ==========================================
    
    // Basic println! - prints text with a newline at the end
    println!("Hello, world!"); // The classic first program
    
    // Print without newline using print! (will need to flush output to see immediately)
    print!("This is on the same line...");
    print!(" still on the same line!\n");
    
    // Printing with formatting: {} are placeholders for values
    println!("My name is {} and I am {} years old.", "Rustacean", 1);
    
    // Named parameters in formatting
    println!("Coordinates: {x}, {y}", x = 10, y = 20);
    
    // Different formatting options
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);
    println!("With padding: {:5}", 42); // Right-aligned with width 5
    println!("Left aligned: {:<5}", 42); // Left-aligned with width 5
    println!("Zero-padded: {:05}", 42); // Zero-padded to width 5
    
    // Debug printing with {:?} - useful for complex data structures
    println!("Debug print: {:?}", (1, 2, "hello"));
    
    // Pretty debug printing with {:#?} - adds newlines and indentation
    println!("Pretty debug: {:#?}", vec![1, 2, 3]);
    
    // Printing errors with eprintln! (goes to stderr instead of stdout)
    eprintln!("This is an error message");
    
    // ==========================================
    // SECTION 2: VARIABLES AND MUTABILITY
    // ==========================================
    
    // Variables in Rust are immutable by default
    let immutable_var = 42;
    println!("Immutable variable: {}", immutable_var);
    
    // Variables can be made mutable with 'mut' keyword
    let mut mutable_var = 42;
    println!("Mutable variable before: {}", mutable_var);
    mutable_var = 43;
    println!("Mutable variable after: {}", mutable_var);
    
    // Variable shadowing - reusing the same name
    let shadowed = 5;
    println!("Original value: {}", shadowed);
    let shadowed = shadowed + 5; // Creates a new variable, shadows the previous one
    println!("Shadowed value: {}", shadowed);
    
    // Constants - always immutable, type must be annotated
    const MAX_POINTS: u32 = 100_000;
    println!("Constant value: {}", MAX_POINTS);
    
    // Static variables - have a 'static lifetime, can be mutable (but requires unsafe)
    static LANGUAGE: &str = "Rust";
    println!("Static variable: {}", LANGUAGE);
    
    // ==========================================
    // SECTION 3: DATA TYPES
    // ==========================================
    
    // ---------------------
    // 3.1 INTEGER TYPES
    // ---------------------
    // Signed integers (can be negative or positive)
    // i8, i16, i32, i64, i128, isize
    
    let signed_i8: i8 = 127; // 8-bit signed (-128 to 127)
    println!("i8: {} (min: {}, max: {})", signed_i8, i8::MIN, i8::MAX);
    
    let signed_i16: i16 = 32767; // 16-bit signed (-32768 to 32767)
    println!("i16: {} (min: {}, max: {})", signed_i16, i16::MIN, i16::MAX);
    
    let signed_i32: i32 = 2_147_483_647; // 32-bit signed (-2147483648 to 2147483647)
    println!("i32: {} (min: {}, max: {})", signed_i32, i32::MIN, i32::MAX);
    
    let signed_i64: i64 = 9_223_372_036_854_775_807; // 64-bit signed
    println!("i64: {} (min: {}, max: {})", signed_i64, i64::MIN, i64::MAX);
    
    let signed_i128: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727; // 128-bit signed
    println!("i128: {} (min: {}, max: {})", signed_i128, i128::MIN, i128::MAX);
    
    let signed_isize: isize = 9223372036854775807; // Architecture-dependent size (64-bit on most modern systems)
    println!("isize: {} (min: {}, max: {})", signed_isize, isize::MIN, isize::MAX);
    
    // Unsigned integers (only positive, including zero)
    // u8, u16, u32, u64, u128, usize
    
    let unsigned_u8: u8 = 255; // 8-bit unsigned (0 to 255)
    println!("u8: {} (min: {}, max: {})", unsigned_u8, u8::MIN, u8::MAX);
    
    let unsigned_u16: u16 = 65535; // 16-bit unsigned (0 to 65535)
    println!("u16: {} (min: {}, max: {})", unsigned_u16, u16::MIN, u16::MAX);
    
    let unsigned_u32: u32 = 4_294_967_295; // 32-bit unsigned (0 to 4294967295)
    println!("u32: {} (min: {}, max: {})", unsigned_u32, u32::MIN, u32::MAX);
    
    let unsigned_u64: u64 = 18_446_744_073_709_551_615; // 64-bit unsigned
    println!("u64: {} (min: {}, max: {})", unsigned_u64, u64::MIN, u64::MAX);
    
    let unsigned_u128: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455; // 128-bit unsigned
    println!("u128: {} (min: {}, max: {})", unsigned_u128, u128::MIN, u128::MAX);
    
    let unsigned_usize: usize = 18446744073709551615; // Architecture-dependent size (64-bit on most modern systems)
    println!("usize: {} (min: {}, max: {})", unsigned_usize, usize::MIN, usize::MAX);
    
    // Integer literals
    let decimal = 98_222; // Decimal (using _ as separator for readability)
    let hex = 0xff; // Hex
    let octal = 0o77; // Octal
    let binary = 0b1111_0000; // Binary
    let byte = b'A'; // Byte (u8 only)
    
    println!("Integer literals - decimal: {}, hex: {}, octal: {}, binary: {}, byte: {}", 
             decimal, hex, octal, binary, byte);
    
    // Wrapping, overflowing, and checked operations
    let max_u8: u8 = u8::MAX;
    // Using wrapping_add to handle overflow (wraps around to 0)
    let wrapped = max_u8.wrapping_add(1);
    // Using checked_add to return None on overflow
    let checked = max_u8.checked_add(1);
    
    println!("Overflow handling - max u8: {}, wrapped: {}, checked: {:?}", max_u8, wrapped, checked);
    
    // ---------------------
    // 3.2 FLOATING POINT TYPES
    // ---------------------
    // f32, f64
    
    let float_f32: f32 = 3.14159265359; // Single precision
    println!("f32: {} (min: {}, max: {})", float_f32, f32::MIN, f32::MAX);
    
    let float_f64: f64 = 3.141592653589793238462643383279502884; // Double precision (default)
    println!("f64: {} (min: {}, max: {})", float_f64, f64::MIN, f64::MAX);
    
    // Special floating point values
    let infinity = f32::INFINITY;
    let neg_infinity = f32::NEG_INFINITY;
    let nan = f32::NAN;
    
    println!("Special float values - infinity: {}, negative infinity: {}, NaN: {}", 
             infinity, neg_infinity, nan);
    
    // Float constants
    println!("Float constants - PI: {}, E: {}", std::f64::consts::PI, std::f64::consts::E);
    
    // Float calculations
    let sum = 0.1_f32 + 0.2_f32;
    println!("Float precision example: 0.1 + 0.2 = {}", sum); // Not exactly 0.3 due to floating point precision
    
    // ---------------------
    // 3.3 BOOLEAN TYPE
    // ---------------------
    
    let true_val: bool = true;
    let false_val: bool = false;
    
    println!("Booleans - true: {}, false: {}", true_val, false_val);
    println!("Boolean size: {} bytes", std::mem::size_of::<bool>());
    
    // Boolean operations
    let and = true_val && false_val;
    let or = true_val || false_val;
    let not = !true_val;
    
    println!("Boolean operations - AND: {}, OR: {}, NOT: {}", and, or, not);
    
    // ---------------------
    // 3.4 CHARACTER TYPE
    // ---------------------
    
    // Char in Rust is 4 bytes and represents a Unicode Scalar Value
    let c: char = 'z';
    let z: char = 'ℤ'; // Unicode character
    let heart_eyed_cat: char = '😻'; // Emoji
    
    println!("Characters - ASCII: {}, Unicode: {}, Emoji: {}", c, z, heart_eyed_cat);
    println!("Char size: {} bytes", std::mem::size_of::<char>()); // 4 bytes
    
    // Character escapes
    let newline = '\n';
    let tab = '\t';
    let backslash = '\\';
    let single_quote = '\'';
    
    println!("Escaped characters - newline: {:?}, tab: {:?}, backslash: {}, quote: {}", 
             newline, tab, backslash, single_quote);
    
    // Character methods
    println!("Character is alphabetic: {}", c.is_alphabetic());
    println!("Character is numeric: {}", c.is_numeric());
    println!("Character as uppercase: {}", c.to_uppercase());
    
    // ---------------------
    // 3.5 COMPOUND TYPES
    // ---------------------
    
    // Tuples - fixed-length collections of values of different types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("Tuple: {:?}", tup);
    
    // Accessing tuple elements with destructuring
    let (x, y, z) = tup;
    println!("Destructured tuple: x = {}, y = {}, z = {}", x, y, z);
    
    // Accessing tuple elements with dot notation
    println!("Tuple elements: {}, {}, {}", tup.0, tup.1, tup.2);
    
    // Unit tuple - empty tuple, represents an empty value or return type
    let unit: () = ();
    println!("Unit value: {:?}", unit);
    
    // Nested tuples
    let nested_tuple = (1, (2, 3), (4, 5, 6));
    println!("Nested tuple: {:?}", nested_tuple);
    
    // Arrays - fixed length, elements of the same type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array: {:?}", arr);
    
    // Array with repeated elements
    let repeated_arr: [i32; 5] = [3; 5]; // [3, 3, 3, 3, 3]
    println!("Array with repeated elements: {:?}", repeated_arr);
    
    // Accessing array elements
    println!("First array element: {}", arr[0]);
    println!("Array length: {}", arr.len());
    
    // Multi-dimensional arrays
    let matrix: [[i32; 3]; 2] = [[1, 2, 3], [4, 5, 6]];
    println!("2D array/matrix: {:?}", matrix);
    println!("Matrix element at [1][2]: {}", matrix[1][2]);
    
    // ---------------------
    // 3.6 STRING TYPES
    // ---------------------
    
    // String literals (str) - immutable, fixed-length string stored in the binary
    let string_literal: &str = "Hello, world!";
    println!("String literal: {}", string_literal);
    
    // String type - growable, heap-allocated data structure
    let mut string = String::from("Hello");
    string.push_str(", world!"); // Append to string
    println!("String: {}", string);
    
    // Converting between String and &str
    let s: &str = &string; // Borrow a slice of the String
    let string_from_literal = string_literal.to_string(); // Convert literal to String
    
    println!("String from &str: {}", string_from_literal);
    println!("&str from String: {}", s);
    
    // String methods
    println!("String length: {}", string.len());
    println!("String is empty: {}", string.is_empty());
    println!("String contains 'world': {}", string.contains("world"));
    
    // String concatenation
    let s1 = String::from("Hello, ");
    let s2 = String::from("Rust!");
    let s3 = s1 + &s2; // Note: s1 is moved here and can no longer be used
    println!("Concatenated string: {}", s3);
    
    // Using format! macro for string formatting
    let formatted = format!("{} version {:.1}", "Rust", 1.58);
    println!("Formatted string: {}", formatted);
    
    // Unicode support in strings
    let unicode_string = "こんにちは, world! 😄";
    println!("Unicode string: {}", unicode_string);
    
    // Iterating over characters in a string
    print!("Characters in string: ");
    for c in unicode_string.chars() {
        print!("'{}' ", c);
    }
    println!();
    
    // Iterating over bytes in a string
    print!("First 10 bytes in string: ");
    for (i, b) in unicode_string.bytes().enumerate() {
        if i < 10 {
            print!("{} ", b);
        } else {
            break;
        }
    }
    println!();
    
    // Raw string literals - no escape processing
    let raw_string = r"C:\Program Files\Rust";
    println!("Raw string: {}", raw_string);
    
    // Raw string literals with # for quotes
    let raw_with_quotes = r#"This string contains "quotes""#;
    println!("Raw string with quotes: {}", raw_with_quotes);
    
    // ---------------------
    // 3.7 SLICE TYPE
    // ---------------------
    
    // Slices let you reference a contiguous sequence of elements
    let array = [1, 2, 3, 4, 5];
    let slice: &[i32] = &array[1..3]; // Elements at indices 1 and 2
    
    println!("Slice of array: {:?}", slice);
    
    let str_slice: &str = &string[0..5]; // First 5 characters
    println!("Slice of string: {}", str_slice);
    
    // Different ways to create slices
    let full_slice = &array[..]; // All elements
    let from_start = &array[..3]; // First 3 elements
    let to_end = &array[2..]; // Elements from index 2 to the end
    
    println!("Full slice: {:?}", full_slice);
    println!("From start slice: {:?}", from_start);
    println!("To end slice: {:?}", to_end);
    
    // Slice methods
    println!("Slice length: {}", slice.len());
    println!("Slice is empty: {}", slice.is_empty());
    println!("First element of slice: {:?}", slice.first());
    
    // ---------------------
    // 3.8 VECTOR TYPE
    // ---------------------
    
    // Vectors - growable arrays
    let mut vec: Vec<i32> = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    
    println!("Vector: {:?}", vec);
    
    // Creating vectors with initial values using vec! macro
    let vec2 = vec![4, 5, 6];
    println!("Vector from macro: {:?}", vec2);
    
    // Accessing vector elements
    println!("First element: {}", vec[0]);
    println!("Safe access with get: {:?}", vec.get(1)); // Returns Option<&T>
    
    // Vector methods
    println!("Vector length: {}", vec.len());
    println!("Vector capacity: {}", vec.capacity());
    println!("Vector is empty: {}", vec.is_empty());
    
    // Iterating over vector elements
    print!("Vector elements: ");
    for i in &vec {
        print!("{} ", i);
    }
    println!();
    
    // Modifying vector elements in-place
    for i in &mut vec {
        *i += 10;
    }
    println!("Modified vector: {:?}", vec);
    
    // Removing elements
    let removed = vec.pop(); // Removes and returns the last element
    println!("Removed element: {:?}, Vector: {:?}", removed, vec);
    
    // Vectors with different types using enums
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("value")),
    ];
    
    println!("Vector with enum types: {:?}", row.len());
    
    // ---------------------
    // 3.9 HASHMAP TYPE
    // ---------------------
    
    use std::collections::HashMap;
    
    // Creating a HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    
    println!("HashMap: {:?}", scores);
    
    // Accessing HashMap values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);
    println!("Score for {}: {:?}", team_name, score);
    
    // Iterating over a HashMap
    for (key, value) in &scores {
        println!("Key: {}, Value: {}", key, value);
    }
    
    // Updating a HashMap
    scores.insert(String::from("Blue"), 25); // Overwrites existing value
    println!("Updated HashMap: {:?}", scores);
    
    // Only insert if key doesn't exist
    scores.entry(String::from("Yellow")).or_insert(30);
    scores.entry(String::from("Blue")).or_insert(40); // Won't change Blue's value
    println!("HashMap after entry API: {:?}", scores);
    
    // Update value based on old value
    let text = "hello world hello rust";
    let mut word_count = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = word_count.entry(word).or_insert(0);
        *count += 1;
    }
    
    println!("Word count: {:?}", word_count);
    
    // ---------------------
    // 3.10 OPTION TYPE
    // ---------------------
    
    // Option enum is used for values that might be absent
    // enum Option<T> { Some(T), None }
    
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    
    println!("Option values - some_number: {:?}, some_string: {:?}, absent_number: {:?}", 
             some_number, some_string, absent_number);
    
    // Extracting values from Option with match
    match some_number {
        Some(i) => println!("Got a value: {}", i),
        None => println!("No value"),
    }
    
    // Using if let for concise matching
    if let Some(value) = some_string {
        println!("String value: {}", value);
    }
    
    // Common Option methods
    println!("Is some_number Some? {}", some_number.is_some());
    println!("Is absent_number None? {}", absent_number.is_none());
    println!("Unwrapped some_number: {}", some_number.unwrap()); // Panics if None
    println!("some_number unwrap_or: {}", some_number.unwrap_or(0));
    println!("absent_number unwrap_or: {}", absent_number.unwrap_or(0));
    
    // ---------------------
    // 3.11 RESULT TYPE
    // ---------------------
    
    // Result enum is used for operations that might fail
    // enum Result<T, E> { Ok(T), Err(E) }
    
    use std::fs::File;
    
    // Opening a file that may not exist
    let file_result = File::open("hello.txt");
    
    println!("File open result: {:?}", file_result);
    
    // Handling Result with match
    match file_result {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(error) => println!("Failed to open file: {:?}", error),
    }
    
    // Using Result's methods
    let result: Result<i32, &str> = Ok(42);
    let error_result: Result<i32, &str> = Err("Something went wrong");
    
    println!("Is result Ok? {}", result.is_ok());
    println!("Is error_result Err? {}", error_result.is_err());
    println!("Unwrapped result: {}", result.unwrap()); // Panics if Err
    println!("error_result unwrap_or: {}", error_result.unwrap_or(0));
    
    // ---------------------
    // 3.12 CUSTOM TYPES
    // ---------------------
    
    // Struct - named fields
    struct Person {
        name: String,
        age: u32,
        active: bool,
    }
    
    let person = Person {
        name: String::from("Alice"),
        age: 30,
        active: true,
    };
    
    println!("Person - name: {}, age: {}, active: {}", 
             person.name, person.age, person.active);
    
    // Tuple struct - unnamed fields
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    
    println!("Color: ({}, {}, {})", black.0, black.1, black.2);
    println!("Point: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // Unit struct - no fields
    struct UnitStruct;
    
    let unit = UnitStruct;
    println!("Unit struct: {:?}", std::mem::size_of_val(&unit));
    
    // Enum - define a type with multiple possible variants
    enum IpAddrKind {
        V4,
        V6,
    }
    
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    // Function taking enum as parameter
    fn route(ip_kind: IpAddrKind) {
        match ip_kind {
            IpAddrKind::V4 => println!("IPv4 route"),
            IpAddrKind::V6 => println!("IPv6 route"),
        }
    }
    
    route(four);
    route(six);
    
    // Enum with associated values
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    // Pattern matching on enum variants
    match home {
        IpAddr::V4(a, b, c, d) => println!("IPv4 address: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6 address: {}", addr),
    }
    
    match loopback {
        IpAddr::V4(a, b, c, d) => println!("IPv4 address: {}.{}.{}.{}", a, b, c, d),
        IpAddr::V6(addr) => println!("IPv6 address: {}", addr),
    }
    
    // Complex enum with different types for each variant
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    
    let quit_message = Message::Quit;
    let move_message = Message::Move { x: 10, y: 5 };
    let write_message = Message::Write(String::from("Hello"));
    let color_message = Message::ChangeColor(Color(255, 0, 0));
    
    // Using match with complex enum
    fn process_message(msg: Message) {
        match msg {
            Message::Quit => println!("Quit message received"),
            Message::Move { x, y } => println!("Move to coordinates: ({}, {})", x, y),
            Message::Write(text) => println!("Text message: {}", text),
            Message::ChangeColor(Color(r, g, b)) => println!("Change color to: rgb({}, {}, {})", r, g, b),
        }
    }
    
    process_message(quit_message);
    process_message(move_message);
    process_message(write_message);
    process_message(color_message);
    
    // ---------------------
    // 3.13 TYPE ALIASES
    // ---------------------
    
    // Type aliases create a new name for an existing type
    type Kilometers = i32;
    
    let distance: Kilometers = 5;
    println!("Distance in kilometers: {}", distance);
    
    // Commonly used for complex types to reduce repetition
    type Thunk = Box<dyn Fn() + Send + 'static>;
    
    let f: Thunk = Box::new(|| println!("Hi from a Thunk!"));
    f(); // Calls the function inside the Thunk
    
    // ---------------------
    // 3.14 NEVER TYPE
    // ---------------------
    
    // The ! type represents computations that never complete
    fn never_returns() -> ! {
        // This function never returns
        panic!("This function never returns!");
    }
    
    // Uncomment to see the effect
    // never_returns();
    
    // A common use is with loop, which can return a never type
    let never_ending_value = loop {
        // This would run forever if we didn't break
        break "We actually returned from the loop!";
    };
    
    println!("Loop returned: {}", never_ending_value);
    
    // ---------------------
    // 3.15 TYPE INFERENCE
    // ---------------------
    
    // Rust can infer types in most cases
    let inferred_type = 42; // Rust infers i32
    let inferred_float = 3.14; // Rust infers f64
    
    println!("Type inference - integer: {}, float: {}", inferred_type, inferred_float);
    
    // Type inference with generics
    let v1 = vec![1, 2, 3]; // Rust infers Vec<i32>
    let v2 = vec![1.0, 2.0, 3.0]; // Rust infers Vec<f64>
    
    println!("Type inference with generics - v1: {:?}, v2: {:?}", v1, v2);
    
    // ---------------------
    // 3.16 TYPE CASTING
    // ---------------------
    
    let num = 65;
    let character = num as u8 as char; // Convert to ASCII character
    let float_to_int = 3.99 as i32; // Truncates to 3
    
    println!("Type casting - num to char: {}, float to int: {}", character, float_to_int);
    
    // Converting between numeric types
    let large_number: i64 = 9000;
    let smaller_number = large_number as i32;
    
    println!("Type conversion - i64 to i32: {}", smaller_number);
    
    // Casting with potentially lossy conversions
    let large_value: i32 = 1000;
    let byte_value = large_value as u8; // This will truncate to 232 (1000 % 256)
    
    println!("Lossy conversion - i32 to u8: {}", byte_value);
    
    // The From and Into traits for type conversions
    let s = String::from("hello"); // From trait
    let s2: String = "world".into(); // Into trait
    
    println!("From/Into conversions: {}, {}", s, s2);
    
    // TryFrom and TryInto for fallible conversions (returns Result)
    use std::convert::TryFrom;
    
    let try_result = i32::try_from(42_i8);
    println!("TryFrom successful: {:?}", try_result);
    
    let try_result_err = i32::try_from(300_i8); // Will fail, i8 max is 127
    println!("TryFrom unsuccessful: {:?}", try_result_err);
    
    // ==========================================
    // CONCLUSION
    // ==========================================
    
    println!("\nYou've completed Day 1 of Rust programming!");
    println!("You've learned about printing, variables, data types, and custom types in Rust.");
}

// ==========================================
// SECTION 4: CUSTOM TYPE IMPLEMENTATIONS
// ==========================================

// Struct with implementation
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation block for the Rectangle struct
impl Rectangle {
    // Associated function (static method)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    // Method (takes &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method with mutable self
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    // ==========================================
// SECTION 4: CUSTOM TYPE IMPLEMENTATIONS
// ==========================================

// Struct with implementation
struct Rectangle {
    width: u32,
    height: u32,
}

// Implementation block for the Rectangle struct
impl Rectangle {
    // Associated function (static method)
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    
    // Method (takes &self)
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // Method with mutable self
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    
    // Method that takes another Rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Enum with implementation
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    fn time(&self) -> u32 {
        match self {
            TrafficLight::Red => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green => 45,
        }
    }
    
    fn next(&self) -> TrafficLight {
        match self {
            TrafficLight::Red => TrafficLight::Green,
            TrafficLight::Yellow => TrafficLight::Red,
            TrafficLight::Green => TrafficLight::Yellow,
        }
    }
}

// ==========================================
// SECTION 5: TRAITS (INTERFACES)
// ==========================================

// Define a trait (similar to interfaces in other languages)
trait Summary {
    // Method signature (must be implemented)
    fn summarize(&self) -> String;
    
    // Method with default implementation (can be overridden)
    fn default_summary(&self) -> String {
        String::from("(Read more...)")
    }
}

// Struct that will implement the trait
struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

// Implement the Summary trait for NewsArticle
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

// Another struct that will implement the trait
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

// Implement the Summary trait for Tweet
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    
    // Override the default implementation
    fn default_summary(&self) -> String {
        format!("New tweet from {}", self.username)
    }
}

// Function that takes a trait as a parameter
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// ==========================================
// SECTION 6: GENERICS
// ==========================================

// Generic struct
struct Pair<T> {
    first: T,
    second: T,
}

// Implementation for generic struct
impl<T> Pair<T> {
    fn new(first: T, second: T) -> Pair<T> {
        Pair { first, second }
    }
}

// Implementation only for specific types
impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.first >= self.second {
            println!("The largest member is {}", self.first);
        } else {
            println!("The largest member is {}", self.second);
        }
    }
}

// Generic function
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// ==========================================
// SECTION 7: MAIN FUNCTION WITH EXAMPLES OF CUSTOM TYPES
// ==========================================

fn main_custom_types() {
    println!("\n// ==========================================");
    println!("// CUSTOM TYPES EXAMPLES");
    println!("// ==========================================\n");
    
    // Using Rectangle struct and its methods
    let rect1 = Rectangle::new(30, 50);
    println!("Area of rectangle: {}", rect1.area());
    
    let mut rect2 = Rectangle { width: 10, height: 20 };
    println!("Original width: {}", rect2.width);
    rect2.set_width(15);
    println!("Modified width: {}", rect2.width);
    
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    
    // Using TrafficLight enum
    let light = TrafficLight::Red;
    println!("Red light lasts for {} seconds", light.time());
    let next_light = light.next();
    match next_light {
        TrafficLight::Red => println!("Next light is Red"),
        TrafficLight::Yellow => println!("Next light is Yellow"),
        TrafficLight::Green => println!("Next light is Green"),
    }
    
    // Using traits
    let article = NewsArticle {
        headline: String::from("Rust 2.0 Released!"),
        location: String::from("San Francisco"),
        author: String::from("Jane Doe"),
        content: String::from("Rust 2.0 includes many new features..."),
    };
    
    let tweet = Tweet {
        username: String::from("rust_lang"),
        content: String::from("Just announced: Rust 2.0!"),
        reply: false,
        retweet: false,
    };
    
    println!("Article summary: {}", article.summarize());
    println!("Tweet summary: {}", tweet.summarize());
    
    println!("Article default summary: {}", article.default_summary());
    println!("Tweet default summary: {}", tweet.default_summary());
    
    // Using the notify function with trait parameter
    notify(&article);
    notify(&tweet);
    
    // Using generics
    let pair = Pair::new(10, 5);
    pair.cmp_display();
    
    let pair_strings = Pair::new(String::from("hello"), String::from("world"));
    pair_strings.cmp_display();
    
    // Using generic function
    let numbers = vec![34, 50, 25, 100, 65];
    println!("The largest number is {}", largest(&numbers));
    
    let characters = vec!['y', 'm', 'a', 'q'];
    println!("The largest character is {}", largest(&characters));
}

// ==========================================
// SECTION 8: MEMORY MANAGEMENT AND OWNERSHIP
// ==========================================

fn main_ownership() {
    println!("\n// ==========================================");
    println!("// MEMORY MANAGEMENT AND OWNERSHIP");
    println!("// ==========================================\n");
    
    // OWNERSHIP RULES:
    // 1. Each value in Rust has a variable that's called its owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.
    
    // Simple ownership example
    {
        let s = String::from("hello"); // s is valid here
        println!("String: {}", s);
        // s is still valid here
    } // s goes out of scope and is dropped
    
    // Move semantics
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid
    // println!("{}", s1); // This would cause a compile error
    println!("After move: {}", s2);
    
    // Clone to prevent move
    let s1 = String::from("hello");
    let s2 = s1.clone(); // Deep copy of s1 to s2
    println!("Original: {}, Clone: {}", s1, s2);
    
    // Copy for stack-only data (primitives)
    let x = 5;
    let y = x; // x is copied to y, both are valid
    println!("x = {}, y = {}", x, y);
    
    // References and borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Immutable borrow
    println!("The length of '{}' is {}.", s1, len);
    
    // Mutable references
    let mut s = String::from("hello");
    change(&mut s); // Mutable borrow
    println!("After change: {}", s);
    
    // Rules for references:
    // 1. At any given time, you can have either one mutable reference or any number of immutable references.
    // 2. References must always be valid.
    
    // Multiple immutable references are allowed
    let s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("Multiple immutable refs: {} and {}", r1, r2);
    
    // But can't have mutable and immutable references in same scope
    let mut s = String::from("hello");
    let r1 = &s; // Immutable borrow
    let r2 = &s; // Immutable borrow
    // let r3 = &mut s; // Error: can't borrow as mutable when already borrowed as immutable
    println!("Immutable refs: {} and {}", r1, r2);
    
    // After the last usage of immutable refs, we can create a mutable ref
    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("Immutable refs: {} and {}", r1, r2);
    // r1 and r2 are no longer used after this point
    
    let r3 = &mut s; // This is fine
    println!("Mutable ref: {}", r3);
    
    // The Slice Type - a reference to a part of a collection
    let s = String::from("hello world");
    let hello = &s[0..5]; // Reference to "hello"
    let world = &s[6..11]; // Reference to "world"
    println!("Slices: '{}' and '{}'", hello, world);
    
    // String literals are slices
    let s: &str = "Hello, world!"; // s is a slice pointing to that specific point of the binary
    println!("String literal (slice): {}", s);
}

// Function that borrows a value
fn calculate_length(s: &String) -> usize {
    s.len()
} // s goes out of scope, but it doesn't drop what it refers to because it doesn't have ownership

// Function that modifies a borrowed value
fn change(s: &mut String) {
    s.push_str(", world");
}

// ==========================================
// SECTION 9: ERROR HANDLING
// ==========================================

fn main_error_handling() {
    println!("\n// ==========================================");
    println!("// ERROR HANDLING");
    println!("// ==========================================\n");
    
    // Unrecoverable errors with panic!
    // Uncomment to see panic
    // panic!("crash and burn");
    
    // Recoverable errors with Result
    use std::fs::File;
    use std::io::ErrorKind;
    
    // Handling error with match
    let file_result = File::open("hello.txt");
    match file_result {
        Ok(file) => println!("File opened successfully: {:?}", file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("File not found"),
            ErrorKind::PermissionDenied => println!("Permission denied"),
            _ => println!("Other error: {:?}", error),
        },
    }
    
    // More concise error handling with closures
    let file_result = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            println!("File not found, creating it");
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Failed to create file: {:?}", error);
            })
        } else {
            panic!("Failed to open file: {:?}", error);
        }
    });
    
    // Shortcuts: unwrap and expect
    // Unwrap: returns value if Ok, panics if Err
    // let file = File::open("hello.txt").unwrap(); // Panics if file doesn't exist
    
    // Expect: like unwrap, but with custom panic message
    // let file = File::open("hello.txt").expect("Failed to open hello.txt"); // Better panic message
    
    // Propagating errors using the ? operator
    fn read_username_from_file() -> Result<String, std::io::Error> {
        use std::io::Read;
        
        let mut file = File::open("hello.txt")?; // Returns error if there is one
        let mut username = String::new();
        file.read_to_string(&mut username)?; // Returns error if there is one
        Ok(username)
    }
    
    // Even shorter with chaining
    fn read_username_shorter() -> Result<String, std::io::Error> {
        use std::io::Read;
        
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
    
    // Shortest with fs::read_to_string
    fn read_username_shortest() -> Result<String, std::io::Error> {
        std::fs::read_to_string("hello.txt")
    }
    
    // Using the Result of these functions
    match read_username_from_file() {
        Ok(username) => println!("Username: {}", username),
        Err(e) => println!("Error reading username: {:?}", e),
    }
}

// ==========================================
// SECTION 10: PUTTING IT ALL TOGETHER
// ==========================================

fn main() {
    println!("// ==========================================");
    println!("// RUST PROGRAMMING - DAY 1");
    println!("// ==========================================\n");
    
    println!("This tutorial covers all the basics of Rust programming.");
    println!("Let's start with a simple Hello World program.");
    
    // Hello world
    println!("\nHello, Rust World!");
    
    // Call the other main functions for each section
    main_custom_types();
    main_ownership();
    main_error_handling();
    
    println!("\n// ==========================================");
    println!("// CONCLUSION");
    println!("// ==========================================\n");
    
    println!("You've completed Day 1 of Rust programming!");
    println!("You've learned about:");
    println!("1. Printing and basic syntax");
    println!("2. Variables and mutability");
    println!("3. All of Rust's data types (primitives and compound types)");
    println!("4. Custom types (structs and enums)");
    println!("5. Traits (interfaces)");
    println!("6. Generics");
    println!("7. Memory management and ownership");
    println!("8. Error handling");
    
    println!("\nKeep practicing and building more complex programs!");
}