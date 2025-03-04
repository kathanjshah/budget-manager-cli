use std::io;

pub fn get_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

pub fn get_budget_details() -> (String, f64) {
    let name = get_user_input("Enter budget name:");
    let limit_amount: f64 = get_user_input("Enter budget limit: ")
        .parse()
        .expect("Please enter a valid number!");
    (name, limit_amount)
}
