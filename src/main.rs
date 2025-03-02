fn main() {
    println!("Welcome to the retirement calculator!");

    let retirement_age = get_user_input_number("What age do you want to retire?", "You plan to retire at age: ");
    let current_age = get_user_input_number("What is your current age?", "Your current age is: ");
    let current_balance = get_user_input_number("What is your current retirement account balance?", "Your current balance is: ");
    let annual_contribution = get_user_input_number("What is your annual contribution?", "Your annual contribution is: ");
    let annual_interest_rate = get_user_input_number("What is your annual interest rate?", "Your annual interest rate is: ");

    if(current_age < 0 || retirement_age < 0 || current_balance < 0 || annual_contribution < 0 || annual_interest_rate < 0) {
        println!("Please enter positive numbers.");
        return;
    }

    let years_to_retirement = retirement_age - current_age;
    let future_value = calculate_value_over_time(current_balance, annual_contribution, annual_interest_rate, years_to_retirement);

    println!("You will have ${} in your retirement account when you retire at age {}.", future_value, retirement_age);

}

fn calculate_value_over_time(current_balance: i32, annual_contribution: i32, annual_interest_rate: i32, years: i32) -> i32 {
    let mut balance = current_balance;
    for _ in 0..years {
        balance = balance + annual_contribution + (balance * annual_interest_rate / 100);
    }
    balance
}




fn get_user_input_number(question: &str, summary: &str) -> i32 {
    use std::io::{stdin,stdout,Write};
    let mut s=String::new();
    print!("{}", question);
    let _=stdout().flush();
    stdin().read_line(&mut s).expect("Did not enter a correct string");
    if let Some('\n')=s.chars().next_back() {
        s.pop();
    }
    if let Some('\r')=s.chars().next_back() {
        s.pop();
    }
    if let Ok(n) = s.parse::<i32>() {
        println!("{}{}", summary, n);
        n
    } else {
        println!("{} is not a number", s);
        -1
    }

}