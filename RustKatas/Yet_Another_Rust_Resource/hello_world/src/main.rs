fn main() {
    println!("Ahoy, Matey!");
    {
        let name: &str = "I'm Captain Blackbard!";
        let rank: u32 = 1;
        println!("I'm, {name} Rank {rank}");
    }
    let constant_variable = 10;
    let mut mutated_variable = 20;

    let boot = {
        let constant_variable = 2 * constant_variable;
        let mutable_variable = 30;
        constant_variable + mutable_variable
    };
    println!("total value of boot: {boot}");

    let x: i32 = -26;
    let y: i8 = -4;
    let z: u8 = 60;
    let a = x + (y as i32) + (z as i32);

    println!("The sum of 3 numbers = {a}");

    if boot < 100 {
        println!("We'll get em next time boys");
    } else {
        println!("WE MADE IT LADS");
    }

    let is_crewmember = true;
    let greeting = if is_crewmember {
        "Welcome abroad !"
    } else {
        "Who are you amgous!"
    };
    println!("{} matey !", greeting);

    let n = 20;
    println!("Demo of for loop: {:?}", fizzbuzz_forloop(n));
    println!("Demo of while loop: {:?}", fizzbuzz_whileloop(n));

    let x = fibonacci(10);
    println!("fibonacci = {x}");
}

fn fizzbuzz_whileloop(n: i32) -> () {
    let mut x: i32 = 0;
    while x < n {
        x += 1;
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz")
        } else if x % 5 == 0 {
            println!("Buzz")
        }
        println!("{x}");
    }
}

fn fizzbuzz_forloop(n: i32) -> () {
    println!("FizzBuzz using for loop:");
    for x in 0..=n {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz")
        } else if x % 5 == 0 {
            println!("Buzz")
        }
        println!("{x}");
    }
}

fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
