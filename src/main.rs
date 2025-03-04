mod db;
mod cli;

fn main() {
    let conn = db::init_db().expect("Failed to initialize database");

    loop {
        println!("\nBudget Manager CLI");
        println!("1. Add Budget");
        println!("2. View Budgets");
        println!("3. Exit");

        let choice = cli::get_user_input("Enter your choice:");
        match choice.as_str() {
            "1" => {
                let (name, limit_amount) = cli::get_budget_details();
                db::add_budget(&conn, &name, limit_amount).expect("Failed to add budget");
            }
            "2" => println!("Viewing Budgets..."), // Next step
            "3" => break,
            _ => println!("Invalid option!"),
        }
    }
}
