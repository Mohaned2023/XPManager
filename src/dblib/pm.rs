use crate::{
    loglib,
    errorlib,
    filelib
};
use super::{
    params,
    Connection,
    PathBuf,
    Tabled,
    log
};

#[derive(Tabled)]
pub struct PasswordInfoForm {
    pub id: i32,
    pub name: String,
    pub password: String,
    pub create_at: String,
    pub update_at: String,
}

pub fn create_passwords_table(password_manager_db_path: PathBuf) {
    let logger = loglib::Logger::new("create-passwords-table");
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        if let Err(_) = conn.execute("
            CREATE TABLE IF NOT EXISTS passwords(
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                password TEXT NOT NULL,
                create_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                update_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
        ", []) {
            // We run this function when the passwords.db first created,
            // So if the execute of creating the password table not ok
            // we must delete the db file to tell xpm 'there is no database'.
            // Disconnect the conn from the file befor you delete it.
            conn.close().unwrap();
            filelib::delete_file(password_manager_db_path);
            logger.error(
                "can NOT create the password table!", 
                errorlib::ExitErrorCode::DBCreateTable
            );
        }
        logger.info("passwords table created successfully.");
        log::register("create passwords table");
    } else {
        logger.error(
            &format!(
                "can NOT create connection with '{}'", 
                password_manager_db_path.display()
            ),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}

pub fn save_password(password_manager_db_path: PathBuf, name: String, password: String) {
    let logger = loglib::Logger::new("save-password");
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        if let Err(_) = conn.execute("
                INSERT INTO passwords 
                (name, password, update_at) VALUES 
                (?1, ?2, CURRENT_TIMESTAMP)
            ", 
            params![name.clone(), password]
        ) {
            logger.error(
                "can NOT save the password!", 
                errorlib::ExitErrorCode::DBInsert
            );
        }
        let log = format!("'{}' saved successfully.", name);
        logger.info(&log );
        log::register(&log);
    } else {
        logger.error(
            &format!(
                "can NOT create connection with '{}'",
                password_manager_db_path.display()
            ),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}

pub fn find_password(password_manager_db_path: PathBuf, string: String) -> Vec<PasswordInfoForm> {
    let logger = loglib::Logger::new("dblib");
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        if let Ok(mut stmt) = conn.prepare("SELECT * FROM passwords WHERE name LIKE ?1") {
            let pattern = format!("%{}%", string);
            let password: Result<Vec<PasswordInfoForm>, _> = stmt.query_map(params![pattern], |row| {
                Ok(
                    PasswordInfoForm {
                        id: row.get::<_, i32>(0).unwrap(),
                        name: row.get::<_, String>(1).unwrap(),
                        password: row.get::<_, String>(2).unwrap(), 
                        create_at: row.get::<_, String>(3).unwrap(),
                        update_at: row.get::<_, String>(4).unwrap()
                    }
                )
            })
                .unwrap()
                .collect();
            return password.unwrap();
        }
    }
    logger.error(
        &format!(
            "can NOT create connection with '{}'", 
            password_manager_db_path.display()
        ),
        errorlib::ExitErrorCode::DBConnection
    );
}

pub fn get_passwords(password_manager_db_path: PathBuf) -> Vec<PasswordInfoForm> {
    find_password(password_manager_db_path, "".to_owned())
}

pub fn update_password(password_manager_db_path: PathBuf, id: String, password: String) -> usize {
    let logger = loglib::Logger::new("update-password");
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        let rows = conn.execute("
                    UPDATE passwords 
                    SET password = ?1,
                    update_at = CURRENT_TIMESTAMP 
                    WHERE id=?2
                ",
                params![password, id]
            ).unwrap_or(0);
            if rows > 0 {
                log::register(
                    &format!("password with id {} updated", id)
                );
            }
            return rows;
    } else {
        logger.error(
            &format!(
                "can NOT create connection with '{}'",
                password_manager_db_path.display()
            ),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}

pub fn update_password_name(password_manager_db_path: PathBuf, id: String, name: String) -> usize {
    let logger = loglib::Logger::new("update-password");
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        let rows= conn.execute("
                    UPDATE passwords 
                    SET name = ?1,
                    update_at = CURRENT_TIMESTAMP 
                    WHERE id=?2
                ",
                params![name, id]
            ).unwrap_or(0);
            if rows > 0 {
                log::register(
                    &format!("password with id {} updated", id)
                );
            }
            return rows;
    } else {
        logger.error(
            &format!(
                "can NOT create connection with '{}'",
                password_manager_db_path.display()
            ),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}

pub fn get_passwords_number(password_manager_db_path: PathBuf) -> usize {
    let logger = loglib::Logger::new("dblib");
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        return conn.query_row(
            "SELECT COUNT(*) FROM passwords", 
            [],
            |row| row.get::<_, usize>(0),
        ).unwrap();
    }
    logger.error(
        &format!(
            "can NOT create connection with '{}'", 
            password_manager_db_path.display()
        ),
        errorlib::ExitErrorCode::DBConnection
    );
}

pub fn delete_password(password_manager_db_path: PathBuf, id: String) -> usize {
    let logger = loglib::Logger::new("dblib");
    if let Ok(conn) = Connection::open(&password_manager_db_path) {
        let rows = conn.execute("
                DELETE FROM passwords 
                WHERE id=?1
            ",
            params![id]
        ).unwrap_or(0);
        log::register(
            &format!("delete password with id {}: rows affected {}", id, rows)
        );
        return rows;
    } else {
        logger.error(
            &format!(
                "can NOT create connection with '{}'", 
                password_manager_db_path.display()
            ),
            errorlib::ExitErrorCode::DBConnection
        );
    }
}