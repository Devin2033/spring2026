// Assignment #1
const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 { 
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 { 
    c * 9.0 / 5.0 + FREEZING_POINT_F
}
 

// Assignment #2
fn is_even(n: i32) -> bool {
    n % 2 == 0
}


// Assignment #3
fn check_guess(guess: i32, secret: i32) -> i32 {
    if guess == secret {
        0
    } else if guess > secret {
        1
    } else {
        -1
    }
}

fn main() {
    // Assignment #1
    println!("---Assignment #1---");

    let mut temp_f = 29.0;

    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{:.1}°F = {:.2}°C", temp_f, temp_c);
    
    for i in 1..=5 {
        temp_f = FREEZING_POINT_F + i as f64;
        let celsius = fahrenheit_to_celsius(temp_f);
        println!("{:.1}°F = {:.2}°C", temp_f, celsius);
    }

    println!("\nReverse conversion");
    let celsius_temp = 29.0;
    let fahrenheit_temp = celsius_to_fahrenheit(celsius_temp);
    println!("{:.1}°C = {:.2}°F", celsius_temp, fahrenheit_temp);


    // Assignment #2
    println!();
    println!("---Assignment #2---");
    let numbers: [i32; 10] = [20, 56, 97, 61, 29, 74, 36, 16, 25, 15];

    for &num in numbers.iter() {
        if num % 3 == 0 && num % 5 == 0 {
            println!("{}: FizzBuzz", num);
        }
        else if num % 3 == 0 {
            println!("{}: Fizz", num);
        }
        else if num % 5 == 0 {
            println!("{}: Buzz", num);
        }
        else {
            if is_even(num) {
                println!("{}: Even", num);
            } else {
                println!("{}: Odd", num);
            }
        }
    }

    // While loop to find the sum of all numbers
    println!();
    let mut index = 0;
    let mut sum = 0;
    
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }

    println!("Sum of all numbers: {}", sum);


    // Loop to find the largest number
    let mut max = numbers[0];
    let mut i = 1;
    
    loop {
        if i >= numbers.len() {
            break;
        }
        
        if numbers[i] > max {
            max = numbers[i];
        }
        
        i += 1;
    }
    
    println!("Largest number in the array: {}", max);


    //Assigment #3
    println!();
    println!("---Assignment #3---");

    let secret_number = 26;
    let mut guess_count = 0;
    
    let simulated_guesses = [50, 30, 10, 45, 8, 26];
    let mut guess_index = 0;
    
    loop {
        let guess = simulated_guesses[guess_index];
        guess_index += 1;
        guess_count += 1;
        
        println!("Guess #{}: {}", guess_count, guess);
        
        let result = check_guess(guess, secret_number);
        
        if result == 0 {
            println!("You got it!");
            break;
        } else if result == 1 {
            println!("Too high!\n");
        } else {
            println!("Too low!\n");
        }
    }
    
    println!();
    println!("You found the secret number {} in {} guesses", secret_number, guess_count);
}