// The main function, the entry point of every Rust program.
fn main() {
    println!("--- Rust Scope and Ownership Examples ---");

    // --- 1. Variable Scopes: The Lifetime of a Binding ---
    println!("\n--- 1. Variable Scopes ---");
    variable_scopes();

    // --- 2. The `String` Type: Heap Allocation and Ownership ---
    println!("\n--- 2. The `String` Type ---");
    string_type_and_heap();

    // --- 3. Ownership and Moves: Transferring Responsibility (Heap Data) ---
    println!("\n--- 3. Ownership and Moves ---");
    ownership_and_moves();

    // --- 4. The `Copy` Trait: Duplicating Data on the Stack ---
    println!("\n--- 4. The `Copy` Trait ---");
    copy_trait_behavior();

    // --- 5. Ownership and Functions: Passing Values Around ---
    println!("\n--- 5. Ownership and Functions ---");
    ownership_with_functions();

    // --- 6. References and Borrowing: Accessing Data Without Ownership ---
    println!("\n--- 6. References and Borrowing ---");
    references_and_borrowing();

    // --- 7. Mutable References: Modifying Borrowed Data ---
    println!("\n--- 7. Mutable References ---");
    mutable_references();

    // --- 8. The Rules of References: Ensuring Safety ---
    println!("\n--- 8. The Rules of References ---");
    rules_of_references();

    // --- 9. Slices: References to a Portion of a Collection ---
    println!("\n--- 9. Slices ---");
    slices_example();

    // --- 10. Lifetimes: Ensuring References Are Always Valid ---
    println!("\n--- 10. Lifetimes ---");
    lifetimes_explained();

    // --- 11. Structs and Ownership ---
    println!("\n--- 11. Structs and Ownership ---");
    structs_and_ownership();

    // --- 12. Methods, `self`, and Borrowing ---
    println!("\n--- 12. Methods, `self`, and Borrowing ---");
    methods_self_and_borrowing();

    // --- 13. Enums and Ownership ---
    println!("\n--- 13. Enums and Ownership ---");
    enums_and_ownership();

    // --- 14. Collections and Ownership (`Vec<T>`, `HashMap<K, V>`) ---
    println!("\n--- 14. Collections and Ownership ---");
    collections_and_ownership();

    // --- 15. Error Handling with `Result<T, E>` and Ownership ---
    println!("\n--- 15. Error Handling with `Result<T, E>` ---");
    result_and_ownership();

    // --- 16. Closures and Capturing Environment Variables ---
    println!("\n--- 16. Closures and Capturing ---");
    closures_and_capturing();

    // --- 17. Interior Mutability: `Cell<T>` and `RefCell<T>` (A Brief Look) ---
    println!("\n--- 17. Interior Mutability (Brief Look) ---");
    interior_mutability_glimpse();

    // --- 18. Shared Ownership: `Rc<T>` and `Arc<T>` (A Brief Look) ---
    println!("\n--- 18. Shared Ownership (Brief Look) ---");
    shared_ownership_glimpse();
}

// --- 1. Variable Scopes: The Lifetime of a Binding ---
fn variable_scopes() {
    // `outer_scope_var` is declared here. It's valid within this function.
    let outer_scope_var = "I'm in the outer scope.";
    println!("{}", outer_scope_var);

    {
        // This is a new, inner scope.
        // `inner_scope_var` is only valid within these curly braces.
        let inner_scope_var = "I'm in an inner scope!";
        println!("{}", inner_scope_var);
        println!("Still in inner scope, can access: {}", outer_scope_var);
    } // `inner_scope_var` goes out of scope here and is dropped.

    // Trying to use `inner_scope_var` here would be a compile-time error:
    // println!("{}", inner_scope_var); // Error: not found in this scope

    // Shadowing: You can declare a new variable with the same name as a previous one.
    // This new variable "shadows" the previous one.
    let shadowed_var = 5;
    println!("Shadowed var (original): {}", shadowed_var); // Prints 5
    {
        let shadowed_var = shadowed_var * 2; // This is a new variable, shadowing the outer one.
        println!("Shadowed var (inner scope): {}", shadowed_var); // Prints 10
    }
    println!("Shadowed var (after inner scope): {}", shadowed_var); // Prints 5, the original is back in scope.

    let shadowed_var = "Now I'm a string!"; // Shadowing can also change the type.
    println!("Shadowed var (new type): {}", shadowed_var);
} // `outer_scope_var` and the final `shadowed_var` go out of scope here.

// --- 2. The `String` Type: Heap Allocation and Ownership ---
fn string_type_and_heap() {
    // String literals (`&str`) are slices that point to hardcoded text in the program's binary.
    // They are immutable and have a fixed size known at compile time.
    let literal_slice: &str = "I am a string literal (immutable, stack/static)";
    println!("{}", literal_slice);

    // The `String` type is a growable, mutable, owned string.
    // It's allocated on the heap, so its size can change at runtime.
    // `String::from` creates a `String` from a string literal.
    // This involves allocating memory on the heap to hold "Hello, heap!".
    let s1: String = String::from("Hello, heap!"); // `s1` is the owner of this heap data.
    println!("{} - (length: {}, capacity: {})", s1, s1.len(), s1.capacity());

    // We can mutate an owned `String` if it's marked `mut`.
    let mut s2 = String::from("Initial");
    s2.push_str(" and appended."); // Modifies the string on the heap.
    println!("{}", s2);

} // When `s1` and `s2` go out of scope here, Rust automatically calls a special function
  // called `drop` for each `String`. The `drop` function for `String` deallocates
  // the memory on the heap that `s1` and `s2` owned. This prevents memory leaks.

// --- 3. Ownership and Moves: Transferring Responsibility (Heap Data) ---
fn ownership_and_moves() {
    let str_owner1 = String::from("data on heap"); // `str_owner1` owns the "data on heap".
                                                 // It holds a pointer to the heap, length, and capacity.

    // When we assign `str_owner1` to `str_owner2`, a "move" occurs for heap-allocated data.
    // What's copied: The pointer, length, and capacity (which are on the stack).
    // What's NOT copied: The actual data on the heap ("data on heap").
    // Instead, ownership of that heap data is transferred from `str_owner1` to `str_owner2`.
    let str_owner2 = str_owner1;

    // `str_owner1` is no longer considered valid. Rust invalidates it to prevent
    // a "double free" error (where both `str_owner1` and `str_owner2` might try to
    // free the same memory when they go out of scope).
    // println!("str_owner1: {}", str_owner1); // COMPILE-TIME ERROR: borrow of moved value: `str_owner1`

    // `str_owner2` is now the sole owner of the data.
    println!("str_owner2: {}", str_owner2);

    // If you truly need a deep copy of heap data (like a `String`), use the `clone()` method.
    let original_string = String::from("clone me");
    let cloned_string = original_string.clone(); // This allocates new memory on the heap
                                                 // and copies the content of `original_string`.

    println!("Original: {} (still valid)", original_string);
    println!("Cloned:   {}", cloned_string);

} // `str_owner2`, `original_string`, and `cloned_string` go out of scope. Their `drop`
  // methods are called, freeing their respective heap data.

// --- 4. The `Copy` Trait: Duplicating Data on the Stack ---
fn copy_trait_behavior() {
    // Integers and other simple types that are stored entirely on the stack implement the `Copy` trait.
    let x: i32 = 5; // `x` is an i32, stored on the stack.

    // When we assign `x` to `y`, the value `5` is actually copied.
    // This is a bit-for-bit copy because the entire data is on the stack and small.
    let y: i32 = x;

    // Both `x` and `y` are valid and independent.
    println!("x = {}, y = {}", x, y);
    // No "move" occurs for types that implement `Copy`.

    // Common types that are `Copy`:
    // - All integer types (i8, u32, isize, etc.)
    // - Boolean type (bool)
    // - Floating-point types (f32, f64)
    // - Character type (char)
    // - Tuples, if ALL their elements are also `Copy`.
    let point1 = (10, 20); // (i32, i32) is `Copy` because i32 is `Copy`.
    let point2 = point1;
    println!("Point1: {:?}, Point2: {:?}", point1, point2); // Both valid

    // A tuple containing a non-`Copy` type (like `String`) is NOT `Copy`.
    let data1 = (String::from("hello"), 42); // (String, i32) is NOT `Copy`.
    let data2 = data1; // This is a MOVE because String is not Copy.
                       // `data1.0` (the String) is moved.
                       // `data1.1` (the i32) is copied.
    // println!("Data1: {:?}", data1); // COMPILE-TIME ERROR: `data1.0` (the String part) was moved.
    println!("Data2: {:?}", data2);
}

// --- 5. Ownership and Functions: Passing Values Around ---
fn ownership_with_functions() {
    // Scenario 1: Passing an owned value (like String) to a function.
    let s_main = String::from("owned by main");
    takes_ownership_of_string(s_main); // `s_main`'s ownership is MOVED into the function.
    // `s_main` is no longer valid here.
    // println!("{}", s_main); // COMPILE-TIME ERROR!

    // Scenario 2: Passing a `Copy` type value to a function.
    let num_main = 100; // i32 is `Copy`.
    makes_copy_of_i32(num_main); // A copy of `num_main`'s value (100) is passed.
    println!("num_main is still valid: {}", num_main); // `num_main` is still valid.

    // Scenario 3: Function returns an owned value.
    let returned_string = gives_back_ownership(); // `returned_string` takes ownership.
    println!("Received ownership of: {}", returned_string);

    // Scenario 4: Function takes ownership and then returns ownership.
    let string_to_pass_and_get_back = String::from("cycle me");
    let new_owner_string = takes_and_returns_ownership(string_to_pass_and_get_back);
    // `string_to_pass_and_get_back` was moved into the function.
    // `new_owner_string` now owns the String (possibly modified) returned by the function.
    println!("New owner of cycled string: {}", new_owner_string);
}

// This function takes ownership of the `String` passed to it.
fn takes_ownership_of_string(some_string: String) { // `some_string` receives ownership.
    println!("Inside takes_ownership_of_string: {}", some_string);
} // `some_string` goes out of scope, and its `drop` method is called, freeing memory.

// This function receives a copy of an i32.
fn makes_copy_of_i32(some_integer: i32) { // `some_integer` is a copy.
    println!("Inside makes_copy_of_i32: {}", some_integer);
} // `some_integer` goes out of scope. Nothing special for `Copy` types on drop.

// This function creates a String and returns ownership of it.
fn gives_back_ownership() -> String {
    let new_string = String::from("I am newly created");
    new_string // Ownership of `new_string` is MOVED to the calling function.
}

// This function takes ownership of a String and returns ownership of (potentially the same or a new) String.
fn takes_and_returns_ownership(mut a_string: String) -> String {
    a_string.push_str(" and modified");
    a_string // Ownership is MOVED out.
}

// --- 6. References and Borrowing: Accessing Data Without Ownership ---
fn references_and_borrowing() {
    let s1 = String::from("hello from borrowing example");

    // Instead of transferring ownership, we can create a *reference* to `s1`.
    // A reference allows you to use the value but does not own it. This is called *borrowing*.
    // `&s1` creates an immutable reference to `s1`.
    let len = calculate_length_via_borrow(&s1); // Pass a reference to s1.
                                               // `s1` is NOT moved.

    println!("The string '{}' has length {}.", s1, len); // `s1` is still valid and owned here.

    // Attempting to modify data through an immutable reference is a compile-time error.
    // let s_ref = &s1;
    // s_ref.push_str(" can't do this"); // COMPILE-TIME ERROR: `s_ref` is an `&` reference, so the data it refers to cannot be borrowed as mutable
}

// `s: &String` means this function borrows an immutable reference to a String.
// It does not take ownership.
fn calculate_length_via_borrow(s_ref: &String) -> usize {
    // `s_ref` can be used to access the data of the String it points to.
    let length = s_ref.len();
    length
} // `s_ref` goes out of scope here. Since it doesn't own the data,
  // the `String` that `s1` owns (in `references_and_borrowing`) is NOT dropped.

// --- 7. Mutable References: Modifying Borrowed Data ---
fn mutable_references() {
    // To modify borrowed data, we need a *mutable reference*.
    // The original variable must also be declared as `mut`.
    let mut s = String::from("initial text");
    println!("Before change: {}", s);

    // `&mut s` creates a mutable reference to `s`.
    change_string_via_mutable_borrow(&mut s);

    println!("After change: {}", s); // `s` has been modified.
}

// `some_string: &mut String` means this function borrows a *mutable* reference to a String.
fn change_string_via_mutable_borrow(some_string: &mut String) {
    some_string.push_str("... and now it's changed!"); // We can modify it.
}

// --- 8. The Rules of References: Ensuring Safety ---
fn rules_of_references() {
    // Rust enforces these rules at compile time to prevent data races:
    // 1. At any given time, you can have EITHER:
    //    * One mutable reference (`&mut T`).
    //    OR
    //    * Any number of immutable references (`&T`).
    // 2. References must always be valid (this is where lifetimes, covered later, help).

    let mut data = String::from("shared data");

    // Example: Multiple immutable references are OK.
    let r1 = &data;
    let r2 = &data;
    println!("Immutable borrows: r1 = '{}', r2 = '{}'", r1, r2);
    // After r1 and r2 are used here, their "borrow" effectively ends,
    // thanks to Non-Lexical Lifetimes (NLL). The compiler sees they aren't used further.

    // Example: One mutable reference is OK (after immutable ones are no longer active).
    let r3 = &mut data;
    r3.push_str(" (modified by r3)");
    println!("Mutable borrow: r3 = '{}'", r3);
    // r3's borrow ends here.

    // WHAT'S NOT ALLOWED:
    // let mut s_err = String::from("error case");
    // let ref_immut = &s_err;         // Immutable borrow starts
    // let ref_mut = &mut s_err;       // COMPILE-TIME ERROR! Cannot have a mutable borrow
    //                                // while an immutable borrow (`ref_immut`) exists.
    // println!("{}, {}", ref_immut, ref_mut);

    // WHAT'S ALSO NOT ALLOWED:
    // let mut s_err2 = String::from("another error");
    // let ref_mut1 = &mut s_err2;     // First mutable borrow starts
    // let ref_mut2 = &mut s_err2;     // COMPILE-TIME ERROR! Cannot have a second mutable borrow
    //                                // while the first one (`ref_mut1`) exists.
    // println!("{}, {}", ref_mut1, ref_mut2);

    println!("(Compile errors for invalid reference mixing are commented out above)");
}

// --- 9. Slices: References to a Portion of a Collection ---
fn slices_example() {
    let full_string = String::from("Hello beautiful Rust world!");

    // A string slice (`&str`) is a reference to a part of a `String` or a string literal.
    // It does not own the data.
    let hello_slice: &str = &full_string[0..5]; // Slice from index 0 up to (but not including) 5.
    let world_slice: &str = &full_string[20..25]; // "world"
    // Omitting start means from index 0: `&full_string[..5]`
    // Omitting end means to the end: `&full_string[6..]`
    // Omitting both means the whole string: `&full_string[..]` which is `&String -> &str` coercion.

    println!("Slice 1: '{}'", hello_slice);
    println!("Slice 2: '{}'", world_slice);

    // String literals are already slices:
    let literal_example: &str = "This is a literal slice.";
    println!("{}", literal_example);

    // Slices also work for other collections like arrays and vectors.
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let number_slice: &[i32] = &numbers[1..3]; // Contains `[2, 3]`
    println!("Number slice: {:?}", number_slice); // Use `{:?}` (debug format) for arrays/slices.

    // IMPORTANT: If you have an immutable slice (borrow) of a String, you cannot
    // simultaneously have a mutable borrow of the original String that might invalidate the slice.
    let mut s_for_slice = String::from("clear me");
    let slice_of_s = &s_for_slice[0..5];
    // s_for_slice.clear(); // COMPILE-TIME ERROR! `clear` needs a mutable borrow of `s_for_slice`,
                           // but `slice_of_s` holds an immutable borrow.
                           // This prevents `slice_of_s` from becoming a dangling pointer.
    println!("Slice of s_for_slice: '{}' (original string cannot be cleared yet)", slice_of_s);
}

// --- 10. Lifetimes: Ensuring References Are Always Valid ---
// Lifetimes are a way for the Rust compiler to ensure that references are always valid
// and do not "dangle" (point to memory that has been deallocated or is out of scope).
// Often, the compiler can infer lifetimes (this is called "lifetime elision").
// Sometimes, you need to specify them explicitly using an apostrophe (e.g., `'a`).

// This function's purpose is to return the longer of two string slices.
// The returned slice (`&'a str`) must be valid for as long as the *shorter*
// of the lifetimes of the input slices `x` and `y`.
// `'a` is a generic lifetime parameter.
fn longest_slice<'a>(x: &'a str, y: &'a str) -> &'a str {
    // The annotation means: "for some lifetime 'a,
    // `x` is a reference valid for 'a', `y` is a reference valid for 'a',
    // and the returned reference will also be valid for 'a'."
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn lifetimes_explained() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz"); // `string2` has a shorter lifetime (this inner scope).
        result = longest_slice(string1.as_str(), string2.as_str());
        // `result` gets the lifetime of `string2` because it's shorter.
        println!("The longest string inside inner scope: {}", result); // OK
    }
    // `string2` is dropped here.
    // If `result` referred to `string2`, it would now be a dangling reference.
    // println!("The longest string outside inner scope: {}", result); // COMPILE-TIME ERROR!
                                                                  // `result` does not live long enough.
                                                                  // The borrow checker prevents this because `result`'s
                                                                  // lifetime (tied to `string2`) ended with the inner scope.

    println!("(The commented-out line above demonstrates a lifetime error the compiler prevents.)");

    // A valid case:
    let s_a = "short"; // String literal, lives for the static lifetime of the program.
    let s_b = "very long string";
    let longest_static = longest_slice(s_a, s_b);
    println!("Longest of static strings: {}", longest_static); // OK

    // Example of what lifetimes prevent (dangling reference):
    // fn dangling_reference_example() -> &String { // ERROR: missing lifetime specifier
    //     let s = String::from("temporary");
    //     &s // Returning a reference to `s`
    // } // But `s` is dropped here, so the reference would be invalid!
      // The compiler would demand a lifetime for the return type, and then realize
      // that no such lifetime can satisfy the situation where `s` is local.
}

// --- 11. Structs and Ownership ---
#[derive(Debug)] // Allows printing the struct with `{:?}`
struct Book {
    title: String,   // This field owns its String data (heap-allocated).
    author: String,  // This field also owns its String data.
    pages: u32,      // This field is a `Copy` type.
}

#[derive(Debug, Copy, Clone)] // `Point` can be `Copy` because all its fields are `Copy`.
struct Point {
    x: i32,
    y: i32,
}

fn structs_and_ownership() {
    let book1 = Book {
        title: String::from("Rust for Rustaceans"),
        author: String::from("Jon Gjengset"),
        pages: 450,
    };
    println!("Book 1: {:?}", book1);

    // When assigning a struct with owned data, the owned fields are MOVED.
    // `book1.title` and `book1.author` are moved to `book2`. `book1.pages` is copied.
    let book2 = book1;
    // println!("Book 1 after move: {:?}", book1); // COMPILE-TIME ERROR! `book1.title` and `book1.author` moved.
                                                 // Accessing `book1.pages` would be fine, but not the whole struct.
    println!("Book 2 (moved from book1): {:?}", book2);

    // Structs with all `Copy` fields are `Copy` themselves (if explicitly derived).
    let p1 = Point { x: 10, y: 20 };
    let p2 = p1; // `p1` is copied to `p2` because `Point` is `Copy`.
    println!("Point p1: {:?}, Point p2: {:?}", p1, p2); // Both `p1` and `p2` are valid.

    // Partial move: Moving a field out of a struct.
    let mut user_profile = Book {
        title: String::from("My Autobiography"),
        author: String::from("Me"),
        pages: 100,
    };
    let user_name = user_profile.author; // `user_profile.author` (String) is MOVED to `user_name`.
                                         // `user_profile` is now partially moved.
    println!("User name extracted: {}", user_name);
    // We can still access `Copy` fields or fields not moved:
    println!("Book title: {}", user_profile.title); // OK
    println!("Book pages: {}", user_profile.pages);   // OK
    // But we cannot use `user_profile.author` again or pass `user_profile` to a function expecting a whole `Book`.
    // println!("Author from profile: {}", user_profile.author); // COMPILE-TIME ERROR!
    // fn process_book(_b: Book) {}
    // process_book(user_profile); // COMPILE-TIME ERROR!

    // Struct update syntax and moves:
    let book3 = Book {
        title: String::from("Another Book"),
        pages: 300,
        ..book2 // `book2.author` (String) is MOVED here. `book2.title` would also be moved if not specified.
                // Since `title` and `pages` are specified for `book3`, only unspecified fields
                // from `book2` are used. If `author` was already specified, this would be fine.
                // If `book2` was `Copy`, its fields would be copied.
    };
    println!("Book 3 (from book2 parts): {:?}", book3);
    // println!("Book 2 author after update syntax: {}", book2.author); // COMPILE-TIME ERROR! `book2.author` was moved.
}

// --- 12. Methods, `self`, and Borrowing ---
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function (like a static method), doesn't take `self`.
    // Often used as constructors.
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height } // Field init shorthand
    }

    // Method that takes an immutable borrow of `self` (`&self`).
    // `self` refers to the instance of `Rectangle` the method is called on.
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method that takes a mutable borrow of `self` (`&mut self`).
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }

    // Method that takes ownership of `self` (`self`).
    // After this method is called, the instance is consumed (moved).
    fn describe_and_consume(self) -> String {
        format!("Rectangle: {}x{} (consumed)", self.width, self.height)
    }
}

fn methods_self_and_borrowing() {
    let mut rect1 = Rectangle::new(10, 20);
    println!("Rect1: {:?}", rect1);

    // Calling `area` which takes `&self`. `rect1` is immutably borrowed.
    println!("Area of rect1: {}", rect1.area());
    println!("Rect1 still valid: {:?}", rect1); // `rect1` is fine.

    // Calling `scale` which takes `&mut self`. `rect1` is mutably borrowed.
    rect1.scale(2);
    println!("Rect1 after scaling: {:?}", rect1);

    // Calling `describe_and_consume` which takes `self`. `rect1` is MOVED.
    let description = rect1.describe_and_consume();
    println!("{}", description);
    // println!("Rect1 after consume: {:?}", rect1); // COMPILE-TIME ERROR! `rect1` was moved.
}

// --- 13. Enums and Ownership ---
#[derive(Debug)]
enum Message {
    Quit, // No data associated
    Move { x: i32, y: i32 }, // Struct-like variant with named fields
    Write(String), // Tuple-like variant with one owned String
    ChangeColor(i32, i32, i32), // Tuple-like variant with Copy types
}

impl Message {
    fn process(self) { // This method takes ownership of the Message enum instance.
        match self { // `self` is moved into the match.
            Message::Quit => println!("Quit message received."),
            Message::Move { x, y } => { // `x` and `y` are i32 (Copy), so they are copied.
                println!("Move to x: {}, y: {}", x, y);
            }
            Message::Write(text) => { // `text` is a String. Ownership of the String is MOVED here.
                println!("Text message: {}", text);
            } // `text` (the String) is dropped here if it was moved.
            Message::ChangeColor(r, g, b) => { // r, g, b are i32 (Copy), so copied.
                println!("Change color to R:{}, G:{}, B:{}", r, g, b);
            }
        }
    }

    fn process_by_ref(&self) { // Borrows the Message.
        match self {
            Message::Quit => println!("Ref: Quit message."),
            Message::Move { x, y } => { // x and y are &i32 (references to i32s). Dereference for value.
                println!("Ref: Move to x: {}, y: {}", *x, *y);
            }
            Message::Write(text_ref) => { // `text_ref` is an `&String`.
                println!("Ref: Text message: {}", text_ref);
                // text_ref.push_str("no"); // Error: cannot mutate through &String
            }
            Message::ChangeColor(r, g, b) => { // r,g,b are &i32.
                println!("Ref: Change color to R:{}, G:{}, B:{}", *r, *g, *b);
            }
        }
    }
}

fn enums_and_ownership() {
    let msg1 = Message::Write(String::from("Hello from enum"));
    msg1.process(); // `msg1` (and the String it contains) is moved into `process`.
    // msg1.process(); // COMPILE-TIME ERROR! `msg1` already moved.

    let msg2 = Message::Move { x: 10, y: 20 };
    msg2.process_by_ref(); // `msg2` is borrowed.
    msg2.process_by_ref(); // Can call again.
    msg2.process();        // Now `msg2` is moved.

    // Option<T> is an enum: enum Option<T> { Some(T), None }
    let mut opt_string: Option<String> = Some(String::from("optional data"));

    // Taking ownership from Option:
    if let Some(s_val) = opt_string.take() { // `take()` moves the value out, leaving `None`.
        println!("Taken from Option: {}", s_val); // `s_val` now owns the String.
    }
    println!("Option after take: {:?}", opt_string); // Prints `None`.
}

// --- 14. Collections and Ownership (`Vec<T>`, `HashMap<K, V>`) ---
use std::collections::HashMap;

fn collections_and_ownership() {
    // Vector (`Vec<T>`)
    let mut names: Vec<String> = Vec::new();
    let name1 = String::from("Alice");
    let name2 = String::from("Bob");

    names.push(name1); // `name1` (String) is MOVED into the vector.
    names.push(name2); // `name2` (String) is MOVED into the vector.
    // println!("{}", name1); // COMPILE-TIME ERROR!

    println!("Names in vector: {:?}", names);

    // Iterating and borrowing immutably:
    for name_ref in &names { // `name_ref` is an `&String`
        println!("Hello, {}!", name_ref.to_uppercase());
    }

    // Iterating and borrowing mutably:
    for name_mut_ref in &mut names { // `name_mut_ref` is an `&mut String`
        name_mut_ref.push_str(" (VIP)");
    }
    println!("Modified names: {:?}", names);

    // Iterating and taking ownership (consuming the vector):
    // for owned_name in names.into_iter() { // `owned_name` is `String`. `names` is consumed.
    //     println!("Processing owned: {}", owned_name);
    // }
    // println!("{:?}", names); // COMPILE-TIME ERROR if `into_iter()` was used above.

    // HashMap (`HashMap<K, V>`)
    let mut scores: HashMap<String, u32> = HashMap::new();
    let team_blue_name = String::from("Blue Team");
    let team_red_name = String::from("Red Team");

    // Keys and values are MOVED into the HashMap if they are owned types.
    scores.insert(team_blue_name, 10); // `team_blue_name` (String) moved. 10 (u32) copied.
    scores.insert(team_red_name, 50);  // `team_red_name` (String) moved.
    // println!("{}", team_blue_name); // COMPILE-TIME ERROR!

    let blue_score = scores.get("Blue Team"); // `get` returns `Option<&u32>` (borrows the value).
    if let Some(score) = blue_score {
        println!("Blue Team score: {}", score);
    }

    for (team_name_ref, score_ref) in &scores { // Iterating borrows `&String` and `&u32`.
        println!("{}: {}", team_name_ref, score_ref);
    }
}

// --- 15. Error Handling with `Result<T, E>` and Ownership ---
// `Result<T, E>` is an enum: enum Result<T, E> { Ok(T), Err(E) }
// It's used for functions that can succeed (returning `Ok(T)`) or fail (returning `Err(E)`).
// `T` and `E` can be owned types.

fn parse_number_owned(s: &str) -> Result<i32, String> {
    match s.parse::<i32>() {
        Ok(num) => Ok(num), // `num` (i32) is `Copy`, so it's copied into `Ok`.
        Err(parse_err) => {
            // Create an owned String for the error message.
            let error_message = format!("Failed to parse '{}': {}", s, parse_err);
            Err(error_message) // Ownership of `error_message` (String) is moved into `Err`.
        }
    }
}

fn result_and_ownership() {
    let good_input = "42";
    let bad_input = "not_a_number";

    match parse_number_owned(good_input) {
        Ok(number) => println!("Parsed successfully: {}", number),
        Err(error_string) => { // `error_string` takes ownership of the String from `Err`.
            println!("Error: {}", error_string);
        } // `error_string` is dropped here if it was an Err.
    }

    match parse_number_owned(bad_input) {
        Ok(number) => println!("Parsed successfully: {}", number),
        Err(error_string) => {
            println!("Error: {}", error_string);
        }
    }
}

// --- 16. Closures and Capturing Environment Variables ---
// Closures are anonymous functions that can capture variables from their enclosing scope.
// They can capture by:
// - `FnOnce`: Takes ownership of captured variables (moves or copies). Can only be called once if it consumes them.
// - `FnMut`: Mutably borrows captured variables. Can be called multiple times.
// - `Fn`: Immutably borrows captured variables. Can be called multiple times.
// Rust infers which trait to use based on how the closure uses the variables.
// The `move` keyword forces the closure to take ownership of all captured variables.

fn closures_and_capturing() {
    let an_i32 = 10; // `Copy` type
    let a_string = String::from("hello closure"); // Owned type

    // Immutable borrow: `Fn` trait.
    let print_vars = || {
        println!("i32: {}, String: {}", an_i32, a_string); // `an_i32` copied, `a_string` borrowed.
        // a_string.push_str("x"); // Error: closure may outlive current function, but it borrows `a_string`
    };
    print_vars();
    print_vars(); // Can call multiple times.
    println!("Original string still valid: {}", a_string); // `a_string` is still owned here.

    // Mutable borrow: `FnMut` trait.
    let mut count = 0;
    let mut increment = || {
        count += 1; // Mutably borrows `count`.
        println!("Count: {}", count);
    };
    increment();
    increment();

    // Taking ownership: `FnOnce` trait (forced with `move`).
    let string_to_move = String::from("move me into closure");
    let consuming_closure = move || {
        // `string_to_move` is MOVED into the closure.
        // `an_i32` is COPIED into the closure (because i32 is `Copy`).
        println!("Moved string: {}", string_to_move);
        println!("Copied i32: {}", an_i32);
        // If the closure body actually consumed `string_to_move` (e.g., passed it to a function
        // that takes ownership), then `consuming_closure` could only be called once.
        // Here, it just prints, so it might implement `Fn` or `FnMut` as well if not for `move`.
        // The `move` keyword ensures `string_to_move` is owned, making the closure self-contained.
    };
    consuming_closure();
    // println!("{}", string_to_move); // COMPILE-TIME ERROR! `string_to_move` was moved.
}


// --- 17. Interior Mutability: `Cell<T>` and `RefCell<T>` (A Brief Look) ---
// Sometimes, you need to mutate data even when you only have an immutable reference (`&T`).
// This pattern is called interior mutability. Rust provides types like `Cell<T>` and `RefCell<T>`.
// They bypass the compile-time borrow checker rules and enforce them at *runtime*.
use std::cell::{Cell, RefCell};

struct Observer {
    // `Cell<T>` is for `Copy` types. `get()` returns a copy, `set()` replaces the value.
    // No runtime borrow checking needed because operations are atomic copies/replacements.
    updates_received: Cell<u32>,
    // `RefCell<T>` is for non-`Copy` types.
    // It allows mutable borrows (`borrow_mut()`) or immutable borrows (`borrow()`)
    // dynamically. If borrowing rules are violated at runtime, it will `panic!`.
    last_message: RefCell<String>,
}

impl Observer {
    fn new() -> Self {
        Observer {
            updates_received: Cell::new(0),
            last_message: RefCell::new(String::from("N/A")),
        }
    }

    // This method takes `&self` (immutable borrow).
    fn record_update(&self, message: &str) {
        // We can still mutate `updates_received` because it's a `Cell`.
        let current_updates = self.updates_received.get();
        self.updates_received.set(current_updates + 1);

        // To mutate `last_message` (a `String` inside `RefCell`), we need to `borrow_mut()`.
        // This returns a smart pointer `RefMut<String>`.
        // If another mutable borrow (or active immutable borrows) existed, this would panic.
        let mut last_msg_mut_ref = self.last_message.borrow_mut();
        *last_msg_mut_ref = String::from(message); // Dereference and assign.
    } // `last_msg_mut_ref` (the `RefMut`) is dropped here, releasing the mutable borrow.

    fn print_status(&self) { // Also takes `&self`.
        println!(
            "Updates: {}, Last Message: '{}'",
            self.updates_received.get(),
            self.last_message.borrow() // `borrow()` gets an immutable borrow (`Ref<String>`).
                                       // Panics if a mutable borrow is active.
        );
    }
}

fn interior_mutability_glimpse() {
    let obs = Observer::new();
    obs.print_status();

    obs.record_update("First event happened.");
    obs.print_status();

    obs.record_update("Second event, more important!");
    obs.print_status();

    // Example of RefCell runtime panic (if uncommented and misused):
    // let _borrow1 = obs.last_message.borrow_mut();
    // let _borrow2 = obs.last_message.borrow_mut(); // This would panic!
    // let _borrow_immut = obs.last_message.borrow(); // This would also panic if _borrow1 is active.
}

// --- 18. Shared Ownership: `Rc<T>` and `Arc<T>` (A Brief Look) ---
// When a single value needs multiple owners (e.g., in graph data structures).
// `Rc<T>` (Reference Counting): For single-threaded scenarios. Keeps track of the number of
// references to a value. The value is dropped when the count reaches zero.
// `Arc<T>` (Atomic Reference Counting): For multi-threaded scenarios (thread-safe `Rc<T>`).
use std::rc::Rc;
// For Arc: use std::sync::Arc;

#[derive(Debug)]
struct SharedItem {
    id: u32,
    data: String,
}

fn shared_ownership_glimpse() {
    // Create an `Rc` pointing to a `SharedItem` on the heap.
    // `owner1` is an `Rc<SharedItem>`. The reference count is now 1.
    let owner1 = Rc::new(SharedItem {
        id: 1,
        data: String::from("Shared data"),
    });
    println!("Owner1: {:?}, Count: {}", owner1, Rc::strong_count(&owner1));

    // `Rc::clone(&owner1)` doesn't deep-copy `SharedItem`.
    // It creates a new `Rc` pointer to the *same* `SharedItem` and increments the reference count.
    let owner2 = Rc::clone(&owner1);
    println!("Owner2: {:?}, Count: {}", owner2, Rc::strong_count(&owner1)); // or Rc::strong_count(&owner2)

    {
        let owner3 = Rc::clone(&owner1);
        println!("Owner3: {:?}, Count: {}", owner3, Rc::strong_count(&owner1));
        // `owner3.data` is immutable by default through `Rc`.
        // To mutate data inside an `Rc` (or `Arc`), you typically combine it with `RefCell` (or `Mutex` for `Arc`).
        // e.g., `Rc<RefCell<SharedItem>>`
    } // `owner3` goes out of scope. Its `Rc` pointer is dropped, decrementing the count.

    println!("After owner3 drops, Count: {}", Rc::strong_count(&owner1));

    // The `SharedItem` data itself is only dropped when the strong reference count becomes 0.
} // `owner2` then `owner1` go out of scope, count becomes 0, `SharedItem` is dropped.
