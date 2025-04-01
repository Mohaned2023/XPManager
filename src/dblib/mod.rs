use crate::loglib;
use crate::utilities;
use rusqlite::params;
use rusqlite::Connection;

pub fn save_password(name: String, password: String) {
    let logger = loglib::Logger::new("save-password");
    let mut password_manager_db_path = utilities::get_password_manager_db_path();
    if utilities::is_encrypted(&password_manager_db_path) {
        // TODO: decrypt the db.
        // TODO: encrypt the db.
        // TODO: secure delete the decrypt db.
    }

    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        let _ = conn.execute("CREATE TABLE IF NOT EXISTS passwords(
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            password TEXT NOT NULL
        )", []);
        let _ = conn.execute("INSERT INTO passwords (name, password) VALUES (?1, ?2)", params![name.clone(), password]);
        logger.info(
            &format!("password saved successfully wiht name '{}'", name)
        );
    } else {
        logger.error(
            &format!("can NOT create connection with '{}'", password_manager_db_path.display())
        );
    }
}