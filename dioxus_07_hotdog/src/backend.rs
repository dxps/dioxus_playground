use dioxus::prelude::*;

/////////////////////
// the server side //
/////////////////////

#[cfg(feature = "server")]
thread_local! {
    pub static DB: rusqlite::Connection = {
        // Open the database from the persisted "hotdog.db" file.
        let conn = rusqlite::Connection::open("hotdog.db").expect("Failed to open database");

        // Create the "dogs" table if it doesn't already exist.
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS dogs (
                id INTEGER PRIMARY KEY,
                url TEXT NOT NULL
            );",
        ).unwrap();

        conn
    };
}

// Query the database and return the last 10 dogs and their url.
#[server(endpoint = "list_dogs")]
#[get("/api/list_dogs")]
pub async fn sf_list_dogs() -> Result<Vec<(usize, String)>, ServerFnError> {
    let dogs = DB.with(|f| {
        f.prepare("SELECT id, url FROM dogs ORDER BY id DESC LIMIT 10")
            .unwrap()
            .query_map([], |row| Ok((row.get(0)?, row.get(1)?)))
            .unwrap()
            .map(|r| r.unwrap())
            .collect()
    });

    Ok(dogs)
}

#[server(endpoint = "fav_dog")]
#[post("/api/fav_dog")]
pub async fn sf_fav_dog(image: String) -> Result<()> {
    DB.with(|f| f.execute("INSERT INTO dogs (url) VALUES (?)", &[&image]))?;

    Ok(())
}

#[server(endpoint = "unfav_dog")]
#[post("/api/unfav_dog")]
pub async fn sf_unfav_dog(id: usize) -> Result<()> {
    DB.with(|f| f.execute("DELETE FROM dogs WHERE id=?", &[&id]))?;
    Ok(())
}
