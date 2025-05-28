// ===================================================
// RUST PROGRAMMING - DAY 1 (IMPROVED VERSION)
// ===================================================
// A comprehensive introduction to Rust programming language
// Covering: Basics, Data Types, Custom Types, Memory Management,
// Error Handling, Generics, Traits, and more

// The 'main' function is the entry point of every Rust program
fn main() {
    print_section_header("WELCOME TO RUST DAY 1");
    println!("This comprehensive tutorial covers all the fundamentals of Rust programming.");
    println!("Each section builds on the previous one, with practical examples to reinforce learning.");
    println!("Let's begin our journey into Rust!\n");

    // Call each section function
    section_printing();
    section_variables();
    section_data_types();
    section_custom_types();
    section_ownership();
    section_error_handling();
    section_traits();
    section_generics();
    section_advanced_features();
    
    print_conclusion();
}

// Helper function to print consistent section headers
fn print_section_header(title: &str) {
    println!("\n// ===================================================");
    println!("// {}", title);
    println!("// ===================================================");
}

// ==========================================
// SECTION 1: PRINTING IN RUST
// ==========================================
fn section_printing() {
    print_section_header("SECTION 1: PRINTING IN RUST");
    
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

    // Formatting with precision for floating-point numbers
    println!("Pi with 2 decimal places: {:.2}", 3.14159265359);
    println!("Pi with 5 decimal places: {:.5}", 3.14159265359);
    
    // Printing with width and alignment
    println!("{:-^20}", "centered"); // Center-aligned with dashes
    println!("{:_^20}", "centered"); // Center-aligned with underscores
    println!("{:*>10}", "right"); // Right-aligned with asterisks
    println!("{:.<10}", "left"); // Left-aligned with dots
}

// ==========================================
// SECTION 2: VARIABLES AND MUTABILITY
// ==========================================
fn section_variables() {
    print_section_header("SECTION 2: VARIABLES AND MUTABILITY");
    
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
    
    // Shadowing can also change type
    let spaces = "   "; // string type
    println!("Original spaces (length): {}", spaces.len());
    let spaces = spaces.len(); // now it's a number
    println!("Spaces count: {}", spaces);
    
    // Constants - always immutable, type must be annotated
    const MAX_POINTS: u32 = 100_000;
    println!("Constant value: {}", MAX_POINTS);
    
    // Static variables - have a 'static lifetime, can be mutable (but requires unsafe)
    static LANGUAGE: &str = "Rust";
    println!("Static variable: {}", LANGUAGE);
    
    // Scope demonstration
    {
        // This variable only exists in this scope
        let scoped_variable = "I'm scoped!";
        println!("Inside scope: {}", scoped_variable);
    }
    // println!("Outside scope: {}", scoped_variable); // This would error - scoped_variable not found
    
    // Multiple variable declaration
    let (x, y, z) = (1, 2, 3);
    println!("Multiple variables: x={}, y={}, z={}", x, y, z);
}

// ==========================================
// SECTION 3: DATA TYPES
// ==========================================
fn section_data_types() {
    print_section_header("SECTION 3: DATA TYPES");
    
    // ---------------------
    // 3.1 INTEGER TYPES
    // ---------------------
    println!("\n// INTEGERS:");
    
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
    println!("\n// UNSIGNED INTEGERS:");
    
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
    println!("\n// INTEGER LITERALS:");
    
    let decimal = 98_222; // Decimal (using _ as separator for readability)
    let hex = 0xff; // Hex
    let octal = 0o77; // Octal
    let binary = 0b1111_0000; // Binary
    let byte = b'A'; // Byte (u8 only)
    
    println!("Integer literals - decimal: {}, hex: {}, octal: {}, binary: {}, byte: {}", 
             decimal, hex, octal, binary, byte);
    
    // Wrapping, overflowing, and checked operations
    println!("\n// OVERFLOW HANDLING:");
    
    let max_u8: u8 = u8::MAX;
    // Using wrapping_add to handle overflow (wraps around to 0)
    let wrapped = max_u8.wrapping_add(1);
    // Using checked_add to return None on overflow
    let checked = max_u8.checked_add(1);
    // Using saturating_add to cap at the max value
    let saturated = max_u8.saturating_add(1);
    
    println!("Overflow handling - max u8: {}, wrapped: {}, checked: {:?}, saturated: {}", 
             max_u8, wrapped, checked, saturated);
    
    // ---------------------
    // 3.2 FLOATING POINT TYPES
    // ---------------------
    println!("\n// FLOATING POINT TYPES:");
    
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
    println!("\n// BOOLEAN TYPE:");
    
    let true_val: bool = true;
    let false_val: bool = false;
    
    println!("Booleans - true: {}, false: {}", true_val, false_val);
    println!("Boolean size: {} bytes", std::mem::size_of::<bool>());
    
    // Boolean operations
    let and = true_val && false_val;
    let or = true_val || false_val;
    let not = !true_val;
    
    println!("Boolean operations - AND: {}, OR: {}, NOT: {}", and, or, not);
    
    // Boolean with control flow
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Conditional value: {}", number);
    
    // ---------------------
    // 3.4 CHARACTER TYPE
    // ---------------------
    println!("\n// CHARACTER TYPE:");
    
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
    
    // Character conversions
    let digit = '9';
    let digit_value = digit.to_digit(10).unwrap(); // Convert to numeric value in base 10
    println!("Character '9' as number: {}", digit_value);
    println!("Character from number: {}", std::char::from_digit(5, 10).unwrap());
    
    // ---------------------
    // 3.5 COMPOUND TYPES
    // ---------------------
    println!("\n// COMPOUND TYPES:");
    println!("\n// TUPLES:");
    
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
    println!("Accessing nested element: {}", (nested_tuple.2).0);
    
    // Function returning a tuple
    fn return_tuple() -> (i32, &'static str) {
        (42, "Answer")
    }
    let (value, description) = return_tuple();
    println!("Tuple from function: {} is the {}", value, description);
    
    println!("\n// ARRAYS:");
    
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
    
    // Array slicing
    let slice = &arr[1..4]; // Elements at indices 1, 2, and 3
    println!("Array slice: {:?}", slice);
    
    // ---------------------
    // 3.6 STRING TYPES
    // ---------------------
    println!("\n// STRING TYPES:");
    
    // String literals (str) - immutable, fixed-length string stored in the binary
    let string_literal: &str = "Hello, world!";
    println!("String literal: {}", string_literal);
    
    // String type - growable, heap-allocated data structure
    let mut string = String::from("Hello");
    string.push_str(", world!"); // Append to string
    println!("String: {}", string);
    
    // String methods for manipulation
    string.push('!'); // Add a single character
    println!("After push: {}", string);
    
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
    
    // Multi-line strings
    let multiline = "This is a
multiline string
in Rust";
    println!("Multiline string:\n{}", multiline);
    
    // String splitting and joining
    let words: Vec<&str> = "hello world rust programming".split(' ').collect();
    println!("Split words: {:?}", words);
    let joined = words.join("-");
    println!("Joined words: {}", joined);
    
    // String replacing
    let replaced = "Hello, world!".replace("world", "Rust");
    println!("Replaced string: {}", replaced);
    
    // ---------------------
    // 3.7 SLICE TYPE
    // ---------------------
    println!("\n// SLICE TYPE:");
    
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
    println!("\n// VECTOR TYPE:");
    
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
    
    // Insert element at specific position
    vec.insert(1, 25);
    println!("After insert: {:?}", vec);
    
    // Remove element at specific position
    vec.remove(0);
    println!("After remove: {:?}", vec);
    
    // Vector with different types using enum
    #[derive(Debug)]
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
    
    println!("Vector with enum types: {:?}", row);
    
    // Sorting a vector
    let mut numbers = vec![5, 1, 8, 3, 2];
    numbers.sort();
    println!("Sorted vector: {:?}", numbers);
    
    // Deduplicating a sorted vector
    let mut with_duplicates = vec![1, 2, 2, 3, 4, 4, 4, 5];
    with_duplicates.dedup();
    println!("Deduplicated vector: {:?}", with_duplicates);
    
    // ---------------------
    // 3.9 HASHMAP TYPE
    // ---------------------
    println!("\n// HASHMAP TYPE:");
    
    use std::collections::HashMap;
    
    // Creating a HashMap
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Red"), 50);
    
    println!("HashMap: {:?}", scores);
    
    // Creating from vectors
    let teams = vec![String::from("Yellow"), String::from("Green")];
    let initial_scores = vec![25, 30];
    
    let mut team_scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("HashMap from iterators: {:?}", team_scores);
    
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
    
    // Removing entries
    scores.remove("Red");
    println!("After remove: {:?}", scores);
    
    // Checking if key exists
    println!("Contains 'Blue'? {}", scores.contains_key("Blue"));
    
    // ---------------------
    // 3.10 OPTION TYPE
    // ---------------------
    println!("\n// OPTION TYPE:");
    
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
    
    // Map, and_then, filter for transforming Options
    let mapped = some_number.map(|x| x * 2);
    println!("Mapped option: {:?}", mapped);
    
    let filtered = some_number.filter(|x| *x > 10);
    println!("Filtered option: {:?}", filtered); // None because 5 is not > 10
    
    // ---------------------
    // 3.11 RESULT TYPE
    // ---------------------
    println!("\n// RESULT TYPE:");
    
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
    
    // Transforming Results
    let mapped_result = result.map(|x| x * 2);
    println!("Mapped result: {:?}", mapped_result);
    
    // Chaining results with and_then
    let chained = result.and_then(|x| Ok(x + 1));
    println!("Chained result: {:?}", chained);
        
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
