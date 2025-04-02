use crate::loglib;
use rusqlite::params;
use rusqlite::Connection;
use std::path::PathBuf;
use crate::errorlib;

pub struct PasswordInfoForm {
    pub id: i32,
    pub name: String,
    pub password: String
}

pub fn create_passwords_table(password_manager_db_path: PathBuf) {
    let logger = loglib::Logger::new("create-passwords-table");
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        let _ = conn.execute("CREATE TABLE IF NOT EXISTS passwords(
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            password TEXT NOT NULL
        )", []);
        logger.info("passwords table created successfully.");
    } else {
        logger.error(
            &format!("can NOT create connection with '{}'", password_manager_db_path.display()),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}

pub fn save_password(password_manager_db_path: PathBuf, name: String, password: String) {
    let logger = loglib::Logger::new("save-password");
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        let _ = conn.execute("INSERT INTO passwords (name, password) VALUES (?1, ?2)", params![name.clone(), password]);
        logger.info(
            &format!("'{}' saved successfully.", name)
        );
    } else {
        logger.error(
            &format!("can NOT create connection with '{}'", password_manager_db_path.display()),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}

pub fn find_password(password_manager_db_path: PathBuf, string: String) -> Vec<PasswordInfoForm> {
    let logger = loglib::Logger::new("dblib");
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        if let Ok(mut stmt) = conn.prepare("SELECT * FROM passwords WHERE name LIKE ?1") {
            let pattern = format!("%{}%", string);
            let password = stmt.query_map(params![pattern], |row| {
                Ok(
                    PasswordInfoForm {
                        id: row.get::<_, i32>(0).unwrap(),
                        name: row.get::<_, String>(1).unwrap(),
                        password: row.get::<_, String>(2).unwrap(), 
                    }
                )
            })
                .unwrap()
                .collect::<Result<Vec<_>, _>>();
            return password.unwrap();
        }
        logger.error("can NOT create the query!", errorlib::ExitErrorCode::NoDataAvilable);
    }
    logger.error(
        &format!("can NOT create connection with '{}'", password_manager_db_path.display()),
        errorlib::ExitErrorCode::DBConnection
    );
}