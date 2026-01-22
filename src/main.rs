use std::collections::HashMap;

fn main() {
    // ============================================
    // VARIABLES AND MUTABILITY
    // ============================================
    // In Rust, variables are immutable by default.
    // Use 'mut' keyword to make them mutable (changeable).
    let mut name = "John";
    let mut age = 30;
    println!("My first name is: {} and my age is {}", name, age);

    // Reassigning mutable variables
    name = "Alice";
    age = 45;
    println!("My first name is: {} and my age is {}", name, age);

    // ============================================
    // DATA TYPES
    // ============================================
    // Rust has several primitive data types:
    // - i8, i16, i32, i64, i128: signed integers
    // - u8, u16, u32, u64, u128: unsigned integers
    // - f32, f64: floating point numbers
    // - char: single Unicode character
    // - bool: true or false
    // - &str: string slice (immutable)
    // - String: heap-allocated string (mutable)
    let my_num: i8 = 5; // 8-bit signed integer
    let my_double: f32 = 5.99; // 32-bit floating point
    let my_letter = 'D'; // character (single quotes)
    let my_bool = true; // boolean
    let my_text = "Hello"; // string slice (&str)

    println!(
        "{} {} {} {} {}",
        my_num, my_double, my_letter, my_bool, my_text
    );

    // ============================================
    // CONSTANTS
    // ============================================
    // Constants are always immutable and must have type annotations.
    // They are declared with 'const' and are usually named in SCREAMING_SNAKE_CASE.
    const PI: f64 = 3.141592653589793;
    println!("The value of PI is: {}", PI);

    // ============================================
    // ARITHMETIC OPERATORS
    // ============================================
    // Basic arithmetic operations: +, -, *, /, %
    let add = 5 + 3;
    let sub = 10 - 4;
    let mul = 6 * 2;
    let div = 12 / 3;
    let rem = 10 % 3; // Modulo (remainder)

    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}", rem);

    // ============================================
    // COMPOUND ASSIGNMENT OPERATORS
    // ============================================
    // Combine arithmetic with assignment: +=, -=, *=, /=, %=
    let mut x = 10;
    println!("Start: {}", x);

    x += 5; // x = x + 5
    println!("After += 5: {}", x);

    x -= 2; // x = x - 2
    println!("After -= 2: {}", x);

    x *= 2; // x = x * 2
    println!("After *= 2: {}", x);

    x /= 3; // x = x / 3
    println!("After /= 3: {}", x);

    x %= 4; // x = x % 4
    println!("After %= 4: {}", x);

    // ============================================
    // COMPARISON OPERATORS
    // ============================================
    // Compare values and return boolean: ==, !=, <, >, <=, >=
    let a = 5;
    let b = 10;

    println!("5 == 10: {}", a == b); // false
    println!("5 != 10: {}", a != b); // true
    println!("5 < 10: {}", a < b); // true
    println!("5 >= 10: {}", a >= b); // false

    // ============================================
    // LOGICAL OPERATORS
    // ============================================
    // Combine boolean expressions: && (AND), || (OR), ! (NOT)
    let logged_in = true;
    let is_admin = false;

    println!("Is regular user: {}", logged_in && !is_admin); // true AND true = true
    println!("Has any access: {}", logged_in || is_admin); // true OR false = true
    println!("Not logged in: {}", !logged_in); // NOT true = false

    // More boolean examples
    let is_programming_fun: bool = true;
    let is_fish_tasty: bool = false;

    println!("Is Programming Fun? {}", is_programming_fun);
    println!("Is Fish Tasty? {}", is_fish_tasty);

    // ============================================
    // IF/ELSE STATEMENTS
    // ============================================
    // Conditional execution based on boolean expressions
    let age = 14;
    let can_vote = age >= 18;

    println!("Can vote? {}", can_vote);

    // Check if age is even or odd
    if age % 2 == 0 {
        println!("Your an even aged person");
    } else {
        println!("Your an odd aged person");
    }

    // Voting eligibility check
    let age = 16;

    if age >= 18 {
        println!("You can vote.");
    } else {
        println!("You are too young to vote.");
    }

    // ============================================
    // IF/ELSE IF/ELSE CHAINS
    // ============================================
    // Multiple conditions checked in order
    let score = 85;

    if score >= 90 {
        println!("Grade: A");
    } else if score >= 80 {
        println!("Grade: B");
    } else if score >= 70 {
        println!("Grade: C");
    } else {
        println!("Grade: F");
    }

    // ============================================
    // IF AS EXPRESSION
    // ============================================
    // In Rust, if can be used as an expression that returns a value.
    // Both branches must return the same type.
    let time = 20;
    let greeting = if time < 18 {
        "Good day."
    } else {
        "Good evening."
    };
    println!("{}", greeting);

    // ============================================
    // MATCH EXPRESSIONS
    // ============================================
    // Match is like switch in other languages but more powerful.
    // It must be exhaustive (cover all possible values).
    // The _ pattern is a catch-all for any value not matched.
    let day = 4;

    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }

    // Match with multiple patterns using | (OR)
    match day {
        1 | 2 | 3 | 4 | 5 => println!("Weekday"),
        6 | 7 => println!("Weekend"),
        _ => println!("Invalid day"),
    }

    // ============================================
    // LOOP - INFINITE LOOP
    // ============================================
    // 'loop' creates an infinite loop that can be broken with 'break'.
    // 'break' can return a value from the loop.
    let mut loop_var = 1;
    let run = loop {
        println!("bangladesh {} times", loop_var);
        if loop_var == 5 {
            break loop_var; // Break and return the value of loop_var
        }
        loop_var += 1;
    };

    println!("Loop run value: {}", run);

    // ============================================
    // WHILE LOOP
    // ============================================
    // 'while' loop continues as long as the condition is true.
    // 'continue' skips the rest of the current iteration.
    let mut count = 1;
    while count <= 10 {
        if count == 3 {
            count += 1;
            continue; // Skip printing 3
        }

        println!("Bangladesh {} times", count);
        count += 1;
    }

    // ============================================
    // FOR LOOP
    // ============================================
    // 'for' loop iterates over a range or collection.
    // 1..=6 means from 1 to 6 inclusive (1, 2, 3, 4, 5, 6)
    // 1..6 means from 1 to 5 exclusive (1, 2, 3, 4, 5)
    for i in 1..=6 {
        println!("Banguradesh {} times", i);
    }

    // For loop with continue and break
    for i in 1..=10 {
        if i == 3 {
            continue; // skip 3
        }
        if i == 5 {
            break; // stop before printing 5
        }
        println!("Bangladesh is in for loop where i is: {}", i);
    }

    // ============================================
    // FUNCTIONS
    // ============================================
    // Functions are defined with 'fn' keyword.
    // They can take parameters and return values.
    println!("hello form {}", greet());

    // ============================================
    // VARIABLE SHADOWING
    // ============================================
    // You can declare a new variable with the same name as a previous one.
    // The new variable shadows the old one within its scope.
    let x = 5;

    {
        let x = 10; // This x shadows the outer x
        println!("Inside block: {}", x);
    }

    println!("Outside block: {}", x); // Still 5

    // ============================================
    // STRING OPERATIONS
    // ============================================
    // There are two main string types in Rust:
    // - &str: string slice (immutable, fixed size)
    // - String: heap-allocated string (mutable, growable)

    // Creating Strings
    let greeting = "hello, world!".to_string(); // Convert &str to String
    let greeting2 = String::from("hello, world!"); // Create String from &str
    println!("Greeting 1: {} \nGreeting 2: {}", greeting, greeting2);

    // Modifying Strings
    let mut greet_3 = String::from("Whats up");
    greet_3.push_str(", amigos"); // Append a string slice
    greet_3.push('!'); // Append a single character

    println!("Greet 3: {}", greet_3);

    // String Formatting
    let s1 = String::from("Hello");
    let s2 = String::from("World!");
    let s3 = String::from("What a beautiful day!");

    let result = format!("{} and {} {}", s1, s2, s3);
    println!("Formatted String: {}", result);

    let name = String::from("John");
    println!("Length: {}", name.len()); // 4

    // ============================================
    // OWNERSHIP AND BORROWING
    // ============================================
    // Rust's ownership system ensures memory safety without a garbage collector.
    // When you assign a String to another variable, ownership is moved.
    // The original variable can no longer be used after the move.
    let a = String::from("Hello");
    let b = &a; // Borrowing: b references a without taking ownership

    println!("{}", a); // Ok: a still owns the value because we only borrowed it
    println!("{}", b); // Ok: b is a reference to a

    // ============================================
    // HASHMAP - KEY-VALUE PAIRS
    // ============================================
    // HashMap stores data as key-value pairs for fast lookup.
    // Use insert() to add entries and [] to access values by key.
    let mut capital_cities = HashMap::new();
    capital_cities.insert("Bangladesh", "Dhaka");
    capital_cities.insert("Japan", "Tokyo");

    print!("Capital of Bangladesh: {}\n", capital_cities["Bangladesh"]);
    print!("Capital of Japan: {}\n", capital_cities["Japan"]);

    // ============================================
    // VEC - DYNAMIC ARRAY
    // ============================================
    // Vec<T> is a growable array type that stores elements of the same type.
    // push() adds elements to the end of the vector.
    let mut fruits = vec!["Apple", "Banana", "Mango"];
    fruits.push("Orange");
    println!("Fruits: {:?}", fruits);

    // ============================================
    // TUPLES - FIXED-SIZE COLLECTIONS
    // ============================================
    // Tuples can hold values of different types.
    // They have a fixed size and cannot grow or shrink.
    let person = ("John", 30, true);
    println!("Person: {:?}", person);

    // ============================================
    // VECTOR OPERATIONS
    // ============================================
    // pop() removes and returns the last element
    // insert(index, value) inserts a value at the specified index
    // remove(index) removes and returns the element at the specified index
    let mut fruits = vec!["apple", "banana", "cherry"];
    fruits.pop(); // Remove "cherry"
    println!("{:?}", fruits);
    fruits.insert(0, "mangustine"); // Insert at index 0
    println!("{:?}", fruits);
    fruits.remove(0); // Remove element at index 0
    println!("{:?}", fruits);

    // ============================================
    // TUPLE DESTRUCTURING
    // ============================================
    // You can unpack tuple values into separate variables using pattern matching.
    let person = ("Jenny", 45, false);
    let (name, age, active) = person;

    println!("Name: {}", name);
    println!("Age: {}", age);
    println!("Active: {}", active);

    // ============================================
    // FUNCTION CALL WITH TUPLE RETURN
    // ============================================
    // Functions can return tuples to return multiple values.
    let user = get_user();
    println!("User: {} ({} years old)", user.0, user.1);

    // ============================================
    // STRUCTS - CUSTOM DATA TYPES
    // ============================================
    // Structs let you create custom data types with named fields.
    // They group related data together.
    struct Person {
        name: String,
        age: u32,
        can_vote: bool,
    }
    let mut user = Person {
        name: String::from("John"),
        age: 35,
        can_vote: true,
    };
    user.age = 40; // Modify struct field (requires mut)
    println!("Name: {}", user.name);
    println!("Age: {}", user.age);
    println!("Can vote? {}", user.can_vote);

    // ============================================
    // ENUMS - CUSTOM TYPES WITH VARIANTS
    // ============================================
    // Enums let you define a type with specific variants.
    // Variants can hold associated data of different types.
    enum LoginStatus {
        Success(String), // Variant with a String payload
        Error(String),   // Variant with a String payload
    }

    let result1 = LoginStatus::Success(String::from("Welcome, John!"));
    let result2 = LoginStatus::Error(String::from("Incorrect password"));

    // Pattern matching on enum variants
    match result1 {
        LoginStatus::Success(message) => println!("Success: {}", message),
        LoginStatus::Error(message) => println!("Error: {}", message),
    }
    match result2 {
        LoginStatus::Success(message) => println!("Success: {}", message),
        LoginStatus::Error(message) => println!("Error: {}", message),
    }
}

fn get_user() -> (String, i8) {
    (String::from("Alice"), 28)
}

fn greet() -> String {
    return "John".to_string();
}
