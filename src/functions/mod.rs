pub fn run(){
    println!("--- Functions ---");

    // Basic  function definition and call
    fn greet() {
        println!("Hello from greet() function!");
    }
    greet();

    // Function with parameters 
    fn add_numbers(a: i32, b: i32)  {
        println!("The sum of {} and {} is {}", a, b, a + b);
    }
    add_numbers(5, 10);

    // Function with return value
    fn multiply_numbers(a: i32, b: i32) -> i32 {
        a * b // last expression without semicolon is the return value
        // You can explicitly use 'return a * b;' if preferred
    }
    let result: i32 = multiply_numbers(4, 6);
    println!("Returned product of 4 and 6 is {}", result);

    // Recursive function
    fn factorial(n: u32) -> u32 {
        // Base case 
        if n == 0 {
            1
        } else {
        // it will be 5 * factorial(4), then 4 * factorial(3) and so on
            n * factorial(n - 1)
        }
    }
    let fact_result = factorial(5);
    println!("Factorial of 5 is {}", fact_result);

   // Returning Option<T> for safe handling
   // option is an enum that can be either Some(value) if operation is successful or None if it fails
   fn get_sqrt(number: f64) -> Option<f64> {
    if number < 0.0 {
        None // Return None for negative numbers
    } else {
        // some is used to wrap the value in an Option
        Some((number as f64).sqrt()) 
    }
   }
   let number = 9.0;
   let sqrt_result = get_sqrt(number);
   println!("Square root of {} is {:?}", number, sqrt_result);

}