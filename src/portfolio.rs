use std::sync::{Arc, Mutex};

pub struct ShareholderInfo {
    pub username: String,
    pub money: i32,
    pub invested: bool,
    pub price: i32,
}

pub fn init(
    settings: &crate::settings::Settings,
) -> Result<Arc<Mutex<rusqlite::Connection>>, Box<dyn std::error::Error>> {
    let conn = rusqlite::Connection::open(&settings.game.database_path)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS shareholders (
             username TEXT PRIMARY KEY,
             money INTEGER,
             invested INTEGER,
             price INTEGER)",
        [],
    )?;
    Ok(Arc::new(Mutex::new(conn)))
}

pub fn add_shareholder(
    conn: &Arc<Mutex<rusqlite::Connection>>,
    settings: &crate::settings::Settings,
    username: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut db = conn.lock().unwrap();
    let existing_count: i32 = db.query_row(
        "SELECT COUNT(*) FROM shareholders WHERE username = ?1",
        [username],
        |row| row.get(0),
    )?;
    if existing_count > 0 {
        return Err(format!("@{}: you have already joined.", username).into());
    }
    let tx = db.transaction()?;
    tx.execute(
        "INSERT INTO shareholders (username, money, invested, price) VALUES (?1, ?2, 0, 0)",
        [username, &settings.game.starting_money.to_string()],
    )?;
    tx.commit()?;
    Ok(())
}

pub fn get_shareholder(
    conn: &Arc<Mutex<rusqlite::Connection>>,
    username: &str,
) -> Result<ShareholderInfo, Box<dyn std::error::Error>> {
    let db = conn.lock().unwrap();
    let mut stmt = db
        .prepare("SELECT * FROM shareholders WHERE username = ?1")
        .unwrap();
    let mut rows = stmt.query([username])?;
    if let Some(row) = rows.next()? {
        Ok(ShareholderInfo {
            username: row.get(0)?,
            money: row.get(1)?,
            invested: row.get(2)?,
            price: row.get(3)?,
        })
    } else {
        Err(format!("@{} has not done !join yet.", username).into())
    }
}

pub fn invest(
    conn: &Arc<Mutex<rusqlite::Connection>>,
    username: &str,
    value: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut db = conn.lock().unwrap();
    if db
        .query_row(
            "SELECT * FROM shareholders WHERE username = ?1",
            [username],
            |_| Ok(()),
        )
        .is_err()
    {
        return Err(format!("@{} has not done !join yet.", username).into());
    }
    let (money, invested): (i32, i32) = db.query_row(
        "SELECT money, invested FROM shareholders WHERE username = ?1",
        [username],
        |row| Ok((row.get::<usize, i32>(0)?, row.get::<usize, i32>(1)?)),
    )?;
    if invested != 0 {
        return Err(format!("@{}: you are already invested.", username).into());
    }
    if money < value {
        return Err(format!("@{}: you cannot afford to buy.", username).into());
    }
    let tx = db.transaction()?;
    tx.execute(
        "UPDATE shareholders SET money = money - ?1, invested = 1, price = ?2 WHERE username = ?3",
        [&value.to_string(), &value.to_string(), username],
    )?;
    tx.commit()?;

    Ok(())
}

pub fn sell(
    conn: &Arc<Mutex<rusqlite::Connection>>,
    username: &str,
    value: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut db = conn.lock().unwrap();
    if db
        .query_row(
            "SELECT * FROM shareholders WHERE username = ?1",
            [username],
            |_| Ok(()),
        )
        .is_err()
    {
        return Err(format!("@{} has not done !join yet.", username).into());
    }
    let invested: i32 = db.query_row(
        "SELECT invested FROM shareholders WHERE username = ?1",
        [username],
        |row| Ok(row.get::<usize, i32>(0)?),
    )?;
    if invested == 0 {
        return Err(format!("@{} cannot sell because you are not invested.", username).into());
    }
    let tx = db.transaction()?;
    tx.execute(
        "UPDATE shareholders SET money = money + ?1, invested = 0 WHERE username = ?2",
        [&value.to_string(), username],
    )?;
    tx.commit()?;

    Ok(())
}

pub fn sell_all(
    conn: &Arc<Mutex<rusqlite::Connection>>,
    value: i32,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut db = conn.lock().unwrap();
    let tx = db.transaction()?;
    tx.execute(
        "UPDATE shareholders SET money = money + ?1, invested = 0 WHERE invested = 1",
        [value],
    )?;
    tx.commit()?;
    println!("Sold all at ${}", value);
    Ok(())
}
