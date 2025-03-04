use rusqlite::{params, Connection, Result};

pub fn init_db() -> Result<Connection> {
    let conn = Connection::open("budget_manager.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS budgets (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            limit_amount REAL NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY,
            budget_id INTEGER NOT NULL,
            description TEXT NOT NULL,
            amount REAL NOT NULL,
            date TEXT NOT NULL,
            FOREIGN KEY (budget_id) REFERENCES budgets(id)
        )",
        [],
    )?;
    Ok(conn)
}

pub fn add_budget(conn: &Connection, name: &str, limit_amount: f64) -> Result<()> {
    conn.execute(
        "INSERT INTO budgets (name, limit_amount) VALUES (?1, ?2)",
        params![name, limit_amount],
    )?;
    println!("✅ Budget '{}' added with a limit of ${:.2}", name, limit_amount);
    Ok(())
}
