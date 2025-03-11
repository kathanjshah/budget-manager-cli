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

pub fn get_budgets(conn: &Connection) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, name, limit_amount FROM budgets")?;
    let budget_iter = stmt.query_map([], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?, row.get::<_, f64>(2)?))
    })?;
    println!("\nYour Budgets:");
    for budget in budget_iter {
        let (id, name, limit) = budget?;
        println!("[{}] {} - Limit: ${:.2}", id, name, limit);
    }
    Ok(())
}

pub fn delete_budget(conn: &Connection, budget_id: i32) -> Result<()> {
    conn.execute("DELETE FROM budgets WHERE id = ?1", params![budget_id])?;
    println!("✅ Budget with ID {} deleted.", budget_id);
    Ok(())
}

pub fn update_budget(conn: &Connection, id: i32, name: &str, limit_amount: f64) -> Result<()> {
    conn.execute("UPDATE budgets SET name ?1, limit_amount ?2, id ?3", params![name, limit_amount,id])?;
    println!("✅ Budget {} updated.", id);
    Ok(())
}

pub fn add_transaction(conn: &Connection, budget_id: i32, desc: &str, amount: f64, date: &str) -> Result<()> {
    conn.execute("INSERT INTO transactions (budget_id, description, amount, date) VALUES (?1, ?2, ?3 ?4)", params![budget_id, desc, amount, date])?;
    println!("✅ Transaction '{}' added under Budget {}.", desc, budget_id);
    Ok(())
}

pub fn view_transaction(conn: &Connection, budget_id: i32) -> Result<()> {
    let mut stmt = conn.prepare("SELECT id, description, amount, date FROM transactions WHERE budget_id = ?1")?;
    let trans_iter = stmt.query_map(params![budget_id], |row| {
        Ok((row.get::<_, i32>(0)?, row.get::<_, String>(1)?, row.get::<_, f64>(2)?, row.get::<_, String>(3)? ))
    })?;
    println!("\nTransactions for Budget {}:", budget_id);
    for trans in trans_iter {
        let (id, desc, amount, date) = trans?;
        println!("[{}] {} - ${:.2} on {}", id, desc, amount, date);
    }
    Ok(())
}