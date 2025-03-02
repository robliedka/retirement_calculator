use std::task::ready;
use num_format::{Locale, ToFormattedString};

fn main() {
    println!("Welcome to the retirement calculator!");

    let retirement_age = get_user_input_int("What age do you want to retire?", "You plan to retire at age: ");
    let current_age = get_user_input_int("What is your current age?", "Your current age is: ");
    let current_balance = get_user_input_float("What is your current retirement account balance?", "Your current balance is: ");
    let monthly_contribution = get_user_input_float("What is your month contribution?", "Your month contribution is: ");
    let annual_interest_rate = get_user_input_int("What is your annual interest rate?", "Your annual interest rate is: ");


    let years_to_retirement = retirement_age - current_age;
    let future_value = calculate_value_over_time(current_balance, (monthly_contribution * 12.0), annual_interest_rate, years_to_retirement);

    println!("You will have ${} in your retirement account when you retire at age {}.", future_value.to_formatted_string(&Locale::en), retirement_age);

}

fn calculate_value_over_time(current_balance: f32, annual_contribution: f32, annual_interest_rate: i32, years: i32) -> f32 {
    let mut balance = current_balance;
    for _ in 0..years {
        balance = balance + annual_contribution + (balance * (annual_interest_rate as f32) / 100.0);
    }
    balance
}




fn get_user_input_int(question: &str, summary: &str) -> i32 {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    let acceptable_input = false;
    let mut result = 0;
    while(!acceptable_input) {
        print!("{}", question);
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        if let Ok(n) = s.parse::<i32>() {
            println!("{}{}", summary, n);
            result = n;
        } else {
            println!("{} is not a number", s);
        }
    }
    result
}

fn get_user_input_float(question: &str, summary: &str) -> f32 {
    use std::io::{stdin, stdout, Write};
    let mut s = String::new();
    let acceptable_input = false;
    let mut result = 0.0;
    while (!acceptable_input) {
        print!("{}", question);
        let _ = stdout().flush();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }
        if let Ok(n) = s.parse::<f32>() {
            println!("{}{}", summary, n);
            result = n;
        } else {
            println!("{} is not a number", s);
        }
    }
    result
}