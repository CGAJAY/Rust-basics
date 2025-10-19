pub fn run() {
    println!("--- Variables & Constants ---");

    // Rust variables are immutable by default
    println!("Rust variables are immutable by default.");
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 10; // This will cause a compile-time error

    // To make a variable mutable, use the 'mut' keyword
println!("To make a variable mutable, use the 'mut' keyword.");
    let mut y = 10;
    println!("The initial value of y is: {}", y);
    y = 20;  // Allowed because "y" is mutable
    println!("The updated value of y is: {}", y);

    // Shadowing allows you to declare a new variable with the same name
    println!("Shadowing allows you to declare a new variable with the same name.");
    let z = 15;
    println!("The initial value of z is: {}", z);
    let z = z + 5;  // This shadows the previous "z"
    println!("The shadowed value of z is: {}", z);
    let z = "Shadowed again"; // Shadows "z" with a different type
    println!("The shadowed value of z is now: {}", z);

    // Constants are immutable and must be annotated with a type
    // Are evaluated at compile time
    println!("--- Constants ---");
    const PI: f64 = 3.14159;
    println!("The value of PI is: {}", PI);
}