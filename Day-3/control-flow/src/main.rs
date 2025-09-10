use std::io;

fn main() {
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let num: i32 = input.trim().parse().expect("Please enter a valid number");

    if num % 2 == 0 {
        println!("{} is even.", num);
    } else {
        println!("{} is odd.", num);
    }

    println!("Printing numbers from 1 to 5:");
    for i in 1..=5 {
        println!("{}", i);
    }

    let day = "Monday";
    match day {
        "Monday" => println!("Start of the week!"),
        "Friday" => println!("Weekend is coming!"),
        "Saturday" | "Sunday" => println!("It's the weekend!"),
        _ => println!("Midweek day."),
    }
}