pub fn run(){
    println!("--- Data Types ---");

    // --- SCALAR TYPES ---
    // scalar types represent a single value
    // Examples include integers, floating-point numbers, and booleans
    // Integer: Signed and unsigned integers of various sizes
    let a: i32 = -10; // 32-bit signed integer
    let b: u8 = 255;  // 8-bit unsigned integer
    let c = 100; // inferred as i32
    println!("Integer values: a = {}, b = {}, c = {}", a, b, c);

    // Floating-point: f32 and f64
    let f1 = 3.1415; // inferred as f64
    let f2: f32 = 2.718; // 32-bit floating point
    println!("Floating-point values: f1 = {}, f2 = {}", f1, f2);

    // Boolean: true or false
    let is_active: bool = true;
    let is_inactive = false; // inferred as bool
    println!("Boolean values: is_active = {}, is_inactive = {}", is_active, is_inactive);

    // Character: represents a single Unicode scalar value
    let letter: char = 'A';
    let emoji = '😊'; // inferred as char
    println!("Character values: letter = {}, emoji = {}", letter, emoji);

    // --- COMPOUND TYPES ---
    // Compound types can group multiple values into one type
    // Examples include tuples, arrays, slices
    // Tuple: A fixed-size collection of values of different types
    // Stored on the stack
    let tup: (i32, f64, char) = (500, 6.4, 'Z');
    let (x, y, z) = tup; // destructuring
    println!("Tuple values: x = {}, y = {}, z = {}", x, y, z);
    println!("Accessing tuple elements directly: tup.0 = {}, tup.1 = {}, tup.2 = {}", tup.0, tup.1, tup.2);

    // Array: A fixed-size collection of values of the same type
    // Stored on the stack
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array length: {}", arr.len());
    println!("arr[2] = {}", arr[2]);

    // Slice: A dynamically-sized view 
    // into a contiguous sequence of elements in an array
    // Slices are references and do not own the data
    // They are stored on the stack
    let slice: &[i32] = &arr[1..4]; // slice of arr from index 1 to 3
    println!("Slice values: {:?}", slice);
    println!("Accessing slice elements directly: slice[0] = {}, slice[1] = {}, slice[2] = {}", slice[0], slice[1], slice[2]);

    // String types: String and &str
    // String: A growable, heap-allocated data structure
    let mut string1 = String::from("Hello");
    println!("Initial string value: {}", string1);
    string1.push_str(", world!");
    println!("Updated string value: {}", string1);

    // &str: A string slice, typically used for string literals
    // Always stored on the stack so they have a fixed size
    let string2: &str = "Hello, Rust!";
    println!("String slice value: {}", string2);
}