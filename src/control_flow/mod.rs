pub fn run(){
	println!("---  Control Flow in Rust ---");

    // if-else statement
    let age: u32 = 20;
    if age < 18 {
        println!("You are a minor.");
    } else if age < 21 {
        println!("You are an adult, but not old enough to drink in the US.");
    } else {
        println!("You are an adult.");
    }

    // Using if as an expression that returns a value
    let is_even:bool = if age % 2 ==0 {true} else {false};
    println!("Is age even? {}", is_even);

    println!("--- MATCH STATEMENT ---");
    // match statement is similar to switch-case in other languages
    let grade:char = 'B';
    match grade {
        'A' => println!("Excellent!"),
        'B' => println!("Good job!"),
        'C' => println!("You passed."),
        'D' => println!("You need to work harder."),
        'F' => println!("Failed. Better luck next time."),
        _   => println!("Invalid grade."),
    }
    // match can also return values
    let feedback: &str = match grade {
        'A' => "Excellent!",
        'B' => "Good job!",
        'C' => "You passed.",
        'D' => "You need to work harder.",
        'F' => "Failed. Better luck next time.",
        _   => "Invalid grade.",
    };
    println!("Feedback: {}", feedback);
    // range in match
    // a..b means From a up to but not including b
    // a..=b means From a up to and including b
    let score:u32 = 85;
    let performance: &str = match score {
        90..=100 => "Outstanding",
        75..=89  => "Very Good",
        50..=74  => "Good",
        0..=49   => "Needs Improvement",
        _        => "Invalid score",
    };
    println!("Performance: {}", performance);

    println!("--- LOOPS IN RUST ---");
    // loop returns a value using break
    let mut count: u32 = 0;
    let result: u32 = loop {
        count += 1;
        if count == 5 {
            break count * 2; // breaks the loop and returns count * 2
        }
        println!("Count is: {}", count);
    };
    println!("Result from loop is: {}", result);
    // while loop
    println!("--- WHILE LOOP ---");
    let mut number: u32 = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;    
    }
    // for loop with array iteration
    println!("--- FOR LOOP ---");
    let fruits: [&str; 3] = ["Apple", "Banana", "Cherry"];
    for fruit in fruits.iter() {
        println!("Fruit: {}", fruit);   
    }
    // for loop with range
    println!("--- FOR LOOP WITH RANGE ---");
    for i in 1..=5 { // 1 to 5 inclusive
        println!("Number: {}", i);  
    }
}
